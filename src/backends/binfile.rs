//! Generic binary files.

use log::*;

use crate::mime::mime_db;
use crate::pager::page_command;
use crate::programs::{find_program, run_command};
use crate::views::meta::FileMetaField;
use crate::{
    interface::*,
    styling::{
        names::{FILE_SIZE, FILE_TYPE},
        text::{styled, unstyled},
    },
    views::meta::FileMetaDisplay,
};

/// Non-specialized text file backend.
pub struct BinfileBackend {}

/// Viewer for generic binary files.
impl FileViewer for BinfileBackend {
    fn can_view(&self, req: &FileRequest, _mode: &Option<ViewType>) -> bool {
        let db = mime_db();
        db.is_subtype(&req.mime_type, "application/octet-stream")
    }

    fn default_view(&self) -> ViewType {
        ViewType::Meta
    }

    fn meta_view(&self, req: &FileRequest) -> Result<FileMetaDisplay, ViewError> {
        let db = mime_db();
        let mut headline = vec![unstyled("file of type "), styled(&req.mime_type, FILE_TYPE)];
        let mut fields = Vec::new();

        if let Some(bytes) = req.file_size() {
            headline.push(unstyled(" ("));
            headline.push(styled(format!("{}", friendly::bytes(bytes)), &FILE_SIZE));
            headline.push(unstyled(")"));
        }

        if let Some(desc) = db.description(&req.mime_type) {
            fields.push(FileMetaField {
                name: "Type Description".into(),
                value: vec![unstyled(desc)],
                extra: vec![],
            })
        };
        if let Some(size) = req.file_size() {
            fields.push(FileMetaField {
                name: "Size".into(),
                value: vec![unstyled(format!("{}", friendly::bytes(size)))],
                extra: vec![],
            })
        }
        Ok(FileMetaDisplay { headline, fields })
    }

    fn head_view(&self, req: &FileRequest) -> Result<(), ViewError> {
        let cmd = if let Some(mut xxd) = find_program("xxd")? {
            info!("previewing with xxd");
            xxd.args(&["-l", "256"]);
            xxd.arg(&req.path);
            xxd
        } else if let Some(mut hd) = find_program("hexdump")? {
            info!("previewing with hexdump");
            hd.args(&["-n", "256"]);
            hd.args(&req.path);
            hd
        } else {
            error!("no usable hex viewer found");
            return Err(ViewError::Unspecified("no hex viewer".into()));
        };
        run_command(cmd)?;
        Ok(())
    }

    fn full_view(&self, req: &FileRequest) -> Result<(), ViewError> {
        let cmd = if let Some(mut xxd) = find_program("xxd")? {
            info!("previewing with xxd");
            xxd.arg(&req.path);
            xxd
        } else if let Some(mut hd) = find_program("hexdump")? {
            info!("previewing with hexdump");
            hd.args(&req.path);
            hd
        } else {
            error!("no usable hex viewer found");
            return Err(ViewError::Unspecified("no hex viewer".into()));
        };
        page_command(cmd)?;
        Ok(())
    }
}
