use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub static RUNTIME: OnceLock<Runtime> = OnceLock::new();

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rt;
