use std::io::Write;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use owo_colors::{OwoColorize, Style};
use parking_lot::Mutex;

use crate::config::base::consts::{BUSY_WIDTH, FRAMES, FRAMES_PLAIN, GLYPH_BEAT, GLYPH_COOL, GLYPH_FAIL, GLYPH_INFO, GLYPH_OK, GLYPH_PARTY, GLYPH_RAGE, GLYPH_STEP, GLYPH_STUDY, GLYPH_THINK, GLYPH_WARN, TICK_MS};
use crate::core::term::Term;
use crate::core::text::Text;
use crate::app::{Loader, Mark, Ui};

static LOADER: OnceLock<Loader> = OnceLock::new();
static LAST_BLANK: AtomicBool = AtomicBool::new(true);
static PENDING: AtomicBool = AtomicBool::new(false);

impl Ui {

    pub fn loading ( label: &str ) {

        if !Term::ansi() { return; }

        let loader = LOADER.get_or_init(Loader::new);

        *loader.label.lock() = label.to_string();
        *loader.start.lock() = Instant::now();
        loader.frame.store(0, Ordering::Relaxed);

        if loader.active.swap(true, Ordering::Relaxed) { return; }

        thread::spawn(|| {

            loop {

                thread::sleep(Duration::from_millis(TICK_MS));

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

        if !Term::ansi() { return; }

        let mut out = std::io::stdout().lock();
        let _ = write!(out, "{}", if visible { "\x1b[?25h" } else { "\x1b[?25l" });
        let _ = out.flush();

    }

    pub fn home () {

        if !Term::ansi() { return; }

        let mut out = std::io::stdout().lock();
        let _ = write!(out, "\x1b[H\x1b[0J");
        let _ = out.flush();

    }

    pub fn screen ( alt: bool ) {

        if !Term::ansi() { return; }

        let mut out = std::io::stdout().lock();
        let _ = write!(out, "{}", if alt { "\x1b[?1049h\x1b[H" } else { "\x1b[?1049l" });
        let _ = out.flush();

    }

    pub(super) fn line ( text: &str ) {

        if text.trim().is_empty() {

            PENDING.store(true, Ordering::Relaxed);

            return;

        }

        if PENDING.swap(false, Ordering::Relaxed) && !LAST_BLANK.load(Ordering::Relaxed) { Self::emit_raw(""); }

        Self::emit_raw(text);

    }

    pub(crate) fn pad ( count: usize ) {

        PENDING.store(false, Ordering::Relaxed);

        for _ in 0..count { Self::emit_raw(""); }

    }

    pub(super) fn settle () {

        if PENDING.swap(false, Ordering::Relaxed) && !LAST_BLANK.load(Ordering::Relaxed) { Self::emit_raw(""); }

    }

    pub(super) fn printed () {

        PENDING.store(false, Ordering::Relaxed);
        LAST_BLANK.store(false, Ordering::Relaxed);

    }

    fn emit_raw ( text: &str ) {

        LAST_BLANK.store(text.trim().is_empty(), Ordering::Relaxed);

        match LOADER.get() {
            Some(loader) if loader.active.load(Ordering::Relaxed) => loader.render(Some(text)),
            _ => { let _ = writeln!(std::io::stdout(), "{text}"); }
        }

    }

    pub(super) fn emit ( depth: usize, glyph: &str, style: Style, message: &str ) {

        Self::line(&format!("{}{}  {message}", "  ".repeat(depth + 1), Self::paint(glyph, style)));

    }

    pub(super) fn glyph ( mark: Mark ) -> ( &'static str, Style ) {

        match mark {
            Mark::Ok    => ( Self::pick(GLYPH_OK),    Style::new().bright_green().bold() ),
            Mark::Fail  => ( Self::pick(GLYPH_FAIL),  Style::new().bright_red().bold() ),
            Mark::Warn  => ( Self::pick(GLYPH_WARN),  Style::new().bright_yellow().bold() ),
            Mark::Info  => ( Self::pick(GLYPH_INFO),  Style::new().bright_blue().bold() ),
            Mark::Step  => ( Self::pick(GLYPH_STEP),  Style::new().bright_cyan().bold() ),
            Mark::Beat  => ( Self::pick(GLYPH_BEAT),  Style::new().bright_magenta().bold() ),
            Mark::Cool  => ( Self::pick(GLYPH_COOL),  Style::new().bright_green().bold() ),
            Mark::Rage  => ( Self::pick(GLYPH_RAGE),  Style::new().bright_red().bold() ),
            Mark::Think => ( Self::pick(GLYPH_THINK), Style::new().bright_cyan().bold() ),
            Mark::Party => ( Self::pick(GLYPH_PARTY), Style::new().bright_magenta().bold() ),
            Mark::Study => ( Self::pick(GLYPH_STUDY), Style::new().bright_cyan().bold() ),
        }

    }

    pub(super) fn pick ( pair: ( &'static str, &'static str ) ) -> &'static str {

        if Term::icons() { pair.0 } else { pair.1 }

    }

    pub(crate) fn mark ( depth: usize, mark: Mark, message: &str ) {

        let ( glyph, style ) = Self::glyph(mark);

        Self::emit(depth, glyph, style, message);

    }

    pub(crate) fn done ( depth: usize, mark: Mark, message: &str, from: Instant ) {

        let elapsed = Self::clock(from.elapsed().as_secs());

        Self::mark(depth, mark, &format!("{message}  {}", Self::paint(&format!("({elapsed})"), Self::muted())));

    }

    pub(crate) fn working ( depth: usize, mark: Mark, message: &str ) {

        Self::mark(depth, mark, message);
        Self::busy(message);

    }

    pub(super) fn accent () -> Style {

        Style::new().bright_cyan().bold()

    }

    pub(super) fn brand () -> Style {

        Style::new().bright_magenta().bold()

    }

    pub(super) fn muted () -> Style {

        Style::new().bright_blue()

    }

    pub(super) fn good () -> Style {

        Style::new().bright_green()

    }

    pub(super) fn busy ( label: &str ) {

        if let Some(loader) = LOADER.get() && loader.active.load(Ordering::Relaxed) {

            *loader.label.lock() = Text::plain(label);
            *loader.start.lock() = Instant::now();

        }

    }

    fn clock ( secs: u64 ) -> String {

        match secs {
            s if s >= 3600 => format!("{}h {}m {}s", s / 3600, ( s % 3600 ) / 60, s % 60),
            s if s >= 60   => format!("{}m {}s", s / 60, s % 60),
            s              => format!("{s}s"),
        }

    }

    pub(super) fn paint ( text: &str, style: Style ) -> String {

        if Self::tinted() { text.style(style).to_string() } else { text.to_string() }

    }

    fn tinted () -> bool {

        Term::colors()

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

        let frames: &[&str] = if Term::icons() { &FRAMES } else { &FRAMES_PLAIN };

        let glyph = frames[self.frame.load(Ordering::Relaxed) % frames.len()];
        let label = Self::clip(&self.label.lock());
        let elapsed = Ui::clock(self.start.lock().elapsed().as_secs());

        format!("  {}  {label}  {}", Ui::paint(glyph, Ui::accent()), Ui::paint(&format!("· {elapsed}"), Ui::muted()))

    }

    fn clip ( label: &str ) -> String {

        if label.chars().count() <= BUSY_WIDTH { return label.to_string(); }

        let mark = if Term::icons() { "…" } else { "..." };
        let head: String = label.chars().take(BUSY_WIDTH.saturating_sub(mark.chars().count())).collect();

        format!("{}{mark}", head.trim_end())

    }

}
