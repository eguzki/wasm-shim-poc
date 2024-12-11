use log::{debug, warn};
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::Action;
use std::rc::Rc;

struct HttpContextImpl;

impl HttpContext for HttpContextImpl {}
impl Context for HttpContextImpl {}

pub struct Filter {
    pub context_id: u32,
    pub index: Rc<crate::core::PolicyChainIndex>,
    pub dispatcher: crate::core::PolicyChainDispatcher,
}

impl Filter {
    fn request_authority(&self) -> String {
        match self.get_http_request_header(":authority") {
            None => {
                warn!(":authority header not found");
                String::new()
            }
            Some(host) => {
                let split_host = host.split(':').collect::<Vec<_>>();
                split_host[0].to_owned()
            }
        }
    }
}

impl HttpContext for Filter {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        debug!("#{} on_http_request_headers", self.context_id);

        match self
            .index
            .get_longest_match(self.request_authority().as_str())
        {
            None => {
                debug!(
                    "#{} resume request as not policy chain was found for it",
                    self.context_id
                );
                Action::Continue
            }
            Some(policy_chain) => {
                self.dispatcher.load(policy_chain);
                self.dispatcher.start(&HttpContextImpl)
            }
        }
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        debug!("#{} on_http_response_headers", self.context_id);
        self.dispatcher.on_http_response_headers(&HttpContextImpl);
        Action::Continue
    }

    fn on_log(&mut self) {
        debug!("#{} completed.", self.context_id);
    }
}

impl Context for Filter {
    fn on_grpc_call_response(&mut self, token_id: u32, status_code: u32, resp_size: usize) {
        debug!(
            "#{} on_grpc_call_response: token: {token_id}, status: {status_code}",
            self.context_id
        );
        self.dispatcher
            .on_grpc_call_response(token_id, status_code, resp_size, &HttpContextImpl);
        //  Here three possible actions:
        //  A) nothing to be done, we need to wait for more events (maybe other grpc response or
        //  http responses
        //  B) Send error: self.send_http_response(500, vec![], Some(b"Internal Server Error.\n"))
        //  C) Resume request: self.resume_http_request()
    }

    fn on_http_call_response(
        &mut self,
        token_id: u32,
        num_headers: usize,
        body_size: usize,
        num_trailers: usize,
    ) {
        debug!(
            "#{} on_http_call_response: token: {token_id}, num_headers: {num_headers}, body_size: {body_size}, num_trailers: {num_trailers}",
            self.context_id
        );
        self.dispatcher.on_http_call_response(
            token_id,
            num_headers,
            body_size,
            num_trailers,
            &HttpContextImpl,
        );
        //  Here three possible actions:
        //  A) nothing to be done, we need to wait for more events (maybe other grpc response or
        //  http responses
        //  B) Send error: self.send_http_response(500, vec![], Some(b"Internal Server Error.\n"))
        //  C) Resume request: self.resume_http_request()
    }
}
