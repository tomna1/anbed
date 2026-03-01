use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnbedError {
    #[error("Failed to open the file '{0}'")]
    FileOpen(PathBuf),
    #[error("Unexpected IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not find either input file as argument or stdin to embed")]
    BadArguments,
}
