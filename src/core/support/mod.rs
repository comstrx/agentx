//! A personal std-lib — neutral, fast, ergonomic. Knows nothing about agentx.
//!
//! Each child is a self-contained module exposing a zero-sized namespace
//! (`Text`, `Time`, `File`, ...) whose API is spread across concern files.

pub mod fs;
pub mod list;
pub mod net;
pub mod num;
pub mod parse;
pub mod proc;
pub mod rt;
pub mod text;
pub mod thread;
pub mod time;
