use super::PolicyChain;
use proxy_wasm::types::Action;
use std::rc::Rc;

pub struct PolicyChainDispatcher {}

impl PolicyChainDispatcher {
    pub fn load(&mut self, _policy_chain: Rc<PolicyChain>) {
        // read index and return policy chain
        todo!();
    }

    pub fn start<T: proxy_wasm::traits::HttpContext>(&mut self, _ctx: &T) -> Action {
        todo!();
    }

    pub fn on_http_response_headers<T: proxy_wasm::traits::HttpContext>(
        &mut self,
        _ctx: &T,
    ) -> Action {
        todo!();
    }

    pub fn on_grpc_call_response<T: proxy_wasm::traits::HttpContext>(
        &mut self,
        _token_id: u32,
        _status_code: u32,
        _resp_size: usize,
        _ctx: &T,
    ) {
        todo!();
    }

    pub fn on_http_call_response<T: proxy_wasm::traits::HttpContext>(
        &mut self,
        _token_id: u32,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
        _ctx: &T,
    ) {
        todo!();
    }
}
