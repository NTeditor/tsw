use std::{fmt::Display, hash::Hash};

use super::*;

#[test]
fn shell() {
    let runner = MockRunner;
    let env = MockEnv;
    let shell = String::from("bash");
    let command = Vec::new();
    let su_shell = SuShell::new(command, shell, runner, env);
    let _ = su_shell.run().unwrap();
}

#[test]
fn shell_cmd() {
    let runner = MockRunnerCmd;
    let env = MockEnv;
    let shell = String::from("bash");
    let command = vec![String::from("echo"), String::from("'")];
    let su_shell = SuShell::new(command, shell, runner, env);
    let _ = su_shell.run().unwrap();
}

#[derive(Debug)]
struct MockRunner;
impl ProcessRunner for MockRunner {
    fn run(
        &self,
        _: &str,
        command: Option<String>,
        envs: &HashMap<&str, String>,
        program: &Path,
    ) -> Result<i32> {
        if program != Path::new("/usr/bin/su") {
            panic!(
                "Invalid program. Expected: '/usr/bin/su', found: '{}'",
                program.to_string_lossy()
            );
        }
        check_hash_map(envs, "PATH", String::from("/usr/bin:/usr/sbin"));
        check_hash_map(envs, "HOME", String::from("/root"));
        check_hash_map(envs, "TERM", String::from("alacritty"));
        if command.is_some() {
            panic!("Invalid command. Expected: 'None', found: 'Some'")
        }
        Ok(0)
    }
}

#[derive(Debug)]
struct MockRunnerCmd;
impl ProcessRunner for MockRunnerCmd {
    fn run(
        &self,
        _: &str,
        command: Option<String>,
        envs: &HashMap<&str, String>,
        program: &Path,
    ) -> Result<i32> {
        if program != Path::new("/usr/bin/su") {
            panic!(
                "Invalid program. Expected: '/usr/bin/su', found: '{}'",
                program.to_string_lossy()
            );
        }
        check_hash_map(envs, "PATH", String::from("/usr/bin:/usr/sbin"));
        check_hash_map(envs, "HOME", String::from("/root"));
        check_hash_map(envs, "TERM", String::from("alacritty"));
        if let Some(command) = command {
            if &command != "echo \"'\"" {
                panic!(
                    "Invalid command. expected: 'echo \"'\"', found: '{}'",
                    command
                );
            }
        } else {
            panic!("Invalid command. Expected: 'Some', found: 'None'")
        }
        Ok(0)
    }
}

fn check_hash_map<K: Display + Eq + Hash, V: Display + PartialEq>(
    map: &HashMap<K, V>,
    key: K,
    expected: V,
) {
    let actual = map.get(&key).unwrap_or_else(|| {
        panic!("Key '{}' not found in map", key);
    });

    if actual != &expected {
        panic!(
            "Invalid value for key '{}'. Expected '{}', found '{}'",
            key, expected, actual
        );
    }
}

#[derive(Debug)]
struct MockEnv;
impl EnvProvider for MockEnv {
    fn get_su_path(&self) -> Result<PathBuf> {
        Ok(PathBuf::from("/usr/bin/su"))
    }

    fn get_env_map(&self) -> Result<HashMap<&'static str, String>> {
        let mut env_map = HashMap::new();
        env_map.insert("PATH", String::from("/usr/bin:/usr/sbin"));
        env_map.insert("HOME", String::from("/root"));
        env_map.insert("TERM", String::from("alacritty"));
        Ok(env_map)
    }

    fn is_shell_exists(&self, _: &str) -> Result<()> {
        Ok(())
    }
}
