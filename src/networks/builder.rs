use std::{
  net::{AddrParseError, IpAddr},
  str::FromStr,
  time::Duration,
};

use ipnet::{AddrParseError as IpNetParseError, IpNet};

use crate::{
  networks::types::{Network, NetworkDhcp, NetworkGroup, NetworkVpn, VpnType},
  NetworkPurpose, UnifiedError,
};

/// Builder used to configure a network.
pub struct NetworkBuilder<'n> {
  pub(crate) network: Network<'n>,
}

impl<'n> NetworkBuilder<'n> {
  /// Set the purpose (type) for the network.
  pub fn purpose(mut self, purpose: NetworkPurpose) -> NetworkBuilder<'n> {
    self.network.purpose = purpose;
    self
  }

  /// Set the network group (physical interface) for the network.
  pub fn group(mut self, group: NetworkGroup) -> NetworkBuilder<'n> {
    self.network.group = group;
    self
  }

  /// Set the gateway-subnet for the network.
  pub fn subnet(mut self, subnet: &str) -> Result<NetworkBuilder<'n>, IpNetParseError> {
    self.network.subnet = Some(IpNet::from_str(subnet)?);

    Ok(self)
  }

  /// Set the domain name for the network.
  pub fn domain(mut self, domain: &str) -> NetworkBuilder<'n> {
    self.network.domain = Some(domain.to_string());
    self
  }

  /// Enable VLAN tagging and set the VLAN ID for the network.
  pub fn vlan(mut self, vlan: u16) -> NetworkBuilder<'n> {
    self.network.vlan_enabled = true;
    self.network.vlan = Some(vlan);
    self
  }

  /// Enable and configure DHCP on the network.
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

  /// Disable DHCP on the network.
  pub fn disable_dhcp(mut self) -> NetworkBuilder<'n> {
    if let Some(dhcp) = &mut self.network.dhcp {
      dhcp.enabled = false;
    }

    self
  }

  /// Enable and configure VPN access on the network.
  pub fn vpn(mut self, vpn_type: VpnType, preshared_key: &str) -> NetworkBuilder<'n> {
    self.network.vpn = Some(NetworkVpn {
      kind: vpn_type,
      preshared_key: Some(preshared_key.to_string()),
    });

    self
  }

  /// Build the network.
  pub fn build(self) -> Result<Network<'n>, UnifiedError> {
    if self.network.subnet.is_none() {
      return Err(UnifiedError::MissingAttribute("subnet".to_string()));
    }
    if let NetworkPurpose::Invalid = self.network.purpose {
      return Err(UnifiedError::MissingAttribute("purpose".to_string()));
    }
    if let NetworkGroup::Invalid = self.network.group {
      return Err(UnifiedError::MissingAttribute("group".to_string()));
    }

    Ok(self.network)
  }
}
