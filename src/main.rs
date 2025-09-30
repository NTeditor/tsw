mod error;
mod termux;

use std::process::Command;

use clap::{Parser, Subcommand, arg, command};

use crate::{error::AppError, termux::Termux};

#[derive(Subcommand, Debug)]
enum Commands {
    Exec {
        #[arg(num_args = 1.., required = true)]
        args: Vec<String>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = "bash")]
    shell: String,
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<(), AppError> {
    init_logger();
    let cli = Cli::parse();
    log::info!("args parsed");
    let mut command = match cli.command {
        Some(Commands::Exec { args }) => build_user_command(cli.shell, args),
        None => build_command(cli.shell),
    }?;

    let mut child = command.spawn().map_err(|e| {
        log::error!("failed spawn process: {}", e);
        e
    })?;
    let status = child.wait().map_err(|e| {
        log::error!("failed wait end process: {}", e);
        e
    })?;
    let exit_code = status.code().unwrap_or_else(|| {
        log::warn!("failed get exit code, usege default");
        1
    });
    log::info!("process end. exit code: {}", exit_code);
    std::process::exit(exit_code);
}

fn init_logger() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("tsw"),
    );
    log::info!("logger initialized");
}

fn build_command(shell_name: String) -> Result<Command, AppError> {
    log::info!("running interactive shell");
    let termux = Termux::new()?;

    let mut command = Command::new(&termux.su_file);
    command.args(["-i", "-c"]);

    let env_vars = termux.env.to_string();
    log::info!("env vars: {}", env_vars);
    let shell = termux.get_shell_file(shell_name)?;
    log::info!("shell path: {}", shell);

    let root_command_str = format!("env '-i' {} '{}'", env_vars, shell);
    log::info!("final root command: {}", root_command_str);
    command.arg(root_command_str);
    Ok(command)
}

fn build_user_command(shell_name: String, user_command: Vec<String>) -> Result<Command, AppError> {
    log::info!("running user command");
    let termux = Termux::new()?;

    let mut command = Command::new(&termux.su_file);
    command.args(["-i", "-c"]);

    let env_vars = termux.env.to_string();
    log::info!("env vars: {}", env_vars);
    let user_command_str = user_command.join(" ");
    log::info!("user command: {}", user_command_str);
    let shell = termux.get_shell_file(shell_name)?;
    log::info!("shell path: {}", shell);

    let root_command_str = format!(
        "env '-i' {} '{}' '-c' '{}'",
        env_vars, shell, user_command_str,
    );
    log::info!("final root command: {}", root_command_str);
    command.arg(root_command_str);
    Ok(command)
}
