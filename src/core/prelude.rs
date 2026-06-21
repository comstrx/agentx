//! Common imports for the upper layers: `use crate::core::prelude::*;`.
//!
//! Deliberately excludes `fs::{File, Dir, Path}` (they collide with `std`)
//! and the rarely-used `net`/`yaml` types — import those explicitly.

pub use crate::core::context::{AppContext, ContextValue};
pub use crate::core::error::{AppError, AppExitCode, AppResult};
pub use crate::core::support::list::List;
pub use crate::core::support::num::Num;
pub use crate::core::support::parse::{Json, Parse, Toml};
pub use crate::core::support::proc::Proc;
pub use crate::core::support::rt::Rt;
pub use crate::core::support::text::Text;
pub use crate::core::support::thread::Thread;
pub use crate::core::support::time::Time;
