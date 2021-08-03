use std::str::FromStr;

use reqwest::Method;

use crate::{http::ApiV1, wireless::networks::types::*, Unified, UnifiedError};

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
      .request::<ApiV1<Vec<RemoteWirelessNetwork>>>(Method::GET, &format!("/api/s/{}/rest/wlanconf", site))
      .query()
      .await?;

    let networks = response
      .into_iter()
      .map(|network| {
        let wpa = match network.security.as_str() {
          "wpapsk" | "wpaeap" => Some(WirelessNetworkWpa {
            mode: network.wpa_mode.map(WirelessNetworkWpaMode::from).unwrap_or_else(|| WirelessNetworkWpaMode::Invalid),
            encryption: network.wpa_enc.unwrap_or_default(),
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
          network: network.network,
          ap_groups: network.ap_groups,
          band: network.band.map(WirelessBand::from),
          advertised: !network.hide_ssid,
          security: WirelessNetworkSecurity::from(network.security),
          wpa,
          passphrase,
          vlan: network.vlan.map(|vlan| u16::from_str(&vlan).ok()).flatten(),
          radius_profile: network.radius_profile,
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
