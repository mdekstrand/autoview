use crate::interface::FileViewer;

mod binfile;
mod directory;
mod image;
mod text;

static BUILTIN_BACKENDS: &'static [&(dyn FileViewer + Send + Sync)] = &[
    &directory::DirBackend {},
    &image::ImageBackend,
    &text::TextBackend {},
    &binfile::BinfileBackend {},
];

/// Get the registered backends.
pub fn backends() -> Vec<&'static (dyn FileViewer + Send + Sync)> {
    Vec::from(BUILTIN_BACKENDS)
}
