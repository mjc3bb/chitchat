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
            #[arg(short, long, env = "PORT")]
            port: u16,
        },
        #[command()]
        Ping {
            #[arg(short, long, env = "PORT")]
            port: u16,
        },
    }

    pub fn handle(args: ClientArgs) {
        match args.command {
            Commands::Start { port } => {
                println!("starting client... {}", port);
            }
            Commands::Ping { port } => {
                println!("pinging server... {}", port);
            }
        }
    }
}

mod chatserver {
    use clap::Args;

    #[derive(Debug, Args)]
    pub struct ChatServerArgs {}
}

use chatserver::ChatServerArgs;
use client::ClientArgs;

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
        Commands::Client(_) => {
            println!("");
        }
        Commands::ChatServer(_) => {
            println!("command 2");
        }
    };
}
