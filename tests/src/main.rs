use clap::Parser;
use notwithouttests::Cli;

fn main() -> anyhow::Result<()> {
    Cli::parse().dispatch()
}
