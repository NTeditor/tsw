use super::{SuBinding, SuBindingFactory};
use anyhow::{Result, bail};
use std::{
    fmt::Debug,
    process::{Command, Stdio},
};

macro_rules! add_flag {
    ($name:ident, $flag:expr) => {
        fn $name(&mut self) -> &mut Self {
            tracing::info!(flag = $flag, "Add {} flag to su command", stringify!($name));
            self.arg($flag);
            self
        }
    };
}

macro_rules! add_value_flag {
    ($name:ident, $flag:expr) => {
        fn $name<S: Into<String>>(&mut self, value: S) -> &mut Self {
            let value = value.into();
            tracing::info!(
                flag = $flag,
                value = value,
                "Add {} flag to su command",
                stringify!($name),
            );
            self.arg($flag);
            self.arg(value);
            self
        }
    };
}

pub struct SuCmdFactory;
impl SuCmdFactory {
    pub fn new() -> Self {
        Self
    }
}
impl SuBindingFactory for SuCmdFactory {
    type Binding = SuCmd;
    fn create<S: Into<String>>(&self, su_path: S) -> Self::Binding {
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
    pub fn new<S: Into<String>>(su_path: S) -> Self {
        let path = su_path.into();
        tracing::info!(su_path = path, "Creating SuCmd instance");
        Self {
            path,
            args: Vec::new(),
            envs: Vec::new(),
        }
    }

    fn arg<S: Into<String>>(&mut self, arg: S) {
        let arg = arg.into();
        self.args.push(arg);
    }
}

impl SuBinding for SuCmd {
    add_flag!(interactive, "-i");
    add_flag!(mount_master, "--mount-master");
    add_flag!(preserve_environment, "--preserve-environment");
    add_value_flag!(shell, "--shell");
    add_value_flag!(command, "-c");

    fn set_envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        tracing::info!("Cleaning env in su command");
        self.envs.clear();
        for (k, v) in vars {
            let k = k.into();
            let v = v.into();
            tracing::info!(key = k, value = v, "Setting env var in su command");
            self.envs.push((k, v));
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
        tracing::info!(command = ?cmd, "Final command struct");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_new_works() {
        const EXPECTED: &str = "/system/bin/su";
        let binding = SuCmd::new(EXPECTED);
        assert_eq!(binding.path.as_str(), EXPECTED);
    }

    #[test]
    fn it_interactive_works() {
        const EXPECTED: &[&str] = &["-i"];
        let mut binding = SuCmd::new("/system/bin/su");
        binding.interactive();
        assert_eq!(binding.args.as_slice(), EXPECTED);
    }

    #[test]
    fn it_shell_works() {
        const EXPECTED: &[&str] = &["--shell", "/system/bin/sh"];
        let mut binding = SuCmd::new("/system/bin/su");
        binding.shell("/system/bin/sh");
        assert_eq!(binding.args.as_slice(), EXPECTED);
    }

    #[test]
    fn it_multiple_flags_work() {
        const EXPECTED: &[&str] = &[
            "-i",
            "--mount-master",
            "--preserve-environment",
            "--shell",
            "/system/bin/sh",
        ];
        let mut binding = SuCmd::new("/system/bin/su");
        binding.interactive();
        binding.mount_master();
        binding.preserve_environment();
        binding.shell("/system/bin/sh");
        assert_eq!(binding.args.as_slice(), EXPECTED);
    }

    #[test]
    fn it_set_envs_works() {
        const EXPECTED: &[(&str, &str)] = &[("PATH", "/system/bin")];
        let mut binding = SuCmd::new("/system/bin/su");
        binding.set_envs(EXPECTED.iter().copied());
        let result: Vec<(&str, &str)> = binding
            .envs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(result, EXPECTED);
    }
}
