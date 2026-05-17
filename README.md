# mike-t-oznn-super-cool-tools

`mike-t-oznn-super-cool-tools` is a Rust workspace for a browser-based tools
shell and individual tool crates.

## Workspace layout

- `crates/toolbox-core`: shared library crate for core types and abstractions
- `crates/toolbox-shell`: application crate for the shell frontend/runtime
- `tools/calculator`: first tool crate reserved for the calculator tool

## Getting started

1. Copy `.env.example` to `.env` and set values for your environment.
2. Build the workspace:

```bash
cargo build
```
