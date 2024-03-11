use crate::client;

use super::server;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Start {},
    Ping {},
}

pub fn start() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Start {}) => Some(server::start()),
        Some(Commands::Ping {}) => Some(client::ping()),
        None => None,
    };
}
