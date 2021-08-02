use serde::Deserialize;

use crate::Unified;

#[derive(Deserialize)]
pub(super) struct RemoteWirelessNetwork {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub enabled: bool,
  pub security: String,
  pub wpa_mode: String,
  pub wpa_enc: String,
  #[serde(rename = "x_passphrase")]
  pub passphrase: Option<String>,
  pub vlan: Option<String>,
  #[serde(default)]
  pub hide_ssid: bool,
}

/// Representation of the attribute used to select a wireless network.
pub enum WirelessNetworkRef<'r> {
  Id(&'r str),
  Ssid(&'r str),
}

/// A configured wireless network.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct WirelessNetwork<'wn> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'wn Unified,
  pub(crate) site: String,

  pub id: String,
  pub name: String,
  pub enabled: bool,
  pub advertised: bool,
  pub security: WirelessNetworkSecurity,
  pub wpa: Option<WirelessNetworkWpa>,
  pub passphrase: Option<String>,
  pub vlan: Option<u16>,
}

#[derive(Debug)]
pub enum WirelessNetworkSecurity {
  Open,
  Wep,
  Wpa2,
  WpaEap,
}

impl ToString for WirelessNetworkSecurity {
  fn to_string(&self) -> String {
    let value = match self {
      Self::Open => "open",
      Self::Wep => "wep",
      Self::Wpa2 => "wpa2",
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
      "wpa2" => Self::Wpa2,
      "wpaeap" => Self::WpaEap,
      _ => Self::Wpa2,
    }
  }
}

/// The WPA security settings for a wireless network.
#[derive(Debug)]
pub struct WirelessNetworkWpa {
  pub mode: String,
  pub encryption: String,
}
