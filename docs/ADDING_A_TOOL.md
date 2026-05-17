# Adding a New Tool

This workspace treats each tool as its own Rust crate plus one entry in
`tools-registry.toml`. The calculator at `crates/tools/calculator` is the
reference implementation.

## 1. Create the crate

Add a new crate under `crates/tools/<slug>` and register it in the workspace
root `Cargo.toml`.

Minimal `Cargo.toml` shape:

```toml
[package]
name = "my-tool"
edition.workspace = true
license.workspace = true
version.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]
path = "src/lib.rs"

[dependencies]
wasm-bindgen = "0.2.121"
web-sys = { version = "0.3.82", features = ["HtmlElement"] }
```

The important part is `crate-type = ["cdylib", "rlib"]`, which lets the tool
build to WebAssembly for browser loading.

## 2. Implement the loader contract

Each tool crate must export the functions expected by the shell loader:

- `mount(host_element)`
- `unmount()`

The shell packages each tool behind `/tools/<slug>/loader.js`, then calls the
wasm exports through that shim. The calculator shows the expected Rust shape:

```rust
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

#[wasm_bindgen]
pub fn mount(host_element: HtmlElement) -> Result<(), JsValue> {
    host_element.set_inner_html("<p>Hello from my tool</p>");
    Ok(())
}

#[wasm_bindgen]
pub fn unmount() -> Result<(), JsValue> {
    Ok(())
}
```

Use `mount` to render into the provided host element and attach any event
listeners you need. Use `unmount` to remove listeners and clean up any global
state before the shell navigates away.

If you need a more complete example, see
`crates/tools/calculator/src/lib.rs`, which keeps mounted state in a
`thread_local!` and tears it down from `unmount`.

## 3. Add the registry entry

Add one `[[tools]]` block to `tools-registry.toml`:

```toml
[[tools]]
id = "my-tool"
slug = "my-tool"
name = "My Tool"
category = "utilities"
tags = ["utility"]
description = "A short summary shown on cards and search results."
thumbnail = "/public/thumbnails/my-tool.svg"
crate_path = "crates/tools/my-tool"
entry_symbol = "mount"
wasm_url = "/tools/my-tool/my_tool_bg.wasm"
```

Notes:

- `slug` becomes the route at `/tools/<slug>`.
- `crate_path` must point to the crate directory.
- `entry_symbol` should match the wasm export used for mounting. In this repo
  that is `mount`.
- `wasm_url` must match the packaged artifact name that `wasm-bindgen`
  generates. The calculator crate uses the slug `calculator`, so its wasm file
  is `/tools/calculator/calculator_bg.wasm`.
- `thumbnail` should point at a file under `public/` so Trunk copies it into
  `dist/`.

## 4. Build the shell and tool packages

Run the workspace orchestrator:

```bash
cargo xtask build
```

That command:

- runs `trunk build` for the shell
- reads `tools-registry.toml`
- packages each tool crate
- writes tool assets under `dist/tools/<slug>/`
- generates `dist/registry.json`

For the calculator example, the packaged output lands under
`dist/tools/calculator/` and includes:

- `calculator.js`
- `calculator_bg.wasm`
- `loader.js`

## 5. Verify the tool in the browser

After building:

1. Open the index and confirm the new tool card appears.
2. Open `/tools/<slug>` and verify the tool mounts successfully.
3. If the tool participates in end-to-end coverage, rerun:

```bash
cargo xtask e2e
```

The calculator is the current worked example for both packaging and browser
verification.
