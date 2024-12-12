use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

mod configuration;
mod core;
mod root_context;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    std::panic::set_hook(Box::new(|panic_info| {
        proxy_wasm::hostcalls::log(LogLevel::Critical, &panic_info.to_string())
            .expect("failed to log panic_info");
    }));
    proxy_wasm::set_root_context(|context_id| -> Box<dyn RootContext> {
        info!("#{} set_root_context", context_id);
        Box::new(root_context::FilterRoot { context_id, index: Default::default(), })
    });
}}
