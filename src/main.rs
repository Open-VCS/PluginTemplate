use openvcs_core::models::Capabilities;
use openvcs_core::plugin_protocol::RpcRequest;
use openvcs_core::plugin_runtime::{
    HandlerResult, PluginCtx, register_delegate, run_registered, set_fallback_rpc,
};
use openvcs_core::plugin_stdio::{ok, ok_null};
use openvcs_core::info;

fn caps(_ctx: &mut PluginCtx, _req: RpcRequest) -> HandlerResult {
    ok(Capabilities::default())
}

fn main() -> std::io::Result<()> {
    // Runs once when OpenVCS starts the plugin.
    info!("Hello, World from PluginTemplate!");

    // Minimal stub RPC so OpenVCS can safely query module metadata without errors.
    register_delegate("caps", caps);
    set_fallback_rpc(|_ctx, _req| ok_null());
    run_registered()
}
