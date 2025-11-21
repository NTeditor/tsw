use std::{ffi::OsStr, path::Path, process::Command};

use anyhow::{Ok, Result, bail};

pub struct SuCmd(Command);
impl SuCmd {
    pub fn new<S>(su: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        Self(Command::new(su))
    }

    pub fn interactive(&mut self) -> &mut Self {
        log::info!("Add -i flag to su command");
        self.0.arg("-i");
        self
    }

    pub fn mount_master(&mut self) -> &mut Self {
        log::info!("Add --mount-master flag to su command");
        self.0.arg("--mount-master");
        self
    }

    pub fn shell<S>(&mut self, shell: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        log::info!("Add --shell flag to su command");
        self.0.arg("--shell").arg(shell);
        self
    }

    pub fn preserve_environment(&mut self) -> &mut Self {
        log::info!("Add --preserve-environment flag to su command");
        self.0.arg("--preserve-environment");
        self
    }

    pub fn command<S>(&mut self, command: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        log::info!("Add -c flag to su command");
        self.0.arg("-c").arg(command);
        self
    }

    pub fn set_envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        log::info!("set environment variables to su command");
        self.0.env_clear().envs(vars);
        self
    }

    pub fn spawn_and_wait(mut self) -> Result<i32> {
        let child = self.0.spawn()?;
        let output = child.wait_with_output()?;
        match output.status.code() {
            Some(v) if v == 0 => {
                log::info!("Success execute su");
                Ok(0)
            }
            Some(v) => {
                log::warn!(code = v; "su exit code is not null");
                Ok(v)
            }
            None => {
                log::error!("Failed get su exit code, using default -1");
                Ok(-1)
            }
        }
    }

    pub fn is_magisk<S>(su: S) -> Result<bool>
    where
        S: AsRef<OsStr>,
    {
        let mut cmd = Command::new(su);
        cmd.arg("--help");
        let output = cmd.output()?;
        if !output.status.success() {
            bail!("Failed execute su. Exit code is not null");
        }

        let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if output_str.contains("magisk") {
            log::info!("Found magisk patern in stdout");
            Ok(true)
        } else {
            log::info!("Magisk patern is not found in stdout");
            Ok(false)
        }
    }
}
