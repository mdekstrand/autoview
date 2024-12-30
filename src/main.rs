use std::io::{stdout, IsTerminal};
use std::{fs::metadata, path::PathBuf};

use anyhow::Result;
use clap::{Args, CommandFactory, FromArgMatches, Parser};
use colorchoice::ColorChoice;
use interface::{FileRequest, ViewSpeed, ViewType};
use log::*;
use shared_mime::{Answer, FileQuery, MimeDB};
use stderrlog::StdErrLog;
use styling::stylesheet::StyleSheet;

mod backends;
mod interface;
pub mod mime;
pub mod pager;
pub mod programs;
mod styling;
pub mod views;

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

    let color = ColorChoice::global();
    let color_enabled = match color {
        ColorChoice::Always | ColorChoice::AlwaysAnsi => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => stdout().is_terminal(),
    };
    debug!(
        "color {}",
        if color_enabled { "enabled" } else { "disabled" }
    );
    let styles = if color_enabled {
        StyleSheet::term_default()
    } else {
        StyleSheet::empty()
    };

    let db = mime::mime_db();
    debug!("guessing type from {}", cli.file.display());
    let query = FileQuery::for_path(&cli.file)?;
    let guess = db.query(&query)?;
    debug!("mime type result: {:?}", guess);

    if cli.action.mime_type {
        info!("outputting MIME type information");
        return cli.show_mime(db.as_ref(), &guess);
    }

    let view = if cli.action.head {
        Some(ViewType::Head)
    } else if cli.action.meta {
        Some(ViewType::Meta)
    } else if cli.action.show {
        Some(ViewType::Full)
    } else {
        None
    };

    let meta = metadata(&cli.file)?;
    let request = FileRequest {
        path: cli.file.clone(),
        meta: Some(meta),
        mime_type: guess.best().unwrap_or("application/octet-stream").into(),
        long_display: cli.long,
        speed: if cli.fast {
            ViewSpeed::Fast
        } else if cli.slow {
            ViewSpeed::Fast
        } else {
            ViewSpeed::Default
        },
        color,
    };

    for back in backends::backends() {
        if back.can_view(&request, &view) {
            let view = view.clone().unwrap_or_else(|| back.default_view());
            match view {
                ViewType::Meta => {
                    let meta = back.meta_view(&request)?;
                    debug!("meta: {:?}", meta);
                    meta.render(&styles, stdout())?;
                }
                ViewType::Full => {
                    back.full_view(&request)?;
                }
                ViewType::Head => {
                    back.head_view(&request)?;
                }
            }
            break;
        }
    }

    Ok(())
}

impl CLI {
    fn show_mime(&self, db: &MimeDB, guess: &Answer) -> Result<()> {
        if let Some(ft) = guess.best() {
            debug!("found best guess {}", ft);
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
            debug!("no best guess");
            println!("{}: type is uncertain", self.file.display());
        }

        Ok(())
    }
}
