use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

pub const TERMUX_FS: &str = "/data/data/com.termux/files";
const DEFAULT_SYSPATH_ENV: &str =
    "/system/bin:/debug_ramdisk:/sbin:/sbin/su:/su/bin:/su/xbin:/system/bin:/system/xbin";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub su_file: Utf8PathBuf,
    pub home_dir: Utf8PathBuf,
    pub shell: Utf8PathBuf,
    pub path_env: String,
    pub master_namespace: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            su_file: "/system/bin/su".into(),
            home_dir: format!("{}/root", TERMUX_FS).into(),
            shell: "bash".into(),
            path_env: format!("{}/usr/bin:{}", TERMUX_FS, DEFAULT_SYSPATH_ENV),
            master_namespace: false,
        }
    }
}
