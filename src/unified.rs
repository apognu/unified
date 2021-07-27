use cookie::Cookie;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};

use crate::UnifiedError;

#[derive(Serialize)]
struct Credentials {
  username: String,
  password: String,
  remember: bool,
}

#[derive(Deserialize)]
pub(crate) struct Response<T> {
  pub(crate) data: T,
}

/// HTTP scheme to be used for the connection to the Unifi controller.
pub enum Scheme {
  Http,
  Https,
}

impl Scheme {
  fn as_str(&self) -> &'static str {
    match self {
      &Self::Http => "http",
      &Self::Https => "https",
    }
  }
}

/// Handle to an authenticated connection to a Unifi controller.
pub struct Unified {
  scheme: Scheme,
  host: String,
  token: String,
}

/// HTTP scheme used to connect the a Unifi controller.
pub(crate) enum Method {
  Get,
  Post,
  Put,
  Delete,
}

impl Unified {
  /// Creates a Unified handle with a separately acquired token.
  ///
  /// # Arguments
  ///
  ///  * `scheme`  - Whether the connection should use HTTP or HTTPS
  ///  * `host`    - Hostname and port of the Unifi controller
  ///  * `token`   - Previously acquired authentication token
  ///
  /// # Example
  ///
  /// ```
  /// let unifi = Unified::new(Scheme::Https, "unifi.acme.corp", "unifises=abcdef").await?;
  /// ```
  pub fn new(scheme: Scheme, host: &str, token: &str) -> Unified {
    Unified {
      scheme,
      host: host.to_string(),
      token: token.to_string(),
    }
  }

  /// Creates a Unified handle from a username and password.
  ///
  /// # Arguments
  ///
  ///  * `scheme`   - Whether the connection should use HTTP or HTTPS
  ///  * `host`     - Hostname and port of the Unifi controller
  ///  * `username` - Username of the account
  ///  * `password` - Password of the account
  ///
  /// # Example
  ///
  /// ```
  /// let unifi = Unified::auth(Scheme::Https, "unifi.acme.corp", "joe.shmoe", "mypassword").await?;
  /// ```
  pub async fn auth(scheme: Scheme, host: &str, username: &str, password: &str) -> Result<Unified, UnifiedError> {
    let host = host.to_string();
    let credentials = Credentials {
      username: username.to_string(),
      password: password.to_string(),
      remember: true,
    };

    let client = reqwest::Client::new();
    let response = client.post(&format!("{}://{}/api/login", scheme.as_str(), host)).json(&credentials).send().await?;

    if response.status() == StatusCode::UNAUTHORIZED {
      return Err(UnifiedError::InvalidCredentials);
    }
    if response.status() != StatusCode::OK {
      return Err(UnifiedError::Unknown);
    }

    let cookies = response
      .headers()
      .get_all("set-cookie")
      .into_iter()
      .map(|cookie| Cookie::parse(cookie.to_str().unwrap_or_default()).ok())
      .flatten()
      .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
      .collect::<Vec<String>>();

    Ok(Unified {
      scheme,
      host,
      token: cookies.join("; "),
    })
  }

  pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
    let client = Client::new();
    let url = format!("{}://{}{}", self.scheme.as_str(), self.host, path);

    let builder = match method {
      Method::Get => client.get(&url),
      Method::Post => client.post(&url),
      Method::Put => client.put(&url),
      Method::Delete => client.delete(&url),
    };

    builder.header("cookie", &self.token)
  }
}
