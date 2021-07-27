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
    vlan: Option<String>,
}

impl Unified {
    pub async fn wireless_networks(
        &self,
        site: &str,
    ) -> Result<Vec<WirelessNetwork<'_>>, UnifiedError> {
        let response = self
            .request(Method::Get, &format!("/api/s/{}/rest/wlanconf", site))
            .send()
            .await?
            .json::<Response<Vec<RemoteWirelessNetwork>>>()
            .await?;

        let networks = response
            .data
            .into_iter()
            .map(|network| WirelessNetwork {
                unified: &self,
                site: site.to_string(),

                id: network.id,
                name: network.name,
                enabled: network.enabled,
                security: network.security,
                wpa: WirelessNetworkWpa {
                    mode: network.wpa_mode,
                    encryption: network.wpa_enc,
                },
                vlan: network.vlan.map(|vlan| u16::from_str(&vlan).ok()).flatten(),
            })
            .collect();

        Ok(networks)
    }

    pub async fn wireless_network(
        &self,
        site: &str,
        network_ref: WirelessNetworkRef<'_>,
    ) -> Result<Option<WirelessNetwork<'_>>, UnifiedError> {
        Ok(self
            .wireless_networks(&site)
            .await?
            .into_iter()
            .find(|network| match network_ref {
                WirelessNetworkRef::Id(id) => network.id == id,
                WirelessNetworkRef::Ssid(ssid) => network.name == ssid,
            }))
    }
}
