use std::collections::HashMap;
use std::sync::OnceLock;
use parking_lot::RwLock;

/// The backing map: dotted string keys to typed values.
pub type ContextMap = HashMap<String, ContextValue>;

/// One global, process-wide store. `parking_lot::RwLock` never poisons, so the
/// whole DSL stays infallible — no `Result` on every read/write.
pub static CONTEXT: OnceLock<RwLock<ContextMap>> = OnceLock::new();

/// A zero-sized handle. All access is through associated functions on this type,
/// e.g. `AppContext::set(...)`, so the global store reads like a namespace.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AppContext;

/// A dynamically-typed value. Type recovery on read goes through `TryFrom`
/// (see `value.rs`), so callers stay type-safe at the edges.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ContextValue {

    #[default]
    Null,
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Text(String),
    List(Vec<ContextValue>),
    Map(HashMap<String, ContextValue>),
}
