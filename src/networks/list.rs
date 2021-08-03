use std::{net::IpAddr, str::FromStr, time::Duration};

use ipnet::IpNet;
use reqwest::Method;

use crate::{http::ApiV1, networks::types::*, Unified, UnifiedError};

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
  pub async fn networks(&self, site: &str) -> Result<Vec<Network<'_>>, UnifiedError> {
    let response = self.request::<ApiV1<Vec<RemoteNetwork>>>(Method::GET, &format!("/api/s/{}/rest/networkconf", site)).query().await?;

    let networks = response
      .into_iter()
      .map(|network| {
        let group = match (network.network_group, network.wan_network_group) {
          (Some(group), None) => NetworkGroup::Lan(group),
          (None, Some(group)) => NetworkGroup::Wan(group),
          _ => NetworkGroup::Invalid,
        };

        let subnet = network.subnet.and_then(|subnet| IpNet::from_str(&subnet).ok());
        let dhcpd_start = network.dhcpd_start.and_then(|ip| IpAddr::from_str(&ip).ok());
        let dhcpd_end = network.dhcpd_end.and_then(|ip| IpAddr::from_str(&ip).ok());

        let vpn = match network.vpn_type {
          Some(kind) => Some(NetworkVpn {
            kind: VpnType::from(kind),
            preshared_key: network.preshared_key,
          }),

          None => None,
        };

        Network {
          unified: self,
          site: site.to_string(),

          id: network.id,
          name: network.name,
          enabled: network.enabled,

          purpose: NetworkPurpose::from(network.purpose),
          group,

          subnet,
          domain: network.domain_name,

          vlan_enabled: network.vlan_enabled,
          vlan: network.vlan.map(|vlan| u16::from_str(&vlan).ok()).flatten(),

          dhcp: Some(NetworkDhcp {
            enabled: network.dhcpd_enabled,
            start: dhcpd_start,
            end: dhcpd_end,
            lease_duration: network.dhcpd_lease_time.map(Duration::from_secs),
          }),

          vpn,
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
  pub async fn network(&self, site: &str, network_ref: NetworkRef<'_>) -> Result<Option<Network<'_>>, UnifiedError> {
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
