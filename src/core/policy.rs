pub struct Policy {}

impl Policy {
    pub fn on_http_response_headers<T: proxy_wasm::traits::HttpContext>(&self, _ctx: &T) {
        todo!();
    }

    pub fn start<T: proxy_wasm::traits::HttpContext>(&self, _ctx: &T) -> Result<(), String> {
        todo!();
    }
}
