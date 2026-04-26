# PluginTemplate: Hello, World!

This folder is a starter template for building OpenVCS plugins with the same SDK/runtime architecture used by the Git plugin.

It keeps the behavior simple: when the plugin starts, it logs one line:

- `Hello, World from PluginTemplate!`

## Build

From `PluginTemplate/`, run:

```bash
npm install
npm run build
```

## Validate

```bash
npm run lint
npm test
```

## What it contains

- `package.json`: OpenVCS plugin manifest and npm scripts.
- `tsconfig.json`: TypeScript build settings for the plugin runtime.
- `src/plugin.ts`: plugin definition and startup hook.
- `test/plugin.test.ts`: basic smoke tests for the exported plugin contract.

## Customizing

1. Change the `openvcs.id` in `package.json`.
2. Update the startup log line in `src/plugin.ts`.
3. Add more plugin behavior using the same `PluginDefinition` / `OnPluginStart()` entrypoint pattern.
