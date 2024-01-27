use clap::Parser;

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

