use std::collections::HashMap;
use std::sync::OnceLock;
use parking_lot::RwLock;

pub type ContextMap = HashMap<String, ContextValue>;

pub static CONTEXT: OnceLock<RwLock<ContextMap>> = OnceLock::new();

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AppContext;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ContextValue {
    #[default]
    Null,
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<ContextValue>),
    Map(HashMap<String, ContextValue>),
}
