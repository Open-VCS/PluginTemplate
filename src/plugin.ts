// Copyright © 2025-2026 OpenVCS Contributors
// SPDX-License-Identifier: GPL-3.0-or-later

import type { PluginModuleDefinition } from '@openvcs/sdk/runtime';

/** Stores the log target used by the template plugin runtime. */
const logTarget = 'openvcs.plugin-template.plugin';

/** Exposes the plugin definition consumed by the OpenVCS SDK runtime. */
export const PluginDefinition: PluginModuleDefinition = {
  logTarget,
};

/** Starts the template plugin and emits the same hello-world message. */
export function OnPluginStart(): void {
  console.info('Hello, World from PluginTemplate!');
}
