#[macro_use]
extern crate derivative;

mod alerts;
mod clients;
mod devices;
mod error;
mod events;
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
  networks::{Network, NetworkRef},
  radius::{RadiusUser, RadiusUserRef},
  sites::{Site, SiteHealth, SiteRef},
  unified::{Scheme, Unified},
  wireless::{WirelessNetwork, WirelessNetworkRef, WirelessNetworkWpa},
};
