//! Styling for autoview outputs based on [anstyle].

use std::fmt::{Display, Formatter};

use anstyle::{AnsiColor, Color, Style};

use super::names::*;
use super::text::{StyledList, StyledText};

/// A stylesheet for styling displays.
///
/// This is passed to backends to tell them what styles to use for their
/// displays, particualrly metadata displays.  Use beyond metadata is not
/// consistent between backends, many will use their own internal styling.
#[derive(Default, Clone)]
pub struct StyleSheet {
    styles: Vec<(&'static str, Style)>,
}

/// Wrapper to display a styled list.
pub struct StyleListDisplay<'a, 't> {
    styles: &'a StyleSheet,
    text: &'t StyledList,
}

/// Wrapper to display a styled texts.
pub struct StyleDisplay<'a, 't> {
    styles: &'a StyleSheet,
    text: &'t StyledText,
}

static DEFAULT_STYLES: &[(&str, Style)] = &[
    (FIELD_NAME, Style::new().bold()),
    (EXTRA_MARKER, Style::new().dimmed()),
    (FILE_SIZE, Style::new().fg_color(acolor(AnsiColor::Cyan))),
    (FILE_TYPE, Style::new().fg_color(acolor(AnsiColor::Magenta))),
];

impl StyleSheet {
    /// Create an empty stylesheet.
    pub const fn empty() -> StyleSheet {
        StyleSheet { styles: Vec::new() }
    }

    pub const fn create(styles: Vec<(&'static str, Style)>) -> StyleSheet {
        StyleSheet { styles }
    }

    /// Get the default terminal styles.
    pub fn term_default() -> StyleSheet {
        StyleSheet {
            styles: Vec::from(DEFAULT_STYLES),
        }
    }

    /// Look up a style. Returns the empty style if no style is found.
    pub fn lookup(&self, name: &str) -> Style {
        if let Some((_n, sty)) = self.styles.iter().find(|(n, _s)| *n == name) {
            sty.clone()
        } else {
            Style::new()
        }
    }

    /// Return a wrapper that, when displayed, will style the text.
    pub fn render<'t>(&self, text: &'t StyledText) -> StyleDisplay<'_, 't> {
        StyleDisplay {
            styles: &self,
            text,
        }
    }

    /// Return a wrapper that, when displayed, will style the text.
    pub fn render_list<'t>(&self, text: &'t StyledList) -> StyleListDisplay<'_, 't> {
        StyleListDisplay {
            styles: &self,
            text,
        }
    }
}

impl<'a, 't> Display for StyleListDisplay<'a, 't> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for elt in self.text {
            write!(f, "{}", self.styles.render(elt))?;
        }
        Ok(())
    }
}

impl<'a, 't> Display for StyleDisplay<'a, 't> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.text {
            StyledText::Unstyled(text) => f.write_str(text),
            StyledText::Styled(text, style) => {
                let style = self.styles.lookup(*style);
                write!(f, "{style}{text}{style:#}")
            }
            StyledText::LiteralStyled(text, style) => {
                write!(f, "{style}{text}{style:#}")
            }
        }
    }
}

const fn acolor(color: AnsiColor) -> Option<Color> {
    Some(Color::Ansi(color))
}
