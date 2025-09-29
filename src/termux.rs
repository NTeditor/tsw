use std::{collections::HashMap, path::PathBuf};

const TERMUX_FS: &'static str = "/data/data/com.termux/files";
pub const TERMUX_TERM: &'static str = "xterm-256color";

pub fn get_termux_fs() -> PathBuf {
    PathBuf::from(TERMUX_FS)
}

pub fn get_prefix_path() -> PathBuf {
    get_termux_fs().join("usr")
}

pub fn get_bin_path() -> PathBuf {
    get_prefix_path().join("bin")
}

pub fn get_root_path() -> PathBuf {
    get_termux_fs().join("root")
}

pub fn get_su_file() -> PathBuf {
    get_bin_path().join("su")
}

pub fn get_bash_file() -> PathBuf {
    get_bin_path().join("bash")
}

pub fn get_env() -> HashMap<&'static str, String> {
    let mut env: HashMap<&'static str, String> = HashMap::new();
    let path = format!(
        "{}:{}",
        get_bin_path().display().to_string(),
        get_system_path(),
    );
    env.insert("PATH", path);
    env.insert("HOME", get_root_path().display().to_string());
    env.insert("PREFIX", get_prefix_path().display().to_string());
    env.insert("TERM", TERMUX_TERM.to_string());
    env
}

pub fn get_system_path() -> String {
    let output = sh_output!("su -c 'echo $PATH'");
    let fallback = "/system/bin";

    match output {
        Ok(value) => {
            let stdout_raw = String::from_utf8(value.stdout).unwrap_or_else(|e| {
                eprintln!("failed decode system path, use fallback. {}", e);
                fallback.to_string()
            });
            let stdout = stdout_raw.trim().to_owned();
            stdout
        }
        Err(e) => {
            eprintln!("failed get system path, use fallback. {}", e);
            fallback.to_string()
        }
    }
}
