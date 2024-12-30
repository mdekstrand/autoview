use std::fs::File;
use std::io::{BufRead, BufReader};

use bat::line_range::{LineRange, LineRanges};
use bat::PrettyPrinter;
use log::*;

use crate::mime::mime_db;
use crate::pager::page_file;
use crate::styling::FIELD_NAME;
use crate::{
    interface::*,
    styling::{styled, FILE_SIZE, FILE_TYPE},
};

/// Non-specialized text file backend.
pub struct TextBackend {}
struct TextMeta;
struct TextView {
    lines: Option<usize>,
}

const BIG_FILE: u64 = 32 * 1024 * 1024;

impl FileViewer for TextBackend {
    fn make_view(&self, req: &FileRequest, mode: &Option<ViewType>) -> Option<Box<dyn FileView>> {
        let db = mime_db();
        if db.is_subtype(&req.mime_type, "text/plain") {
            match mode {
                Some(ViewType::Meta) => Some(Box::new(TextMeta)),
                Some(ViewType::Head) => Some(Box::new(TextView { lines: Some(15) })),
                _ => Some(Box::new(TextView { lines: None })),
            }
        } else {
            None
        }
    }
}

impl FileView for TextMeta {
    fn display(&self, req: &FileRequest, options: &ViewOptions) -> Result<(), ViewError> {
        let db = mime_db();

        print!("text file of type {}", styled(&req.mime_type, &FILE_TYPE));

        if let Some(lines) = self.count_lines(req, options) {
            print!(
                " with {} lines",
                styled(format!("{}", friendly::integer(lines)), &FILE_SIZE)
            );
        }
        if let Some(bytes) = req.file_size() {
            print!(
                " ({})",
                styled(format!("{}", friendly::bytes(bytes)), &FILE_SIZE)
            );
        }
        println!();

        if let Some(desc) = db.description(&req.mime_type) {
            println!("{}: {}", styled("Text Description", &FIELD_NAME), desc);
        };
        if let Some(size) = req.file_size() {
            println!("{}: {}", styled("Size", &FIELD_NAME), friendly::bytes(size));
        }
        Ok(())
    }
}

impl FileView for TextView {
    fn display(&self, req: &FileRequest, _options: &ViewOptions) -> Result<(), ViewError> {
        let size = req.file_size().unwrap_or_default();
        if size > BIG_FILE && self.lines.is_none() {
            info!("paging large file {:?}", req.path);
            page_file(&req.path)?;
        } else {
            let mut printer = PrettyPrinter::new();
            printer.input_file(&req.path);
            if let Some(lines) = self.lines {
                printer.line_ranges(LineRanges::from(vec![LineRange::new(1, lines)]));
            }
            printer.paging_mode(bat::PagingMode::QuitIfOneScreen);
            printer.print().map_err(ViewError::wrap)?;
        }
        Ok(())
    }
}

impl TextMeta {
    fn want_scan(&self, req: &FileRequest, options: &ViewOptions) -> bool {
        match options.speed {
            ViewSpeed::Slow => true,
            ViewSpeed::Default => req.file_size().unwrap_or_default() <= BIG_FILE,
            _ => false,
        }
    }

    fn count_lines(&self, req: &FileRequest, options: &ViewOptions) -> Option<usize> {
        if !self.want_scan(req, options) {
            return None;
        }

        let file = File::open(&req.path).ok()?;
        let buf = BufReader::new(file);
        Some(buf.lines().count())
    }
}
