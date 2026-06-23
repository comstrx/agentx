use std::collections::HashMap;

use crate::core::error::AppError;
use super::arch::ContextValue;

macro_rules! from_int   { ($($t:ty),*) => { $( impl From<$t> for ContextValue { fn from ( v: $t ) -> Self { Self::Int(v as i64) } } )* }; }
macro_rules! from_uint  { ($($t:ty),*) => { $( impl From<$t> for ContextValue { fn from ( v: $t ) -> Self { Self::UInt(v as u64) } } )* }; }
macro_rules! from_float { ($($t:ty),*) => { $( impl From<$t> for ContextValue { fn from ( v: $t ) -> Self { Self::Float(v as f64) } } )* }; }

from_int!(i8, i16, i32, i64, isize);
from_uint!(u8, u16, u32, u64, usize);
from_float!(f32, f64);

macro_rules! narrow_int {

    ($($t:ty => $name:literal),* $(,)?) => { $(

        impl TryFrom<&ContextValue> for $t {

            type Error = AppError;

            fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

                match value {
                    ContextValue::Int(n)  => <$t>::try_from(*n).map_err(|_| ContextValue::mismatch($name, value)),
                    ContextValue::UInt(n) => <$t>::try_from(*n).map_err(|_| ContextValue::mismatch($name, value)),
                    other => Err(ContextValue::mismatch($name, other)),
                }

            }

        }

        impl TryFrom<ContextValue> for $t {

            type Error = AppError;

            fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

                <$t>::try_from(&value)

            }

        }

    )* };

}

macro_rules! owned_extract {

    ($($t:ty => ($name:literal, $variant:ident)),* $(,)?) => { $(

        impl TryFrom<&ContextValue> for $t {

            type Error = AppError;

            fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

                match value {
                    ContextValue::$variant(inner) => Ok(inner.clone()),
                    other => Err(ContextValue::mismatch($name, other)),
                }

            }

        }

        impl TryFrom<ContextValue> for $t {

            type Error = AppError;

            fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

                match value {
                    ContextValue::$variant(inner) => Ok(inner),
                    other => Err(ContextValue::mismatch($name, &other)),
                }

            }

        }

    )* };

}

narrow_int!(
    i8    => "i8",
    i16   => "i16",
    i32   => "i32",
    isize => "isize",
    u8    => "u8",
    u16   => "u16",
    u32   => "u32",
    usize => "usize",
);
owned_extract!(
    String            => ("text", Text),
    Vec<u8>           => ("bytes", Bytes),
    Vec<ContextValue> => ("list", List),
    HashMap<String,
    ContextValue>     => ("map", Map),
);

impl ContextValue {

    pub fn mismatch ( want: &str, found: &ContextValue ) -> AppError {

        AppError::invalid("context value", format!("expected {want}, found {found:?}"))

    }

    pub fn list <I, T> ( items: I ) -> Self where I: IntoIterator<Item = T>, T: Into<ContextValue> {

        Self::List(items.into_iter().map(Into::into).collect())

    }

    pub fn map <I, K, V> ( entries: I ) -> Self where I: IntoIterator<Item = ( K, V )>, K: Into<String>, V: Into<ContextValue> {

        Self::Map(entries.into_iter().map(|( key, value )| ( key.into(), value.into() )).collect())

    }

    pub fn is_null ( &self ) -> bool {

        matches!(self, Self::Null)

    }

    pub fn as_str ( &self ) -> Option<&str> {

        if let Self::Text(value) = self { Some(value) } else { None }

    }

    pub fn as_bytes ( &self ) -> Option<&[u8]> {

        if let Self::Bytes(value) = self { Some(value) } else { None }

    }

    pub fn as_int ( &self ) -> Option<i64> {

        i64::try_from(self).ok()

    }

    pub fn as_uint ( &self ) -> Option<u64> {

        u64::try_from(self).ok()

    }

    pub fn as_float ( &self ) -> Option<f64> {

        f64::try_from(self).ok()

    }

    pub fn as_bool ( &self ) -> Option<bool> {

        bool::try_from(self).ok()

    }

    pub fn as_list ( &self ) -> Option<&[ContextValue]> {

        if let Self::List(value) = self { Some(value) } else { None }

    }

    pub fn as_map ( &self ) -> Option<&HashMap<String, ContextValue>> {

        if let Self::Map(value) = self { Some(value) } else { None }

    }

}

impl From<()> for ContextValue {

    fn from ( _: () ) -> Self {

        Self::Null

    }

}

impl From<bool> for ContextValue {

    fn from ( value: bool ) -> Self {

        Self::Bool(value)

    }

}

impl From<&str> for ContextValue {

    fn from ( value: &str ) -> Self {

        Self::Text(value.to_string())

    }

}

impl From<String> for ContextValue {

    fn from ( value: String ) -> Self {

        Self::Text(value)

    }

}

impl From<&String> for ContextValue {

    fn from ( value: &String ) -> Self {

        Self::Text(value.clone())

    }

}

impl From<&[u8]> for ContextValue {

    fn from ( value: &[u8] ) -> Self {

        Self::Bytes(value.to_vec())

    }

}

impl From<Vec<u8>> for ContextValue {

    fn from ( value: Vec<u8> ) -> Self {

        Self::Bytes(value)

    }

}

