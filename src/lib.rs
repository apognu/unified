#[macro_use]
extern crate derivative;

mod alerts;
mod clients;
mod devices;
mod error;
mod events;
mod http;
mod networks;
mod radius;
mod sites;
mod unified;
mod util;
mod wireless;

pub use crate::{
  clients::{Client, ClientRef},
  devices::{Device, DeviceRef, DeviceState},
  error::UnifiedError,
  http::Scheme,
  networks::{Network, NetworkRef},
  radius::{RadiusUser, RadiusUserBuilder, RadiusUserRef, TunnelMediumType, TunnelType},
  sites::{Site, SiteHealth, SiteRef},
  unified::Unified,
  wireless::{WirelessNetwork, WirelessNetworkRef, WirelessNetworkWpa},
};
