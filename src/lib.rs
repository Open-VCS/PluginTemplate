//! OpenVCS Plugin Template
//!
//! A minimal plugin that demonstrates the plugin ABI structure.

use openvcs_core::app_api::PluginError;
use openvcs_core::info;
use openvcs_core::openvcs_plugin;

// Internal helpers - NOT ABI
#[allow(dead_code)]
fn helper() -> String {
    "Hello from internal helper!".to_string()
}

// Plugin ABI - functions in mod plugin are exported
#[openvcs_plugin]
mod plugin {
    use super::*;

    pub fn init() -> Result<(), PluginError> {
        info!("Hello, World!");
        Ok(())
    }

    pub fn deinit() -> Result<(), PluginError> {
        Ok(())
    }
}

// Generate WIT Guest impl
openvcs_core::export_plugin!(plugin);
