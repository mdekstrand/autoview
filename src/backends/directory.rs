use std::fs::read_dir;
use std::process::Command;
use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

use log::*;
use uu_ls::uumain;

use crate::programs::{find_program, program_name, run_command};
use crate::{
    interface::*,
    styling::{styled, FILE_SIZE, FILE_TYPE},
};

const FILE_LIST_PROGRAMS: &[&str] = &["eza", "lsd", "exa", "lla", "ls"];

/// Directory backend.
pub struct DirBackend {}

struct DirMeta {}
struct LSView {}

impl FileViewer for DirBackend {
    fn make_view(&self, req: &FileRequest, mode: &Option<ViewType>) -> Option<Box<dyn FileView>> {
        if req.mime_type == "inode/directory" {
            match mode {
                Some(ViewType::Meta) => Some(Box::new(DirMeta {})),
                _ => Some(Box::new(LSView {})),
            }
        } else {
            None
        }
    }
}

impl FileView for DirMeta {
    fn display(&self, req: &FileRequest, _options: &ViewOptions) -> Result<(), ViewError> {
        let rd = read_dir(&req.path)?;
        let nfiles = rd.count();
        println!(
            "{} with {}",
            styled("directory", &FILE_TYPE),
            styled(format!("{} entries", nfiles), &FILE_SIZE)
        );
        Ok(())
    }
}

impl FileView for LSView {
    fn display(&self, req: &FileRequest, options: &ViewOptions) -> Result<(), ViewError> {
        for prog in FILE_LIST_PROGRAMS {
            if let Some(cmd) = find_program(OsStr::from_bytes(prog.as_bytes()))? {
                return self.external_ls(req, options, cmd);
            }
        }
        self.fallback_ls(req, options)
    }
}

impl LSView {
    fn external_ls(
        &self,
        req: &FileRequest,
        options: &ViewOptions,
        mut cmd: Command,
    ) -> Result<(), ViewError> {
        let name = program_name(&cmd);
        info!("listing directory with {}", name);
        if options.long_display {
            cmd.arg("-l");
        }
        if name == "eza" {
            cmd.arg("--color=always");
        }
        cmd.arg(&req.path);
        run_command(cmd)?;
        Ok(())
    }

    fn fallback_ls(&self, req: &FileRequest, options: &ViewOptions) -> Result<(), ViewError> {
        info!("listing directory with fallback uu_ls");
        let mut args = vec!["ls-internal".into(), "-F".into()];
        if options.long_display {
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
