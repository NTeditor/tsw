use std::{ffi::OsStr, path::Path, process::Command};

use anyhow::{Ok, Result};

pub struct SuCmd(Command);
impl SuCmd {
    pub fn new<S>(su: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        Self(Command::new(su))
    }

    pub fn interactive(&mut self) -> &mut Self {
        self.0.arg("-i");
        self
    }

    pub fn mount_master(&mut self) -> &mut Self {
        self.0.arg("--mount-master");
        self
    }

    pub fn shell<S>(&mut self, shell: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        self.0.arg("--shell").arg(shell);
        self
    }

    pub fn preserve_environment(&mut self) -> &mut Self {
        self.0.arg("--preserve-environment");
        self
    }

    pub fn set_envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.env_clear().envs(vars);
        self
    }

    pub fn spawn_and_wait(mut self) -> Result<i32> {
        let child = self.0.spawn()?;
        let output = child.wait_with_output()?;
        match output.status.code() {
            Some(v) if v == 0 => Ok(0),
            Some(v) => Ok(v),
            None => Ok(-1),
        }
    }
}
