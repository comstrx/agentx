use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::time::Instant;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{ArgAction, Parser, Subcommand};
use clap_complete::Shell;
use parking_lot::Mutex;
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
    disable_help_subcommand = true,
    disable_version_flag = true,
    styles = HELP_STYLES,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(
        short = 'v',
        short_alias = 'V',
        long = "version",
        global = true,
        action = ArgAction::Version,
        help = "Print version",
    )]
    pub version: Option<bool>,

    #[arg(
        short = 'C',
        long = "dir",
        global = true,
        value_name = "DIR",
        help = "Operate as if agentx were started in DIR instead of the current directory",
    )]
    pub dir: Option<PathBuf>,

    #[arg(
        short = 'i',
        long = "inspire",
        global = true,
        value_name = "NAME|N",
        help = "Bind to a training-center archetype by name or its list number (overrides [project].inspire)",
    )]
    pub inspire: Option<String>,

    #[arg(
        short = 'g',
        long = "gate",
        global = true,
        value_name = "COMMAND",
        help = "Set the quality-gate shell command (overrides [gate].command)",
    )]
    pub gate: Option<String>,

    #[arg(
        short = 't',
        long = "tests",
        global = true,
        value_name = "BOOL",
        help = "Whether verifiers write real project tests (overrides [project].tests): true/false, 1/0, yes/no",
    )]
    pub tests: Option<String>,

    #[arg(
        short = 'b',
        long = "background",
        visible_alias = "bg",
        global = true,
        help = "Run start/restart detached in the background; drive it with status/drain/stop",
    )]
    pub background: bool,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Scaffold and configure the project: fill Agentx.toml (detect inspiration + gate, model/effort defaults), create .agentx/")]
    Init,

    #[command(about = "Resolve the project root and run a full orchestration cycle (resumes a drained run)")]
    Start {
        #[arg(long = "ignore", value_name = "PATH", num_args = 1.., help = "Paths to skip during classification (merged into the persisted ignore list)")]
        ignore: Vec<PathBuf>,

        #[arg(long = "include", value_name = "PATH", num_args = 1.., help = "Paths to force into classification, overriding ignore (persisted)")]
        include: Vec<PathBuf>,
    },

    #[command(about = "Clear the .agentx runtime, then start a fresh cycle from scratch (clear + start)")]
    Restart {
        #[arg(long = "ignore", value_name = "PATH", num_args = 1.., help = "Paths to skip during classification (merged into the persisted ignore list)")]
        ignore: Vec<PathBuf>,

        #[arg(long = "include", value_name = "PATH", num_args = 1.., help = "Paths to force into classification, overriding ignore (persisted)")]
        include: Vec<PathBuf>,
    },

    #[command(about = "Kill the running cycle and its agents immediately (resumable)")]
    Stop,

    #[command(about = "Stop the running cycle cleanly after the current turn (resumable)")]
    Drain,

    #[command(about = "Clear the .agentx runtime files, keeping the directory layout")]
    Clear,

    #[command(about = "Mark files or dirs to skip during classification (persists in Agentx.toml; survives clear)")]
    Ignore {
        #[arg(value_name = "PATH", required = true, num_args = 1..)]
        paths: Vec<PathBuf>,
    },

    #[command(about = "Force files or dirs into classification, overriding ignore (unknown names land in overview)")]
    Include {
        #[arg(value_name = "PATH", required = true, num_args = 1..)]
        paths: Vec<PathBuf>,
    },

    #[command(about = "Reset the ignore/include lists and re-run classification from scratch")]
    Refresh {
        #[arg(long = "ignore", value_name = "PATH", num_args = 1.., help = "Seed the fresh ignore list after the reset")]
        ignore: Vec<PathBuf>,

        #[arg(long = "include", value_name = "PATH", num_args = 1.., help = "Seed the fresh include list after the reset")]
        include: Vec<PathBuf>,
    },

    #[command(about = "Print a clean snapshot of the project: config, paths, classification, journey, sessions")]
    Info,

    #[command(about = "Show the live run status: state, journey progress, workers, and pids")]
    Status {
        #[arg(short = 'f', long = "tail", help = "Refresh the status in place every second until Ctrl+C (live dashboard)")]
        tail: bool,
    },

    #[command(about = "Check that every required agent CLI and tool is installed and runnable before a run")]
    Doctor,

    #[command(about = "Sync the shipped training (overview/contracts/skills/requires/about) into ~/.agentx, keeping learned history")]
    Sync,

    #[command(about = "Wipe and re-seed the global training center (~/.agentx) from the binary, learned history included")]
    Reset,

    #[command(about = "Print a shell completion script (bash, zsh, fish, elvish, powershell) to stdout")]
    Completions {
        #[arg(value_name = "SHELL", help = "Shell to generate completions for")]
        shell: Shell,
    },

    #[command(about = "Print the man page in roff format to stdout")]
    Man,

    #[command(about = "Print this message, or the help of the given subcommand")]
    Help {
        #[arg(value_name = "COMMAND", help = "Subcommand to show help for")]
        command: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, Default)]
pub struct App;

#[derive(Clone, Copy, Debug, Default)]
pub struct Flags<'a> {
    pub inspire: Option<&'a str>,
    pub gate: Option<&'a str>,
    pub tests: Option<&'a str>,
    pub ignore: &'a [PathBuf],
    pub include: &'a [PathBuf],
    pub background: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Project;

#[derive(Clone, Copy, Debug, Default)]
pub struct Compose;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ui;

pub(crate) struct Loader {
    pub(crate) active: AtomicBool,
    pub(crate) live: AtomicBool,
    pub(crate) frame: AtomicUsize,
    pub(crate) start: Mutex<Instant>,
    pub(crate) label: Mutex<String>,
}

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
    Stopped,
    Failed(AppError),
}

pub type Flow<T> = Result<T, Halt>;
