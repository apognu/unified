use crate::{
  wireless::networks::types::{WirelessNetwork, WirelessNetworkSecurity, WirelessNetworkWpa},
  WirelessBand,
};

pub struct WirelessNetworkBuilder<'n> {
  pub(crate) network: WirelessNetwork<'n>,
}

impl<'wn> WirelessNetworkBuilder<'wn> {
  pub fn enabled(mut self, enabled: bool) -> WirelessNetworkBuilder<'wn> {
    self.network.enabled = enabled;
    self
  }

  pub fn network(mut self, network: &str) -> WirelessNetworkBuilder<'wn> {
    self.network.network = Some(network.to_string());
    self
  }

  pub fn band(mut self, band: WirelessBand) -> WirelessNetworkBuilder<'wn> {
    self.network.band = Some(band);
    self
  }

  pub fn ap_groups(mut self, groups: Vec<&str>) -> WirelessNetworkBuilder<'wn> {
    self.network.ap_groups = groups.iter().map(ToString::to_string).collect();
    self
  }

  pub fn security(mut self, security: WirelessNetworkSecurity) -> WirelessNetworkBuilder<'wn> {
    self.network.security = security;
    self
  }

  pub fn wpa(mut self, wpa: WirelessNetworkWpa) -> WirelessNetworkBuilder<'wn> {
    self.network.wpa = Some(wpa);
    self
  }

  pub fn passphrase(mut self, passphrase: &str) -> WirelessNetworkBuilder<'wn> {
    self.network.passphrase = Some(passphrase.to_string());
    self
  }

  pub fn vlan(mut self, vlan: u16) -> WirelessNetworkBuilder<'wn> {
    self.network.vlan = Some(vlan);
    self
  }

  pub fn advertised(mut self, advertised: bool) -> WirelessNetworkBuilder<'wn> {
    self.network.advertised = advertised;
    self
  }

  pub fn build(self) -> WirelessNetwork<'wn> {
    self.network
  }
}
