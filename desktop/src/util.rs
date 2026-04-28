use core::time;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::sync::{Arc, RwLock};

use crate::error::AppError;

#[derive(Clone)]
pub struct ApiClient {
    pub base_url: String,
    pub client: Client,
    // last_succecful_request_send: ,
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

    pub fn remove_token(&self) {
        *self.token.write().unwrap() = None
    }

    pub fn is_token_present(&self) -> bool {
        self.token.read().is_ok_and(|token| token.is_some())
    }

    pub fn is_token_valid(&self) -> bool {
        true
    }

    pub fn auth_request(&self, req: RequestBuilder) -> RequestBuilder {
        if let Some(token) = self.token.read().unwrap().clone() {
            req.bearer_auth(token)
        } else {
            req
        }
    }

    /// sends an api request using the provided url
    /// if there is a token present its validity will be checked before hand
    ///
    /// returns `Ok(Option<T>)` if the request was succesful
    /// else returns `Err(AppError)`
    pub async fn send_get_request<T: DeserializeOwned>(
        &self,
        url_path: &str,
    ) -> Result<T, AppError> {
        if !self.is_token_present() || !self.is_token_valid() {
            self.remove_token();
            Err(AppError::LoginExpired)?;
        }

        let result = self
            .auth_request(self.client.get(format!("{}{}", self.base_url, url_path)))
            .send()
            .await;

        match result {
            Ok(response) => {
                if response.status() != StatusCode::OK {
                    todo!("Handle different errorcodes like unauthorized");
                }
                //todo!("Update last succeful request time");
                response.json::<T>().await.or(Err(AppError::JsonParseError))
            }
            Err(e) => {
                eprintln!("Error sending get request to server:/n {}", e);
                Err(AppError::ApiClientError)
            }
        }
    }
}
