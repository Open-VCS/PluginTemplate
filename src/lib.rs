//! OpenVCS Plugin Template
//!
//! A minimal plugin that demonstrates the plugin ABI structure.

use openvcs_core::prelude::*;

// Internal helpers - NOT ABI
#[allow(dead_code)]
/// Returns a sample helper string used by the template.
fn helper() -> String {
    "Hello from internal helper!".to_string()
}

// Plugin ABI - functions in mod plugin are exported
#[openvcs_plugin]
mod plugin {
    use super::*;

    /// Initializes the template plugin.
    ///
    /// # Returns
    /// - `Ok(())` when plugin startup succeeds.
    pub fn init() -> Result<(), PluginError> {
        info!("Hello, World!");
        Ok(())
    }

    /// Deinitializes the template plugin.
    ///
    /// # Returns
    /// - `Ok(())` when plugin shutdown succeeds.
    pub fn deinit() -> Result<(), PluginError> {
        Ok(())
    }
}

// Generate WIT Guest impl
openvcs_core::export_plugin!(plugin);
