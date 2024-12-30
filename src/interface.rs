//! Interface between AutoView and the backends.
use std::error::Error;
use std::fs::Metadata;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use colorchoice::ColorChoice;
use thiserror::Error;

use crate::programs::ProgramError;

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
    #[error("external program error: {0}")]
    External(#[from] ProgramError),
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

impl ViewError {
    pub fn wrap<E: Error + Send + Sync + 'static>(err: E) -> ViewError {
        ViewError::Wrapped(Box::new(err))
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
}

/// Options for a view.
#[allow(dead_code)]
#[derive(Clone)]
pub struct ViewOptions {
    /// Whether to use a long display with more details.
    pub long_display: bool,
    /// The requested view speed.
    pub speed: ViewSpeed,
    /// The user's sepcified choice of color mode.
    ///
    /// Styling functions will automatically respect the color choice, this
    /// makes it available for controlling other programs.
    pub color_choice: ColorChoice,
    /// Whether color display is enabled.  This is the result of resolving
    /// [color_choice] with the current terminal settings.
    ///
    /// Styling functions will automatically respect the color choice, this
    /// makes it available for controlling other programs.
    pub color_enabled: bool,
}

/// Interface for file view backends.
///
/// A viewer is selected by processing the viewers in definition order and
/// checking each one with [FileViewer::can_view].
pub trait FileViewer {
    /// Obtain a view if this backend can supply one.
    ///
    /// If `mode` is `None`, the backend should return a default view for this
    /// file, if it can supply one.
    fn make_view(&self, req: &FileRequest, mode: &Option<ViewType>) -> Option<Box<dyn FileView>>;
}

/// Implementation of a single file view request.
pub trait FileView {
    fn display(&self, req: &FileRequest, options: &ViewOptions) -> Result<(), ViewError>;
}

impl FileRequest {
    pub fn file_size(&self) -> Option<u64> {
        self.meta.as_ref().map(|m| m.size())
    }
}
