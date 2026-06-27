use nix::sys::termios::Termios;

pub struct Term;

pub enum Key {
    Up,
    Down,
    Enter,
    Cancel,
    Other,
}

pub(super) struct RawGuard {
    pub(super) original: Termios,
}
