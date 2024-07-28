//! Interface between AutoView and the backends.
use std::error::Error;
use std::fs::Metadata;
use std::io;
use std::path::PathBuf;

use colorchoice::ColorChoice;
use thiserror::Error;

use crate::views::meta::FileMetaDisplay;

/// Request for speed of operations.
///
/// This is primarily respected by the meta operation — viewing often needs to
/// display data. The exact interpretation of these is up to individual
/// backends.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ViewSpeed {
    /// Default speed limit — should return in a second or two.
    Default,
    /// Avoid all slow operations.
    Fast,
    /// Allow slow operations (e.g. decompressing files).
    Slow,
}

/// Enum for the different view types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ViewType {
    /// View file metadata.
    Meta,
    /// View the first entries of a file.
    Head,
    /// View the full file.
    Full,
}

/// Errors from viewing backends.
#[derive(Debug, Error)]
pub enum ViewError {
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
    #[error("Wrapped error: {0}")]
    Wrapped(Box<dyn Error + Send + Sync>),
    #[error("view error: {0}")]
    Unspecified(String),
}

impl From<String> for ViewError {
    fn from(value: String) -> Self {
        ViewError::Unspecified(value)
    }
}

impl From<&str> for ViewError {
    fn from(value: &str) -> Self {
        ViewError::Unspecified(value.to_string())
    }
}

/// Information to request a file view.
#[derive(Debug, Clone)]
pub struct FileRequest {
    /// Path to the file to display.
    pub path: PathBuf,
    /// File metadata (from [std::fs::metadata]).
    pub meta: Option<Metadata>,
    /// The file's MIME type.
    pub mime_type: String,
    /// Whether to use a long display with more details.
    pub long_display: bool,
    /// The requested view speed.
    pub speed: ViewSpeed,
    /// Whether to display color in this view.
    pub color: ColorChoice,
}

/// Interface for file view backends.
///
/// A viewer is selected by processing the viewers in definition order and
/// checking each one with [FileViewer::can_view].
pub trait FileViewer {
    /// Query whether this viewer can view the file in the specified mode.
    fn can_view(&self, req: &FileRequest, mode: &Option<ViewType>) -> bool;

    /// Get the default view operation for this backend.
    fn default_view(&self) -> ViewType;

    /// Get the metadata from this backend.
    fn meta_view(&self, req: &FileRequest) -> Result<FileMetaDisplay, ViewError>;
}
