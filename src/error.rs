use reqwest::StatusCode;
use thiserror::Error;

/// Error type that can be returned by unified.
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum UnifiedError {
  #[error("network error")]
  NetworkError(#[from] reqwest::Error),
  #[error("http error: {0}")]
  HttpError(StatusCode),
  #[error("unifi error: {0}")]
  UnifiError(String),

  #[error("missing attribute: {0}")]
  MissingAttribute(String),

  #[error("unknown error")]
  Unknown,

  #[error("could not parse MAC address")]
  InvalidMacAddress,
  #[error("could not parse IP address")]
  InvalidIpAddress,
}
