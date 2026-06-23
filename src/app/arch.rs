use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::core::error::AppError;

const HELP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::BrightBlack.on_default());

#[derive(Parser)]
#[command(
    name = "agentx",
    version,
    about = "Hierarchical multi-agent orchestrator - turns requirements into reviewed, gate-kept code.",
    long_about = "Hierarchical multi-agent orchestrator - turns requirements into reviewed, gate-kept code.",
    propagate_version = true,
    arg_required_else_help = true,
    styles = HELP_STYLES,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(
        short = 'C',
        long = "dir",
        global = true,
        value_name = "DIR",
        help = "Operate as if agentx were started in DIR instead of the current directory",
    )]
    pub dir: Option<PathBuf>,

    #[arg(
        short = 'p',
        long = "project",
        global = true,
        value_name = "NAME|N",
        help = "Bind to a training-center archetype by name or its list number (overrides project_type)",
    )]
    pub project: Option<String>,

    #[arg(
        short = 'g',
        long = "gate",
        global = true,
        value_name = "COMMAND",
        help = "Set the quality-gate shell command (overrides gate_cmd)",
    )]
    pub gate: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Scaffold Agentx.toml, agents/, and .agentx/ in the project directory")]
    Init,

    #[command(about = "Resolve the project root and run a full orchestration cycle (resumes a drained run)")]
    Start,

    #[command(about = "Clear the .agentx runtime, then start a fresh cycle from scratch (clean + start)")]
    Restart,

    #[command(about = "Kill the running cycle and its agents immediately (resumable)")]
    Stop,

    #[command(about = "Stop the running cycle cleanly after the current turn (resumable)")]
    Drain,

    #[command(about = "Clear the .agentx runtime files, keeping the directory layout")]
    Clean,

    #[command(about = "Print a clean snapshot of the project: config, paths, classification, journey, sessions")]
    Info,

    #[command(about = "Wipe and re-seed the global training center (~/.agentx) from the binary")]
    Reset,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct App;

#[derive(Clone, Copy, Debug, Default)]
pub struct Project;

#[derive(Clone, Copy, Debug, Default)]
pub struct Compose;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ui;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    #[default]
    Idle,
    Requires,
    Tasks,
    Tests,
    Finalize,
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
    pub primed: bool,
    pub intake_done: bool,
    pub phase: Phase,
    pub status: Status,
    pub current_task: String,
    pub current_agent: String,
    pub current_round: u32,
    pub manager_review: String,
    pub task_status: BTreeMap<String, String>,
    pub agents_done: Vec<String>,
    pub agents_pending: Vec<String>,
    pub blocked: Vec<String>,
    pub last_action: String,
    pub started_at: String,
    pub updated_at: String,
}

pub struct Orchestrator<'a> {
    pub cfg: &'a Config,
    pub journey: Journey,
    pub sessions: HashMap<String, String>,
}

pub enum Halt {
    Drained,
    Failed(AppError),
}

pub type Flow<T> = Result<T, Halt>;
