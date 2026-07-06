use std::path::PathBuf;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{ArgAction, Parser, Subcommand};
use clap_complete::Shell;

use crate::config::base::consts::TOOL;

const HELP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::BrightBlue.on_default());

pub(super) const HELP_TEMPLATE: &str = "{about-with-newline}\n{usage-heading}\n\n  {usage}\n\n{all-args}{after-help}";

#[derive(Parser)]
#[command(
    name = TOOL,
    version,
    about = "Hierarchical multi-agent orchestrator - turns requirements into reviewed, gate-kept code.",
    long_about = "Hierarchical multi-agent orchestrator - turns requirements into reviewed, gate-kept code.",
    propagate_version = true,
    arg_required_else_help = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    styles = HELP_STYLES,
    help_template = HELP_TEMPLATE,
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
        help = "Bind to a project node by name or its list number (overrides [project].inspire)",
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
        short = 'd',
        long = "description",
        global = true,
        value_name = "TEXT",
        help = "A short free-text description of the project to guide the manager (overrides [project].description)",
    )]
    pub description: Option<String>,

    #[arg(
        long = "lint",
        global = true,
        value_name = "BOOL",
        help = "Gate includes a lint / static-analysis pillar (overrides [option].lint): true/false, 1/0, yes/no",
    )]
    pub lint: Option<String>,

    #[arg(
        long = "format",
        global = true,
        value_name = "BOOL",
        help = "Gate includes a non-mutating format check + executors keep code formatted (overrides [option].format)",
    )]
    pub format: Option<String>,

    #[arg(
        long = "audits",
        global = true,
        value_name = "BOOL",
        help = "Run the audit phase after tasks (auditors hunt integration/quality defects, write remediation tasks) (overrides [option].audits)",
    )]
    pub audits: Option<String>,

    #[arg(
        short = 't',
        long = "tests",
        global = true,
        value_name = "BOOL",
        help = "Run the tests phase + gate test pillar (overrides [option].tests): true/false, 1/0, yes/no",
    )]
    pub tests: Option<String>,

    #[arg(
        long = "fuzzes",
        global = true,
        value_name = "BOOL",
        help = "Run the fuzzes phase — real fuzzing of the executed work (overrides [option].fuzzes)",
    )]
    pub fuzzes: Option<String>,

    #[arg(
        long = "benches",
        global = true,
        value_name = "BOOL",
        help = "Run the benches phase — real benchmarks for the executed work (overrides [option].benches)",
    )]
    pub benches: Option<String>,

    #[arg(
        long = "examples",
        global = true,
        value_name = "BOOL",
        help = "Run the examples phase — real runnable examples of the executed work (overrides [option].examples)",
    )]
    pub examples: Option<String>,

    #[arg(
        long = "comments",
        global = true,
        value_name = "BOOL",
        help = "Executors add inline comments explaining non-obvious logic; off = zero inline comments (overrides [option].comments)",
    )]
    pub comments: Option<String>,

    #[arg(
        long = "doc-blocks",
        global = true,
        value_name = "BOOL",
        help = "Document every public item in the native doc format (overrides [option].doc_blocks): true/false, 1/0, yes/no",
    )]
    pub doc_blocks: Option<String>,

    #[arg(
        long = "doc-contracts",
        global = true,
        value_name = "BOOL",
        help = "Document non-obvious units that don't return explicit types (overrides [option].doc_contracts): true/false, 1/0, yes/no",
    )]
    pub doc_contracts: Option<String>,

    #[arg(
        short = 'y',
        long = "yes",
        global = true,
        help = "Assume yes on every confirmation prompt (e.g. reset) — for CI and scripts",
    )]
    pub yes: bool,

    #[arg(
        short = 'f',
        long = "force",
        global = true,
        help = "Never pause intake on an objection — conflicts resolve to the narrowest assumption, recorded in the backlog",
    )]
    pub force: bool,

    #[arg(
        short = 'b',
        long = "background",
        visible_alias = "bg",
        global = true,
        help = "Run new/start/restart detached in the background; drive it with status/drain/stop",
    )]
    pub background: bool,

    #[arg(
        long = "no-train",
        global = true,
        help = "Do NOT auto-record the run into the training center when it finishes (sets [option].train = false)",
    )]
    pub no_train: bool,

    #[arg(
        long = "no-clear",
        global = true,
        help = "Do NOT auto-clear the .agentx runtime when the run finishes (sets [option].clear = false)",
    )]
    pub no_clear: bool,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Scaffold and configure the project: fill Agentx.toml from flags, create .agentx/ and agentx/")]
    Init,

    #[command(about = "Create a brand-new project of a chosen project node: scaffold it, then the manager builds the skeleton")]
    New {
        #[arg(value_name = "DIR", help = "Directory to create the new project in")]
        path: PathBuf,
    },

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

    #[command(about = "Record the finished run into the training center (manager writes a report per requirement); auto-run after a clean cycle")]
    Train,

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

    #[command(about = "Bind or switch the project's inspiration node — the training-center kind it learns from")]
    Inspire {
        #[arg(value_name = "NAME|N", help = "Project node name or its list number; omit to pick from the menu")]
        name: Option<String>,

        #[arg(long = "show", conflicts_with = "name", help = "Print the current inspiration binding and exit — no changes")]
        show: bool,
    },

    #[command(about = "Set or switch the project's quality-gate command — the check this tool runs after every code turn")]
    Gate {
        #[arg(value_name = "COMMAND", help = "The shell command; omit to type it at a prompt")]
        command: Option<String>,

        #[arg(long = "show", conflicts_with = "command", help = "Print the current gate command and exit — no changes")]
        show: bool,
    },

    #[command(about = "Print a clean snapshot of the project: config, pids, paths, classification, journey, sessions")]
    Info,

    #[command(about = "Show the run status once: state, engines, journey progress, live-log tail, stats, and what's happening now")]
    Status,

    #[command(about = "Watch the status live — full-screen refresh every second until you exit (Ctrl+C)")]
    Watch,

    #[command(about = "Check that every required agent CLI and tool is installed and runnable before a run")]
    Doctor,

    #[command(about = "Print the exact assembled prompts the agents receive — real project values, zero agent calls")]
    Compose {
        #[arg(short = 'r', long = "role", value_name = "ROLE", help = "Only this role: manager, architect, executor, auditor, tester, bencher, exampler, fuzzer (aliases + phase names work)")]
        role: Option<String>,

        #[arg(short = 'p', long = "phase", value_name = "PHASE", help = "Only this turn: init, confirm, intake, convert, work, discover, gate, review, finalize — or a single fragment: fence, evidence, report")]
        phase: Option<String>,

        #[arg(short = 'o', long = "out", value_name = "FILE", help = "Write the blocks to FILE instead of the terminal")]
        out: Option<PathBuf>,
    },

    #[command(about = "Re-extract the shipped knowledge nodes into ~/.agentx, keeping learned history")]
    Sync,

    #[command(about = "Wipe and re-seed the global training center (~/.agentx) from the binary — always asks first; headless needs -y")]
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
pub struct Flags<'a> {
    pub inspire: Option<&'a str>,
    pub description: Option<&'a str>,
    pub gate: Option<&'a str>,
    pub lint: Option<&'a str>,
    pub format: Option<&'a str>,
    pub audits: Option<&'a str>,
    pub tests: Option<&'a str>,
    pub fuzzes: Option<&'a str>,
    pub benches: Option<&'a str>,
    pub examples: Option<&'a str>,
    pub comments: Option<&'a str>,
    pub doc_blocks: Option<&'a str>,
    pub doc_contracts: Option<&'a str>,
    pub ignore: &'a [PathBuf],
    pub include: &'a [PathBuf],
    pub yes: bool,
    pub force: bool,
    pub background: bool,
    pub no_train: bool,
    pub no_clear: bool,
}
