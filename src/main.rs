use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser};
use log::*;
use shared_mime_embedded::{load_mime_db, FileQuery};
use stderrlog::StdErrLog;

/// Automatically view files and file information.
#[derive(Parser)]
#[command(name = "av")]
struct CLI {
    #[command(flatten)]
    action: AVAction,

    /// Specify number of lines to show with -H
    #[arg(short = 'n', long = "lines", default_value = "10", requires = "head")]
    num_lines: i32,

    /// Enable verbose diagnostic logging
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,

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

    /// Display the file's MIME type and exit
    #[arg(short = 'T', long = "mime-type")]
    mime_type: bool,
}

fn main() -> Result<()> {
    let opts = CLI::parse();
    StdErrLog::new()
        .verbosity(opts.verbose as usize + 1)
        .init()?;
    info!("CLI launching");

    let db = load_mime_db()?;
    info!("guessing type from {}", opts.file.display());
    let query = FileQuery::for_path(&opts.file)?;
    let guess = db.query(&query)?;
    if let Some(ft) = guess.best() {
        println!("{}: {}", opts.file.display(), ft);
        println!("supertypes:");
        for sup in db.supertypes(ft) {
            println!("- {}", sup);
        }
        if db.is_subtype(ft, "text/plain") {
            println!("{}: is text file", opts.file.display());
        }
    } else {
        println!("{}: type is uncertain", opts.file.display());
    }

    Ok(())
}
