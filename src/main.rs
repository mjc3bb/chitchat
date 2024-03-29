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
    V1(ClientArgs),

    #[command()]
    V2(ChatServerArgs),
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::V1(s) => {
            println!("{}", s.port);
        }
        Commands::V2(_) => {
            println!("command 2");
        }
    };
}
