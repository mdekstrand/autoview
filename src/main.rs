use anyhow::Result;
use clap::{Args, Parser};
use log::*;
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
}

#[derive(Args)]
#[group(multiple = false)]
struct AVAction {
    /// Show only the first several lines of the file
    #[arg(short = 'H', long = "head", group = "operation")]
    head: bool,

    /// Show the file's metadata
    #[arg(short = 'M', long = "meta", group = "operation")]
    meta: bool,
}

fn main() -> Result<()> {
    let opts = CLI::parse();
    StdErrLog::new()
        .verbosity(opts.verbose as usize + 1)
        .init()?;
    info!("CLI launching");
    Ok(())
}
