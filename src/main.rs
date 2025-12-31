use openvcs_core::models::{Capabilities, VcsEvent};
use openvcs_core::plugin_protocol::{PluginMessage, RpcRequest};
use openvcs_core::plugin_runtime::{
    HandlerResult, PluginCtx, register_delegate, run_registered, set_fallback_rpc,
};
use openvcs_core::plugin_stdio::{ok, ok_null, send_message_shared};
use std::io::{self, LineWriter};
use std::sync::{Arc, Mutex};

const HELLO_ON_LOAD: &str = "Hello, World from PluginTemplate!";

fn emit_hello_on_load() {
    let out = Arc::new(Mutex::new(LineWriter::new(io::stdout())));
    send_message_shared(
        &out,
        &PluginMessage::Event {
            event: VcsEvent::Info {
                msg: HELLO_ON_LOAD.to_string(),
            },
        },
    );
}

fn caps(_ctx: &mut PluginCtx, _req: RpcRequest) -> HandlerResult {
    ok(Capabilities::default())
}

fn main() -> std::io::Result<()> {
    // Runs once when OpenVCS starts the plugin.
    emit_hello_on_load();

    // Minimal stub RPC so OpenVCS can safely query the backend list without errors.
    register_delegate("caps", caps);
    set_fallback_rpc(|_ctx, _req| ok_null());
    run_registered()
}
