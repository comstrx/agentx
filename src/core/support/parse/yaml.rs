use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::core::error::{AppError, AppResult};
use super::arch::Yaml;

/// YAML is stubbed: the surface matches `Toml`/`Json` so callers and the
/// extension dispatcher stay uniform, but no backend is wired in yet. Drop in a
/// maintained yaml crate here when a real consumer appears.
impl Yaml {

    pub fn parse<T: DeserializeOwned> ( _input: &str ) -> AppResult<T> {

        Err(AppError::parse("yaml", "no yaml backend enabled"))

    }

    pub fn to_string<T: Serialize> ( _value: &T ) -> AppResult<String> {

        Err(AppError::parse("yaml", "no yaml backend enabled"))

    }

}
