use std::collections::BTreeMap;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Train;

#[derive(Debug, Clone, Default)]
pub struct Needs ( pub Vec<String> );

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Stack {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub history: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub dependency: BTreeMap<String, Needs>,
}
