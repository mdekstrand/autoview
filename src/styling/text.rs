use std::borrow::Cow;

use anstyle::Style;

/// Text with a style (maybe).
#[derive(Debug, Clone)]
pub enum StyledText {
    Unstyled(String),
    Styled(String, &'static str),
    LiteralStyled(String, Style),
}

/// List of styled text.
pub type StyledList = Vec<StyledText>;

impl From<String> for StyledText {
    fn from(value: String) -> Self {
        unstyled(&value)
    }
}

impl From<&str> for StyledText {
    fn from(value: &str) -> Self {
        unstyled(&value)
    }
}

impl<'a> From<Cow<'a, str>> for StyledText {
    fn from(value: Cow<'a, str>) -> Self {
        unstyled(value)
    }
}

/// An unstyled space.
pub fn space() -> StyledText {
    return unstyled(" ");
}

/// A block of unstyled text.
pub fn unstyled<S: AsRef<str>>(text: S) -> StyledText {
    StyledText::Unstyled(text.as_ref().to_string())
}

/// Style text with a stylesheet style.
pub fn styled<S: AsRef<str>>(text: S, style: &'static str) -> StyledText {
    StyledText::Styled(text.as_ref().to_string(), style)
}

/// Style text a specific style.  Using this is not recommended.
pub fn literal_styled<S: AsRef<str>>(text: S, style: &Style) -> StyledText {
    StyledText::LiteralStyled(text.as_ref().to_string(), style.clone())
}
