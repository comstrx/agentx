use std::collections::HashSet;
use std::fmt;
use std::path::{Path as StdPath, PathBuf};
use serde::Deserialize;
use serde::de::{self, Deserializer, SeqAccess, Visitor};

use crate::core::fs::{Dir, File, Path};
use crate::core::parse::Json;
use crate::core::text::Text;
use crate::config::base::consts::NODE_FILE;
use super::arch::{Needs, Stack, Train};

impl <'de> Deserialize<'de> for Needs {

    fn deserialize <D> ( deserializer: D ) -> Result<Self, D::Error> where D: Deserializer<'de> {

        struct Many;

        impl <'de> Visitor <'de> for Many {

            type Value = Vec<String>;

            fn expecting ( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {

                formatter.write_str("a node name, a list of node names, or nothing")

            }

            fn visit_str <E> ( self, value: &str ) -> Result<Vec<String>, E> where E: de::Error {

                match value.trim().is_empty() {
                    true => Ok(Vec::new()),
                    false => Ok(vec![value.trim().to_string()]),
                }

            }

            fn visit_unit <E> ( self ) -> Result<Vec<String>, E> where E: de::Error {

                Ok(Vec::new())

            }

            fn visit_seq <A> ( self, mut seq: A ) -> Result<Vec<String>, A::Error> where A: SeqAccess<'de> {

                let mut out = Vec::new();

                while let Some(item) = seq.next_element::<String>()? {

                    if !item.trim().is_empty() { out.push(item.trim().to_string()); }

                }

                Ok(out)

            }

        }

        deserializer.deserialize_any(Many).map(Needs)

    }

}

impl Train {

    pub(crate) fn resolve ( name: &str ) -> Vec<PathBuf> {

        Self::walk(name).0

    }

    pub(crate) fn trace ( name: &str ) -> ( Vec<String>, Vec<String> ) {

        let ( order, missing ) = Self::walk(name);

        let names = order.iter().map(|node| Text::unprefix(&Path::name_of(node)).to_string()).collect();

        ( names, missing )

    }

    fn walk ( name: &str ) -> ( Vec<PathBuf>, Vec<String> ) {

        let project = Self::project(name);

        let mut order = Vec::new();
        let mut missing = Vec::new();
        let mut visited: HashSet<PathBuf> = HashSet::new();

        Self::descend(&project, "", &mut order, &mut missing, &mut visited);

        if project.is_dir() && visited.insert(project.clone()) { order.push(project); }

        ( order, missing )

    }

    fn descend ( node: &StdPath, trail: &str, order: &mut Vec<PathBuf>, missing: &mut Vec<String>, visited: &mut HashSet<PathBuf> ) {

        let stack = Self::stack(node);

        if stack.dependency.is_empty() { return; }

        let mut matched: HashSet<String> = HashSet::new();

        for axis in Dir::subdirs(&Self::base()) {

            let folder = Path::name_of(&axis);
            let label = Text::unprefix(&folder);

            let Some(( key, needs )) = stack.dependency.iter().find(|( name, _ )| Text::unprefix(name).eq_ignore_ascii_case(label)) else { continue; };

            matched.insert(key.clone());

            for value in &needs.0 {

                let target = Dir::locate(&axis, value);

                if !target.is_dir() {

                    missing.push(Self::tag(trail, &format!("{label}/{value}")));

                }
                else if visited.insert(target.clone()) {

                    let name = Path::name_of(&target);
                    let deeper = Self::tag(trail, Text::unprefix(&name));

                    Self::descend(&target, &deeper, order, missing, visited);
                    order.push(target);

                }

            }

        }

        for ( key, needs ) in &stack.dependency {

            if matched.contains(key) { continue; }

            for value in &needs.0 { missing.push(Self::tag(trail, &format!("{key}/{value}"))); }

        }

    }

    fn tag ( trail: &str, leaf: &str ) -> String {

        match trail.is_empty() {
            true => leaf.to_string(),
            false => format!("{trail} > {leaf}"),
        }

    }

    pub(crate) fn stack ( node: &StdPath ) -> Stack {

        let body = File::read(&node.join(NODE_FILE));

        if body.trim().is_empty() { return Stack::default(); }

        Json::parse(&body).unwrap_or_default()

    }

}
