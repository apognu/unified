use std::marker::PhantomData;

use reqwest::{Method, RequestBuilder};
use serde::Deserialize;

use crate::{Unified, UnifiedError};

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

#[derive(Deserialize)]
pub(crate) struct Response<T> {
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

impl<T> Response<T> {
  pub(crate) fn short(self) -> Result<T, UnifiedError> {
    if self.meta.code != "ok" {
      return match self.meta.message {
        Some(message) => Err(UnifiedError::UnifiError(message)),
        None => Err(UnifiedError::Unknown),
      };
    }

    Ok(self.data)
  }
}

#[derive(Deserialize)]
pub(crate) struct UdmProAuthResponse {
  errors: Option<Vec<String>>,
}

impl UdmProAuthResponse {
  pub(crate) fn short(self) -> Result<(), UnifiedError> {
    if self.errors.is_some() {
      return match self.errors.unwrap().get(0) {
        Some(message) => Err(UnifiedError::UnifiError(message.to_owned())),
        None => Err(UnifiedError::Unknown),
      };
    }

    Ok(())
  }
}

/// HTTP scheme to be used for the connection to the Unifi controller.
pub(crate) struct UnifiRequest<F> {
  pub(crate) builder: RequestBuilder,
  pub(crate) _phantom: PhantomData<F>,
}

impl<T> UnifiRequest<T>
where
  T: for<'de> Deserialize<'de>,
{
  pub fn map<F>(mut self, mut callback: F) -> UnifiRequest<T>
  where
    F: FnMut(RequestBuilder) -> RequestBuilder,
  {
    self.builder = callback(self.builder);
    self
  }

  pub async fn query(self) -> Result<T, UnifiedError> {
    Ok(self.builder.send().await?.json::<Response<T>>().await?.short()?)
  }

  pub async fn send(self) -> Result<(), UnifiedError> {
    self.builder.send().await?.json::<Response<Vec<()>>>().await?.short()?;

    Ok(())
  }
}

impl Unified {
  pub(crate) fn request<T>(&self, method: Method, path: &str) -> UnifiRequest<T>
  where
    T: for<'ser> Deserialize<'ser>,
  {
    let client = reqwest::ClientBuilder::new().danger_accept_invalid_certs(true).build().unwrap();

    let url = match self.is_udm_pro {
      true => format!("{}://{}/proxy/network{}", self.scheme.as_str(), self.host, path),
      false => format!("{}://{}{}", self.scheme.as_str(), self.host, path),
    };

    UnifiRequest {
      builder: client.request(method, &url).header("cookie", &self.token),
      _phantom: PhantomData::<T>::default(),
    }
  }
}
