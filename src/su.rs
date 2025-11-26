mod binding;
pub mod env;

use anyhow::{Context, Result};
use binding::*;
use camino::Utf8Path;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::process::Output;

pub trait EnvProvider: Debug {
    fn get_su_path(&self) -> Result<&Utf8Path>;
    fn get_env_map<'a>(&'a self) -> Result<HashMap<&'a str, Cow<'a, str>>>;
    fn get_shell_path<'a>(&'a self) -> Result<Cow<'a, Utf8Path>>;
    fn is_master_namespace(&self) -> bool;
}

pub trait SuBinding: Debug {
    fn help(&mut self) -> &mut Self;
    fn interactive(&mut self) -> &mut Self;
    fn mount_master(&mut self) -> &mut Self;
    fn preserve_environment(&mut self) -> &mut Self;
    fn shell<S: AsRef<str>>(&mut self, shell: S) -> &mut Self;
    fn command<S: AsRef<str>>(&mut self, command: S) -> &mut Self;
    fn set_envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>;

    fn spawn_and_wait(self) -> Result<i32>;
    fn get_output(self) -> Result<Output>;
    fn is_magisk<S: AsRef<str>>(su: S) -> Result<bool>;
}

pub trait SuBindingFactory {
    type Binding: SuBinding;
    fn create<S: AsRef<str>>(&self, su: S) -> Self::Binding;
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
        let shell = &self
            .env
            .get_shell_path()
            .context("Failed to get shell path")?;
        let env_map = self.env.get_env_map().context("Failed to get env map")?;
        let su_path = self.env.get_su_path().context("Failed to get SU path")?;

        let mut su_cmd = self.factory.create(su_path);
        su_cmd
            .preserve_environment()
            .shell(shell.as_str())
            .set_envs(env_map);

        if SuCmd::is_magisk(su_path)? {
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
