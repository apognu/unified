use std::{net::IpAddr, time::Duration};

use ipnet::IpNet;
use serde::{Deserialize, Serialize};

use crate::Unified;

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct RemoteNetwork {
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

  #[serde(rename = "ip_subnet", skip_serializing_if = "Option::is_none")]
  pub subnet: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain_name: Option<String>,

  #[serde(default)]
  pub vlan_enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub vlan: Option<String>,

  #[serde(default)]
  pub dhcpd_enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dhcpd_start: Option<String>,
  #[serde(rename = "dhcpd_stop", skip_serializing_if = "Option::is_none")]
  pub dhcpd_end: Option<String>,
  #[serde(rename = "dhcpd_leasetime", skip_serializing_if = "Option::is_none")]
  pub dhcpd_lease_time: Option<u64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub vpn_type: Option<String>,
  #[serde(rename = "x_ipsec_pre_shared_key", skip_serializing_if = "Option::is_none")]
  pub preshared_key: Option<String>,
}

impl From<Network<'_>> for RemoteNetwork {
  fn from(network: Network) -> RemoteNetwork {
    let (lan_group, wan_group) = match network.group {
      NetworkGroup::Invalid => (None, None),
      NetworkGroup::Lan(group) => (Some(group), None),
      NetworkGroup::Wan(group) => (None, Some(group)),
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
  /// Select the network by its internal ID
  Id(&'r str),
  /// Select the network by its name
  Name(&'r str),
  /// Select the network by its domain name
  Domain(&'r str),
  /// Select the network by its subnet
  Subnet(&'r str),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum NetworkPurpose {
  Invalid,
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
      Self::Invalid => "invalid",
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
      _ => Self::Invalid,
    }
  }
}

/// Physical interface on which the network will operate
#[derive(Debug, Clone)]
pub enum NetworkGroup {
  /// No network group, this should not be used
  Invalid,
  /// A LAN interface (LAN1, LAN2, etc.)
  Lan(String),
  /// A WAN interface (WAN1, WAN2, etc.)
  Wan(String),
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

  /// Network internal ID
  pub id: String,
  /// Network name
  pub name: String,
  /// Is the network enabled?
  pub enabled: bool,

  /// Type of network
  pub purpose: NetworkPurpose,
  /// Physical interface for the network
  pub group: NetworkGroup,

  /// "Gateway/subnet" for the network. Should contain the address of the
  /// gateway, and the CIDR of its subnet (e.g. "10.0.0.254/24").
  pub subnet: Option<IpNet>,
  /// Domain name for the network
  pub domain: Option<String>,

  /// Enable VLAN-tagging on the network
  pub vlan_enabled: bool,
  /// VLAN ID for this network
  pub vlan: Option<u16>,

  /// Configure DHCP on this network
  pub dhcp: Option<NetworkDhcp>,
  /// Configure a VPN on this network
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
