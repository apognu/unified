use crate::{
  wireless::networks::types::{WirelessNetwork, WirelessNetworkSecurity, WirelessNetworkWpa},
  WirelessBand,
};

/// Builder used to configure a wireless network.
pub struct WirelessNetworkBuilder<'n> {
  pub(crate) network: WirelessNetwork<'n>,
}

impl<'wn> WirelessNetworkBuilder<'wn> {
  /// Set the state of the wireless network.
  pub fn enabled(mut self, enabled: bool) -> WirelessNetworkBuilder<'wn> {
    self.network.enabled = enabled;
    self
  }

  /// Configure the logical network this wireless network is attached to.
  pub fn network(mut self, network: &str) -> WirelessNetworkBuilder<'wn> {
    self.network.network = Some(network.to_string());
    self
  }

  /// Configure the wireless band this wireless network is broadcast on.
  pub fn band(mut self, band: WirelessBand) -> WirelessNetworkBuilder<'wn> {
    self.network.band = Some(band);
    self
  }

  /// Set the Access Point group this wireless network will be broadcast on.
  pub fn ap_groups(mut self, groups: Vec<&str>) -> WirelessNetworkBuilder<'wn> {
    self.network.ap_groups = groups.iter().map(ToString::to_string).collect();
    self
  }

  /// Configure the type of security for this network.
  pub fn security(mut self, security: WirelessNetworkSecurity) -> WirelessNetworkBuilder<'wn> {
    self.network.security = security;
    self
  }

  /// Configure WPA security for this network.
  pub fn wpa(mut self, wpa: WirelessNetworkWpa) -> WirelessNetworkBuilder<'wn> {
    self.network.wpa = Some(wpa);
    self
  }

  /// Set the passphrase (if applicable) for this network.
  pub fn passphrase(mut self, passphrase: &str) -> WirelessNetworkBuilder<'wn> {
    self.network.passphrase = Some(passphrase.to_string());
    self
  }

  /// Set the VLAN ID for traffic on this network.
  pub fn vlan(mut self, vlan: u16) -> WirelessNetworkBuilder<'wn> {
    self.network.vlan = Some(vlan);
    self
  }

  /// Set the state of advertisement for this SSID.
  pub fn advertised(mut self, advertised: bool) -> WirelessNetworkBuilder<'wn> {
    self.network.advertised = advertised;
    self
  }

  /// Build the wireless network.
  pub fn build(self) -> WirelessNetwork<'wn> {
    self.network
  }
}
