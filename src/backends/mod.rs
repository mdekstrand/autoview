use crate::interface::FileViewer;

mod directory;
mod text;

static BUILTIN_BACKENDS: &'static [&(dyn FileViewer + Send + Sync)] =
    &[&directory::DirBackend {}, &text::TextBackend {}];

/// Get the registered backends.
pub fn backends() -> Vec<&'static (dyn FileViewer + Send + Sync)> {
    Vec::from(BUILTIN_BACKENDS)
}
