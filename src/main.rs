mod shell;

use anyhow::Result;
use clap::Parser;
use shell::SuShell;
use shell::termux::{RootRunner, TermuxEnv};
use std::{env, process};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(trailing_var_arg = true)]
    command: Option<Vec<String>>,
    #[arg(short, long, default_value_t = get_default_shell())]
    shell: String,
}

fn get_default_shell() -> String {
    env::var("TSW_SHELL").unwrap_or(String::from("bash"))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let runner = RootRunner::new();
    let env = TermuxEnv::new();
    let su_shell = SuShell::new(cli.command, cli.shell, runner, env);
    let exit_code = su_shell.run()?;
    if exit_code != 0 {
        process::exit(exit_code);
    }
    Ok(())
}
