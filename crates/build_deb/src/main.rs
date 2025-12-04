mod binding;
mod config;
mod types;

use anyhow::Result;
use binding::CargoCmd;
use clap::Parser;
use config::Config;
use deb_rust::{DebFile, binary::DebPackage};
use std::fs::File;
use types::Target;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    target: Target,
    #[arg(short, long, default_value = "false")]
    release: bool,
    #[arg(short, long, default_value = "true")]
    cargo_ndk: bool,
}

fn build(target: &Target, cargo_ndk: bool, release: bool) -> Result<()> {
    println!("Building");
    let mut binding = CargoCmd::new("cargo");
    if cargo_ndk {
        binding.ndk();
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

    let config = Config::load()?;
    let package = DebPackage::new(&config.name)
        .set_version(&config.version)
        .set_architecture(target.as_deb_arch())
        .set_maintainer(&config.authors.join(", "))
        .with_file(DebFile::from_path(
            &source_path,
            "/data/data/com.termux/files/usr/bin/tsw",
        )?);

    let package = if let Some(description) = &config.description {
        package.set_description(description)
    } else {
        package
    };

    package.build()?.write(File::create("tsw.deb")?)?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    build(&cli.target, cli.cargo_ndk, cli.release)?;
    package(&cli.target, cli.release)?;
    Ok(())
}
