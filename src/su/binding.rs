use super::{SuBinding, SuBindingIsProvider, SuBindingRunner};
use anyhow::{Context, Result, bail};
use std::{
    fmt::Debug,
    process::{Command, Output},
};

#[derive(Debug)]
pub struct SuCmd(Command);
impl SuCmd {
    pub fn new<S>(su: S) -> Self
    where
        S: AsRef<str>,
    {
        let su = su.as_ref();
        log::info!(su_file = su; "Creating SuCmd");
        Self(Command::new(su))
    }
}

impl SuBinding for SuCmd {
    fn help(&mut self) -> &mut Self {
        log::info!("Add --help flag to su command");
        self.0.arg("--help");
        self
    }

    fn interactive(&mut self) -> &mut Self {
        log::info!("Add -i flag to su command");
        self.0.arg("-i");
        self
    }

    fn mount_master(&mut self) -> &mut Self {
        log::info!("Add --mount-master flag to su command");
        self.0.arg("--mount-master");
        self
    }

    fn shell<S>(&mut self, shell: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        let shell = shell.as_ref();
        log::info!(shell; "Add --shell flag to su command");
        self.0.arg("--shell").arg(shell);
        self
    }

    fn preserve_environment(&mut self) -> &mut Self {
        log::info!("Add --preserve-environment flag to su command");
        self.0.arg("--preserve-environment");
        self
    }

    fn command<S>(&mut self, command: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        let command = command.as_ref();
        log::info!(command; "Add -c flag to su command");
        self.0.arg("-c").arg(command);
        self
    }

    fn set_envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        log::info!("Cleaning env in su command");
        self.0.env_clear();
        for (k, v) in vars {
            let k = k.as_ref();
            let v = v.as_ref();
            log::info!(key = k, value = v; "Setting env var in su command");
            self.0.env(k, v);
        }
        self
    }
}

impl SuBindingRunner for SuCmd {
    fn spawn_and_wait(mut self) -> Result<i32> {
        let child = self.0.spawn()?;
        let output = child.wait_with_output()?;
        match output.status.code() {
            Some(0) => {
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

    fn get_output(mut self) -> Result<Output> {
        let output = self.0.output().context("Failed run su command")?;
        Ok(output)
    }
}

impl SuBindingIsProvider for SuCmd {
    fn is_magisk<S>(su: S) -> Result<bool>
    where
        S: AsRef<str>,
    {
        let mut cmd = Self::new(su);
        cmd.help();
        let output = cmd.get_output()?;
        if !output.status.success() {
            bail!("Failed execute su. Exit code is not null");
        }

        const MAGISK_PATTERN: &str = "magisk";

        let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if output_str.contains(MAGISK_PATTERN) {
            log::info!("Found magisk pattern in stdout");
            Ok(true)
        } else {
            log::info!("Magisk pattern is not found in stdout");
            Ok(false)
        }
    }
}
