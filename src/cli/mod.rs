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
    Set { key: String, value: String },
    Get { key: String },
}

pub fn start() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Start {}) => Some(server::start()),
        Some(Commands::Ping {}) => Some(client::ping()),
        Some(Commands::Set { key, value }) => Some(client::set(key.to_owned(), value.to_owned())),
        Some(Commands::Get { key }) => Some(client::get(key.to_owned())),
        // start the server even if there are no commands for now
        None => Some(server::start()),
    };
}
