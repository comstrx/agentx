use std::path::PathBuf;

use super::arch::{Config, Context};
use super::consts::{BUCKETS, BUCKET_DIRS, BUCKET_STEMS};

impl Config {

    pub fn manager ( &self ) -> &str {

        &self.spec.manager_model

    }

}

impl Context {

    pub fn bucket ( &self, name: &str ) -> &[PathBuf] {

        match name {
            "overview" => &self.overview,
            "contracts" => &self.contracts,
            "skills" => &self.skills,
            "history" => &self.history,
            "requires" => &self.requires,
            _ => &[],
        }

    }

    pub fn add ( &mut self, name: &str, path: PathBuf ) {

        let target = match name {
            "overview" => &mut self.overview,
            "contracts" => &mut self.contracts,
            "skills" => &mut self.skills,
            "history" => &mut self.history,
            "requires" => &mut self.requires,
            _ => return,
        };

        if !target.contains(&path) { target.push(path); }

    }

    pub fn buckets_of_stem ( stem: &str ) -> Vec<&'static str> {

        BUCKET_STEMS.iter().filter_map(|( bucket, stems )| stems.contains(&stem).then_some(*bucket)).collect()

    }

    pub fn bucket_of_dir ( name: &str ) -> Option<&'static str> {

        BUCKET_DIRS.iter().find_map(|( bucket, names )| names.contains(&name).then_some(*bucket))

    }

    pub fn has_requires ( &self ) -> bool {

        !self.requires.is_empty()

    }

    pub fn is_empty ( &self ) -> bool {

        BUCKETS.iter().all(|name| self.bucket(name).is_empty())

    }

}
