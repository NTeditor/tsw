#[macro_export]
macro_rules! sh_output {
    ($command:expr) => {{
        use std::process::Command;
        
        Command::new("sh")
            .arg("-c")
            .arg($command)
            .output()
    }};
}