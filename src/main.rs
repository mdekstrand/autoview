use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, CommandFactory, FromArgMatches, Parser};
use interface::ViewType;
use log::*;
#[cfg(not(feature = "xdg-embedded"))]
use shared_mime::load_mime_db;
use shared_mime::{Answer, FileQuery, MimeDB};
#[cfg(feature = "xdg-embedded")]
use shared_mime_embedded::load_mime_db;
use stderrlog::StdErrLog;

mod backends;
mod interface;

#[cfg(not(feature = "gpl"))]
static LICENSE_HEADER: &str =
    "Copyright (c) Michael Ekstrand. Free software under the MIT license.";
#[cfg(feature = "gpl")]
static LICENSE_HEADER: &str = "Copyright (c) Michael Ekstrand. Free software under the GNU GPLv3+.";

/// Automatically view files and file information.
#[derive(Parser)]
#[command(name = "autoview", version = "0.1.0")]
struct CLI {
    #[command(flatten)]
    action: AVAction,

    /// Specify number of lines to show with -H
    #[arg(short = 'n', long = "lines", default_value = "10", requires = "head")]
    num_lines: i32,

    /// Enable verbose diagnostic logging
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,

    /// Use a longer output when supported
    #[arg(short = 'l', long = "long")]
    long: bool,

    /// Avoid slow display or listing operations
    #[arg(short = 'f', long = "fast")]
    fast: bool,

    /// Use slower operations for more thorough display when available
    #[arg(short = 's', long = "slow")]
    slow: bool,

    /// File to display
    file: PathBuf,
}

#[derive(Args)]
#[group(multiple = false)]
struct AVAction {
    /// Show only the first several lines of the file
    #[arg(short = 'H', long = "head")]
    head: bool,

    /// Show the file's metadata
    #[arg(short = 'M', long = "meta")]
    meta: bool,

    /// Show the file content (through a pager if appropriate)
    #[arg(short = 'S', long = "show")]
    show: bool,

    /// Display the file's MIME type and exit
    #[arg(short = 'T', long = "mime-type")]
    mime_type: bool,
}

fn main() -> Result<()> {
    let mut cmd = CLI::command();
    cmd = cmd.after_help(LICENSE_HEADER);
    let matches = cmd.get_matches();
    let cli = CLI::from_arg_matches(&matches)?;
    StdErrLog::new()
        .verbosity(cli.verbose as usize + 1)
        .init()?;
    info!("CLI launching");

    let mut view = None;
    if cli.action.head {
        view = Some(ViewType::Head)
    } else if cli.action.meta {
        view = Some(ViewType::Meta)
    } else if cli.action.show {
        view = Some(ViewType::Full)
    }

    let db = load_mime_db()?;
    debug!("guessing type from {}", cli.file.display());
    let query = FileQuery::for_path(&cli.file)?;
    let guess = db.query(&query)?;

    if cli.action.mime_type {
        cli.show_mime(&db, &guess)
    } else {
        Ok(())
    }
}

impl CLI {
    fn show_mime(&self, db: &MimeDB, guess: &Answer) -> Result<()> {
        if let Some(ft) = guess.best() {
            println!("{}: {}", self.file.display(), ft);
            if let Some(desc) = db.description(ft) {
                println!("description: {}", desc);
            }
            println!("supertypes:");
            for sup in db.supertypes(ft) {
                println!("- {}", sup);
            }
            if db.is_subtype(ft, "text/plain") {
                println!("{}: is text file", self.file.display());
            }
        } else {
            println!("{}: type is uncertain", self.file.display());
        }

        Ok(())
    }
}
