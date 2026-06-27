use std::path::Path as StdPath;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::core::error::{AppError, AppResult};
use crate::core::fs::{File, Path};
use super::arch::{Json, Parse, Toml, Yaml};

impl Parse {

    pub fn load <T: DeserializeOwned> ( path: &StdPath ) -> AppResult<T> {

        let body = File::read_opt(path).ok_or_else(|| AppError::not_found(path.to_string_lossy()))?;

        match Path::extension(path).as_str() {
            "toml" => Toml::parse(&body),
            "json" => Json::parse(&body),
            "yaml" | "yml" => Yaml::parse(&body),
            other => Err(AppError::invalid("format", format!("unknown extension `{other}`"))),
        }

    }

    pub fn save <T: Serialize> ( path: &StdPath, value: &T ) -> AppResult<()> {

        let body = match Path::extension(path).as_str() {
            "toml" => Toml::to_string_pretty(value)?,
            "json" => Json::to_string_pretty(value)?,
            "yaml" | "yml" => Yaml::to_string(value)?,
            other => return Err(AppError::invalid("format", format!("unknown extension `{other}`"))),
        };

        File::write(path, &body)

    }

}
