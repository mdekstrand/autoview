//! Utilities for finding and invoking external programs.
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

use thiserror::Error;
use which::{which, Error as WhichError};

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("error searching for external program")]
    ProgramSearchError,
    #[error("program exited with nonzero exit {0}")]
    ExitError(ExitStatus),
    #[error("I/O error running program")]
    IOError(#[from] std::io::Error),
}
impl ProgramError {
    pub fn check(status: ExitStatus) -> Result<(), ProgramError> {
        if status.success() {
            Ok(())
        } else {
            Err(ProgramError::ExitError(status))
        }
    }
}

/// Find a program.
///
/// This wraps [which], returning Ok(None) when the program cannot be found.
pub fn find_program<S: Into<OsString>>(name: S) -> Result<Option<Command>, ProgramError> {
    match which(name.into()) {
        Ok(path) => Ok(Some(Command::new(path))),
        Err(WhichError::CannotFindBinaryPath) => Ok(None),
        Err(_) => Err(ProgramError::ProgramSearchError),
    }
}

pub fn run_command(mut cmd: Command) -> Result<(), ProgramError> {
    let res = cmd.status()?;
    ProgramError::check(res)
}

pub fn program_name(cmd: &Command) -> String {
    let exe = cmd.get_program();
    let path = PathBuf::from(exe);
    if let Some(name) = path.file_name() {
        name.to_string_lossy().to_string()
    } else {
        "<unknown>".to_string()
    }
}
