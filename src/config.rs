use anyhow::{Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const TERMUX_FS: &str = "/data/data/com.termux/files";
const DEFAULT_SYSPATH_ENV: &str =
    "/system/bin:/debug_ramdisk:/sbin:/sbin/su:/su/bin:/su/xbin:/system/bin:/system/xbin";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub su_file: Utf8PathBuf,
    pub home_dir: Utf8PathBuf,
    pub shell: Utf8PathBuf,
    pub path_env: String,
}

impl Config {
    pub fn validate_su_file(&self) -> Result<()> {
        if !self.su_file.is_absolute() {
            bail!("Path is not absolute")
        }

        if !self.su_file.exists() {
            bail!("File doas not exists")
        }

        if !self.su_file.is_file() {
            bail!("Path is not a file")
        }
        Ok(())
    }

    pub fn validate_root_home(&self) -> Result<()> {
        if !self.home_dir.is_absolute() {
            bail!("Path does not absolute")
        }

        if self.home_dir.exists() && !self.home_dir.is_dir() {
            bail!("Path is not a directory")
        }
        Ok(())
    }

    pub fn get_shell_absolute<F>(&self, to_absolute_path: F) -> Result<Cow<'_, Utf8Path>>
    where
        F: Fn(&Utf8Path) -> Result<Utf8PathBuf>,
    {
        let shell = &self.shell;
        if shell.is_absolute() {
            if !shell.exists() {
                bail!("File does not exists");
            }

            if !shell.is_file() {
                bail!("Path is not a file");
            }

            return Ok(shell.as_path().into());
        }

        let shell_absolute = to_absolute_path(&self.shell)?;
        Ok(shell_absolute.into())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            su_file: "/system/bin/su".into(),
            home_dir: format!("{}/root", TERMUX_FS).into(),
            shell: "bash".into(),
            path_env: format!("{}/usr/bin:{}", TERMUX_FS, DEFAULT_SYSPATH_ENV),
        }
    }
}
