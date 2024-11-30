use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug, Clone)]
pub struct GenSigningKey {
    #[arg(short, long, default_value = "./key.pem")]
    pub output: PathBuf,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Generate a new private signing key.
    GenSigningKey(GenSigningKey),
}
