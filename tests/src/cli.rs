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
    /// The path of the PEM-formatted private key to generate.
    #[arg(short, long, default_value = "./key.pem")]
    pub output: PathBuf,
}

#[derive(Args, Debug, Clone)]
pub struct RespondChallenge {
    /// The challenge token received from the server.
    ///
    /// Pass `-` to read from stdin.
    pub token: String,

    /// The path of the PEM-formatted private key to use for signing the nonce.
    #[arg(short, long, default_value = "./key.pem")]
    pub key: PathBuf,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Generate a new private signing key.
    ///
    /// This prints the base64-encoded public key to stdout.
    GenSigningKey(GenSigningKey),

    /// Generate a valid response to an authentication challenge.
    RespondChallenge(RespondChallenge),
}
