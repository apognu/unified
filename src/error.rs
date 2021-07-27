use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnifiedError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("network error")]
    NetworkError(#[from] reqwest::Error),

    #[error("unknown error")]
    Unknown,

    #[error("could not parse MAC address")]
    InvalidMacAddress,
    #[error("could not parse IP address")]
    InvalidIpAddress,
}
