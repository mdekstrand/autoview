use std::fs::File;
use std::io::{BufRead, BufReader};

use bat::line_range::{LineRange, LineRanges};
use bat::PrettyPrinter;
use log::*;

use crate::mime::mime_db;
use crate::pager::page_file;
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
pub struct TextBackend {}

const BIG_FILE: u64 = 32 * 1024 * 1024;

impl FileViewer for TextBackend {
    fn can_view(&self, req: &FileRequest, _mode: &Option<ViewType>) -> bool {
        let db = mime_db();
        db.is_subtype(&req.mime_type, "text/plain")
    }

    fn default_view(&self) -> ViewType {
        ViewType::Full
    }

    fn meta_view(&self, req: &FileRequest) -> Result<FileMetaDisplay, ViewError> {
        let db = mime_db();
        let mut headline = vec![
            unstyled("text file of type "),
            styled(&req.mime_type, FILE_TYPE),
        ];
        let mut fields = Vec::new();

        if let Some(lines) = self.count_lines(req) {
            headline.push(unstyled(" with "));
            headline.push(styled(format!("{}", friendly::integer(lines)), &FILE_SIZE));
            headline.push(unstyled(" lines"));
        } else if let Some(bytes) = req.file_size() {
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
        let mut printer = PrettyPrinter::new();
        printer.input_file(&req.path);
        printer.line_ranges(LineRanges::from(vec![LineRange::new(1, 10)]));
        printer.paging_mode(bat::PagingMode::QuitIfOneScreen);
        printer.print().map_err(ViewError::wrap)?;
        Ok(())
    }

    fn full_view(&self, req: &FileRequest) -> Result<(), ViewError> {
        let size = req.file_size().unwrap_or_default();
        if size > BIG_FILE {
            info!("paging large file {:?}", req.path);
            page_file(&req.path)?;
        } else {
            info!("pretty-printing input {:?}", req.path);
            let mut printer = PrettyPrinter::new();
            printer.input_file(&req.path);
            printer.paging_mode(bat::PagingMode::QuitIfOneScreen);
            printer.print().map_err(ViewError::wrap)?;
        }
        Ok(())
    }
}

impl TextBackend {
    fn want_scan(&self, req: &FileRequest) -> bool {
        match req.speed {
            ViewSpeed::Slow => true,
            ViewSpeed::Default => req.file_size().unwrap_or_default() <= BIG_FILE,
            _ => false,
        }
    }

    fn count_lines(&self, req: &FileRequest) -> Option<usize> {
        if !self.want_scan(req) {
            return None;
        }

        let file = File::open(&req.path).ok()?;
        let buf = BufReader::new(file);
        Some(buf.lines().count())
    }
}
