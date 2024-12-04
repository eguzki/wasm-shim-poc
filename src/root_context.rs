use log::{debug, error, info};
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::ContextType;
use std::rc::Rc;

pub struct FilterRoot {
    pub context_id: u32,
    pub core: Rc<Core>,
}

impl RootContext for FilterRoot {
    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        debug!("#{} create_http_context", context_id);
        let header_resolver = Rc::new(HeaderResolver::new());
        Some(Box::new(Filter {
            context_id,
            core: Rc::clone(&self.core),
            response_headers_to_add: Vec::default(),
            operation_dispatcher: OperationDispatcher::new(header_resolver).into(),
        }))
    }

    fn on_configure(&mut self, _config_size: usize) -> bool {
        info!("#{} on_configure", self.context_id);
        let configuration: Vec<u8> = match self.get_plugin_configuration() {
            Some(c) => c,
            None => return false,
        };
        match serde_json::from_slice::<PluginConfiguration>(&configuration) {
            Ok(config) => {
                info!("plugin config parsed: {:?}", config);
                let runtime_config = match <PluginConfiguration as TryInto<Core>>::try_into(config)
                {
                    Ok(cfg) => cfg,
                    Err(err) => {
                        error!("failed to compile plugin config: {}", err);
                        return false;
                    }
                };
                self.config = Rc::new(runtime_config);
            }
            Err(e) => {
                error!("failed to parse plugin config: {}", e);
                return false;
            }
        }
        true
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

impl Context for FilterRoot {}
