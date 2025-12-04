mod binding;
mod config;
mod types;

use anyhow::Result;
use binding::CargoCmd;
use clap::Parser;
use config::Config;
use constcat::concat;
use deb_rust::{DebFile, binary::DebPackage};
use std::fs::File;
use types::Target;

const TERMUX_PREFIX: &str = "/data/data/com.termux/files/usr";
const TERMUX_TSW: &str = concat!(TERMUX_PREFIX, "/bin/tsw");

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    target: Target,
    #[arg(short, long, default_value_t = false)]
    release: bool,
    #[arg(short, long)]
    cargo_subcommand: Option<String>,
}

fn build(target: &Target, cargo_subcommand: Option<String>, release: bool) -> Result<()> {
    println!("Building");
    let mut binding = CargoCmd::new("cargo");
    if let Some(subcommand) = cargo_subcommand {
        binding.cargo_subcommand(subcommand);
    }

    binding.build().target(target.as_ref());
    if release {
        binding.release();
    }

    binding.spawn_and_wait()?;
    Ok(())
}

fn package(target: &Target, release: bool) -> Result<()> {
    println!("Packaging");
    let source_path = {
        let build_type = if release { "release" } else { "debug" };
        format!("target/{}/{}/tsw", target, build_type)
    };
    let deb_output = {
        let build_type = if release { "release" } else { "debug" };
        format!("target/{}/{}/tsw.deb", target, build_type)
    };

    let config = Config::load()?;
    let package = DebPackage::new(&config.name)
        .set_version(&config.version)
        .set_architecture(target.as_deb_arch())
        .set_maintainer(&config.authors.join(", "))
        .with_file(DebFile::from_path(&source_path, TERMUX_TSW)?);

    let package = if let Some(description) = &config.description {
        package.set_description(description)
    } else {
        package
    };

    package.build()?.write(File::create(deb_output)?)?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Cli {
        target,
        release,
        cargo_subcommand,
    } = cli;
    build(&target, cargo_subcommand, cli.release)?;
    package(&target, release)?;
    Ok(())
}
