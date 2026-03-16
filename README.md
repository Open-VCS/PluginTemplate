# PluginTemplate: Hello, World!

This folder is a starter template for building OpenVCS plugins.

It includes a single **WASI/Rust library** (`src/lib.rs`) that prints a message when the plugin starts.
The template uses `use openvcs_core::prelude::*;` so plugin code can stay concise.

## Build (`.ovcsp`)

From `PluginTemplate/`, run:

```bash
npx --package @openvcs/sdk openvcs dist --plugin-dir . --out dist
```

This bundles the plugin into `dist/*.ovcsp`.

Preferred install path for repeated use:

```bash
npm install --save-dev @openvcs/sdk
```

Then run:

```bash
npx openvcs dist --plugin-dir . --out dist
```

Notes:
- These plugins build for `wasm32-wasip1`. If needed: `rustup target add wasm32-wasip1`.

## What it does

- When enabled/loaded, it logs one line:
  - `Hello, World from PluginTemplate!`

## Files

- `openvcs.plugin.json`: Plugin manifest (id + module exec).
- `Cargo.toml`, `src/lib.rs`: Rust/WASI library.
- `src/lib.rs` - Rust library using `openvcs_core::prelude::*` plus `#[openvcs_plugin]` and `export_plugin!`

## Customizing for your own plugin

1. Pick a new plugin id (example: `com.yourname.my-plugin`).
2. Update these in sync:
   - `PluginTemplate/openvcs.plugin.json` (`id`, and any file names you change)
   - `PluginTemplate/src/lib.rs` (the startup message and any method names you add)
3. If you rename the bundled module filename, update `PluginTemplate/openvcs.plugin.json` → `module.exec`.
