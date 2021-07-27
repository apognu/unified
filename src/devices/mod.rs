mod list;

use std::{net::IpAddr, time::Duration};

use macaddr::MacAddr;
use num_derive::FromPrimitive;

pub enum DeviceRef<'r> {
    Id(&'r str),
    Mac(&'r str),
    Ip(&'r str),
}

#[derive(Debug, FromPrimitive)]
pub enum DeviceState {
    Disconnected = 0,
    Connected = 1,
    PendingAdoption = 2,
    PendingUpgrade = 3,
    Upgrading = 4,
    Provisionning = 5,
    HeartbeatMissed = 6,
    Adopting = 7,
    Deleting = 8,
    InformError = 9,
    AdoptionRequired = 10,
    AdoptionFailed = 11,
    Isolated = 12,
    RFScanning = 13,
    ManagedByOther = 14,
    Unknown = 15,
}

#[derive(Debug)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub model: String,
    pub mac: MacAddr,
    pub ip: Option<IpAddr>,
    pub version: String,
    pub upgradable: bool,
    pub state: DeviceState,
    pub uptime: Duration,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}
