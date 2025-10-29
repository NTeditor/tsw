pub mod termux;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

pub trait ProcessRunner: Debug {
    fn run(
        &self,
        shell: &str,
        command: Option<String>,
        envs: &HashMap<&str, String>,
        program: &Path,
    ) -> Result<i32>;
}

pub trait EnvProvider: Debug {
    fn get_su_path(&self) -> Result<PathBuf>;
    fn get_env_map(&self) -> Result<HashMap<&'static str, String>>;
    fn is_shell_exists(&self, shell: &str) -> Result<()>;
}

#[derive(Debug)]
pub struct SuShell<P: ProcessRunner, E: EnvProvider> {
    command: Vec<String>,
    shell: String,
    runner: P,
    env: E,
}

impl<P: ProcessRunner, E: EnvProvider> SuShell<P, E> {
    pub fn new(command: Vec<String>, shell: String, runner: P, env: E) -> Self {
        Self {
            command,
            shell,
            runner,
            env,
        }
    }

    pub fn run(&self) -> Result<i32> {
        let shell = &self.shell;
        self.env.is_shell_exists(shell)?;
        let env_map = self.env.get_env_map().context("Failed to get env map")?;
        let su_path = self.env.get_su_path().context("Failed to get SU path")?;
        let cmd = if self.command.is_empty() {
            let command = shlex::try_join(self.command.iter().map(|s| s.as_str()))
                .context("Failed to escape command")?;
            Some(command)
        } else {
            None
        };

        let exit_code = self
            .runner
            .run(shell, cmd, &env_map, &su_path)
            .context("Failed to run command")?;
        Ok(exit_code)
    }
}
