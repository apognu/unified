use async_trait::async_trait;
use reqwest::{Response, StatusCode};
use serde::Deserialize;

use crate::UnifiedError;

pub(crate) enum Scheme {
  Http,
  Https,
}

impl Scheme {
  pub(crate) fn as_str(&self) -> &'static str {
    match self {
      Self::Http => "http",
      Self::Https => "https",
    }
  }
}

#[async_trait]
pub(crate) trait UnifiResponse {
  async fn deserialize<O>(self) -> Result<O, UnifiedError>
  where
    O: UnifiData;
}

#[async_trait]
impl UnifiResponse for Response {
  async fn deserialize<O>(self) -> Result<O, UnifiedError>
  where
    O: UnifiData,
  {
    if self.status().as_u16() > 299 && self.status() != StatusCode::BAD_REQUEST {
      return Err(UnifiedError::HttpError(self.status()));
    }

    Ok(self.json::<O>().await?)
  }
}

#[async_trait]
pub(crate) trait UnifiData: for<'de> Deserialize<'de> {
  type Output;

  fn catch(self) -> Result<Self::Output, UnifiedError>;
}

#[derive(Deserialize)]
pub(crate) struct ApiV1<T> {
  meta: ResponseMeta,
  data: T,
}

#[derive(Deserialize)]
pub(crate) struct ResponseMeta {
  #[serde(rename = "rc")]
  code: String,
  #[serde(rename = "msg")]
  message: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct ApiV1NoData {
  meta: ResponseMeta,
}

#[async_trait]
impl<T> UnifiData for ApiV1<T>
where
  for<'de> T: Deserialize<'de>,
{
  type Output = T;

  fn catch(self) -> Result<Self::Output, UnifiedError> {
    if self.meta.code != "ok" {
      return match self.meta.message {
        Some(message) => Err(UnifiedError::UnifiError(message)),
        None => Err(UnifiedError::Unknown),
      };
    }

    Ok(self.data)
  }
}

#[async_trait]
impl UnifiData for ApiV1NoData {
  type Output = ();

  fn catch(self) -> Result<(), UnifiedError> {
    if self.meta.code != "ok" {
      return match self.meta.message {
        Some(message) => Err(UnifiedError::UnifiError(message)),
        None => Err(UnifiedError::Unknown),
      };
    }

    Ok(())
  }
}

#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct ApiV2<T> {
  data: T,
}

#[async_trait]
impl<T> UnifiData for ApiV2<T>
where
  for<'de> T: Deserialize<'de>,
{
  type Output = T;

  fn catch(self) -> Result<Self::Output, UnifiedError> {
    Ok(self.data)
  }
}

#[derive(Deserialize)]
pub(crate) struct UdmProAuthResponse {
  errors: Option<Vec<String>>,
}

#[async_trait]
impl UnifiData for UdmProAuthResponse {
  type Output = ();

  fn catch(self) -> Result<Self::Output, UnifiedError> {
    if self.errors.is_some() {
      return match self.errors.unwrap().get(0) {
        Some(message) => Err(UnifiedError::UnifiError(message.to_string())),
        None => Err(UnifiedError::Unknown),
      };
    }

    Ok(())
  }
}
