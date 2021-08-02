use serde::{Deserialize, Serialize};

use crate::Unified;

#[derive(Serialize, Deserialize)]
pub(super) struct RemoteWirelessNetwork {
  #[serde(rename = "_id")]
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
  pub wpa_mode: String,
  pub wpa_enc: String,
  #[serde(rename = "x_passphrase", skip_serializing_if = "Option::is_none")]
  pub passphrase: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub vlan: Option<String>,
  #[serde(default)]
  pub hide_ssid: bool,
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
      wpa_mode: network.wpa.as_ref().map(|wpa| wpa.mode.clone()).unwrap_or_default(),
      wpa_enc: network.wpa.as_ref().map(|wpa| wpa.encryption.clone()).unwrap_or_default(),
      passphrase: network.passphrase,
      vlan: network.vlan.map(|vlan| vlan.to_string()),
      hide_ssid: !network.advertised,
    }
  }
}

/// Representation of the attribute used to select a wireless network.
pub enum WirelessNetworkRef<'r> {
  Id(&'r str),
  Ssid(&'r str),
}

/// A configured wireless network.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct WirelessNetwork<'wn> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'wn Unified,
  pub(crate) site: String,

  pub id: String,
  pub name: String,
  pub enabled: bool,
  pub network: Option<String>,
  pub ap_groups: Vec<String>,
  pub band: Option<WirelessBand>,
  pub advertised: bool,
  pub security: WirelessNetworkSecurity,
  pub wpa: Option<WirelessNetworkWpa>,
  pub passphrase: Option<String>,
  pub vlan: Option<u16>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct WirelessNetworkWpa {
  pub mode: String,
  pub encryption: String,
}
