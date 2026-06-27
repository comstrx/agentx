use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::core::error::{AppError, AppResult};
use super::arch::Json;

impl Json {

    pub fn parse <T: DeserializeOwned> ( input: &str ) -> AppResult<T> {

        serde_json::from_str(input).map_err(|error| AppError::parse("json", error.to_string()))

    }

    pub fn to_string <T: Serialize> ( value: &T ) -> AppResult<String> {

        serde_json::to_string(value).map_err(|error| AppError::encode("json", error.to_string()))

    }

    pub fn to_string_pretty <T: Serialize> ( value: &T ) -> AppResult<String> {

        serde_json::to_string_pretty(value).map_err(|error| AppError::encode("json", error.to_string()))

    }

}
