use std::collections::HashMap;

use crate::core::error::AppError;
use super::arch::ContextValue;

impl ContextValue {

    /// Build a `List` from anything iterable of convertible items.
    pub fn list<I, T> ( items: I ) -> Self where I: IntoIterator<Item = T>, T: Into<ContextValue> {

        Self::List(items.into_iter().map(Into::into).collect())

    }

    pub fn is_null ( &self ) -> bool {

        matches!(self, Self::Null)

    }

    pub fn as_str ( &self ) -> Option<&str> {

        if let Self::Text(value) = self { Some(value) } else { None }

    }

}

// ---- From: owned value -> ContextValue ----

impl From<()> for ContextValue {

    fn from ( _: () ) -> Self { Self::Null }

}

impl From<bool> for ContextValue {

    fn from ( value: bool ) -> Self { Self::Bool(value) }

}

impl From<&str> for ContextValue {

    fn from ( value: &str ) -> Self { Self::Text(value.to_string()) }

}

impl From<String> for ContextValue {

    fn from ( value: String ) -> Self { Self::Text(value) }

}

impl From<&String> for ContextValue {

    fn from ( value: &String ) -> Self { Self::Text(value.clone()) }

}

impl From<Vec<ContextValue>> for ContextValue {

    fn from ( value: Vec<ContextValue> ) -> Self { Self::List(value) }

}

impl From<HashMap<String, ContextValue>> for ContextValue {

    fn from ( value: HashMap<String, ContextValue> ) -> Self { Self::Map(value) }

}

impl<T: Into<ContextValue>> From<Option<T>> for ContextValue {

    fn from ( value: Option<T> ) -> Self {

        match value {
            Some(inner) => inner.into(),
            None => Self::Null,
        }

    }

}

macro_rules! from_int  { ($($t:ty),*) => { $( impl From<$t> for ContextValue { fn from ( v: $t ) -> Self { Self::Int(v as i64) } } )* }; }
macro_rules! from_uint { ($($t:ty),*) => { $( impl From<$t> for ContextValue { fn from ( v: $t ) -> Self { Self::UInt(v as u64) } } )* }; }
macro_rules! from_float { ($($t:ty),*) => { $( impl From<$t> for ContextValue { fn from ( v: $t ) -> Self { Self::Float(v as f64) } } )* }; }

from_int!(i8, i16, i32, i64, isize);
from_uint!(u8, u16, u32, u64, usize);
from_float!(f32, f64);

// ---- TryFrom: &ContextValue -> typed value ----

fn mismatch ( want: &str, found: &ContextValue ) -> AppError {

    AppError::invalid("context value", format!("expected {want}, found {found:?}"))

}

impl TryFrom<&ContextValue> for i64 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Int(n) => Ok(*n),
            ContextValue::UInt(n) if *n <= i64::MAX as u64 => Ok(*n as i64),
            other => Err(mismatch("int", other)),
        }

    }

}

impl TryFrom<&ContextValue> for u64 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::UInt(n) => Ok(*n),
            ContextValue::Int(n) if *n >= 0 => Ok(*n as u64),
            other => Err(mismatch("uint", other)),
        }

    }

}

impl TryFrom<&ContextValue> for f64 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Float(n) => Ok(*n),
            ContextValue::Int(n) => Ok(*n as f64),
            ContextValue::UInt(n) => Ok(*n as f64),
            other => Err(mismatch("float", other)),
        }

    }

}

impl TryFrom<&ContextValue> for bool {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Bool(b) => Ok(*b),
            other => Err(mismatch("bool", other)),
        }

    }

}

impl TryFrom<&ContextValue> for String {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Text(s) => Ok(s.clone()),
            other => Err(mismatch("text", other)),
        }

    }

}

impl TryFrom<&ContextValue> for Vec<String> {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::List(items) => items.iter().map(String::try_from).collect(),
            other => Err(mismatch("list", other)),
        }

    }

}
