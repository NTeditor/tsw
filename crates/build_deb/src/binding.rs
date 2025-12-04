use anyhow::{Result, bail};
use std::process::{Command, Stdio};

macro_rules! add_flag {
    ($name:ident, $flag:expr) => {
        pub fn $name(&mut self) -> &mut Self {
            self.arg($flag);
            self
        }
    };
}

#[derive(Debug)]
pub struct CargoCmd {
    path: String,
    args: Vec<String>,
}

impl CargoCmd {
    pub fn new<S: Into<String>>(cargo_path: S) -> Self {
        let path = cargo_path.into();
        Self {
            path,
            args: Vec::new(),
        }
    }

    fn arg<S: Into<String>>(&mut self, value: S) {
        let value = value.into();
        self.args.push(value);
    }

    add_flag!(build, "build");
    add_flag!(release, "--release");

    pub fn cargo_subcommand<S: Into<String>>(&mut self, subcommand: S) -> &mut Self {
        let subcommand = subcommand.into();
        self.arg(subcommand);
        self
    }

    pub fn target<S: Into<String>>(&mut self, target: S) -> &mut Self {
        let target = target.into();
        self.arg("--target");
        self.arg(target);
        self
    }

    pub fn spawn_and_wait(self) -> Result<()> {
        let mut cmd = Command::new(self.path);
        cmd.args(self.args);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let child = cmd.spawn()?;
        let output = child.wait_with_output()?;
        if !output.status.success() {
            bail!("The cargo process exited with a non-zero code");
        }
        Ok(())
    }
}
