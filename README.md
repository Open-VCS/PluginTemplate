# PluginTemplate: Hello, World!

This folder is a starter template for building OpenVCS plugins.

It includes a single **WASI/Rust library** (`src/lib.rs`) that prints a message when the plugin starts.
The template uses `use openvcs_core::prelude::*;` so plugin code can stay concise.

## Build

From `PluginTemplate/`, run:

```bash
npx --package @openvcs/sdk openvcs build --plugin-dir .
```

This builds the plugin runtime assets in place.

Preferred install path for repeated use:

```bash
npm install --save-dev @openvcs/sdk
```

Then run:

```bash
npx openvcs build --plugin-dir .
```

Notes:
- These plugins build for `wasm32-wasip1`. If needed: `rustup target add wasm32-wasip1`.

## What it does

- When enabled/loaded, it logs one line:
  - `Hello, World from PluginTemplate!`

## Files

- `package.json`: npm manifest with `openvcs` plugin metadata.
- `Cargo.toml`, `src/lib.rs`: Rust/WASI library.
- `src/lib.rs` - Rust library using `openvcs_core::prelude::*` plus `#[openvcs_plugin]` and `export_plugin!`

## Customizing for your own plugin

1. Pick a new plugin id (example: `com.yourname.my-plugin`).
2. Update these in sync:
   - `PluginTemplate/package.json` (`openvcs.id`, and any file names you change)
   - `PluginTemplate/src/lib.rs` (the startup message and any method names you add)
3. If you rename the bundled module filename, update `PluginTemplate/package.json` → `openvcs.module.exec`.
