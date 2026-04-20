use std::sync::{Arc, RwLock};

use reqwest::{Client, RequestBuilder};

#[derive(Clone)]
pub struct ApiClient {
    pub base_url: String,
    pub client: Client,
    token: Arc<RwLock<Option<String>>>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
            token: Arc::new(RwLock::new(None)),
        }
    }

    pub fn new_with_token(base_url: String, token: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
            token: Arc::new(RwLock::new(Some(token))),
        }
    }

    pub fn set_token(&self, token: String) {
        *self.token.write().unwrap() = Some(token)
    }

    pub fn remove_token(&mut self) {
        *self.token.write().unwrap() = None
    }

    pub fn auth_request(&self, req: RequestBuilder) -> RequestBuilder {
        if let Some(token) = self.token.read().unwrap().clone() {
            req.bearer_auth(token)
        } else {
            req
        }
    }
}
