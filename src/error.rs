use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid shell '{shell}'")]
    InvalidShell {
        shell: String,
    },
    #[error("Failed get system path: {source}")]
    FailedGetSystemPath {
        #[from]
        source: std::io::Error,
    },
    #[error("Failed get system path: {source}")]
    FailedDecodeSystemPath {
        #[from]
        source: std::string::FromUtf8Error,
    },
}