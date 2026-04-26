// Copyright © 2025-2026 OpenVCS Contributors
// SPDX-License-Identifier: GPL-3.0-or-later

import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { OnPluginStart, PluginDefinition } from '../src/plugin.js';

/** Captures console info output while running the plugin startup hook. */
function captureConsoleInfo(action: () => void): string[] {
  const messages: string[] = [];
  const originalInfo = console.info;
  console.info = ((...args: unknown[]) => {
    messages.push(args.map((value) => String(value)).join(' '));
  }) as typeof console.info;

  try {
    action();
  } finally {
    console.info = originalInfo;
  }

  return messages;
}

describe('PluginTemplate', () => {
  it('exports the expected plugin definition', () => {
    assert.strictEqual(PluginDefinition.logTarget, 'openvcs.plugin-template.plugin');
  });

  it('logs the hello-world message on startup', () => {
    const messages = captureConsoleInfo(() => {
      OnPluginStart();
    });

    assert.deepStrictEqual(messages, ['Hello, World from PluginTemplate!']);
  });
});
