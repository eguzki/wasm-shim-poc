use crate::configuration::PluginConfiguration;
use crate::core::{PolicyChainDispatcher, PolicyChainIndex};
use http_context::Filter;
use log::{debug, error, info};
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::ContextType;
use std::rc::Rc;

mod http_context;

pub struct FilterRoot {
    pub context_id: u32,
    pub index: Rc<PolicyChainIndex>,
}

impl RootContext for FilterRoot {
    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        debug!("#{} create_http_context", context_id);
        Some(Box::new(Filter {
            context_id,
            index: Rc::clone(&self.index),
            dispatcher: PolicyChainDispatcher::default(),
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
                let index = match crate::core::PolicyChainIndex::try_from(config) {
                    Ok(index) => index,
                    Err(err) => {
                        error!("failed to compile plugin config: {}", err);
                        return false;
                    }
                };
                self.index = Rc::new(index);
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
