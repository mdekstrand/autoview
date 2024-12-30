//! Utilities for finding and invoking external programs.
use std::process::{Command, ExitStatus};
use std::{ffi::OsStr, path::PathBuf};

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

/// Find a program.
///
/// This wraps [which], returning Ok(None) when the program cannot be found.
pub fn find_program(name: &OsStr) -> Result<Option<PathBuf>, ProgramError> {
    match which(name) {
        Ok(path) => Ok(Some(path)),
        Err(WhichError::CannotFindBinaryPath) => Ok(None),
        Err(_) => Err(ProgramError::ProgramSearchError),
    }
}

/// Run an external program.
pub fn run_program<P: AsRef<OsStr>, A: AsRef<OsStr>>(
    program: P,
    args: &[A],
) -> Result<(), ProgramError> {
    let res = Command::new(program).args(args).status()?;
    if res.success() {
        Ok(())
    } else {
        Err(ProgramError::ExitError(res))
    }
}
