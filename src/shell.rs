mod binding;
pub mod termux;

#[cfg(test)]
mod test;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use binding::SuCmd;

pub trait EnvProvider: Debug {
    fn get_su_path(&self) -> Result<PathBuf>;
    fn get_env_map(&self) -> Result<HashMap<&'static str, String>>;
    fn is_shell_exists(&self, shell: &str) -> Result<()>;
}

#[derive(Debug)]
pub struct SuShell<E: EnvProvider> {
    command: Option<Vec<String>>,
    shell: String,
    env: E,
}

impl<E: EnvProvider> SuShell<E> {
    pub fn new(command: Option<Vec<String>>, shell: String, env: E) -> Self {
        Self {
            command,
            shell,
            env,
        }
    }

    pub fn run(&self) -> Result<i32> {
        let shell = &self.shell;
        self.env.is_shell_exists(shell)?;
        let env_map = self.env.get_env_map().context("Failed to get env map")?;
        let su_path = self.env.get_su_path().context("Failed to get SU path")?;

        let mut su_cmd = SuCmd::new(&su_path);
        su_cmd
            .mount_master()
            .set_envs(env_map)
            .preserve_environment()
            .shell(shell);

        if SuCmd::is_magisk(&su_path)? {
            su_cmd.interactive();
        }

        if let Some(command) = &self.command {
            let command_str = command.iter().map(|s| s.as_str());
            let command = shlex::try_join(command_str).context("Failed to escape command")?;

            su_cmd.command(command);
        }

        let exit_code = su_cmd.spawn_and_wait()?;
        Ok(exit_code)
    }
}
