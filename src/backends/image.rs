use image::{ImageDecoder, ImageFormat, ImageReader};

use crate::interface::{FileRequest, FileView, FileViewer, ViewError, ViewOptions, ViewType};
use crate::mime::mime_db;
use crate::styling::*;

pub struct ImageBackend;
pub struct ImageMeta;

impl FileViewer for ImageBackend {
    fn make_view(&self, req: &FileRequest, mode: &Option<ViewType>) -> Option<Box<dyn FileView>> {
        let mode = mode.clone().unwrap_or(ViewType::Meta);
        if mode == ViewType::Meta && req.mime_type.starts_with("image/") {
            if let Ok(_fmt) = ImageFormat::from_path(&req.path) {
                return Some(Box::new(ImageMeta));
            }
        }

        None
    }
}

impl FileView for ImageMeta {
    fn display(&self, req: &FileRequest, _options: &ViewOptions) -> Result<(), ViewError> {
        let db = mime_db();
        let reader = ImageReader::open(&req.path)?;
        let decoder = reader.into_decoder().map_err(ViewError::wrap)?;
        let (w, h) = decoder.dimensions();
        println!(
            "{} with {} pixels ({:?})",
            styled(
                db.description(&req.mime_type).unwrap_or("Unknown image"),
                &FILE_TYPE
            ),
            styled(format!("{}x{}", w, h), &FILE_SIZE),
            decoder.color_type()
        );
        Ok(())
    }
}
