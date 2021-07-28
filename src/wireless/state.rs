use reqwest::Method;
use serde_json::json;

use crate::{wireless::WirelessNetwork, UnifiedError};

impl<'wn> WirelessNetwork<'wn> {
  /// Enable the wireless network.
  ///
  // # Example
  ///
  /// ```
  /// if let Some(network) = unifi.wireless_network("default", WirelessNetworkRef::Ssid("ACME Corp")).await? {
  ///   network.enable().await?;
  /// }
  /// ```
  pub async fn enable(&self) -> Result<(), UnifiedError> {
    self.set_state(true).await
  }

  /// Disable the wireless network.
  ///
  // # Example
  ///
  /// ```
  /// if let Some(network) = unifi.wireless_network("default", WirelessNetworkRef::Ssid("ACME Corp")).await? {
  ///   network.disable().await?;
  /// }
  /// ```
  pub async fn disable(&self) -> Result<(), UnifiedError> {
    self.set_state(false).await
  }

  async fn set_state(&self, state: bool) -> Result<(), UnifiedError> {
    self
      .unified
      .request(Method::PUT, &format!("/api/s/{}/rest/wlanconf/{}", self.site, self.id))
      .json(&json!({ "enabled": state }))
      .send()
      .await?;

    Ok(())
  }

  /// Delete the wireless network.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(network) = unifi.wireless_network("default", WirelessNetworkRef::Ssid("ACME Corp")).await? {
  ///   network.delete().await?;
  /// }
  /// ```
  pub async fn delete(&self) -> Result<(), UnifiedError> {
    self.unified.request(Method::DELETE, &format!("/api/s/{}/rest/wlanconf/{}", self.site, self.id)).send().await?;

    Ok(())
  }
}
