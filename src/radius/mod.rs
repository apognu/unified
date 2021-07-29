mod builder;
mod list;
mod state;

use crate::Unified;

use num_derive::{FromPrimitive, ToPrimitive};

pub use self::builder::RadiusUserBuilder;

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
