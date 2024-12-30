use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::programs::ProgramError;

pub fn page_file(path: &Path) -> Result<(), ProgramError> {
    let pager = env::var_os("PAGER").unwrap_or("less".into());
    let status = Command::new(pager).arg(path.as_os_str()).status()?;
    ProgramError::check(status)
}

pub fn page_command(mut cmd: Command) -> Result<(), ProgramError> {
    let pager = env::var_os("PAGER").unwrap_or("less".into());
    let mut child = cmd.stdout(Stdio::piped()).spawn()?;
    let data = child.stdout.take().expect("no stdout");

    let pager_res = Command::new(pager).stdin(Stdio::from(data)).status()?;
    ProgramError::check(pager_res)?;

    let res = child.wait()?;
    ProgramError::check(res)
}
