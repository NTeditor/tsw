#[macro_use]
mod macros;

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
    println!("{}", get_system_path());
}

fn get_system_path() -> String {
    let output = sh_output!("su -c 'echo $PATH'");
    let fallback = "/system/bin".to_string();

    match output {
        Ok(value) => {
            let stdout_raw = String::from_utf8(value.stdout).unwrap_or_else(|e| {
                eprintln!("failed decode system path, use fallback. {}", e);
                fallback
            });
            let stdout = stdout_raw.trim().to_owned();
            stdout
        }
        Err(e) => {
            eprintln!("failed get system path, use fallback. {}", e);
            fallback
        }
    }
}
