use std::io::{IsTerminal, Write};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use owo_colors::{OwoColorize, Style};
use parking_lot::Mutex;

use crate::config::base::consts::FRAMES;
use crate::app::{Loader, Ui};

static COLOR: OnceLock<bool> = OnceLock::new();
static LOADER: OnceLock<Loader> = OnceLock::new();

impl Ui {

    pub fn loading ( label: &str ) {

        if !Self::tinted() { return; }

        let loader = LOADER.get_or_init(Loader::new);

        *loader.label.lock() = label.to_string();
        *loader.start.lock() = Instant::now();
        loader.frame.store(0, Ordering::Relaxed);

        if loader.active.swap(true, Ordering::Relaxed) { return; }

        thread::spawn(|| {

            loop {

                thread::sleep(Duration::from_millis(90));

                let Some(loader) = LOADER.get() else { break; };

                if !loader.active.load(Ordering::Relaxed) { break; }

                loader.frame.fetch_add(1, Ordering::Relaxed);
                loader.render(None);

            }

        });

    }

    pub fn loaded () {

        let Some(loader) = LOADER.get() else { return; };

        if !loader.active.swap(false, Ordering::Relaxed) { return; }

        let mut out = std::io::stdout().lock();

        if loader.live.swap(false, Ordering::Relaxed) {

            let _ = write!(out, "\r\x1b[2K\x1b[1A\r\x1b[2K");

        }

        let _ = out.flush();

    }

    pub fn cursor ( visible: bool ) {

        if !Self::tinted() { return; }

        let mut out = std::io::stdout().lock();
        let _ = write!(out, "{}", if visible { "\x1b[?25h" } else { "\x1b[?25l" });
        let _ = out.flush();

    }

    pub fn home () {

        if !Self::tinted() { return; }

        let mut out = std::io::stdout().lock();
        let _ = write!(out, "\x1b[H\x1b[0J");
        let _ = out.flush();

    }

    pub(super) fn line ( text: &str ) {

        match LOADER.get() {
            Some(loader) if loader.active.load(Ordering::Relaxed) => loader.render(Some(text)),
            _ => { let _ = writeln!(std::io::stdout(), "{text}"); }
        }

    }

    pub(super) fn emit ( depth: usize, glyph: &str, style: Style, message: &str ) {

        Self::line(&format!("{}{}  {message}", "  ".repeat(depth + 1), Self::paint(glyph, style)));

    }

    pub(super) fn busy ( label: &str ) {

        if let Some(loader) = LOADER.get() && loader.active.load(Ordering::Relaxed) {

            *loader.label.lock() = label.to_string();
            *loader.start.lock() = Instant::now();

        }

    }

    fn clock ( secs: u64 ) -> String {

        if secs >= 60 { format!("{}m{:02}s", secs / 60, secs % 60) } else { format!("{secs}s") }

    }

    pub(super) fn paint ( text: &str, style: Style ) -> String {

        if Self::tinted() { text.style(style).to_string() } else { text.to_string() }

    }

    fn tinted () -> bool {

        *COLOR.get_or_init(|| std::io::stdout().is_terminal())

    }

}

impl Loader {

    fn new () -> Loader {

        Loader {
            active: AtomicBool::new(false),
            live: AtomicBool::new(false),
            frame: AtomicUsize::new(0),
            start: Mutex::new(Instant::now()),
            label: Mutex::new(String::new()),
        }

    }

    fn render ( &self, text: Option<&str> ) {

        let mut out = std::io::stdout().lock();

        if text.is_none() && !self.active.load(Ordering::Relaxed) { return; }

        if self.live.load(Ordering::Relaxed) {

            let _ = write!(out, "\r\x1b[2K\x1b[1A\r\x1b[2K");

        }

        if let Some(line) = text {

            let _ = writeln!(out, "{line}");

        }

        let _ = write!(out, "\x1b[2K\n\x1b[2K{}", self.bar());

        self.live.store(true, Ordering::Relaxed);

        let _ = out.flush();

    }

    fn bar ( &self ) -> String {

        let glyph = FRAMES[self.frame.load(Ordering::Relaxed) % FRAMES.len()];
        let label = self.label.lock().clone();
        let elapsed = Ui::clock(self.start.lock().elapsed().as_secs());

        format!("  {}  {label}  {}", Ui::paint(glyph, Style::new().bright_cyan().bold()), Ui::paint(&format!("· {elapsed}"), Style::new().bright_black()))

    }

}
