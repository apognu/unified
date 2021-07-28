use cookie::Cookie;
use serde_json::json;

use crate::{
  http::{Response, Scheme},
  UnifiedError,
};

/// Handle to an authenticated connection to a Unifi controller.
pub struct Unified {
  pub(crate) scheme: Scheme,
  pub(crate) host: String,
  pub(crate) token: String,
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
    let credentials = json!({
      "username": username.to_string(),
      "password": password.to_string(),
      "remember": true,
    });

    let client = reqwest::Client::new();
    let response = client.post(&format!("{}://{}/api/login", scheme.as_str(), host)).json(&credentials).send().await?;

    let cookies = response
      .headers()
      .get_all("set-cookie")
      .into_iter()
      .map(|cookie| Cookie::parse(cookie.to_str().unwrap_or_default()).ok())
      .flatten()
      .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
      .collect::<Vec<String>>();

    response.json::<Response<Vec<()>>>().await?.short()?;

    Ok(Unified {
      scheme,
      host: host.to_owned(),
      token: cookies.join("; "),
    })
  }
}
