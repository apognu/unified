use std::str::FromStr;

use ipnet::IpNet;
use reqwest::Method;
use serde::Deserialize;

use crate::{
  networks::{Network, NetworkRef},
  Unified, UnifiedError,
};

#[derive(Deserialize)]
pub struct RemoteNetwork {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  #[serde(default = "crate::util::is_true")]
  pub enabled: bool,
  pub purpose: String,
  pub ip_subnet: Option<String>,
  pub domain_name: Option<String>,
  pub vlan: Option<String>,
}

impl Unified {
  /// List all configured networks on the given site.
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
  pub async fn networks(&self, site: &str) -> Result<Vec<Network>, UnifiedError> {
    let response: Vec<RemoteNetwork> = self.request(Method::GET, &format!("/api/s/{}/rest/networkconf", site)).send().await?;

    let networks = response
      .into_iter()
      .map(|network| {
        let subnet = network.ip_subnet.map(|ip| IpNet::from_str(&ip).ok()).flatten();

        Network {
          id: network.id,
          name: network.name,
          enabled: network.enabled,
          subnet,
          purpose: network.purpose,
          domain: network.domain_name,
          vlan: network.vlan.map(|vlan| u16::from_str(&vlan).ok()).flatten(),
        }
      })
      .collect();

    Ok(networks)
  }

  /// Find a specific configured network by the provided ref.
  ///
  /// Wired networks can by looked by ID, name, domain name and subnet. The
  /// attribute to use is selected according to the variant of [`NetworkRef`]
  /// provided to the function.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///  * `network_ref` - Attribute and value to use to look up the network
  ///
  /// # Example
  ///
  /// ```
  /// let network = unifi.network("default", ClientRef::Subnet("10.10.0.0/16")).await?;
  /// ```
  pub async fn network(&self, site: &str, network_ref: NetworkRef<'_>) -> Result<Option<Network>, UnifiedError> {
    let subnet = match network_ref {
      NetworkRef::Subnet(subnet) => IpNet::from_str(subnet).ok(),
      _ => None,
    };

    Ok(self.networks(site).await?.into_iter().find(|network| match network_ref {
      NetworkRef::Id(id) => network.id == id,
      NetworkRef::Name(name) => network.name == name,
      NetworkRef::Subnet(_) => network.subnet == subnet,
      NetworkRef::Domain(domain) => network.domain.as_ref().map(|dom| dom == domain).unwrap_or_default(),
    }))
  }
}
