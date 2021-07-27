use serde_json::json;

use crate::{unified::Method, wireless::WirelessNetwork, UnifiedError};

impl<'wn> WirelessNetwork<'wn> {
  /// Enable the wireless network.
  ///
  // # Example
  ///
  /// ```
  /// let network = unifi.wireless_network("default", WirelessNetworkRef::Ssid("ACME Corp")).await?;
  ///
  /// if let Some(network) = network {
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
  /// let network = unifi.wireless_network("default", WirelessNetworkRef::Ssid("ACME Corp")).await?;
  ///
  /// if let Some(network) = network {
  ///   network.disable().await?;
  /// }
  /// ```
  pub async fn disable(&self) -> Result<(), UnifiedError> {
    self.set_state(false).await
  }

  async fn set_state(&self, state: bool) -> Result<(), UnifiedError> {
    self
      .unified
      .request(Method::Put, &format!("/api/s/{}/rest/wlanconf/{}", self.site, self.id))
      .json(&json!({ "enabled": state }))
      .send()
      .await?;

    Ok(())
  }
}
