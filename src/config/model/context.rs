use std::path::{Path as StdPath, PathBuf};

use crate::config::base::consts::{BUCKET_DIRS, BUCKET_STEMS, DESIGNS, MD_EXT, REFERENCES};
use crate::core::fs::{Dir, Path};
use crate::core::text::Text;
use super::arch::Context;

impl Context {

    pub fn collect ( &mut self, base: &StdPath, folders: bool ) {

        for entry in Dir::entries(base) {

            if entry.is_file() {

                if !Path::has_extension(&entry, MD_EXT) { continue; }

                for bucket in Self::buckets_of_stem(&Path::stem_of(&entry).to_ascii_lowercase()) {

                    self.add(bucket, entry.clone());

                }

            }
            else if folders && let Some(bucket) = Self::bucket_of_dir(&Path::name_of(&entry).to_ascii_lowercase()) {

                let any = bucket == DESIGNS || bucket == REFERENCES;

                for file in Dir::walk(&entry) {

                    if file.is_file() && ( any || Path::has_extension(&file, MD_EXT) ) { self.add(bucket, file); }

                }

            }

        }

    }

    pub fn retain ( &mut self, keep: impl Fn(&PathBuf) -> bool ) {

        for bucket in self.buckets_mut() {

            bucket.retain(&keep);

        }

    }

    pub fn sort ( &mut self ) {

        for bucket in self.buckets_mut() {

            bucket.sort_by(|a, b| Text::natural_compare(&Path::name_of(a), &Path::name_of(b)));

        }

    }

    fn buckets_mut ( &mut self ) -> [&mut Vec<PathBuf>; 7] {

        [
            &mut self.overview,
            &mut self.contracts,
            &mut self.skills,
            &mut self.designs,
            &mut self.references,
            &mut self.history,
            &mut self.requires,
        ]

    }

    pub fn bucket ( &self, name: &str ) -> &[PathBuf] {

        match name {
            "overview" => &self.overview,
            "contracts" => &self.contracts,
            "skills" => &self.skills,
            "designs" => &self.designs,
            "references" => &self.references,
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
            "designs" => &mut self.designs,
            "references" => &mut self.references,
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

}
