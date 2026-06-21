use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::core::error::{AppError, AppResult};
use super::arch::Toml;

impl Toml {

    pub fn parse<T: DeserializeOwned> ( input: &str ) -> AppResult<T> {

        ::toml::from_str(input).map_err(|error| AppError::parse("toml", error.to_string()))

    }

    pub fn to_string<T: Serialize> ( value: &T ) -> AppResult<String> {

        ::toml::to_string(value).map_err(|error| AppError::parse("toml", error.to_string()))

    }

    pub fn to_string_pretty<T: Serialize> ( value: &T ) -> AppResult<String> {

        ::toml::to_string_pretty(value).map_err(|error| AppError::parse("toml", error.to_string()))

    }

}
