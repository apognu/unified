use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::Unified;

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct RemoteRadiusUser {
  #[serde(skip_serializing, rename = "_id")]
  pub(crate) id: String,
  pub(crate) name: String,
  #[serde(rename = "x_password")]
  pub(crate) password: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) vlan: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) tunnel_type: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) tunnel_medium_type: Option<u16>,
}

impl From<RadiusUser<'_>> for RemoteRadiusUser {
  fn from(user: RadiusUser) -> RemoteRadiusUser {
    RemoteRadiusUser {
      id: user.id,
      name: user.name,
      password: user.password,
      vlan: user.vlan.map(|vlan| vlan.to_string()),
      tunnel_type: user.tunnel_type.and_then(|tt| ToPrimitive::to_u16(&tt)),
      tunnel_medium_type: user.tunnel_medium_type.and_then(|tmt| ToPrimitive::to_u16(&tmt)),
    }
  }
}

/// Representation of the attribute used to select a RADIUS user.
pub enum RadiusUserRef<'r> {
  Id(&'r str),
  Name(&'r str),
}

/// List of tunnel types for RADIUS users
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum TunnelType {
  Unknown = 0,
  Pptp = 1,
  L2f = 2,
  L2tp = 3,
  Atmp = 4,
  Vtp = 5,
  Ah = 6,
  IpIp = 7,
  MinIpIp = 8,
  Esp = 9,
  Gre = 10,
  Dvs = 11,
  IpInIpTunneling = 12,
  VirtualLan = 13,
}

/// Lit of tunnel media types for RADIUS users
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum TunnelMediumType {
  Unknown = 0,
  Ip4 = 1,
  Ip6 = 2,
  Nsap = 3,
  Hdlc = 4,
  Bbn1822 = 5,
  Ethernet802 = 6,
  E163 = 7,
  E164 = 8,
  F69 = 9,
  X121 = 10,
  Ipx = 11,
  AppleTalk = 12,
  DecnetIV = 13,
  BanyanVines = 14,
  E164WithNsap = 15,
}

/// A RADIUS user configured in your RADIUS profile.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct RadiusUser<'ru> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'ru Unified,
  pub(crate) site: String,

  pub id: String,
  pub name: String,
  pub password: String,
  pub vlan: Option<u16>,
  pub tunnel_type: Option<TunnelType>,
  pub tunnel_medium_type: Option<TunnelMediumType>,
}
