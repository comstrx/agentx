use std::time::Duration;
use serde::Serialize;

use crate::core::error::{AppError, AppResult};
use crate::core::rt::Rt;
use super::arch::{Request, Response, CLIENT};

impl Request {

    pub fn new ( method: impl Into<String>, url: impl Into<String> ) -> Self {

        Self { method: method.into(), url: url.into(), ..Self::default() }

    }

    pub fn get ( url: impl Into<String> ) -> Self {

        Self::new("GET", url)

    }

    pub fn post ( url: impl Into<String> ) -> Self {

        Self::new("POST", url)

    }

    pub fn put ( url: impl Into<String> ) -> Self {

        Self::new("PUT", url)

    }

    pub fn patch ( url: impl Into<String> ) -> Self {

        Self::new("PATCH", url)

    }

    pub fn delete ( url: impl Into<String> ) -> Self {

        Self::new("DELETE", url)

    }

    pub fn head ( url: impl Into<String> ) -> Self {

        Self::new("HEAD", url)

    }

    pub fn header ( mut self, key: impl Into<String>, value: impl Into<String> ) -> Self {

        self.headers.push(( key.into(), value.into() ));
        self

    }

    pub fn query ( mut self, key: impl Into<String>, value: impl Into<String> ) -> Self {

        self.query.push(( key.into(), value.into() ));
        self

    }

    pub fn bearer ( self, token: impl Into<String> ) -> Self {

        self.header("Authorization", format!("Bearer {}", token.into()))

    }

    pub fn basic ( mut self, user: impl Into<String>, password: impl Into<String> ) -> Self {

        self.basic_auth = Some(( user.into(), password.into() ));
        self

    }

    pub fn body ( mut self, body: impl Into<String> ) -> Self {

        self.body = Some(body.into());
        self

    }

    pub fn json ( mut self, body: impl Into<String> ) -> Self {

        self.headers.push(( "Content-Type".to_string(), "application/json".to_string() ));
        self.body = Some(body.into());
        self

    }

    pub fn json_value <T: Serialize> ( self, value: &T ) -> AppResult<Self> {

        let body = serde_json::to_string(value).map_err(|error| AppError::encode("json", error.to_string()))?;

        Ok(self.json(body))

    }

    pub fn timeout ( mut self, secs: u64 ) -> Self {

        self.timeout = secs;
        self

    }

    pub fn retries ( mut self, count: u32 ) -> Self {

        self.retries = count;
        self

    }

    pub fn send ( self ) -> AppResult<Response> {

        Rt::block_on(Self::dispatch(self))

    }

    async fn dispatch ( request: Request ) -> AppResult<Response> {

        let client = CLIENT.get_or_init(reqwest::Client::new);

        let method = reqwest::Method::from_bytes(request.method.to_uppercase().as_bytes())
            .map_err(|error| AppError::invalid("http method", error.to_string()))?;

        let url = Self::resolve_url(&request)?;

        let mut last = None;

        for attempt in 0..=request.retries {

            let builder = Self::build(client, &method, &url, &request);

            match builder.send().await {

                Ok(response) => {

                    let status = response.status().as_u16();

                    if status >= 500 && attempt < request.retries {

                        last = Some(AppError::network(&request.url, format!("server error {status}")));
                        Self::backoff(attempt).await;

                        continue;

                    }

                    return Self::collect(response).await;

                }
                Err(error) => {

                    last = Some(AppError::network(&request.url, format!("request failed: {error}")));

                    if attempt < request.retries { Self::backoff(attempt).await; }

                }
            }

        }

        Err(last.unwrap_or_else(|| AppError::network(&request.url, "request failed")))

    }

    fn resolve_url ( request: &Request ) -> AppResult<reqwest::Url> {

        let parsed = if request.query.is_empty() {

            reqwest::Url::parse(&request.url)

        }
        else {

            reqwest::Url::parse_with_params(&request.url, &request.query)

        };

        parsed.map_err(|error| AppError::invalid("url", error.to_string()))

    }

    fn build ( client: &reqwest::Client, method: &reqwest::Method, url: &reqwest::Url, request: &Request ) -> reqwest::RequestBuilder {

        let mut builder = client.request(method.clone(), url.clone());

        for ( key, value ) in &request.headers {

            builder = builder.header(key, value);

        }

        if let Some(( user, password )) = &request.basic_auth { builder = builder.basic_auth(user, Some(password)); }

        if request.timeout > 0 { builder = builder.timeout(Duration::from_secs(request.timeout)); }

        if let Some(body) = &request.body { builder = builder.body(body.clone()); }

        builder

    }

    async fn collect ( response: reqwest::Response ) -> AppResult<Response> {

        let status = response.status().as_u16();
        let url = response.url().to_string();

        let headers = response
            .headers()
            .iter()
            .map(|( name, value )| ( name.to_string(), value.to_str().unwrap_or_default().to_string() ))
            .collect();

        let body = response.text().await.map_err(|error| AppError::network(&url, format!("reading body failed: {error}")))?;

        Ok(Response { status, url, headers, body })

    }

    async fn backoff ( attempt: u32 ) {

        let millis = 200u64.saturating_mul(1u64 << attempt.min(5));
        tokio::time::sleep(Duration::from_millis(millis.min(5000))).await;

    }

}
