use core::fmt;
use std::{path::PathBuf, vec};

use crate::error::AppError;

const TERMUX_FS: &'static str = "/data/data/com.termux/files";
const TERMUX_TERM: &'static str = "xterm-256color";

pub struct Termux {
    pub env: TermuxEnv,
    pub su_file: String,
}

impl Termux {
    pub fn new() -> Result<Self, AppError> {
        let home = format!("{}/root", TERMUX_FS);
        let prefix = format!("{}/usr", TERMUX_FS);
        let path = format!("{}/bin:{}", prefix, Self::get_system_path()?);
        let su_file = format!("{}/bin/su", prefix);

        let env = TermuxEnv {
            path,
            home,
            prefix,
            term: TERMUX_TERM.to_string(),
        };
        Ok(Self { env, su_file })
    }

    pub fn get_shell_file(&self, shell_name: String) -> Result<String, AppError> {
        let bin_path = format!("{}/bin", self.env.prefix);
        let shell = PathBuf::from(bin_path).join(shell_name);

        if shell.exists() {
            Ok(shell.to_str().unwrap().to_owned())
        } else {
            Err(AppError::InvalidShell {
                shell: shell.to_string_lossy().into_owned(),
            })
        }
    }

    pub fn get_system_path() -> Result<String, AppError> {
        let output = sh_output!("su -c 'echo $PATH'")?;

        let stdout_raw = String::from_utf8(output.stdout)?;
        let stdout = stdout_raw.trim().to_owned();
        Ok(stdout)
    }
}

pub struct TermuxEnv {
    pub path: String,
    pub home: String,
    pub prefix: String,
    pub term: String,
}

impl fmt::Display for TermuxEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts = vec![
            format!("PATH={}", self.path),
            format!("HOME={}", self.home),
            format!("PREFIX={}", self.prefix),
            format!("TERM={}", self.term),
        ];

        write!(f, "{}", parts.join(" "))
    }
}
