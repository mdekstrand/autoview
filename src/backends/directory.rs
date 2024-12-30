use std::fs::read_dir;
use std::path::Path;
use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

use log::*;
use uu_ls::uumain;

use crate::programs::run_program;
use crate::{
    interface::*,
    programs::find_program,
    styling::{
        names::{FILE_SIZE, FILE_TYPE},
        text::{styled, unstyled},
    },
    views::meta::FileMetaDisplay,
};

const FILE_LIST_PROGRAMS: &[&str] = &["eza", "lsd", "exa", "lla", "ls"];

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

    fn head_view(&self, req: &FileRequest) -> Result<(), ViewError> {
        // FIXME: actuall display first lines
        self.full_view(req)
    }

    fn full_view(&self, req: &FileRequest) -> Result<(), ViewError> {
        for prog in FILE_LIST_PROGRAMS {
            if let Some(path) = find_program(OsStr::from_bytes(prog.as_bytes()))? {
                return self.external_ls(req, prog, path.as_path());
            }
        }
        self.fallback_ls(req)
    }
}

impl DirBackend {
    fn external_ls(&self, req: &FileRequest, program: &str, path: &Path) -> Result<(), ViewError> {
        info!("listing directory with {}", program);
        let mut args = Vec::new();
        if req.long_display {
            args.push("-l".into());
        }
        if program == "eza" {
            args.push("--color=always".into())
        }
        args.push(req.path.as_os_str().to_os_string());
        run_program(path, &args)?;
        Ok(())
    }

    fn fallback_ls(&self, req: &FileRequest) -> Result<(), ViewError> {
        info!("listing directory with fallback uu_ls");
        let mut args = vec!["ls-internal".into(), "-F".into()];
        if req.long_display {
            args.push("-l".into());
        }
        args.push(req.path.as_os_str().to_os_string());
        debug!("invocation: {:?}", args);
        let rc = uumain(args.into_iter());
        if rc != 0 {
            Err(ViewError::Unspecified(format!(
                "ls exited with code {}",
                rc
            )))
        } else {
            Ok(())
        }
    }
}
