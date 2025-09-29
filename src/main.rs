#[macro_use]
mod macros;
mod termux;

use std::{collections::HashMap, process::Command};

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
    let mut command = if cli.command.is_empty() {
        build_command()
    } else {
        build_user_command(cli.command)
    };

    match command.spawn() {
        Ok(mut child) => {
            let status = child
                .wait()
                .expect("Error while waiting for child process to finish");

            let exit_code = status.code().unwrap_or(1);

            std::process::exit(exit_code);
        }
        Err(e) => {
            eprintln!("Command invoked cannot execute: {}", e);
            std::process::exit(126);
        }
    }
}

fn build_command() -> Command {
    let mut command = Command::new(termux::get_su_file());
    command.args(["-i", "-c"]);

    let env_vars = env_to_string(termux::get_env());

    let root_command_string = format!(
        "env '-i' {} '{}'",
        env_vars,
        termux::get_bash_file()
            .to_str()
            .expect("Shell path contains invalid UTF-8. Only UTF-8 is supported.")
    );
    command.arg(root_command_string);
    command
}

fn build_user_command(user_command: Vec<String>) -> Command {
    let mut command = Command::new(termux::get_su_file());
    command.args(["-i", "-c"]);

    let env_vars = env_to_string(termux::get_env());
    let user_command_string = user_command.join(" ");

    let root_command_string = format!(
        "env '-i' {} '{}' '-c' '{}'",
        env_vars,
        termux::get_bash_file().display().to_string(),
        user_command_string,
    );
    command.arg(root_command_string);
    command
}

fn env_to_string(env_vars: HashMap<&'static str, String>) -> String {
    env_vars
        .iter()
        .map(|(k, v)| format!("'{}={}'", k, v))
        .collect::<Vec<_>>()
        .join(" ")
}
