use super::{EnvProvider, ProcessRunner};
use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const TERMUX_FS: &str = "/data/data/com.termux/files";
const DEFAULT_SU_PATH: &str = "/system/bin/su";
const DEFAULT_HOME_PATH: &str = "root";
const ROOT_PATH_ENV: &str =
    "/debug_ramdisk:/sbin:/sbin/su:/su/bin:/su/xbin:/system/bin:/system/xbin";

#[derive(Debug)]
pub struct TermuxEnv;
impl TermuxEnv {
    pub fn new() -> Self {
        Self
    }

    fn get_home_path(&self) -> Result<PathBuf> {
        if let Ok(home_env) = env::var("TSW_HOME_ENV") {
            let home_path = PathBuf::from(home_env);
            let full_home_path = if home_path.is_absolute() {
                home_path
            } else {
                let mut termux_fs_path = PathBuf::from(TERMUX_FS);
                termux_fs_path.push(home_path);
                termux_fs_path
            };

            if full_home_path.exists() && !full_home_path.is_dir() {
                bail!(
                    "Invalid $TSW_HOME_ENV. '{}' is not directory",
                    full_home_path.to_string_lossy()
                )
            }

            Ok(full_home_path)
        } else {
            let mut home_path = PathBuf::from(TERMUX_FS);
            home_path.push(DEFAULT_HOME_PATH);
            if home_path.exists() && !home_path.is_dir() {
                bail!(
                    "'{}' is not directory. Fix it or use $TSW_HOME_ENV to override the path to root home",
                    home_path.to_string_lossy()
                );
            }
            Ok(home_path)
        }
    }
}

impl EnvProvider for TermuxEnv {
    fn get_su_path(&self) -> Result<PathBuf> {
        let su_path = if let Ok(su_path_string) = env::var("TSW_SU_PATH") {
            let su_path = PathBuf::from(su_path_string);
            if !su_path.is_absolute() {
                bail!(
                    "Invalid $TSW_SU_PATH. Path '{}' is not absolute",
                    su_path.to_string_lossy()
                )
            } else if !su_path.exists() {
                bail!(
                    "Invalid $TSW_SU_PATH. File '{}' is not exists",
                    su_path.to_string_lossy()
                )
            } else if !su_path.is_file() {
                bail!(
                    "Invalid $TSW_SU_PATH. '{}' is not file",
                    su_path.to_string_lossy()
                )
            }

            su_path
        } else {
            let su_path = PathBuf::from(DEFAULT_SU_PATH);
            if !su_path.exists() {
                bail!(
                    "File '{}' is not exists. Use $TSW_SU_PATH to override the path to SU",
                    su_path.to_string_lossy()
                )
            } else if !su_path.is_file() {
                bail!(
                    "'{}' is not file. Use $TSW_SU_PATH to override the path to SU",
                    su_path.to_string_lossy()
                )
            }

            su_path
        };

        Ok(su_path)
    }

    fn get_env_map(&self) -> Result<HashMap<&'static str, String>> {
        let mut env_map = HashMap::new();
        let path_env = if let Ok(path_env) = env::var("TSW_PATH_ENV") {
            path_env
        } else {
            format!("{}/usr/bin:{}", TERMUX_FS, ROOT_PATH_ENV)
        };
        env_map.insert("PATH", path_env);

        let home_path = self.get_home_path()?;
        let home_env = home_path
            .to_str()
            .context("Found non UTF-8 chars in root home path")?;
        env_map.insert("HOME", home_env.to_owned());

        let term_env = env::var("TERM").unwrap_or(String::from("xterm-256color"));
        env_map.insert("TERM", term_env);
        Ok(env_map)
    }

    fn is_shell_exists(&self, shell: &str) -> Result<()> {
        let path_env = env::var("PATH")?;
        let shell_path = Path::new(shell);
        if shell_path.is_absolute() {
            if !shell_path.exists() {
                bail!(
                    "Invalid shell. '{}' is not exists",
                    shell_path.to_string_lossy()
                );
            } else if !shell_path.is_file() {
                bail!(
                    "Invalid shell. '{}' is not file",
                    shell_path.to_string_lossy()
                );
            } else {
                return Ok(());
            }
        }

        for path in env::split_paths(&path_env) {
            let shell_path = path.join(shell);
            if shell_path.exists() && shell_path.is_file() {
                return Ok(());
            }
        }

        bail!("Invalid shell. '{}' not found in $PATH", shell);
    }
}

#[derive(Debug)]
pub struct RootRunner;
impl RootRunner {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProcessRunner for RootRunner {
    fn run(
        &self,
        shell: &str,
        command: Option<String>,
        envs: &HashMap<&str, String>,
        program: &Path,
    ) -> Result<i32> {
        let mut proc = Command::new(program);
        proc.env_clear();
        proc.envs(envs);
        proc.arg("-i");
        proc.arg("-p");
        proc.arg("--shell");
        proc.arg(shell);
        if let Some(command) = command {
            proc.arg("-c");
            proc.arg(command);
        }

        let mut child = proc.spawn()?;
        let exit_status = child.wait()?;
        let exit_code = exit_status.code().context("Failed get exit code")?;
        Ok(exit_code)
    }
}
