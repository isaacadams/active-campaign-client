#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("IO")]
    IO(#[from] std::io::Error),

    #[error("serde_json")]
    Serde(#[from] serde_json::Error),

    #[error("reqwest")]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}
