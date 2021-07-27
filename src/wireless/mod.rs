mod list;
mod state;

use crate::Unified;

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
  pub security: String,
  pub wpa: Option<WirelessNetworkWpa>,
  pub passphrase: Option<String>,
  pub vlan: Option<u16>,
}

/// The WPA security settings for a wireless network.
#[derive(Debug)]
pub struct WirelessNetworkWpa {
  pub mode: String,
  pub encryption: String,
}
