use std::{borrow::Cow, net::IpAddr, time::Duration};

use ipnet::IpNet;
use serde::{Deserialize, Serialize};

use crate::Unified;

#[derive(Debug, Clone, Copy)]
pub enum NetworkPurpose {
  Corporate,
  Guest,
  Wan,
  VlanOnly,
  RemoteUserVpn,
  SiteToSiteVpn,
  VpnClient,
}

impl ToString for NetworkPurpose {
  fn to_string(&self) -> String {
    let value = match self {
      Self::Corporate => "corporate",
      Self::Guest => "guest",
      Self::Wan => "wan",
      Self::VlanOnly => "vlan-only",
      Self::RemoteUserVpn => "remote-user-vpn",
      Self::SiteToSiteVpn => "site-vpn",
      Self::VpnClient => "vpn-client",
    };

    value.to_string()
  }
}

impl<T> From<T> for NetworkPurpose
where
  T: AsRef<str>,
{
  fn from(value: T) -> Self {
    match value.as_ref() {
      "corporate" => Self::Corporate,
      "guest" => Self::Guest,
      "wan" => Self::Wan,
      "vlan-only" => Self::VlanOnly,
      "remote-user-vpn" => Self::RemoteUserVpn,
      "site-vpn" => Self::SiteToSiteVpn,
      "vpn-client" => Self::VpnClient,
      _ => Self::Corporate,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteNetwork {
  #[serde(rename = "_id", skip_serializing)]
  pub id: String,
  pub name: String,
  #[serde(default = "crate::util::is_true")]
  pub enabled: bool,

  pub purpose: String,
  #[serde(rename = "networkgroup", skip_serializing_if = "Option::is_none")]
  pub network_group: Option<String>,
  #[serde(rename = "wan_networkgroup", skip_serializing_if = "Option::is_none")]
  pub wan_network_group: Option<String>,

  #[serde(rename = "ip_subnet")]
  pub subnet: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain_name: Option<String>,

  #[serde(default)]
  pub vlan_enabled: bool,
  pub vlan: Option<String>,

  #[serde(default)]
  pub dhcpd_enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dhcpd_start: Option<String>,
  #[serde(rename = "dhcpd_stop", skip_serializing_if = "Option::is_none")]
  pub dhcpd_end: Option<String>,
  #[serde(rename = "dhcpd_leasetime", skip_serializing_if = "Option::is_none")]
  pub dhcpd_lease_time: Option<u64>,

  pub vpn_type: Option<String>,
  #[serde(rename = "x_ipsec_pre_shared_key")]
  pub preshared_key: Option<String>,
}

impl From<Network<'_>> for RemoteNetwork {
  fn from(network: Network) -> RemoteNetwork {
    let (lan_group, wan_group) = match network.group {
      NetworkGroup::None => (None, None),
      NetworkGroup::Lan(group) => (Some(group.to_string()), None),
      NetworkGroup::Wan(group) => (None, Some(group.to_string())),
    };

    RemoteNetwork {
      id: network.id,
      name: network.name,
      enabled: network.enabled,

      purpose: network.purpose.to_string(),
      network_group: lan_group,
      wan_network_group: wan_group,

      subnet: network.subnet.map(|subnet| subnet.to_string()),
      domain_name: network.domain,

      vlan_enabled: network.vlan_enabled,
      vlan: network.vlan.map(|vlan| vlan.to_string()),

      dhcpd_enabled: network.dhcp.as_ref().map(|dhcp| dhcp.enabled).unwrap_or_default(),
      dhcpd_start: network.dhcp.as_ref().and_then(|dhcp| dhcp.start.map(|ip| ip.to_string())),
      dhcpd_end: network.dhcp.as_ref().and_then(|dhcp| dhcp.end.map(|ip| ip.to_string())),
      dhcpd_lease_time: network.dhcp.as_ref().and_then(|dhcp| dhcp.lease_duration.map(|duration| duration.as_secs())),

      vpn_type: network.vpn.as_ref().map(|vpn| vpn.kind.to_string()),
      preshared_key: network.vpn.and_then(|vpn| vpn.preshared_key),
    }
  }
}

/// Representation of the attribute used to select a wired network.
pub enum NetworkRef<'r> {
  Id(&'r str),
  Name(&'r str),
  Domain(&'r str),
  Subnet(&'r str),
}

#[derive(Debug, Clone)]
pub enum NetworkGroup<'ng> {
  None,
  Lan(Cow<'ng, str>),
  Wan(Cow<'ng, str>),
}

#[derive(Debug, Clone)]
pub enum VpnType {
  Pptp,
  L2tp,
}

impl ToString for VpnType {
  fn to_string(&self) -> String {
    let value = match self {
      Self::Pptp => "pptp-server",
      Self::L2tp => "l2tp-server",
    };

    value.to_string()
  }
}

impl<T> From<T> for VpnType
where
  T: AsRef<str>,
{
  fn from(value: T) -> Self {
    match value.as_ref() {
      "pptp-server" => Self::Pptp,
      "l2tp-server" => Self::L2tp,
      _ => Self::L2tp,
    }
  }
}

/// A wired network configured on your controller.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Network<'n> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'n Unified,
  pub(crate) site: String,

  pub id: String,
  pub name: String,
  pub enabled: bool,

  pub purpose: NetworkPurpose,
  pub group: NetworkGroup<'n>,

  pub subnet: Option<IpNet>,
  pub domain: Option<String>,

  pub vlan_enabled: bool,
  pub vlan: Option<u16>,

  pub dhcp: Option<NetworkDhcp>,
  pub vpn: Option<NetworkVpn>,
}

#[derive(Debug, Clone)]
pub struct NetworkDhcp {
  pub enabled: bool,
  pub start: Option<IpAddr>,
  pub end: Option<IpAddr>,
  pub lease_duration: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct NetworkVpn {
  pub kind: VpnType,
  pub preshared_key: Option<String>,
}
