mod config;
mod su;

use anyhow::{Result, bail};
use camino::Utf8PathBuf;
use clap::Parser;
use config::Config;
use std::path::PathBuf;
use std::process;
use su::SuShell;
use su::env::TermuxEnv;

const DEFAULT_CONFIG_PATH: &str = "/data/data/com.termux/files/usr/etc/tsw.toml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Command to execute (interactive shell if omitted)
    #[arg(trailing_var_arg = true)]
    command: Option<Vec<String>>,
    /// Shell to use with su [default: bash]
    #[arg(short, long)]
    shell: Option<Utf8PathBuf>,
    /// Path to config file
    #[arg(short, long)]
    #[arg(default_value = DEFAULT_CONFIG_PATH)]
    config: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Logger initialized");
    log::info!("Checking target os..");
    if !cfg!(target_os = "android") {
        bail!("This program for termux (android)");
    }
    log::info!("Your system is android");

    log::info!("Parsing cli args");
    let cli = Cli::parse();
    log::info!("Loading config");
    let config: Config = confy::load_path(cli.config)?;
    log::info!("Creating env provider");
    let env = TermuxEnv::new(config, cli.shell.clone());

    log::info!("Creating su shell");
    let su_shell = SuShell::new(cli.command, env);
    log::info!("Running su shell");
    let exit_code = su_shell.run()?;
    if exit_code != 0 {
        process::exit(exit_code);
    }
    Ok(())
}
