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
  clients::types::{Client, ClientRef},
  devices::types::{Device, DeviceRef, DeviceState},
  error::UnifiedError,
  networks::{
    builder::NetworkBuilder,
    types::{Network, NetworkGroup, NetworkPurpose, NetworkRef},
  },
  radius::{
    profiles::types::RadiusProfile,
    users::{
      builder::RadiusUserBuilder,
      types::{RadiusUser, RadiusUserRef, TunnelMediumType, TunnelType},
    },
  },
  sites::types::{Site, SiteHealth, SiteRef},
  unified::Unified,
  wireless::types::{WirelessNetwork, WirelessNetworkRef, WirelessNetworkWpa},
};
