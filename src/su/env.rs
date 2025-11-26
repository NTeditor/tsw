use super::EnvProvider;
use crate::config::{Config, TERMUX_FS};
use anyhow::{Context, Ok, Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct TermuxEnv {
    config: Config,
    shell: Option<Utf8PathBuf>,
    master_namespace: Option<bool>,
}

impl TermuxEnv {
    pub fn new(config: Config, shell: Option<Utf8PathBuf>, master_namespace: Option<bool>) -> Self {
        Self {
            config,
            shell,
            master_namespace,
        }
    }
}

impl EnvProvider for TermuxEnv {
    fn get_su_path(&self) -> Result<&Utf8Path> {
        self.config.validate_su_file().context("Invalid su_file")?;
        Ok(&self.config.su_file)
    }

    fn get_env_map(&self) -> Result<HashMap<&'static str, Cow<'_, str>>> {
        let mut env_map: HashMap<&'static str, Cow<'_, str>> = HashMap::new();
        self.config
            .validate_root_home()
            .context("Invalid home dir")?;

        let home_env = self.config.home_dir.as_str();
        let path_env = &self.config.path_env;
        let term_env = env::var("TERM").unwrap_or(String::from("xterm-256color"));
        let prefix_env = env::var("PREFIX").unwrap_or(format!("{}/usr", TERMUX_FS));

        env_map.insert("HOME", home_env.into());
        env_map.insert("PATH", path_env.into());
        env_map.insert("TERM", term_env.into());
        env_map.insert("PREFIX", prefix_env.into());
        Ok(env_map)
    }

    fn get_shell_path(&self) -> Result<Cow<'_, Utf8Path>> {
        if let Some(shell) = &self.shell {
            let shell_path = to_absolute_path(shell).context("Invalid shell path")?;
            return Ok(shell_path.into());
        }
        let shell_path = self.config.get_shell_absolute(to_absolute_path)?;
        Ok(shell_path)
    }

    fn is_master_namespace(&self) -> bool {
        if let Some(master_namespace) = self.master_namespace {
            master_namespace
        } else {
            self.config.master_namespace
        }
    }
}

fn to_absolute_path(file: &Utf8Path) -> Result<Utf8PathBuf> {
    let path_env = env::var("PATH").context("Failed get $PATH")?;
    for path_part in env::split_paths(&path_env) {
        if let Some(path_part) = Utf8Path::from_path(&path_part) {
            let shell_path = path_part.join(file);
            if shell_path.exists() && shell_path.is_file() {
                return Ok(shell_path.to_path_buf());
            }
        }
    }

    bail!("File '{}' not found in $PATH", file);
}
