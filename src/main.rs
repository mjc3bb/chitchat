#![allow(unused_imports, unused_import_braces)]
use clap::{Parser, Subcommand};

mod client {
    use clap::{Args, Subcommand};

    #[derive(Debug, Args)]
    #[command()]
    pub struct ClientArgs {
        #[arg(short, long)]
        pub port: u16,
    }
}

mod chatserver {
    use clap::{Args, Subcommand};

    #[derive(Debug, Args)]
    pub struct ChatServerArgs {}
}

use client::ClientArgs;
use chatserver::ChatServerArgs;

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

    #[allow(dead_code)]
    #[command(skip)]
    V2,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::V1(s) => {
            println!("{}", s.port)
        }
        _ => todo!("Unhandled command"),
    };
}
