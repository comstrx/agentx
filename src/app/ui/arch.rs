use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::time::Instant;
use parking_lot::Mutex;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ui;

pub(crate) struct Loader {
    pub(crate) active: AtomicBool,
    pub(crate) live: AtomicBool,
    pub(crate) frame: AtomicUsize,
    pub(crate) start: Mutex<Instant>,
    pub(crate) label: Mutex<String>,
}
