/// Structured parallelism over `std::thread::scope` — no runtime, no clones.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Thread;
