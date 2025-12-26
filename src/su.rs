mod binding;
pub mod env;

use anyhow::{Context, Result};
use binding::*;
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::info;

pub trait EnvProvider: Debug {
    fn get_su_path(&self) -> Result<&Utf8Path>;
    fn get_env_map<'a>(&'a self) -> Result<HashMap<&'a str, String>>;
    fn get_shell_path(&self) -> Result<Utf8PathBuf>;
    fn is_master_namespace(&self) -> bool;
}

pub trait SuBinding: Debug {
    /// Force create pty.
    fn interactive(&mut self) -> &mut Self;

    /// Force run in global namespace
    fn mount_master(&mut self) -> &mut Self;

    /// Preserve the entire environment
    fn preserve_environment(&mut self) -> &mut Self;

    /// Use SHELL instead of the default /system/bin/sh
    fn shell<S: Into<String>>(&mut self, shell: S) -> &mut Self;

    /// Pass COMMAND to the invoked shell
    fn command<S: Into<String>>(&mut self, command: S) -> &mut Self;
    fn set_envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>;

    fn spawn_and_wait(self) -> Result<i32>;
    fn is_magisk(&self) -> Result<bool>;
}

pub trait SuBindingFactory {
    type Binding: SuBinding;
    fn create<S: Into<String>>(&self, su: S) -> Self::Binding;
}

#[derive(Debug)]
pub struct SuShell<E, F>
where
    E: EnvProvider,
    F: SuBindingFactory,
{
    command: Option<Vec<String>>,
    env: E,
    factory: F,
}

impl<E> SuShell<E, SuCmdFactory>
where
    E: EnvProvider,
{
    pub fn new(command: Option<Vec<String>>, env: E) -> Self {
        let factory = SuCmdFactory::new();
        Self {
            command,
            env,
            factory,
        }
    }
}

impl<E, F> SuShell<E, F>
where
    E: EnvProvider,
    F: SuBindingFactory,
{
    pub fn run(&self) -> Result<i32> {
        let su_path = self.env.get_su_path().context("Failed to get SU path")?;
        info!(su_path = su_path.as_str(), "Success get su path");
        let shell = &self
            .env
            .get_shell_path()
            .context("Failed to get shell path")?;
        info!(shell_path = shell.as_str(), "Success get shell path");
        let env_map = self.env.get_env_map().context("Failed to get env map")?;
        info!(env_map = ?env_map, "Success get env map");

        let mut su_cmd = self.factory.create(su_path.as_str());
        su_cmd
            .preserve_environment()
            .shell(shell.as_str())
            .set_envs(env_map);

        if su_cmd.is_magisk()? {
            su_cmd.interactive();
        }

        if self.env.is_master_namespace() {
            su_cmd.mount_master();
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
