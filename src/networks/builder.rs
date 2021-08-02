use std::{
  net::{AddrParseError, IpAddr},
  str::FromStr,
  time::Duration,
};

use ipnet::{AddrParseError as IpNetParseError, IpNet};

use crate::networks::types::{Network, NetworkDhcp, NetworkGroup, NetworkVpn, VpnType};

pub struct NetworkBuilder<'n> {
  pub(crate) network: Network<'n>,
}

impl<'n> NetworkBuilder<'n> {
  pub fn group(mut self, group: NetworkGroup<'n>) -> NetworkBuilder<'n> {
    self.network.group = group;
    self
  }

  pub fn subnet(mut self, subnet: &str) -> Result<NetworkBuilder<'n>, IpNetParseError> {
    self.network.subnet = Some(IpNet::from_str(subnet)?);

    Ok(self)
  }

  pub fn domain(mut self, domain: &str) -> NetworkBuilder<'n> {
    self.network.domain = Some(domain.to_string());
    self
  }

  pub fn vlan(mut self, vlan: u16) -> NetworkBuilder<'n> {
    self.network.vlan_enabled = true;
    self.network.vlan = Some(vlan);
    self
  }

  pub fn dhcp(mut self, range: (&str, &str), lease_time: Option<Duration>) -> Result<NetworkBuilder<'n>, AddrParseError> {
    let (start, end) = range;

    self.network.dhcp = Some(NetworkDhcp {
      enabled: true,
      start: Some(IpAddr::from_str(start)?),
      end: Some(IpAddr::from_str(&end)?),
      lease_duration: lease_time,
    });

    Ok(self)
  }

  pub fn disable_dhcp(mut self) -> NetworkBuilder<'n> {
    if let Some(dhcp) = &mut self.network.dhcp {
      dhcp.enabled = false;
    }

    self
  }

  pub fn vpn(mut self, vpn_type: VpnType, preshared_key: &str) -> NetworkBuilder<'n> {
    self.network.vpn = Some(NetworkVpn {
      kind: vpn_type,
      preshared_key: Some(preshared_key.to_string()),
    });

    self
  }

  pub fn build(self) -> Network<'n> {
    self.network
  }
}
