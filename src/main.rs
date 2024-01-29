#![allow(unused_imports, unused_import_braces)]
use clap::{Parser, Subcommand};

mod client {
    use clap::Args;

    #[derive(Debug, Args)]
    #[command()]
    pub struct ClientArgs {
        #[arg(short, long)]
        pub port: u16,
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
        Commands::Client(s) => {
            println!("{}", s.port);
        }
        Commands::ChatServer(_) => {
            println!("command 2");
        }
    };
}
