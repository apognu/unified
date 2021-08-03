use cookie::Cookie;
use serde_json::json;

use crate::{
  http::{ApiV1NoData, Scheme, UdmProAuthResponse, UnifiData, UnifiResponse},
  UnifiedError,
};

/// Handle to an authenticated connection to a Unifi controller.
pub struct Unified {
  pub(crate) scheme: Scheme,
  pub(crate) host: String,
  pub(crate) token: String,
  pub(crate) csrf: String,
  pub(crate) tls_verify: bool,
  pub(crate) is_udm_pro: bool,
}

impl Unified {
  /// Creates a Unified handle.
  ///
  /// # Arguments
  ///
  ///  * `host`    - Hostname and port of the Unifi controller
  ///
  /// # Example
  ///
  /// ```
  /// let unifi = Unified::new("unifi.acme.corp");
  /// ```
  pub fn new(host: &str) -> Unified {
    Unified {
      scheme: Scheme::Https,
      host: host.to_string(),
      token: String::new(),
      csrf: String::new(),
      tls_verify: true,
      is_udm_pro: false,
    }
  }

  /// Use HTTP instead of HTTPS for the connection to the controller.
  ///
  /// # Example
  ///
  /// ```
  /// let unifi = Unified::new("unifi.acme.corp").plaintext();
  /// ```
  pub fn plaintext(mut self) -> Unified {
    self.scheme = Scheme::Http;
    self
  }

  /// Accept self-signed certificates.
  ///
  /// # Example
  ///
  /// ```
  /// let unifi = Unified::new("unifi.acme.corp").no_tls_verify();
  /// ```
  pub fn no_tls_verify(mut self) -> Unified {
    self.tls_verify = false;
    self
  }

  /// The controller runs on a Unifi Dream Machine Pro.
  ///
  /// # Example
  ///
  /// ```
  /// let unifi = Unified::new("unifi.acme.corp").udm_pro();
  /// ```
  pub fn udm_pro(mut self) -> Unified {
    self.is_udm_pro = true;
    self
  }

  /// Use a previously acquired token.
  ///
  /// # Example
  ///
  /// ```
  /// let unifi = Unified::new("unifi.acme.corp").set_token("unifises=abcdefgh");
  /// ```
  pub fn set_token(mut self, token: &str) -> Unified {
    self.token = token.to_string();
    self
  }

  /// Authenticate into a Unifi controller with the provided username and password.
  ///
  /// # Arguments
  ///
  ///  * `username` - Username of the account
  ///  * `password` - Password of the account
  ///
  /// # Example
  ///
  /// ```
  /// let unifi = Unified::new("unifi.acme.corp").auth("joe.shmoe", "mypassword").await?;
  /// ```
  pub async fn auth(mut self, username: &str, password: &str) -> Result<Unified, UnifiedError> {
    let credentials = json!({
      "username": username.to_string(),
      "password": password.to_string(),
      "remember": true,
    });

    let url = match self.is_udm_pro {
      true => format!("{}://{}/api/auth/login", self.scheme.as_str(), self.host),
      false => format!("{}://{}/api/login", self.scheme.as_str(), self.host),
    };

    let client = reqwest::ClientBuilder::new().danger_accept_invalid_certs(!self.tls_verify).build()?;
    let response = client.post(&url).json(&credentials).send().await?;

    let cookies = response
      .headers()
      .get_all("set-cookie")
      .into_iter()
      .map(|cookie| Cookie::parse(cookie.to_str().unwrap_or_default()).ok())
      .flatten()
      .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
      .collect::<Vec<String>>();

    match self.is_udm_pro {
      true => {
        if let Some(csrf) = response.headers().get("x-csrf-token") {
          self.csrf = csrf.to_str().unwrap_or_default().to_owned();
        }

        response.deserialize::<UdmProAuthResponse>().await?.catch()?;
      }
      false => {
        response.deserialize::<ApiV1NoData>().await?.catch()?;
      }
    }

    self.token = cookies.join("; ");

    Ok(self)
  }
}
