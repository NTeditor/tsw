use super::{SuBinding, SuBindingFactory};
use anyhow::{Context, Result, bail};
use std::{
    fmt::Debug,
    process::{Command, Output, Stdio},
};

pub struct SuCmdFactory;
impl SuCmdFactory {
    pub fn new() -> Self {
        Self
    }
}
impl SuBindingFactory for SuCmdFactory {
    type Binding = SuCmd;
    fn create<S: AsRef<str>>(&self, su: S) -> Self::Binding {
        SuCmd::new(su)
    }
}

#[derive(Debug)]
pub struct SuCmd(Command);
impl SuCmd {
    pub fn new<S: AsRef<str>>(su: S) -> Self {
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

    /// Force create pty.
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

    fn shell<S: AsRef<str>>(&mut self, shell: S) -> &mut Self {
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

    fn command<S: AsRef<str>>(&mut self, command: S) -> &mut Self {
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

    fn spawn_and_wait(mut self) -> Result<i32> {
        self.0.stdin(Stdio::inherit());
        self.0.stdout(Stdio::inherit());
        self.0.stderr(Stdio::inherit());
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

    fn is_magisk<S: AsRef<str>>(su: S) -> Result<bool> {
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
