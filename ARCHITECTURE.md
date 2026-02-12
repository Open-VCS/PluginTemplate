# PluginTemplate Architecture

This document describes the starter plugin scaffold in `PluginTemplate/`.

See umbrella context in `../ARCHITECTURE.md`.

## Bird's Eye View

`PluginTemplate/` is a minimal baseline for creating a new OpenVCS plugin.

It demonstrates:
- manifest layout,
- module entrypoint layout,
- packaging path to `.ovcsp`.

## Code Map

- `openvcs.plugin.json`: plugin manifest baseline.
- `Cargo.toml`: crate and binary configuration.
- `src/main.rs`: starter module logic.
- `dist/`: output location for packaged bundles.

## Boundaries

- Template is a scaffold, not a feature-complete reference implementation.
- Runtime plugin hosting and capability enforcement are owned by `Client/Backend`.
- Shared protocol and trait contracts are owned by `Core/`.

## Architecture Invariants

- Template should stay minimal and easy to clone.
- Renaming plugin id/entrypoint should be explicit and localized to manifest + cargo/bin names.

## Cross-Cutting Concerns

- Onboarding:
  The template should optimize first-time plugin author experience.
- Consistency:
  Template structure should remain compatible with SDK packaging and host runtime expectations.
