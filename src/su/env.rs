use super::EnvProvider;
use crate::config::{Config, TERMUX_FS};
use anyhow::{Context, Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashMap;
use std::env;
use tracing::warn;
use which::which;

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
        let su_file = &self.config.su_file;
        if !su_file.is_absolute() {
            bail!("Path is not absolute")
        }

        if !su_file.exists() {
            bail!("File does not exists")
        }

        if !su_file.is_file() {
            bail!("Path is not a file")
        }
        Ok(&self.config.su_file)
    }

    fn get_env_map(&self) -> Result<HashMap<&str, String>> {
        let mut env_map: HashMap<&str, String> = HashMap::new();

        let home_path = &self.config.home_dir;
        if !home_path.is_absolute() {
            bail!("Path is not absolute")
        }

        if home_path.exists() && !home_path.is_dir() {
            bail!("Path is not a directory")
        }

        let home_env = home_path.to_string();
        let path_env = self.config.path_env.clone();
        let term_env = env::var("TERM").unwrap_or_else(|e| {
            warn!(
                err = e.to_string(),
                "Failed to get '$TERM' variable. Fallback to 'xterm-256color'"
            );
            String::from("xterm-256color")
        });
        let prefix_env = env::var("PREFIX").unwrap_or_else(|e| {
            warn!(
                err = e.to_string(),
                "Failed to get '$PREFIX' variable. Fallback to '{TERMUX_FS}/usr'"
            );
            format!("{}/usr", TERMUX_FS)
        });

        env_map.insert("HOME", home_env);
        env_map.insert("PATH", path_env);
        env_map.insert("TERM", term_env);
        env_map.insert("PREFIX", prefix_env);
        Ok(env_map)
    }

    fn get_shell_path(&self) -> Result<Utf8PathBuf> {
        let shell = self.shell.as_deref().unwrap_or(&self.config.shell);
        if shell.is_absolute() {
            if !shell.exists() {
                bail!("Shell does not exists");
            }

            if !shell.is_file() {
                bail!("Shell is not a file");
            }

            return Ok(shell.to_owned());
        }

        let shell_os_path = which(shell).context("Shell is not found in $PATH")?;
        let shell_path = Utf8PathBuf::try_from(shell_os_path)?;
        Ok(shell_path)
    }

    fn is_master_namespace(&self) -> bool {
        self.master_namespace
            .unwrap_or(self.config.master_namespace)
    }
}
