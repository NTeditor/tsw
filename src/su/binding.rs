use super::{SuBinding, SuBindingFactory};
use anyhow::{Result, bail};
use std::{
    fmt::Debug,
    process::{Command, Stdio},
};

pub struct SuCmdFactory;
impl SuCmdFactory {
    pub fn new() -> Self {
        Self
    }
}
impl SuBindingFactory for SuCmdFactory {
    type Binding = SuCmd;
    fn create<S: AsRef<str>>(&self, su_path: S) -> Self::Binding {
        SuCmd::new(su_path)
    }
}

#[derive(Debug)]
pub struct SuCmd {
    path: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
}

impl SuCmd {
    pub fn new<S: AsRef<str>>(su_path: S) -> Self {
        let path = su_path.as_ref();
        tracing::info!(su_path = path, "Creating SuCmd instance");
        Self {
            path: path.to_string(),
            args: Vec::new(),
            envs: Vec::new(),
        }
    }

    fn arg<S: AsRef<str>>(&mut self, arg: S) {
        let arg = arg.as_ref();
        self.args.push(arg.to_string());
    }
}

impl SuBinding for SuCmd {
    fn interactive(&mut self) -> &mut Self {
        tracing::info!("Add -i flag to su command");
        self.arg("-i");
        self
    }

    fn mount_master(&mut self) -> &mut Self {
        tracing::info!("Add --mount-master flag to su command");
        self.arg("--mount-master");
        self
    }

    fn shell<S: AsRef<str>>(&mut self, shell: S) -> &mut Self {
        let shell = shell.as_ref();
        tracing::info!(shell = shell, "Add --shell flag to su command");
        self.arg("--shell");
        self.arg(shell);
        self
    }

    fn preserve_environment(&mut self) -> &mut Self {
        tracing::info!("Add --preserve-environment flag to su command");
        self.arg("--preserve-environment");
        self
    }

    fn command<S: AsRef<str>>(&mut self, command: S) -> &mut Self {
        let command = command.as_ref();
        tracing::info!(command = command, "Add -c flag to su command");
        self.arg("-c");
        self.arg(command);
        self
    }

    fn set_envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        tracing::info!("Cleaning env in su command");
        self.envs.clear();
        for (k, v) in vars {
            let k = k.as_ref();
            let v = v.as_ref();
            tracing::info!(key = k, value = v, "Setting env var in su command");
            self.envs.push((k.to_string(), v.to_string()));
        }
        self
    }

    fn spawn_and_wait(self) -> Result<i32> {
        tracing::info!(
            su_path = ?self.path,
            args = ?self.args,
            envs = ?self.envs,
            "Running su command"
        );
        let mut cmd = Command::new(&self.path);
        cmd.args(self.args);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        cmd.envs(self.envs);
        tracing::info!(
            program = ?cmd.get_program(),
            args = ?cmd.get_args(),
            envs = ?cmd.get_envs(),
            "Final command struct"
        );
        let child = cmd.spawn()?;
        let output = child.wait_with_output()?;
        match output.status.code() {
            Some(0) => {
                tracing::info!("The su process completed successfully");
                Ok(0)
            }
            Some(v) => {
                tracing::warn!(code = v, "The su process exited with a non-zero code");
                Ok(v)
            }
            None => {
                tracing::error!("Failed to get su exit code, using default -1");
                Ok(-1)
            }
        }
    }

    fn is_magisk(&self) -> Result<bool> {
        let mut cmd = Command::new(&self.path);
        cmd.arg("--help");
        let output = cmd.output()?;
        if !output.status.success() {
            bail!("Failed execute su. Exit code is not null");
        }

        const MAGISK_PATTERN: &str = "magisk";

        let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if output_str.contains(MAGISK_PATTERN) {
            tracing::info!("Found magisk pattern in stdout");
            Ok(true)
        } else {
            tracing::info!("Magisk pattern is not found in stdout");
            Ok(false)
        }
    }
}
