use std::fmt;
use serde::Deserializer;
use serde::de::{Error, SeqAccess, Visitor};

use super::arch::{BoolFlag, Spec};

pub(crate) struct Roster;
pub(crate) struct OneAgent;

impl <'de> Visitor <'de> for Roster {

    type Value = Vec<String>;

    fn expecting ( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {

        formatter.write_str("an agent name or a list of agent names")

    }

    fn visit_str <E> ( self, value: &str ) -> Result<Vec<String>, E> where E: Error {

        match value.trim().is_empty() {
            true => Ok(Vec::new()),
            false => Ok(vec![value.trim().to_string()]),
        }

    }

    fn visit_seq <A> ( self, mut seq: A ) -> Result<Vec<String>, A::Error> where A: SeqAccess<'de> {

        let mut out = Vec::new();

        while let Some(item) = seq.next_element::<String>()? {

            if !item.trim().is_empty() { out.push(item.trim().to_string()); }

        }

        Ok(out)

    }

}

impl <'de> Visitor <'de> for OneAgent {

    type Value = String;

    fn expecting ( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {

        formatter.write_str("exactly one agent name (a string, or a single-element list)")

    }

    fn visit_str <E> ( self, value: &str ) -> Result<String, E> where E: Error {

        Ok(value.trim().to_string())

    }

    fn visit_seq <A> ( self, mut seq: A ) -> Result<String, A::Error> where A: SeqAccess<'de> {

        let mut out = Vec::new();

        while let Some(item) = seq.next_element::<String>()? {

            if !item.trim().is_empty() { out.push(item.trim().to_string()); }

        }

        match out.len() {
            0 => Ok(String::new()),
            1 => Ok(out.remove(0)),
            n => Err(A::Error::custom(format!("manager must be exactly one agent, found {n}: {out:?}"))),
        }

    }

}

impl Visitor <'_> for BoolFlag {

    type Value = bool;

    fn expecting ( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {

        formatter.write_str("a boolean, 0 or 1, or \"true\"/\"false\"")

    }

    fn visit_bool <E> ( self, value: bool ) -> Result<bool, E> where E: Error {

        Ok(value)

    }

    fn visit_i64 <E> ( self, value: i64 ) -> Result<bool, E> where E: Error {

        Ok(value != 0)

    }

    fn visit_u64 <E> ( self, value: u64 ) -> Result<bool, E> where E: Error {

        Ok(value != 0)

    }

    fn visit_str <E> ( self, value: &str ) -> Result<bool, E> where E: Error {

        match value.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" | "" => Ok(false),
            other => Err(E::custom(format!("invalid boolean value: {other:?}"))),
        }

    }

}

impl Spec {

    pub(crate) fn de_bool <'de, D> ( deserializer: D ) -> Result<bool, D::Error> where D: Deserializer<'de> {

        deserializer.deserialize_any(BoolFlag)

    }

    pub(crate) fn de_roster <'de, D> ( deserializer: D ) -> Result<Vec<String>, D::Error> where D: Deserializer<'de> {

        deserializer.deserialize_any(Roster)

    }

    pub(crate) fn de_manager <'de, D> ( deserializer: D ) -> Result<String, D::Error> where D: Deserializer<'de> {

        deserializer.deserialize_any(OneAgent)

    }

    pub(crate) fn parse_bool ( value: &str ) -> Option<bool> {

        match value.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Some(true),
            "false" | "0" | "no" | "off" => Some(false),
            _ => None,
        }

    }

}
