use std::path::Path as StdPath;

use crate::core::error::AppResult;
use crate::core::date::Date;
use crate::core::fs::File;
use crate::core::parse::Json;
use super::arch::{Journey, Phase, Status};

impl Journey {

    pub fn fresh () -> Self {

        Self {
            journey_id: format!("{}-{}", Date::stamp(), Date::format("%H%M%S")),
            phase: Phase::Requires,
            status: Status::Running,
            started_at: Date::rfc3339(),
            ..Self::default()
        }

    }

    pub fn create () -> Self {

        Self {
            journey_id: format!("{}-{}", Date::stamp(), Date::format("%H%M%S")),
            mode: "create".to_string(),
            status: Status::Running,
            started_at: Date::rfc3339(),
            ..Self::default()
        }

    }

    pub fn load ( path: &StdPath ) -> Self {

        let body = File::read(path);

        if body.trim().is_empty() { return Self::default(); }

        Json::parse(&body).unwrap_or_default()

    }

    pub fn save ( &mut self, path: &StdPath ) -> AppResult<()> {

        self.updated_at = Date::rfc3339();

        let body = Json::to_string_pretty(self)?;
        File::write_atomic(path, &body)

    }

    pub fn is_resumable ( &self ) -> bool {

        !self.journey_id.is_empty()
            && self.phase != Phase::Idle
            && self.phase != Phase::Completed
            && self.status != Status::Completed

    }

}
