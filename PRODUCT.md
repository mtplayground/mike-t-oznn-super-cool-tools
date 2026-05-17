# Product Snapshot

`mike-t-oznn-super-cool-tools` is a Rust workspace for a browser-based tool
shell that loads individual WebAssembly tools at runtime.

## What ships today

- A Leptos + Trunk frontend shell with routes for `/`, `/category/:slug`, and
  `/tools/:slug`
- A runtime tool registry loaded from `dist/registry.json`
- Category browsing, header search, and a recent-tools strip backed by
  `localStorage`
- A dynamic tool host that imports `/tools/<slug>/loader.js`, initializes the
  wasm module, mounts it into the page, and unmounts on navigation
- One registered tool: `calculator`, a four-function calculator with keyboard
  input, accessibility labels, and unit-tested state logic
- A Playwright smoke test run through `cargo xtask e2e`

## Architecture

- `crates/toolbox-core`: shared contracts and registry parsing
- `crates/toolbox-shell`: the browser shell and runtime loader
- `crates/tools/calculator`: the reference tool crate
- `xtask`: build orchestration for the shell, tool packaging, `registry.json`,
  and end-to-end test entrypoints

## Conventions

- Every tool is its own crate under `crates/tools/<slug>`
- Tools are declared in `tools-registry.toml`
- Tools must export the loader contract expected by the shell:
  `mount(host_element)` and `unmount()`
- Packaged assets are emitted under `dist/tools/<slug>/`
- Use `cargo xtask build` for the real frontend/tool build, not raw ad hoc
  packaging steps

## Current reference flow

The calculator is the worked example for adding tools: crate in the workspace,
registry entry, public thumbnail, wasm-packaged output, shell card, tool route,
and e2e coverage.
