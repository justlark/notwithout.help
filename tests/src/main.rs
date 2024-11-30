use clap::Parser;
use cli::Cli;

mod cli;
mod command;

fn main() -> anyhow::Result<()> {
    Cli::parse().dispatch()
}
