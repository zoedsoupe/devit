use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod commands;
mod config;
mod errors;
mod requests;

#[derive(Parser)]
#[command(author = "Zoey Pessanha <zoey.spessanha@outlook.com>")]
struct Cli {
    #[arg(long, short, value_parser = clap::value_parser!(PathBuf), value_hint = clap::ValueHint::DirPath)]
    publish: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::handle_publish_article(cli.publish)?;
    Ok(())
}
