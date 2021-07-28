use reqwest::Method;
use serde_json::json;

use crate::{clients::Client, UnifiedError};

impl<'c> Client<'c> {
  /// Block the client from accessing the networks.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(client) = unifi.client("default", ClientRef::Ip("1.2.3.4")).await? {
  ///   client.block().await?;
  /// }
  /// ```
  pub async fn block(&self) -> Result<(), UnifiedError> {
    self.set_blocked(true).await
  }

  /// Unblock the client from accessing the networks.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(client) = unifi.client("default", ClientRef::Ip("1.2.3.4")).await? {
  ///   client.unblock().await?;
  /// }
  /// ```
  pub async fn unblock(&self) -> Result<(), UnifiedError> {
    self.set_blocked(false).await
  }

  /// Kick the client from the network.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(client) = unifi.client("default", ClientRef::Ip("1.2.3.4")).await? {
  ///   client.kick().await?;
  /// }
  /// ```
  pub async fn kick(&self) -> Result<(), UnifiedError> {
    self
      .unified
      .request(Method::POST, &format!("/api/s/{}/cmd/stamgr", self.site))
      .json(&json!({ "cmd": "kick-sta", "mac": self.mac.to_string() }))
      .send()
      .await?;

    Ok(())
  }

  async fn set_blocked(&self, blocked: bool) -> Result<(), UnifiedError> {
    let command = if blocked { "block-sta" } else { "unblock-sta" };

    self
      .unified
      .request(Method::POST, &format!("/api/s/{}/cmd/stamgr", self.site))
      .json(&json!({ "cmd": command, "mac": self.mac.to_string() }))
      .send()
      .await?;

    Ok(())
  }
}
