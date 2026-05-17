#![forbid(unsafe_code)]

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use toolbox_core::{
    RegisteredTool, ToolRegistry, TOOL_INIT_EXPORT_NAME, TOOL_LOADER_FILE_NAME,
    TOOL_MOUNT_EXPORT_NAME, TOOL_UNMOUNT_EXPORT_NAME,
};

type DynError = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, DynError>;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        return Err("expected a subcommand, e.g. `cargo xtask build` or `cargo xtask e2e`".into());
    };

    match command.as_str() {
        "build" => build(),
        "e2e" => e2e(),
        other => Err(format!("unsupported xtask subcommand `{other}`").into()),
    }
}

fn build() -> Result<()> {
    let root = workspace_root()?;
    let registry = ToolRegistry::parse_file(root.join("tools-registry.toml"))?;
    let dist_dir = root.join("dist");
    let tools_dist_dir = dist_dir.join("tools");
    let tools_build_dir = root.join("target").join("xtask").join("tools");

    if tools_dist_dir.exists() {
        fs::remove_dir_all(&tools_dist_dir)?;
    }

    if tools_build_dir.exists() {
        fs::remove_dir_all(&tools_build_dir)?;
    }

    fs::create_dir_all(&dist_dir)?;
    fs::create_dir_all(&tools_dist_dir)?;
    fs::create_dir_all(&tools_build_dir)?;

    let mut trunk_command = command_in_root("trunk", ["build", "--config", "Trunk.toml"], &root);
    trunk_command.env("NO_COLOR", "false");
    run_checked(trunk_command, "trunk build")?;

    for tool in &registry.tools {
        build_tool(&root, &tools_build_dir, &tools_dist_dir, tool)?;
    }

    let registry_json_path = dist_dir.join("registry.json");
    let registry_json = serde_json::to_string_pretty(&registry)?;
    fs::write(registry_json_path, registry_json)?;

    Ok(())
}

fn e2e() -> Result<()> {
    let root = workspace_root()?;

    build()?;
    run_checked(
        command_in_root("npm", ["ci"], &root),
        "npm ci for Playwright smoke tests",
    )?;
    run_checked(
        command_in_root(
            "npx",
            ["playwright", "install", "--with-deps", "chromium"],
            &root,
        ),
        "playwright browser install",
    )?;
    run_checked(
        command_in_root("npx", ["playwright", "test"], &root),
        "playwright smoke tests",
    )?;

    Ok(())
}

fn build_tool(
    root: &Path,
    tools_build_dir: &Path,
    tools_dist_dir: &Path,
    tool: &RegisteredTool,
) -> Result<()> {
    let package_dir = tools_build_dir.join(&tool.meta.slug);
    let final_dir = tools_dist_dir.join(&tool.meta.slug);

    fs::create_dir_all(&package_dir)?;
    fs::create_dir_all(&final_dir)?;

    let wasm_pack = wasm_pack_executable(root);
    run_checked(
        command_in_root(
            &wasm_pack,
            [
                "build",
                tool.crate_path.as_str(),
                "--target",
                "web",
                "--out-dir",
                path_to_str(&package_dir)?,
                "--out-name",
                tool.meta.slug.as_str(),
            ],
            root,
        ),
        &format!("wasm-pack build for `{}`", tool.meta.slug),
    )?;

    copy_dir_contents(&package_dir, &final_dir)?;
    ensure_expected_artifacts_exist(&final_dir, tool)?;
    write_loader_shim(&final_dir, tool)?;

    Ok(())
}

fn ensure_expected_artifacts_exist(final_dir: &Path, tool: &RegisteredTool) -> Result<()> {
    let file_name = tool
        .wasm_url
        .rsplit('/')
        .next()
        .ok_or_else(|| format!("tool `{}` has an invalid wasm_url", tool.meta.slug))?;
    let artifact_path = final_dir.join(file_name);
    let bindings_path = final_dir.join(format!("{}.js", tool.meta.slug));

    if !artifact_path.exists() {
        return Err(format!(
            "tool `{}` expected wasm artifact `{}` to exist after packaging",
            tool.meta.slug,
            artifact_path.display()
        )
        .into());
    }

    if !bindings_path.exists() {
        return Err(format!(
            "tool `{}` expected wasm-bindgen JS bindings `{}` to exist after packaging",
            tool.meta.slug,
            bindings_path.display()
        )
        .into());
    }

    Ok(())
}

