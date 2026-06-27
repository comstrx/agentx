use std::collections::HashMap;
use parking_lot::RwLock;

use crate::core::error::{AppError, AppResult};
use super::arch::{AppContext, ContextMap, ContextValue, CONTEXT};

impl AppContext {

    pub fn inner () -> &'static RwLock<ContextMap> {

        CONTEXT.get_or_init(|| RwLock::new(HashMap::new()))

    }

    pub fn set ( key: impl Into<String>, value: impl Into<ContextValue> ) {

        Self::inner().write().insert(key.into(), value.into());

    }

    pub fn set_once ( key: impl Into<String>, value: impl Into<ContextValue> ) {

        Self::inner().write().entry(key.into()).or_insert_with(|| value.into());

    }

    pub fn extend <I, K, V> ( entries: I ) where I: IntoIterator<Item = ( K, V )>, K: Into<String>, V: Into<ContextValue> {

        let mut guard = Self::inner().write();

        for ( key, value ) in entries {

            guard.insert(key.into(), value.into());

        }

    }

    pub fn value ( key: impl AsRef<str> ) -> Option<ContextValue> {

        Self::inner().read().get(key.as_ref()).cloned()

    }

    pub fn get <T> ( key: impl AsRef<str> ) -> Option<T> where for<'a> T: TryFrom<&'a ContextValue> {

        Self::inner().read().get(key.as_ref()).and_then(|value| T::try_from(value).ok())

    }

    pub fn get_or <T> ( key: impl AsRef<str>, default: T ) -> T where for<'a> T: TryFrom<&'a ContextValue> {

        Self::get(key).unwrap_or(default)

    }

    pub fn get_or_else <T> ( key: impl AsRef<str>, default: impl FnOnce() -> T ) -> T where for<'a> T: TryFrom<&'a ContextValue> {

        Self::get(key).unwrap_or_else(default)

    }

    pub fn has ( key: impl AsRef<str> ) -> bool {

        Self::inner().read().contains_key(key.as_ref())

    }

    pub fn need ( key: impl AsRef<str> ) -> AppResult<()> {

        if Self::has(key.as_ref()) { Ok(()) } else { Err(AppError::not_found(key.as_ref())) }

    }

    pub fn remove ( key: impl AsRef<str> ) {

        Self::inner().write().remove(key.as_ref());

    }

    pub fn take ( key: impl AsRef<str> ) -> Option<ContextValue> {

        Self::inner().write().remove(key.as_ref())

    }

    pub fn update ( key: impl Into<String>, func: impl FnOnce(Option<ContextValue>) -> ContextValue ) {

        let key = key.into();
        let mut guard = Self::inner().write();
        let current = guard.remove(&key);

        guard.insert(key, func(current));

    }

    pub fn push ( key: impl Into<String>, value: impl Into<ContextValue> ) {

        let key = key.into();
        let mut guard = Self::inner().write();

        match guard.get_mut(&key) {
            Some(ContextValue::List(items)) => items.push(value.into()),
            _ => { guard.insert(key, ContextValue::List(vec![value.into()])); }
        }

    }

    pub fn incr ( key: impl Into<String>, by: i64 ) -> i64 {

        let key = key.into();
        let mut guard = Self::inner().write();

        let current = match guard.get(&key) {
            Some(ContextValue::Int(n)) => *n,
            Some(ContextValue::UInt(n)) => *n as i64,
            _ => 0,
        };

        let next = current + by;
        guard.insert(key, ContextValue::Int(next));

        next

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

    pub fn values () -> Vec<ContextValue> {

        Self::inner().read().values().cloned().collect()

    }

    pub fn entries () -> Vec<( String, ContextValue )> {

        Self::inner().read().iter().map(|( key, value )| ( key.clone(), value.clone() )).collect()

    }

    pub fn snapshot () -> ContextMap {

        Self::inner().read().clone()

    }

    pub fn with <R> ( func: impl FnOnce(&ContextMap) -> R ) -> R {

        func(&Self::inner().read())

    }

    pub fn with_mut <R> ( func: impl FnOnce(&mut ContextMap) -> R ) -> R {

        func(&mut Self::inner().write())

    }

    pub fn retain ( mut keep: impl FnMut(&str, &ContextValue) -> bool ) {

        Self::inner().write().retain(|key, value| keep(key, value));

    }

    pub fn key <'a> ( parts: impl IntoIterator<Item = &'a str> ) -> String {

        parts.into_iter().collect::<Vec<_>>().join(".")

    }

}
