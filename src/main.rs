mod config;
mod su;

use anyhow::{Result, bail};
use camino::Utf8PathBuf;
use clap::Parser;
use config::Config;
use std::process;
use su::SuShell;
use su::env::TermuxEnv;
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

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
    /// Force run in the global namespace
    #[arg(short, long)]
    mount_master: Option<bool>,
    /// Path to config file
    #[arg(short, long)]
    #[arg(default_value = DEFAULT_CONFIG_PATH)]
    config: Utf8PathBuf,
}

fn check_os() -> Result<()> {
    tracing::info!("Checking target os..");
    if !cfg!(target_os = "android") {
        bail!("This program for termux (android)");
    }
    tracing::info!("Good, your system is android");
    Ok(())
}

fn init_logger() {
    let env_filter = EnvFilter::from_default_env();
    let timer = ChronoUtc::new("%H:%M:%S".to_string());
    let fmt_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_target(true)
        .with_timer(timer)
        .compact();
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

fn main() -> Result<()> {
    init_logger();
    check_os()?;
    tracing::info!("Parsing cli args");
    let cli = Cli::parse();
    tracing::info!(cli = ?cli, "Success parsed cli args");
    let Cli {
        command,
        shell,
        mount_master,
        config: config_path,
    } = cli;

    tracing::info!(config_path = config_path.as_str(), "Loading config");
    let config: Config = confy::load_path(config_path)?;

    tracing::info!("Creating env provider");
    let env = TermuxEnv::new(config, shell, mount_master);

    tracing::info!("Creating su shell");
    let su_shell = SuShell::new(command, env);

    tracing::info!("Running su shell");
    let exit_code = su_shell.run()?;
    if exit_code != 0 {
        process::exit(exit_code);
    }
    Ok(())
}
