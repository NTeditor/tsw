mod shell;

use anyhow::{Result, bail};
use clap::Parser;
use shell::SuShell;
use shell::termux::{RootRunner, TermuxEnv};
use std::{env, process};

const AFTER_HELP: &str = "\
\x1b[1;4mEnvironment variables:\x1b[0m
  \x1b[1mTSW_SHELL\x1b[0m       Shell to use with SU [default: bash]
  \x1b[1mTSW_SU_PATH\x1b[0m     Path to su binary [default: /system/bin/su]
  \x1b[1mTSW_HOME_ENV\x1b[0m    Root user home directory (relative to TERMUX_FS if relative) [default: root]";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, after_help = AFTER_HELP)]
struct Cli {
    /// Command to execute (interactive shell if omitted)
    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
    /// Shell to use with su [env: TSW_SHELL=] [default: bash]
    #[arg(short, long)]
    shell: Option<String>,
}

fn main() -> Result<()> {
    if !cfg!(target_os = "android") {
        bail!("This program for termux (android)");
    }

    let cli = Cli::parse();
    let shell = cli.shell.unwrap_or_else(|| {
        env::var("TSW_SHELL").unwrap_or_else(|_| {
            let default_shell = String::from("bash");
            default_shell
        })
    });

    let runner = RootRunner::new();
    let env = TermuxEnv::new();
    let su_shell = SuShell::new(cli.command, shell, runner, env);
    let exit_code = su_shell.run()?;
    if exit_code != 0 {
        process::exit(exit_code);
    }
    Ok(())
}
