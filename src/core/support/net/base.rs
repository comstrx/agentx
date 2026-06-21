use crate::core::error::{AppError, AppResult};
use super::arch::{Net, Request, Response};

impl Request {

    pub fn new ( method: impl Into<String>, url: impl Into<String> ) -> Self {

        Self { method: method.into(), url: url.into(), body: None }

    }

    pub fn with_body ( mut self, body: impl Into<String> ) -> Self {

        self.body = Some(body.into());
        self

    }

}

impl Net {

    pub fn get ( url: impl Into<String> ) -> AppResult<Response> {

        Self::send(Request::new("GET", url))

    }

    pub fn post ( url: impl Into<String>, body: impl Into<String> ) -> AppResult<Response> {

        Self::send(Request::new("POST", url).with_body(body))

    }

    /// Send a request. Stubbed until a backend is enabled.
    pub fn send ( _request: Request ) -> AppResult<Response> {

        Err(AppError::message("net backend not enabled"))

    }

}
