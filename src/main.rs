#![allow(unused_imports, unused_import_braces)]
use clap::{Parser, Subcommand};

mod client {
    use clap::{Args, Subcommand};

    #[derive(Debug, Args)]
    #[command()]
    pub struct ClientArgs {
        #[command(subcommand)]
        pub command: Commands,
    }

    #[derive(Debug, Subcommand)]
    #[command()]
    pub enum Commands {
        #[command()]
        Start {
            #[arg(short, long, env = "SERVER_ADDRESS", default_value = "0.0.0.0")]
            address: String,
            #[arg(short, long, env = "SERVER_PORT", default_value = "8080")]
            port: u16,
        },
        #[command()]
        Ping {
            #[arg(short, long, env = "SERVER_ADDRESS", default_value = "0.0.0.0")]
            address: String,
            #[arg(short, long, env = "SERVER_PORT", default_value = "8080")]
            port: u16,
        },
    }

    pub fn handle(args: ClientArgs) {
        match args.command {
            Commands::Start { address, port } => {
                println!("starting client... {}:{}", address, port);
            }
            Commands::Ping { address, port } => {
                println!("pinging server... {}:{}", address, port);
            }
        }
    }
}

mod chatserver {
    use clap::{Args, Subcommand};

    #[derive(Debug, Args)]
    pub struct ChatServerArgs {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Debug, Subcommand)]
    #[command()]
    pub enum Commands {
        #[command()]
        Listen {
            #[arg(short, long, env = "ADDRESS", default_value = "0.0.0.0")]
            address: String,
            #[arg(short, long, env = "PORT", default_value = "8080")]
            port: u16,
        },
    }

    #[allow(dead_code, unused_variables)]
    pub fn handle(args: ChatServerArgs) {
        match args.command {
            Commands::Listen { address, port } => println!("Listening... {}:{}", address, port),
        }
    }
}

use chatserver::{handle as chatserver_handle, ChatServerArgs};
use client::{handle as client_handle, ClientArgs};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
#[command()]
enum Commands {
    #[command()]
    Client(ClientArgs),

    #[command()]
    ChatServer(ChatServerArgs),
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Client(s) => client_handle(s),
        Commands::ChatServer(s) => chatserver_handle(s),
    };
}
