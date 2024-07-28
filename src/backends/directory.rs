use std::fs::read_dir;

use crate::{
    interface::*,
    styling::{
        names::{FILE_SIZE, FILE_TYPE},
        text::{styled, unstyled},
    },
    views::meta::FileMetaDisplay,
};

/// Directory backend.
pub struct DirBackend {}

impl FileViewer for DirBackend {
    fn can_view(&self, req: &FileRequest, _mode: &Option<ViewType>) -> bool {
        req.mime_type == "inode/directory"
    }

    fn default_view(&self) -> ViewType {
        ViewType::Full
    }

    fn meta_view(&self, req: &FileRequest) -> Result<FileMetaDisplay, ViewError> {
        let rd = read_dir(&req.path)?;
        let nfiles = rd.count();
        Ok(FileMetaDisplay {
            headline: vec![
                styled("directory", FILE_TYPE),
                unstyled(" with "),
                styled(format!("{} entries", nfiles), FILE_SIZE),
            ],
            fields: Vec::new(),
        })
    }
}
