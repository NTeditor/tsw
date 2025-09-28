use clap::{Parser, arg, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, num_args = 1..)]
    command: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.command);
}
