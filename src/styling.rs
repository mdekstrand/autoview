use std::{
    fmt::{Display, Formatter},
    sync::atomic::{AtomicBool, Ordering},
};

use anstyle::Style;
use anstyle::{AnsiColor, Color};

pub static FIELD_NAME: Style = Style::new().bold();
pub static EXTRA_MARKER: Style = Style::new().dimmed();
pub static FILE_SIZE: Style = Style::new().fg_color(acolor(AnsiColor::Cyan));
pub static FILE_TYPE: Style = Style::new().fg_color(acolor(AnsiColor::Magenta));

static COLOR_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn set_color_enabled(enabled: bool) {
    COLOR_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn color_enabled() -> bool {
    COLOR_ENABLED.load(Ordering::Relaxed)
}

/// Wrap text in styling.
pub fn styled<'s, S: AsRef<str>>(text: S, style: &'s Style) -> StyleDisplay<'s> {
    StyleDisplay {
        text: text.as_ref().to_string(),
        style,
    }
}

/// Wrapper to display a styled texts.
pub struct StyleDisplay<'s> {
    text: String,
    style: &'s Style,
}

impl<'t> Display for StyleDisplay<'t> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{:#}", self.style, self.text, self.style)
    }
}

const fn acolor(color: AnsiColor) -> Option<Color> {
    Some(Color::Ansi(color))
}
