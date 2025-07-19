// src/main.rs
/*
 * Main executable for ChromaTransition
 */

use clap::Parser;
use chromatransition::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChromaTransition - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
