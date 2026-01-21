use super::EnvProvider;
use crate::config::{Config, TERMUX_FS};
use anyhow::{Context, Result, bail};
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
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

    fn add_env_var<'a, S>(env_map: &mut HashMap<&'a str, String>, var: &'a str, fallback: S)
    where
        S: Into<String>,
    {
        let env_var = env::var(var).unwrap_or_else(|e| {
            let fallback = fallback.into();
            warn!(
                err = e.to_string(),
                "Failed to get '${}' variable. Fallback to '{}'", var, fallback
            );
            fallback
        });

        env_map.insert(var, env_var);
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

    #[tracing::instrument]
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
        Self::add_env_var(&mut env_map, "TERM", "xterm-256color");
        Self::add_env_var(&mut env_map, "PREFIX", format!("{}/usr", TERMUX_FS));
        env_map.insert("HOME", home_env);
        env_map.insert("PATH", path_env);
        Ok(env_map)
    }

    fn get_shell_path(&self) -> Result<Utf8PathBuf> {
        let shell = self.shell.as_deref().unwrap_or(&self.config.shell);
        let components: Vec<_> = shell.components().collect();
        if components.len() == 0 {
            bail!("The path cannot be empty. Specify a file name or absolute path");
        }

        if shell.is_absolute() {
            if !shell.exists() {
                bail!("Shell does not exists");
            }

            if !shell.is_file() {
                bail!("Shell is not a file");
            }

            return Ok(shell.to_owned());
        }

        if !components.len() >= 1 || !matches!(components[0], Utf8Component::Normal(_)) {
            bail!("Relative paths are not allowed. Only file names or absolute paths are allowed");
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