impl From<Vec<String>> for ContextValue {

    fn from ( value: Vec<String> ) -> Self {

        Self::List(value.into_iter().map(Self::Text).collect())

    }

}

impl From<Vec<ContextValue>> for ContextValue {

    fn from ( value: Vec<ContextValue> ) -> Self {

        Self::List(value)

    }

}

impl From<HashMap<String, ContextValue>> for ContextValue {

    fn from ( value: HashMap<String, ContextValue> ) -> Self {

        Self::Map(value)

    }

}

impl <T: Into<ContextValue>> From<Option<T>> for ContextValue {

    fn from ( value: Option<T> ) -> Self {

        match value {
            Some(inner) => inner.into(),
            None => Self::Null,
        }

    }

}

impl TryFrom<i128> for ContextValue {

    type Error = AppError;

    fn try_from ( value: i128 ) -> Result<Self, Self::Error> {

        i64::try_from(value).map(Self::Int).map_err(|_| AppError::invalid("context value", "i128 exceeds i64 range"))

    }

}

impl TryFrom<u128> for ContextValue {

    type Error = AppError;

    fn try_from ( value: u128 ) -> Result<Self, Self::Error> {

        u64::try_from(value).map(Self::UInt).map_err(|_| AppError::invalid("context value", "u128 exceeds u64 range"))

    }

}

impl TryFrom<&ContextValue> for i64 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Int(n) => Ok(*n),
            ContextValue::UInt(n) => i64::try_from(*n).map_err(|_| ContextValue::mismatch("i64", value)),
            other => Err(ContextValue::mismatch("i64", other)),
        }

    }

}

impl TryFrom<ContextValue> for i64 {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        i64::try_from(&value)

    }

}

impl TryFrom<&ContextValue> for u64 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::UInt(n) => Ok(*n),
            ContextValue::Int(n) => u64::try_from(*n).map_err(|_| ContextValue::mismatch("u64", value)),
            other => Err(ContextValue::mismatch("u64", other)),
        }

    }

}

impl TryFrom<ContextValue> for u64 {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        u64::try_from(&value)

    }

}

impl TryFrom<&ContextValue> for i128 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Int(n) => Ok(i128::from(*n)),
            ContextValue::UInt(n) => Ok(i128::from(*n)),
            other => Err(ContextValue::mismatch("i128", other)),
        }

    }

}

impl TryFrom<ContextValue> for i128 {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        i128::try_from(&value)

    }

}

impl TryFrom<&ContextValue> for u128 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::UInt(n) => Ok(u128::from(*n)),
            ContextValue::Int(n) => u128::try_from(*n).map_err(|_| ContextValue::mismatch("u128", value)),
            other => Err(ContextValue::mismatch("u128", other)),
        }

    }

}

impl TryFrom<ContextValue> for u128 {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        u128::try_from(&value)

    }

}

impl TryFrom<&ContextValue> for f64 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Float(n) => Ok(*n),
            ContextValue::Int(n) => Ok(*n as f64),
            ContextValue::UInt(n) => Ok(*n as f64),
            other => Err(ContextValue::mismatch("f64", other)),
        }

    }

}

impl TryFrom<ContextValue> for f64 {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        f64::try_from(&value)

    }

}

impl TryFrom<&ContextValue> for f32 {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Float(n) => Ok(*n as f32),
            ContextValue::Int(n) => Ok(*n as f32),
            ContextValue::UInt(n) => Ok(*n as f32),
            other => Err(ContextValue::mismatch("f32", other)),
        }

    }

}

impl TryFrom<ContextValue> for f32 {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        f32::try_from(&value)

    }

}

impl TryFrom<&ContextValue> for bool {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Bool(b) => Ok(*b),
            other => Err(ContextValue::mismatch("bool", other)),
        }

    }

}

impl TryFrom<ContextValue> for bool {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        bool::try_from(&value)

    }

}

impl TryFrom<&ContextValue> for () {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Null => Ok(()),
            other => Err(ContextValue::mismatch("null", other)),
        }

    }

}

impl TryFrom<ContextValue> for () {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        <()>::try_from(&value)

    }

}

impl TryFrom<&ContextValue> for Vec<String> {

    type Error = AppError;

    fn try_from ( value: &ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::List(items) => items.iter().map(String::try_from).collect(),
            other => Err(ContextValue::mismatch("list", other)),
        }

    }

}

impl TryFrom<ContextValue> for Vec<String> {

    type Error = AppError;

    fn try_from ( value: ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::List(items) => items.into_iter().map(String::try_from).collect(),
            other => Err(ContextValue::mismatch("list", &other)),
        }

    }

}

impl <'a> TryFrom<&'a ContextValue> for &'a str {

    type Error = AppError;

    fn try_from ( value: &'a ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Text(text) => Ok(text.as_str()),
            other => Err(ContextValue::mismatch("&str", other)),
        }

    }

}

impl <'a> TryFrom<&'a ContextValue> for &'a [u8] {

    type Error = AppError;

    fn try_from ( value: &'a ContextValue ) -> Result<Self, Self::Error> {

        match value {
            ContextValue::Bytes(bytes) => Ok(bytes.as_slice()),
            other => Err(ContextValue::mismatch("&[u8]", other)),
        }

    }

}
