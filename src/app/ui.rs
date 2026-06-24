use std::io::{IsTerminal, Write};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use owo_colors::{OwoColorize, Style};
use parking_lot::Mutex;

use crate::config::consts::FRAMES;
use super::arch::{Loader, Ui};

static COLOR: OnceLock<bool> = OnceLock::new();
static LOADER: OnceLock<Loader> = OnceLock::new();

impl Ui {

    pub fn blank () {

        Self::line("");

    }

    pub fn rule ( label: &str ) {

        let head = format!("── {label} ");
        let fill = 68usize.saturating_sub(head.chars().count()).max(2);

        Self::line("");
        Self::line(&Self::paint(&format!("{head}{}", "─".repeat(fill)), Style::new().bright_cyan().bold()));
        Self::line("");

    }

    pub fn step ( message: &str ) {

        Self::emit(0, "▸", Style::new().bright_cyan().bold(), message);
        Self::busy(message);

    }

    pub fn ok ( message: &str ) {

        Self::emit(0, "✓", Style::new().bright_green().bold(), message);

    }

    pub fn warn ( message: &str ) {

        Self::emit(0, "▲", Style::new().yellow().bold(), message);

    }

    pub fn info ( message: &str ) {

        Self::emit(0, "·", Style::new().bright_black().bold(), message);

    }

    pub fn arrow ( depth: usize, message: &str ) {

        Self::emit(depth, "▸", Style::new().bright_cyan().bold(), message);
        Self::busy(message);

    }

    pub fn tick ( depth: usize, message: &str ) {

        Self::emit(depth, "✓", Style::new().bright_green().bold(), message);

    }

    pub fn cross ( depth: usize, message: &str ) {

        Self::emit(depth, "✗", Style::new().bright_red().bold(), message);

    }

    pub fn bang ( depth: usize, message: &str ) {

        Self::emit(depth, "▲", Style::new().yellow().bold(), message);

    }

    pub fn dot ( depth: usize, message: &str ) {

        Self::emit(depth, "·", Style::new().bright_black().bold(), &Self::paint(message, Style::new().bright_black()));

    }

    pub fn beat ( depth: usize, message: &str ) {

        Self::emit(depth, "↻", Style::new().bright_magenta().bold(), message);

    }

    pub fn detail ( label: &str, value: &str ) {

        Self::line(&format!("      {}  {value}", Self::paint(&format!("{label:<13}"), Style::new().bright_black())));

    }

    pub fn title ( text: &str ) {

        Self::line(&Self::paint(text, Style::new().bright_magenta().bold()));

    }

    pub fn head ( text: &str ) {

        Self::line(&Self::paint(text, Style::new().bright_cyan().bold()));

    }

    pub fn field ( label: &str, value: &str ) {

        Self::line(&format!("  {}  {value}", Self::paint(&format!("{label:<15}"), Style::new().bright_black())));

    }

    pub fn pair ( key: &str, value: &str ) {

        Self::line(&format!("  {} {value}", Self::paint(&format!("{key:<18}="), Style::new().bright_black())));

    }

    pub fn item ( value: &str ) {

        Self::line(&format!("      {}", Self::paint(value, Style::new().bright_black())));

    }

    pub fn role ( label: &str, members: &str ) {

        Self::line(&format!("      {}  {}{}", Self::paint("▸", Style::new().bright_cyan().bold()), Self::paint(&format!("{label:<12}"), Style::new().bright_black()), Self::paint(members, Style::new().bright_green())));

    }

    pub fn task ( name: &str, status: &str ) {

        let ( glyph, style ) = match status {
            "shipped"   => ( "✓", Style::new().bright_green().bold() ),
            "executing" => ( "↻", Style::new().bright_magenta().bold() ),
            "blocked"   => ( "✗", Style::new().bright_red().bold() ),
            _           => ( "·", Style::new().bright_black().bold() ),
        };

        Self::line(&format!("      {}  {}{}", Self::paint(glyph, style), Self::paint(&format!("{name:<34}"), Style::new().bright_black()), Self::paint(status, style)));

    }

    pub fn state ( label: &str, on: bool, value: &str ) {

        let glyph = match on {
            true => Self::paint("●", Style::new().bright_green().bold()),
            false => Self::paint("○", Style::new().bright_black()),
        };

        Self::line(&format!("  {glyph}  {}  {value}", Self::paint(&format!("{label:<11}"), Style::new().bright_black())));

    }

    pub fn worker ( label: &str, value: &str, on: bool ) {

        let glyph = match on {
            true => Self::paint("●", Style::new().bright_green().bold()),
            false => " ".to_string(),
        };

        let tail = match on {
            true => Self::paint("   ← active", Style::new().bright_green().bold()),
            false => String::new(),
        };

        Self::line(&format!("  {glyph}  {}  {value}{tail}", Self::paint(&format!("{label:<18}"), Style::new().bright_black())));

    }

    pub fn bar ( done: usize, total: usize ) -> String {

        let width = 18;

        let ( filled, percent ) = match total {
            0 => ( 0, 0 ),
            _ => ( done * width / total, done * 100 / total ),
        };

        let full = Self::paint(&"█".repeat(filled), Style::new().bright_green());
        let rest = Self::paint(&"░".repeat(width.saturating_sub(filled)), Style::new().bright_black());

        format!("{full}{rest}  {percent}%")

    }

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

    fn line ( text: &str ) {

        match LOADER.get() {
            Some(loader) if loader.active.load(Ordering::Relaxed) => loader.render(Some(text)),
            _ => { let _ = writeln!(std::io::stdout(), "{text}"); }
        }

    }

    fn emit ( depth: usize, glyph: &str, style: Style, message: &str ) {

        Self::line(&format!("{}{}  {message}", "  ".repeat(depth + 1), Self::paint(glyph, style)));

    }

    fn busy ( label: &str ) {

        if let Some(loader) = LOADER.get() && loader.active.load(Ordering::Relaxed) {

            *loader.label.lock() = label.to_string();
            *loader.start.lock() = Instant::now();

        }

    }

    fn clock ( secs: u64 ) -> String {

        if secs >= 60 { format!("{}m{:02}s", secs / 60, secs % 60) } else { format!("{secs}s") }

    }

    fn paint ( text: &str, style: Style ) -> String {

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
