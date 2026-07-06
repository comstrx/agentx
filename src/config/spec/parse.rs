use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::value::MapAccessDeserializer;
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};

use super::arch::{BoolFlag, Member, Seats, Spec};

impl <'de> Deserialize <'de> for Member {

    fn deserialize <D> ( deserializer: D ) -> Result<Self, D::Error> where D: Deserializer<'de> {

        struct Any;

        impl <'de> Visitor <'de> for Any {

            type Value = Member;

            fn expecting ( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {

                formatter.write_str("an agent name, or a table { agent, model?, effort? }")

            }

            fn visit_str <E> ( self, value: &str ) -> Result<Member, E> where E: Error {

                Ok(Member { agent: value.trim().to_string(), model: String::new(), effort: String::new() })

            }

            fn visit_map <A> ( self, mut map: A ) -> Result<Member, A::Error> where A: MapAccess<'de> {

                let mut agent: Option<String> = None;
                let mut model = String::new();
                let mut effort = String::new();

                while let Some(key) = map.next_key::<String>()? {

                    match key.as_str() {
                        "agent"  => agent = Some(map.next_value::<String>()?),
                        "model"  => model = map.next_value::<String>()?,
                        "effort" => effort = map.next_value::<String>()?,
                        _        => { map.next_value::<serde::de::IgnoredAny>()?; }
                    }

                }

                let agent = agent.ok_or_else(|| A::Error::custom("a member table needs an `agent` key naming the backend"))?;

                Ok(Member { agent: agent.trim().to_string(), model: model.trim().to_string(), effort: effort.trim().to_string() })

            }

        }

        deserializer.deserialize_any(Any)

    }

}

impl Serialize for Member {

    fn serialize <S> ( &self, serializer: S ) -> Result<S::Ok, S::Error> where S: Serializer {

        let model = self.model.trim();
        let effort = self.effort.trim();

        if model.is_empty() && effort.is_empty() {

            return serializer.serialize_str(self.agent.trim());

        }

        let count = 1 + usize::from(!model.is_empty()) + usize::from(!effort.is_empty());
        let mut map = serializer.serialize_map(Some(count))?;

        map.serialize_entry("agent", self.agent.trim())?;

        if !model.is_empty() { map.serialize_entry("model", model)?; }

        if !effort.is_empty() { map.serialize_entry("effort", effort)?; }

        map.end()

    }

}

impl <'de> Deserialize <'de> for Seats {

    fn deserialize <D> ( deserializer: D ) -> Result<Self, D::Error> where D: Deserializer<'de> {

        struct Any;

        impl <'de> Visitor <'de> for Any {

            type Value = Seats;

            fn expecting ( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {

                formatter.write_str("an agent name, a member table, or a list of either")

            }

            fn visit_str <E> ( self, value: &str ) -> Result<Seats, E> where E: Error {

                let members = match value.trim().is_empty() {
                    true => Vec::new(),
                    false => vec![Member { agent: value.trim().to_string(), model: String::new(), effort: String::new() }],
                };

                Ok(Seats { members, seq: false })

            }

            fn visit_map <A> ( self, map: A ) -> Result<Seats, A::Error> where A: MapAccess<'de> {

                let member = Member::deserialize(MapAccessDeserializer::new(map))?;

                Ok(Seats { members: vec![member], seq: false })

            }

            fn visit_seq <A> ( self, mut seq: A ) -> Result<Seats, A::Error> where A: SeqAccess<'de> {

                let mut members = Vec::new();

                while let Some(member) = seq.next_element::<Member>()? {

                    if !member.agent.trim().is_empty() { members.push(member); }

                }

                Ok(Seats { members, seq: true })

            }

        }

        deserializer.deserialize_any(Any)

    }

}

impl Serialize for Seats {

    fn serialize <S> ( &self, serializer: S ) -> Result<S::Ok, S::Error> where S: Serializer {

        if !self.seq && self.members.len() == 1 {

            return self.members[0].serialize(serializer);

        }

        let mut seq = serializer.serialize_seq(Some(self.members.len()))?;

        for member in &self.members { seq.serialize_element(member)?; }

        seq.end()

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

    pub(crate) fn de_stage <'de, D> ( deserializer: D ) -> Result<String, D::Error> where D: Deserializer<'de> {

        use crate::config::base::consts::{DEFAULT_STAGE, STAGES};

        let value = String::deserialize(deserializer)?;

        let stage = match value.trim().to_ascii_lowercase().as_str() {
            "" => DEFAULT_STAGE,
            "dev" | "debug" | "development" => "dev",
            "staging" | "stage" => "staging",
            "live" | "production" | "prod" => "live",
            _ => return Err(Error::custom(format!("unknown stage {value:?} — one of: {} (debug/stage/production accepted as aliases)", STAGES.join(", ")))),
        };

        Ok(stage.to_string())

    }

    pub(crate) fn de_manager <'de, D> ( deserializer: D ) -> Result<Member, D::Error> where D: Deserializer<'de> {

        struct One;

        impl <'de> Visitor <'de> for One {

            type Value = Member;

            fn expecting ( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {

                formatter.write_str("exactly one agent (a name, a table, or a single-element list)")

            }

            fn visit_str <E> ( self, value: &str ) -> Result<Member, E> where E: Error {

                Ok(Member { agent: value.trim().to_string(), model: String::new(), effort: String::new() })

            }

            fn visit_map <A> ( self, map: A ) -> Result<Member, A::Error> where A: MapAccess<'de> {

                Member::deserialize(MapAccessDeserializer::new(map))

            }

            fn visit_seq <A> ( self, mut seq: A ) -> Result<Member, A::Error> where A: SeqAccess<'de> {

                let mut out = Vec::new();

                while let Some(member) = seq.next_element::<Member>()? {

                    if !member.agent.trim().is_empty() { out.push(member); }

                }

                match out.len() {
                    0 => Ok(Member::default()),
                    1 => Ok(out.remove(0)),
                    n => Err(A::Error::custom(format!("manager must be exactly one agent, found {n}"))),
                }

            }

        }

        deserializer.deserialize_any(One)

    }

    pub(crate) fn parse_bool ( value: &str ) -> Option<bool> {

        match value.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Some(true),
            "false" | "0" | "no" | "off" => Some(false),
            _ => None,
        }

    }

}