fn write_loader_shim(final_dir: &Path, tool: &RegisteredTool) -> Result<()> {
    let loader_path = final_dir.join(TOOL_LOADER_FILE_NAME);
    fs::write(loader_path, render_loader_shim(tool))?;
    Ok(())
}

fn render_loader_shim(tool: &RegisteredTool) -> String {
    let bindings_module_path = format!("./{}.js", tool.meta.slug);

    format!(
        "import init, * as bindings from \"{bindings_module_path}\";\n\n\
export {{ init as {tool_init_export_name} }};\n\n\
export function {tool_mount_export_name}(...args) {{\n\
  const mount = bindings[\"{entry_symbol}\"];\n\
  if (typeof mount !== \"function\") {{\n\
    throw new Error(\"Tool `{slug}` is missing wasm-bindgen export `{entry_symbol}` required by the loader contract.\");\n\
  }}\n\
\n\
  return mount(...args);\n\
}}\n\n\
export function {tool_unmount_export_name}(...args) {{\n\
  const unmount = bindings[\"{tool_unmount_export_name}\"];\n\
  if (typeof unmount !== \"function\") {{\n\
    return undefined;\n\
  }}\n\
\n\
  return unmount(...args);\n\
}}\n",
        bindings_module_path = bindings_module_path,
        tool_init_export_name = TOOL_INIT_EXPORT_NAME,
        tool_mount_export_name = TOOL_MOUNT_EXPORT_NAME,
        tool_unmount_export_name = TOOL_UNMOUNT_EXPORT_NAME,
        slug = tool.meta.slug,
        entry_symbol = tool.entry_symbol,
    )
}

fn wasm_pack_executable(root: &Path) -> String {
    let local_shim = root.join("scripts").join("wasm-pack");

    if local_shim.exists() {
        return local_shim.display().to_string();
    }

    "wasm-pack".to_owned()
}

fn command_in_root<I, S>(program: &str, args: I, root: &Path) -> Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    command.args(args);
    command.current_dir(root);
    command.env("PATH", format!("/usr/local/cargo/bin:{}", env::var("PATH").unwrap_or_default()));
    command
}

fn copy_dir_contents(source: &Path, destination: &Path) -> Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            fs::create_dir_all(&destination_path)?;
            copy_dir_contents(&entry_path, &destination_path)?;
        } else if metadata.is_file() {
            fs::copy(&entry_path, &destination_path)?;
        }
    }

    Ok(())
}

fn workspace_root() -> Result<PathBuf> {
    Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("failed to determine workspace root")?
        .to_path_buf())
}

fn run_checked(mut command: Command, context: &str) -> Result<()> {
    let status = command.status()?;
    ensure_success(status, context)
}

fn ensure_success(status: ExitStatus, context: &str) -> Result<()> {
    if status.success() {
        return Ok(());
    }

    Err(format!("{context} failed with status {status}").into())
}

fn path_to_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "non-utf8 path").into())
}

#[cfg(test)]
mod tests {
    use super::render_loader_shim;
    use toolbox_core::{Category, RegisteredTool, ToolMeta};

    fn sample_tool() -> RegisteredTool {
        RegisteredTool {
            meta: ToolMeta::new(
                "calculator",
                "calculator",
                "Calculator",
                Category::Math,
                ["math", "utility"],
                "A simple calculator tool.",
                "/public/thumbnails/calculator.svg",
            ),
            crate_path: "crates/tools/calculator".to_owned(),
            entry_symbol: "mount".to_owned(),
            wasm_url: "/tools/calculator/calculator_bg.wasm".to_owned(),
        }
    }

    #[test]
    fn renders_loader_shim_with_contract_exports() {
        let shim = render_loader_shim(&sample_tool());

        assert!(shim.contains("import init, * as bindings from \"./calculator.js\";"));
        assert!(shim.contains("export { init as default };"));
        assert!(shim.contains("export function mount(...args) {"));
        assert!(shim.contains("bindings[\"mount\"]"));
        assert!(shim.contains("export function unmount(...args) {"));
    }
}
