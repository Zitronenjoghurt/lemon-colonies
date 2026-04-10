use crate::bindings;
use quad_net::http_request::{Method, RequestBuilder};

#[derive(Default)]
pub struct Http {
    logout_request: Option<quad_net::http_request::Request>,
}

impl Http {
    pub fn update(&mut self) {
        if let Some(req) = &mut self.logout_request
            && let Some(_response) = req.try_recv()
        {
            self.logout_request = None;
            bindings::reload();
        }
    }

    pub fn logout(&mut self) {
        self.logout_request = Some(
            RequestBuilder::new("/api/auth/logout")
                .method(Method::Post)
                .send(),
        );
    }
}
