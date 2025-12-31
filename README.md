# PluginTemplate: Hello, World!

This folder is a starter template for building OpenVCS plugins.

It includes a single **WASI/Rust backend** (`src/main.rs`) that prints a message when the plugin starts.

## Build (`.ovcsp`)

From `PluginTemplate/`, run:

```bash
cargo openvcs dist
```

This bundles the plugin into `dist/*.ovcsp`.

If `cargo openvcs` is not installed, you can run the SDK packager directly:

```bash
cargo run --manifest-path ../OpenVCS-SDK/Cargo.toml --bin openvcs-plugin -- --plugin-dir PluginTemplate --out PluginTemplate/dist
```

Notes:
- These plugins build for `wasm32-wasip1`. If needed: `rustup target add wasm32-wasip1`.

## What it does

- When enabled/loaded, it emits a `VcsEvent::Info`:
  - `Hello, World from PluginTemplate!`

## Files

- `openvcs.plugin.json`: Plugin manifest (id + backend exec).
- `Cargo.toml`, `src/main.rs`: Rust/WASI backend executable.

## Customizing for your own plugin

1. Pick a new plugin id (example: `com.yourname.my-plugin`).
2. Update these in sync:
  - `PluginTemplate/openvcs.plugin.json` (`id`, and any file names you change)
  - `PluginTemplate/src/main.rs` (the startup message and any method names you add)
3. If you rename the backend executable, update:
  - `PluginTemplate/openvcs.plugin.json` → `backend.exec`
  - `PluginTemplate/Cargo.toml` → `[[bin]].name`
