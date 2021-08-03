use serde::{Deserialize, Serialize};

use crate::Unified;

#[derive(Serialize, Deserialize)]
pub(super) struct RemoteWirelessNetwork {
  #[serde(rename = "_id", skip_serializing)]
  pub id: String,
  pub name: String,
  pub enabled: bool,
  #[serde(rename = "networkconf_id", skip_serializing_if = "Option::is_none")]
  pub network: Option<String>,
  #[serde(rename = "ap_group_ids")]
  pub ap_groups: Vec<String>,
  #[serde(rename = "wlan_band", skip_serializing_if = "Option::is_none")]
  pub band: Option<String>,
  pub security: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wpa_mode: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wpa_enc: Option<String>,
  #[serde(rename = "x_passphrase", skip_serializing_if = "Option::is_none")]
  pub passphrase: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub vlan: Option<String>,
  #[serde(default)]
  pub hide_ssid: bool,
  #[serde(rename = "radiusprofile_id", skip_serializing_if = "Option::is_none")]
  pub radius_profile: Option<String>,
}

impl From<WirelessNetwork<'_>> for RemoteWirelessNetwork {
  fn from(network: WirelessNetwork) -> RemoteWirelessNetwork {
    RemoteWirelessNetwork {
      id: network.id,
      name: network.name,
      enabled: network.enabled,
      network: network.network,
      band: network.band.map(|band| band.to_string()),
      ap_groups: network.ap_groups,
      security: network.security.to_string(),
      wpa_mode: network.wpa.as_ref().map(|wpa| wpa.mode.to_string()),
      wpa_enc: network.wpa.as_ref().map(|wpa| wpa.encryption.to_string()),
      passphrase: network.passphrase,
      vlan: network.vlan.map(|vlan| vlan.to_string()),
      hide_ssid: !network.advertised,
      radius_profile: network.radius_profile,
    }
  }
}

/// Representation of the attribute used to select a wireless network.
pub enum WirelessNetworkRef<'r> {
  /// Select the wireless network by its internal ID
  Id(&'r str),
  /// Select the internal network by its SSID
  Ssid(&'r str),
}

/// A configured wireless network.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct WirelessNetwork<'wn> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'wn Unified,
  pub(crate) site: String,

  /// Internal ID
  pub id: String,
  /// SSID for the wireless network
  pub name: String,
  /// Is this network enabled?
  pub enabled: bool,
  /// Logical network should traffic from this wireless network by put on
  pub network: Option<String>,
  /// Access Point group this wireless network will be broadcast on
  pub ap_groups: Vec<String>,
  /// Wireless band this wireless network will be broadcast on
  pub band: Option<WirelessBand>,
  /// Should this SSID be advertised?
  pub advertised: bool,
  /// Security type for the wireless network
  pub security: WirelessNetworkSecurity,
  /// Security configuration for WPA2-PSK
  pub wpa: Option<WirelessNetworkWpa>,
  /// Passphrase, if applicable
  pub passphrase: Option<String>,
  /// VLAN ID traffic on this wireless network should be tagged with
  pub vlan: Option<u16>,
  /// RADIUS profile to use in case of 802.1x
  pub radius_profile: Option<String>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum WirelessBand {
  Invalid,
  Band2G,
  Band5G,
  Both,
}

impl ToString for WirelessBand {
  fn to_string(&self) -> String {
    let value = match self {
      Self::Invalid => "invalid",
      Self::Band2G => "2g",
      Self::Band5G => "5g",
      Self::Both => "both",
    };

    value.to_string()
  }
}

impl<T> From<T> for WirelessBand
where
  T: AsRef<str>,
{
  fn from(value: T) -> Self {
    match value.as_ref() {
      "2g" => Self::Band2G,
      "5g" => Self::Band5G,
      "both" => Self::Both,
      _ => Self::Invalid,
    }
  }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum WirelessNetworkSecurity {
  Invalid,
  Open,
  Wep,
  Wpa2,
  WpaEap,
}

impl ToString for WirelessNetworkSecurity {
  fn to_string(&self) -> String {
    let value = match self {
      Self::Invalid => "invalid",
      Self::Open => "open",
      Self::Wep => "wep",
      Self::Wpa2 => "wpapsk",
      Self::WpaEap => "wpaeap",
    };

    value.to_string()
  }
}

impl<T> From<T> for WirelessNetworkSecurity
where
  T: AsRef<str>,
{
  fn from(value: T) -> Self {
    match value.as_ref() {
      "open" => Self::Open,
      "wep" => Self::Wep,
      "wpapsk" => Self::Wpa2,
      "wpaeap" => Self::WpaEap,
      _ => Self::Invalid,
    }
  }
}

/// The WPA security settings for a wireless network.
///
/// TODO: add enums for modes and encryption methods.
#[derive(Debug, Clone)]
pub struct WirelessNetworkWpa {
  /// Supported WPA versions
  pub mode: WirelessNetworkWpaMode,
  /// Encryption method (only `ccmp` is supported as of now)
  pub(crate) encryption: String,
}

/// WPA versions.
#[derive(Debug, Clone, Copy)]
pub enum WirelessNetworkWpaMode {
  /// This value should not be used.
  Invalid,
  /// WPA version 1
  Wpa1,
  /// WPA version 2
  Wpa2,
  /// WPA version 1 and 2
  Both,
}

impl ToString for WirelessNetworkWpaMode {
  fn to_string(&self) -> String {
    let value = match self {
      Self::Invalid => "invalid",
      Self::Wpa1 => "wpa1",
      Self::Wpa2 => "wpa2",
      Self::Both => "both",
    };

    value.to_string()
  }
}

impl<T> From<T> for WirelessNetworkWpaMode
where
  T: AsRef<str>,
{
  fn from(value: T) -> Self {
    match value.as_ref() {
      "wpa1" => Self::Wpa1,
      "wpa2" => Self::Wpa2,
      "both" => Self::Both,
      _ => Self::Invalid,
    }
  }
}
