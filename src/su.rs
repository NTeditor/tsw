mod binding;
pub mod env;

use anyhow::{Context, Result};
use camino::Utf8Path;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::process::Output;

use binding::SuCmd;

pub trait EnvProvider: Debug {
    fn get_su_path(&self) -> Result<&Utf8Path>;
    fn get_env_map(&self) -> Result<HashMap<&'static str, Cow<'_, str>>>;
    fn get_shell_path(&self) -> Result<Cow<'_, Utf8Path>>;
}

trait SuBinding: Debug {
    fn help(&mut self) -> &mut Self;
    fn interactive(&mut self) -> &mut Self;
    fn mount_master(&mut self) -> &mut Self;
    fn preserve_environment(&mut self) -> &mut Self;
    fn shell<S>(&mut self, shell: S) -> &mut Self
    where
        S: AsRef<str>;
    fn command<S>(&mut self, command: S) -> &mut Self
    where
        S: AsRef<str>;
    fn set_envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>;
}

trait SuBindingRunner: SuBinding {
    fn spawn_and_wait(self) -> Result<i32>;
    fn get_output(self) -> Result<Output>;
}

trait SuBindingIsProvider: SuBindingRunner {
    fn is_magisk<S>(su: S) -> Result<bool>
    where
        S: AsRef<str>;
}

#[derive(Debug)]
pub struct SuShell<E: EnvProvider> {
    command: Option<Vec<String>>,
    env: E,
}

impl<E: EnvProvider> SuShell<E> {
    pub fn new(command: Option<Vec<String>>, env: E) -> Self {
        Self { command, env }
    }

    pub fn run(&self) -> Result<i32> {
        let shell = &self
            .env
            .get_shell_path()
            .context("Failed to get shell path")?;
        let env_map = self.env.get_env_map().context("Failed to get env map")?;
        let su_path = self.env.get_su_path().context("Failed to get SU path")?;

        let mut su_cmd = SuCmd::new(su_path);
        su_cmd
            .mount_master()
            .set_envs(env_map)
            .preserve_environment()
            .shell(shell.as_str());

        if SuCmd::is_magisk(su_path)? {
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
