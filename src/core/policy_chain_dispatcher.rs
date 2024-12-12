use super::Policy;
use proxy_wasm::types::Action;
use std::rc::Rc;

pub struct EndRequestOperation {
    pub status: u32,
}

pub enum Operation {
    Await,
    Done,
    Die(EndRequestOperation),
}

#[derive(Default)]
pub struct PolicyChainDispatcher {
    policy_chain: Rc<[Policy]>,
    active_policy_idx: usize,
}

impl PolicyChainDispatcher {
    pub fn load(&mut self, policy_chain: Rc<[Policy]>) {
        self.policy_chain = policy_chain;
    }

    pub fn start<T: proxy_wasm::traits::HttpContext>(&mut self, ctx: &T) -> Operation {
        self.active_policy_idx = 0;

        // Iterate over policies starting from the beginning
        //      run policy.start(callout_register that implements HttpContext)
        //      if error -> early stop iteration; return Die
        //      if callouts register not empty -> early stop iteration; return Await
        //      if callouts register empty -> update active_policy_idx; next iteration
        // When iterator is consumed -> return Done
        todo!();
    }

    pub fn on_http_response_headers<T: proxy_wasm::traits::HttpContext>(&mut self, ctx: &T) {
        for policy in self.policy_chain.iter() {
            policy.on_http_response_headers(ctx);
        }
    }

    pub fn on_grpc_call_response<T: proxy_wasm::traits::HttpContext>(
        &mut self,
        _token_id: u32,
        _status_code: u32,
        _resp_size: usize,
        _ctx: &T,
    ) -> Operation {
        // Iterate over policies starting from active_policy_idx
        //      run policy.on_grpc_call_response(callout_register that implements HttpContext)
        //      if error -> early stop iteration; return Die
        //      if callouts register not empty -> early stop iteration; return Await
        //      if callouts register empty -> update active_policy_idx; next iteration
        // When iterator is consumed -> return Done
        todo!();
    }

    pub fn on_http_call_response<T: proxy_wasm::traits::HttpContext>(
        &mut self,
        _token_id: u32,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
        _ctx: &T,
    ) -> Operation {
        // Iterate over policies starting from the beginning
        //      run policy.on_http_call_response(callout_register that implements HttpContext)
        //      if error -> early stop iteration; return Die
        //      if callouts not empty -> early stop iteration; return Await
        //      if callouts empty -> next iteration
        // When iterator is consumed -> return Done
        todo!();
    }
}
