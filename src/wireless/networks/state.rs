use reqwest::Method;
use serde_json::json;

use crate::{
  http::ApiV1NoData,
  wireless::networks::{builder::WirelessNetworkBuilder, types::*},
  Unified, UnifiedError,
};

impl<'wn> WirelessNetwork<'wn> {
  /// Create a builder for a wireless network.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///  * `name` - Name of the wireless network to create
  ///
  /// # Example
  ///
  /// ```
  /// WirelessNetwork::builder(&unifi, "default", "ACME Corp")
  ///   .ap_groups(vec!["6105d873b49ca605191f4331"])
  ///   .network("6105dd31b49ca605191f4373")
  ///   .security(WirelessNetworkSecurity::WpaPsk)
  ///   .wpa(WirelessNetworkWpaMode::Wpa2)
  ///   .build()?;
  /// ```
  pub fn builder(unified: &'wn Unified, site: &str, name: &str) -> WirelessNetworkBuilder<'wn> {
    WirelessNetworkBuilder {
      network: WirelessNetwork {
        unified,
        site: site.to_string(),
        id: String::new(),
        name: name.to_string(),
        enabled: true,
        network: None,
        band: None,
        ap_groups: vec![],
        security: WirelessNetworkSecurity::Invalid,
        wpa: None,
        passphrase: None,
        vlan: None,
        radius_profile: None,
        advertised: true,
      },
    }
  }

  /// Create the wireless network.
  ///
  /// # Example
  ///
  /// ```
  /// WirelessNetwork::builder(&unifi, "default", "ACME Corp")
  ///   .ap_groups(vec!["6105d873b49ca605191f4331"])
  ///   .network("6105dd31b49ca605191f4373")
  ///   .security(WirelessNetworkSecurity::WpaPsk)
  ///   .wpa(WirelessNetworkWpaMode::Wpa2)
  ///   .build()?;
  ///   .create()
  ///   .await?;
  /// ```
  pub async fn create(self) -> Result<(), UnifiedError> {
    let body: RemoteWirelessNetwork = self.clone().into();

    println!("{}", serde_json::to_string(&body).unwrap());

    self
      .unified
      .request::<ApiV1NoData>(Method::POST, &format!("/api/s/{}/rest/wlanconf", self.site))
      .map(|r| r.json(&body))
      .query()
      .await?;

    Ok(())
  }

  /// Update the wireless network.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(mut network) = unifi.wireless_network("default", WirelessNetworkRef::Ssid("ACME Corp")).await? {
  ///   network.passphrase = Some("azerty".to_string());
  ///   network.update().await?;
  /// }
  /// ```
  pub async fn update(self) -> Result<(), UnifiedError> {
    let body: RemoteWirelessNetwork = self.clone().into();

    self
      .unified
      .request::<ApiV1NoData>(Method::PUT, &format!("/api/s/{}/rest/wlanconf/{}", self.site, self.id))
      .map(|r| r.json(&body))
      .query()
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
  pub async fn delete(self) -> Result<(), UnifiedError> {
    self
      .unified
      .request::<ApiV1NoData>(Method::DELETE, &format!("/api/s/{}/rest/wlanconf/{}", self.site, self.id))
      .query()
      .await?;

    Ok(())
  }

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
      .request::<ApiV1NoData>(Method::PUT, &format!("/api/s/{}/rest/wlanconf/{}", self.site, self.id))
      .map(|r| r.json(&json!({ "enabled": state })))
      .query()
      .await?;

    Ok(())
  }
}
