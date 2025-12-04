use std::fmt::Display;

use clap::ValueEnum;
use deb_rust::DebArchitecture;

const ARM64_V8A_TRIBAL: &str = "aarch64-linux-android";
const ARMEABI_V7A_TRIBAL: &str = "armv7-linux-androideabi";

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Target {
    Arm64V8A,
    ArmeabiV7A,
}

impl AsRef<str> for Target {
    fn as_ref(&self) -> &str {
        match self {
            Self::Arm64V8A => ARM64_V8A_TRIBAL,
            Self::ArmeabiV7A => ARMEABI_V7A_TRIBAL,
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Target {
    pub fn as_deb_arch(&self) -> DebArchitecture {
        match self {
            Target::Arm64V8A => DebArchitecture::Aarch64,
            Target::ArmeabiV7A => DebArchitecture::Arm,
        }
    }
}
