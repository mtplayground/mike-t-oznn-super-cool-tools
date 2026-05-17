use crate::ToolError;

/// Stable filename emitted alongside every packaged tool module.
///
/// The shell loads tools by importing `/tools/<slug>/loader.js`, which must
/// re-export the wasm-bindgen initialization function as the default export and
/// expose named `mount` and `unmount` functions for the runtime host.
pub const TOOL_LOADER_FILE_NAME: &str = "loader.js";

/// Named JS export exposed by the loader shim for mounting a tool.
pub const TOOL_MOUNT_EXPORT_NAME: &str = "mount";

/// Named JS export exposed by the loader shim for unmounting a tool.
pub const TOOL_UNMOUNT_EXPORT_NAME: &str = "unmount";

/// Default JS export exposed by the loader shim for wasm-bindgen initialization.
pub const TOOL_INIT_EXPORT_NAME: &str = "default";

/// Browser-side tool contract implemented by Rust tool crates.
///
/// Build output is wrapped in `/tools/<slug>/loader.js`, which exposes:
/// - `default`: the wasm-bindgen init function
/// - `mount`: a callable that forwards to the registry `entry_symbol`
/// - `unmount`: a callable that forwards to the wasm export named `unmount`
///
/// Tool implementations are therefore expected to provide a mount export
/// declared in `tools-registry.toml` and an `unmount` export for teardown.
pub trait Tool {
    fn mount(&mut self, host_element: web_sys::HtmlElement) -> Result<(), ToolError>;
    fn unmount(&mut self) -> Result<(), ToolError>;
}
