#![allow(unused_imports, unused_import_braces)]
use clap::Parser;

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
    #[arg(short, long)]
    name: String
}

fn main() {
    let args = Args::parse();

    println!("{}", args.name);
}

