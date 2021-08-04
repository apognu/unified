mod types;

use std::marker::PhantomData;

use reqwest::{Method, RequestBuilder};
use serde::Deserialize;

pub(crate) use self::types::*;
use crate::{Unified, UnifiedError};

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

  pub async fn query<O>(self) -> Result<O, UnifiedError>
  where
    T: UnifiData<Output = O>,
  {
    Ok(self.builder.send().await?.deserialize::<T>().await?.catch()?)
  }
}

impl Unified {
  pub(crate) fn request<T>(&self, method: Method, path: &str) -> UnifiRequest<T>
  where
    T: for<'de> Deserialize<'de>,
  {
    let client = reqwest::ClientBuilder::new().danger_accept_invalid_certs(!self.tls_verify).build().unwrap();

    let url = match self.is_udm_pro {
      true => format!("{}://{}/proxy/network{}", self.scheme.as_str(), self.host, path),
      false => format!("{}://{}{}", self.scheme.as_str(), self.host, path),
    };

    UnifiRequest {
      builder: match self.is_udm_pro {
        true => client.request(method, &url).header("cookie", &self.token).header("x-csrf-token", &self.csrf),
        false => client.request(method, &url).header("cookie", &self.token),
      },
      _phantom: PhantomData::<T>::default(),
    }
  }
}
