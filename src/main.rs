use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser};
use log::*;
use mime::Mime;
use stderrlog::StdErrLog;
use xdg_mime::SharedMimeInfo;

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

    let db = SharedMimeInfo::new();
    info!("guessing type from {}", opts.file.display());
    let guess = db.guess_mime_type().path(&opts.file).guess();
    let mime = guess.mime_type();
    println!("{}: {}", opts.file.display(), mime);
    if guess.uncertain() {
        println!("{}: guess is uncertain", opts.file.display());
    }
    if let Some(parents) = db.get_parents(mime) {
        for supertype in parents {
            println!("parent: {}", supertype);
        }
    }
    let text: Mime = "text/plain".parse()?;
    if db.mime_type_subclass(mime, &text) {
        println!("{}: is text file", opts.file.display());
    }

    if let Some(name) = opts.file.file_name() {
        let fntypes = db.get_mime_types_from_file_name(name.to_string_lossy().as_ref());
        for fnt in fntypes {
            println!("{}: filename type {}", name.to_string_lossy(), fnt);
        }
    }

    Ok(())
}
