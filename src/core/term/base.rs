use std::io::{self, IsTerminal, Read, Write};
use std::os::fd::AsFd;
use nix::sys::termios::{LocalFlags, SetArg, tcgetattr, tcsetattr};

use crate::core::error::{AppError, AppResult};
use super::arch::{Key, RawGuard, Term};

impl Term {

    pub fn is_tty () -> bool {

        io::stdin().is_terminal() && io::stdout().is_terminal()

    }

    pub(super) fn enter_raw () -> AppResult<RawGuard> {

        let stdin = io::stdin();

        let original = tcgetattr(stdin.as_fd()).map_err(|error| AppError::message(format!("terminal init failed: {error}")))?;

        let mut raw = original.clone();
        raw.local_flags.remove(LocalFlags::ICANON | LocalFlags::ECHO | LocalFlags::ISIG);

        tcsetattr(stdin.as_fd(), SetArg::TCSANOW, &raw).map_err(|error| AppError::message(format!("terminal init failed: {error}")))?;

        Ok(RawGuard { original })

    }

    pub(super) fn read_key ( reader: &mut impl Read ) -> Key {

        let mut byte = [0u8; 1];

        if reader.read(&mut byte).unwrap_or(0) == 0 { return Key::Cancel; }

        match byte[0] {
            b'\r' | b'\n'        => Key::Enter,
            3 | b'q' | b'Q'      => Key::Cancel,
            b'k' | b'K'          => Key::Up,
            b'j' | b'J'          => Key::Down,
            0x1b                 => Self::read_escape(reader),
            _                    => Key::Other,
        }

    }

    fn read_escape ( reader: &mut impl Read ) -> Key {

        let mut seq = [0u8; 2];

        if reader.read(&mut seq).unwrap_or(0) < 2 { return Key::Cancel; }

        match &seq {
            b"[A" => Key::Up,
            b"[B" => Key::Down,
            _     => Key::Other,
        }

    }

}

impl Drop for RawGuard {

    fn drop ( &mut self ) {

        let _ = tcsetattr(io::stdin().as_fd(), SetArg::TCSANOW, &self.original);

        print!("\x1b[?25h");
        let _ = io::stdout().flush();

    }

}
