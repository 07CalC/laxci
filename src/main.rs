mod init_workflow;
mod run_workflow;
mod sort_jobs;
mod workflow;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::init_workflow::init_workflow;
use crate::run_workflow::run_workflow;

#[derive(Parser)]
#[command(name = "LaxCI")]
#[command(about = "A simple CI tool for running workflows defined in YAML files.", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "laxci.yml")]
    file: String,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Init) => init_workflow()?,
        None => run_workflow(cli.file)?,
    }

    Ok(())
}
