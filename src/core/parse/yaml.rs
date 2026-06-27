use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::core::error::{AppError, AppResult};
use super::arch::Yaml;

impl Yaml {

    pub fn parse <T: DeserializeOwned> ( input: &str ) -> AppResult<T> {

        serde_yaml_ng::from_str(input).map_err(|error| AppError::parse("yaml", error.to_string()))

    }

    pub fn to_string <T: Serialize> ( value: &T ) -> AppResult<String> {

        serde_yaml_ng::to_string(value).map_err(|error| AppError::encode("yaml", error.to_string()))

    }

}
