#![allow(unused_imports, unused_import_braces)]
use clap::{Parser, Subcommand};

mod client {
    use clap::Parser;

    #[derive(Debug, Parser)]
    pub struct ClientArgs {
        #[arg(short, long)]
        port: u16
    }
}

use client::ClientArgs;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
#[command()]
enum Commands {
    V1 {
        #[arg()]
        name: String,
        #[arg()]
        address: String
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::V1{ name, address } => {
            println!("{} {}", name, address)
        }
    };
}

