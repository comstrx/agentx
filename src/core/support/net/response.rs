use serde::de::DeserializeOwned;

use crate::core::error::{AppError, AppResult};
use super::arch::Response;

impl Response {

    pub fn ok ( &self ) -> bool {

        ( 200..300 ).contains(&self.status)

    }

    pub fn is_redirect ( &self ) -> bool {

        ( 300..400 ).contains(&self.status)

    }

    pub fn is_client_error ( &self ) -> bool {

        ( 400..500 ).contains(&self.status)

    }

    pub fn is_server_error ( &self ) -> bool {

        ( 500..600 ).contains(&self.status)

    }

    pub fn header ( &self, name: &str ) -> Option<&str> {

        self.headers.iter().find(|( key, _ )| key.eq_ignore_ascii_case(name)).map(|( _, value )| value.as_str())

    }

    pub fn content_type ( &self ) -> Option<&str> {

        self.header("Content-Type")

    }

    pub fn text ( &self ) -> &str {

        &self.body

    }

    pub fn into_text ( self ) -> String {

        self.body

    }

    pub fn json <T: DeserializeOwned> ( &self ) -> AppResult<T> {

        serde_json::from_str(&self.body).map_err(|error| AppError::parse("json", error.to_string()))

    }

    pub fn error_for_status ( self ) -> AppResult<Self> {

        if self.ok() { Ok(self) }
        else { Err(AppError::network(&self.url, format!("http status {}", self.status))) }

    }

}
