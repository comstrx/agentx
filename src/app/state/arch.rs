use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    #[default]
    Idle,
    Requires,
    Tasks,
    Audit,
    Tests,
    Benches,
    Examples,
    Fuzzes,
    Completed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    Idle,
    Running,
    Draining,
    Drained,
    Stopped,
    Failed,
    Completed,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Journey {
    pub journey_id: String,
    pub mode: String,
    pub note: String,
    pub primed: bool,
    pub intake_done: bool,
    pub phase: Phase,
    pub status: Status,
    pub current_task: String,
    pub current_agent: String,
    pub current_round: u32,
    pub current_audit: u32,
    pub manager_review: String,
    pub task_status: BTreeMap<String, String>,
    pub agents_done: Vec<String>,
    pub agents_pending: Vec<String>,
    pub blocked: Vec<String>,
    pub last_action: String,
    pub started_at: String,
    pub updated_at: String,
}
