use thiserror::Error;

#[derive(Error, Debug)]
pub enum MointerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Clap error: {0}")]
    Clap(#[from] clap::Error),
    #[error("Tokio error: {0}")]
    Tokio(#[from] tokio::task::JoinError),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
}
