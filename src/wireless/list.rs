use std::str::FromStr;

use serde::Deserialize;

use crate::{
  unified::{Method, Response},
  wireless::{WirelessNetwork, WirelessNetworkRef, WirelessNetworkWpa},
  Unified, UnifiedError,
};

#[derive(Deserialize)]
pub struct RemoteWirelessNetwork {
  #[serde(rename = "_id")]
  id: String,
  name: String,
  enabled: bool,
  security: String,
  wpa_mode: String,
  wpa_enc: String,
  #[serde(rename = "x_passphrase")]
  passphrase: Option<String>,
  vlan: Option<String>,
  #[serde(default)]
  hide_ssid: bool,
}

impl Unified {
  /// List all configured wireless networks on the given site.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///
  /// # Example
  ///
  /// ```
  /// let networks = unifi.networks("default").await?;
  /// ```
  pub async fn wireless_networks(&self, site: &str) -> Result<Vec<WirelessNetwork<'_>>, UnifiedError> {
    let response = self
      .request(Method::Get, &format!("/api/s/{}/rest/wlanconf", site))
      .send()
      .await?
      .json::<Response<Vec<RemoteWirelessNetwork>>>()
      .await?;

    let networks = response
      .data
      .into_iter()
      .map(|network| {
        let wpa = match network.security.as_str() {
          "wpa2" | "wpaeap" => Some(WirelessNetworkWpa {
            mode: network.wpa_mode,
            encryption: network.wpa_enc,
          }),
          _ => None,
        };

        let passphrase = match network.security.as_str() {
          "wpaeap" => None,
          _ => network.passphrase,
        };

        WirelessNetwork {
          unified: &self,
          site: site.to_string(),

          id: network.id,
          name: network.name,
          enabled: network.enabled,
          advertised: !network.hide_ssid,
          security: network.security,
          wpa,
          passphrase,
          vlan: network.vlan.map(|vlan| u16::from_str(&vlan).ok()).flatten(),
        }
      })
      .collect();

    Ok(networks)
  }

  /// Find a specific wireless network by the provided ref.
  ///
  /// Wireless networks can by looked by ID and SSID. The attribute to use is
  /// selected according to the variant of [`WirelessNetworkRef`] provided to
  /// the function.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///  * `network_ref` - Attribute and value to use to look up the wireless
  ///                    network
  ///
  /// # Example
  ///
  /// ```
  /// let network = unifi.network("default", ClientRef::Ssid("ACME Corp")).await?;
  /// ```
  pub async fn wireless_network(&self, site: &str, network_ref: WirelessNetworkRef<'_>) -> Result<Option<WirelessNetwork<'_>>, UnifiedError> {
    Ok(self.wireless_networks(&site).await?.into_iter().find(|network| match network_ref {
      WirelessNetworkRef::Id(id) => network.id == id,
      WirelessNetworkRef::Ssid(ssid) => network.name == ssid,
    }))
  }
}
