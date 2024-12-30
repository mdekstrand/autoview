//! Generic binary files.

use log::*;

use crate::mime::mime_db;
use crate::programs::{find_program, run_command};
use crate::styling::FIELD_NAME;
use crate::{
    interface::*,
    styling::{styled, FILE_SIZE, FILE_TYPE},
};

/// Non-specialized text file backend.
pub struct BinfileBackend {}

struct FileMeta;
struct HexView {
    nbytes: Option<u32>,
}

struct FileSummary {
    meta: FileMeta,
    preview: HexView,
}

/// Viewer for generic binary files.
impl FileViewer for BinfileBackend {
    fn make_view(&self, req: &FileRequest, mode: &Option<ViewType>) -> Option<Box<dyn FileView>> {
        let db = mime_db();
        if db.is_subtype(&req.mime_type, "application/octet-stream") {
            match mode {
                Some(ViewType::Meta) => Some(Box::new(FileMeta)),
                Some(ViewType::Full) => Some(Box::new(HexView { nbytes: None })),
                Some(ViewType::Head) => Some(Box::new(HexView { nbytes: Some(256) })),
                None => Some(Box::new(FileSummary {
                    meta: FileMeta,
                    preview: HexView { nbytes: Some(256) },
                })),
            }
        } else {
            None
        }
    }
}

impl FileView for FileMeta {
    fn display(&self, req: &FileRequest, _options: &ViewOptions) -> Result<(), ViewError> {
        let db = mime_db();

        print!("file of type {}", styled(&req.mime_type, &FILE_TYPE));

        if let Some(bytes) = req.file_size() {
            print!(
                " ({})",
                styled(format!("{}", friendly::bytes(bytes)), &FILE_SIZE)
            );
        }
        println!();

        if let Some(desc) = db.description(&req.mime_type) {
            println!("{}: {}", styled("Type description", &FIELD_NAME), desc);
        };
        if let Some(size) = req.file_size() {
            println!("{}: {}", styled("Size", &FIELD_NAME), friendly::bytes(size));
        }
        Ok(())
    }
}

impl FileView for HexView {
    fn display(&self, req: &FileRequest, _options: &ViewOptions) -> Result<(), ViewError> {
        let cmd = if let Some(mut xxd) = find_program("xxd")? {
            info!("previewing with xxd");
            if let Some(l) = self.nbytes {
                xxd.args(&["-l".into(), format!("{}", l)]);
            }
            xxd.arg(&req.path);
            xxd
        } else if let Some(mut hd) = find_program("hexdump")? {
            info!("previewing with hexdump");
            if let Some(l) = self.nbytes {
                hd.args(&["-n".into(), format!("{}", l)]);
            }
            hd.args(&req.path);
            hd
        } else {
            error!("no usable hex viewer found");
            return Err(ViewError::Unspecified("no hex viewer".into()));
        };
        run_command(cmd)?;
        Ok(())
    }
}

impl FileView for FileSummary {
    fn display(&self, req: &FileRequest, options: &ViewOptions) -> Result<(), ViewError> {
        self.meta.display(req, options)?;
        println!("{}", styled("Initial content:", &FIELD_NAME));
        self.preview.display(req, options)?;
        Ok(())
    }
}
