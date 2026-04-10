use crate::bindings;
use lemon_colonies_core::types::user_info::PrivateUserInfo;
use quad_net::http_request::{Method, Request, RequestBuilder};
use serde::de::DeserializeOwned;

#[derive(Default)]
pub struct Http {
    pub me: RequestState<PrivateUserInfo>,
    pub logout: RequestState<()>,
}

impl Http {
    pub fn on_start(&mut self) {
        self.fetch_me();
    }

    pub fn update(&mut self, toasts: &mut egui_notify::Toasts) {
        self.me.poll_json(toasts);
        self.logout.poll_one_off(toasts);

        if let RequestState::Success(_) = self.logout {
            self.logout = RequestState::Idle;
            bindings::reload();
        }
    }

    pub fn fetch_me(&mut self) {
        if matches!(self.me, RequestState::Idle | RequestState::Error(_)) {
            let req = RequestBuilder::new("/api/me").method(Method::Get).send();
            self.me = RequestState::Loading(req);
        }
    }

    pub fn do_logout(&mut self) {
        if matches!(self.logout, RequestState::Idle | RequestState::Error(_)) {
            let req = RequestBuilder::new("/api/auth/logout")
                .method(Method::Post)
                .send();
            self.logout = RequestState::Loading(req);
        }
    }
}

pub enum RequestState<T> {
    Idle,
    Loading(Request),
    Success(T),
    Error(String),
}

#[allow(clippy::derivable_impls)]
impl<T> Default for RequestState<T> {
    fn default() -> Self {
        Self::Idle
    }
}

impl RequestState<()> {
    pub fn poll_one_off(&mut self, toasts: &mut egui_notify::Toasts) {
        if let RequestState::Loading(req) = self
            && let Some(response) = req.try_recv()
        {
            *self = match response {
                Ok(_) => RequestState::Success(()),
                Err(err) => {
                    toasts.error(err.to_string());
                    RequestState::Error(err.to_string())
                }
            };
        }
    }
}

impl<T: DeserializeOwned> RequestState<T> {
    pub fn poll_json(&mut self, toasts: &mut egui_notify::Toasts) {
        if let RequestState::Loading(req) = self
            && let Some(response) = req.try_recv()
        {
            *self = match response {
                Ok(text) => match serde_json::from_str::<T>(&text) {
                    Ok(data) => RequestState::Success(data),
                    Err(err) => {
                        toasts.error(format!("Parse error: {}", err));
                        RequestState::Error(err.to_string())
                    }
                },
                Err(err) => {
                    toasts.error(err.to_string());
                    RequestState::Error(err.to_string())
                }
            };
        }
    }
}
