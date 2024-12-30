use std::env;
use std::path::Path;
use std::process::Command;

use crate::programs::ProgramError;

pub fn page_file(path: &Path) -> Result<(), ProgramError> {
    let pager = env::var_os("PAGER").unwrap_or("less".into());
    let status = Command::new(pager).arg(path.as_os_str()).status()?;
    ProgramError::check(status)
}
