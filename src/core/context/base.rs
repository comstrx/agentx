use std::collections::HashMap;
use parking_lot::RwLock;

use crate::core::error::{AppError, AppResult};
use super::arch::{AppContext, ContextMap, ContextValue, CONTEXT};

impl AppContext {

    /// The global store, initialised on first touch.
    pub fn inner () -> &'static RwLock<ContextMap> {

        CONTEXT.get_or_init(|| RwLock::new(HashMap::new()))

    }

    pub fn set ( key: impl Into<String>, value: impl Into<ContextValue> ) {

        Self::inner().write().insert(key.into(), value.into());

    }

    /// Set only if the key is absent — first writer wins.
    pub fn set_once ( key: impl Into<String>, value: impl Into<ContextValue> ) {

        Self::inner().write().entry(key.into()).or_insert_with(|| value.into());

    }

    pub fn extend<I, K, V> ( entries: I ) where I: IntoIterator<Item = (K, V)>, K: Into<String>, V: Into<ContextValue> {

        let mut guard = Self::inner().write();

        for (key, value) in entries {
            guard.insert(key.into(), value.into());
        }

    }

    /// Raw clone of a value, untyped.
    pub fn value ( key: impl AsRef<str> ) -> Option<ContextValue> {

        Self::inner().read().get(key.as_ref()).cloned()

    }

    /// Typed read — `None` if absent or the type does not match.
    pub fn get<T> ( key: impl AsRef<str> ) -> Option<T> where for<'a> T: TryFrom<&'a ContextValue> {

        Self::inner().read().get(key.as_ref()).and_then(|value| T::try_from(value).ok())

    }

    pub fn get_or<T> ( key: impl AsRef<str>, default: T ) -> T where for<'a> T: TryFrom<&'a ContextValue> {

        Self::get(key).unwrap_or(default)

    }

    pub fn has ( key: impl AsRef<str> ) -> bool {

        Self::inner().read().contains_key(key.as_ref())

    }

    /// Assert a key exists, erroring with [`AppError::NotFound`] otherwise.
    pub fn need ( key: impl AsRef<str> ) -> AppResult<()> {

        if Self::has(key.as_ref()) { Ok(()) } else { Err(AppError::not_found(key.as_ref())) }

    }

    pub fn remove ( key: impl AsRef<str> ) {

        Self::inner().write().remove(key.as_ref());

    }

    pub fn clear () {

        Self::inner().write().clear();

    }

    pub fn len () -> usize {

        Self::inner().read().len()

    }

    pub fn is_empty () -> bool {

        Self::inner().read().is_empty()

    }

    pub fn keys () -> Vec<String> {

        Self::inner().read().keys().cloned().collect()

    }

    /// Read the whole map under a shared lock and project a result out.
    pub fn with<R> ( func: impl FnOnce(&ContextMap) -> R ) -> R {

        func(&Self::inner().read())

    }

    /// Mutate the whole map under an exclusive lock.
    pub fn with_mut<R> ( func: impl FnOnce(&mut ContextMap) -> R ) -> R {

        func(&mut Self::inner().write())

    }

    pub fn retain ( mut keep: impl FnMut(&str, &ContextValue) -> bool ) {

        Self::inner().write().retain(|key, value| keep(key, value));

    }

    /// Join namespace parts into a dotted key, e.g. `key(["agent", id, "session"])`.
    pub fn key<'a> ( parts: impl IntoIterator<Item = &'a str> ) -> String {

        parts.into_iter().collect::<Vec<_>>().join(".")

    }

}
