use std::{net::IpAddr, time::Duration};

use macaddr::MacAddr;
use num_derive::FromPrimitive;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct RemoteDevice {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub model: String,
  pub mac: String,
  pub ip: String,
  #[serde(default)]
  pub network_table: Vec<RemoteDeviceNetwork>,
  pub version: String,
  pub upgradable: bool,
  pub state: u32,
  pub uptime: u64,
  pub rx_bytes: u64,
  pub tx_bytes: u64,
}

#[derive(Deserialize)]
pub(super) struct RemoteDeviceNetwork {
  #[serde(rename = "attr_no_delete", default)]
  pub persistent: bool,
  pub ip: String,
}

/// Representation of the attribute used to select a device.
pub enum DeviceRef<'r> {
  /// Select the device by its internal ID
  Id(&'r str),
  /// Select the device by its MAC address
  Mac(&'r str),
  /// Select the device by its IP address
  Ip(&'r str),
}

/// States a device can be in.
#[allow(missing_docs)]
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

impl ToString for DeviceState {
  fn to_string(&self) -> String {
    match self {
      Self::Disconnected => "Disconnected".to_string(),
      Self::Connected => "Connected".to_string(),
      Self::PendingAdoption => "Pending Adoption".to_string(),
      Self::PendingUpgrade => "Pending Upgrade".to_string(),
      Self::Upgrading => "Upgrading".to_string(),
      Self::Provisionning => "Provisionning".to_string(),
      Self::HeartbeatMissed => "Heartbeat Missed".to_string(),
      Self::Adopting => "Adopting".to_string(),
      Self::Deleting => "Deleting".to_string(),
      Self::InformError => "Inform Error".to_string(),
      Self::AdoptionRequired => "Adoption Required".to_string(),
      Self::AdoptionFailed => "Adoption Failed".to_string(),
      Self::Isolated => "Isolated".to_string(),
      Self::RFScanning => "RF Scanning".to_string(),
      Self::ManagedByOther => "Managed By Other".to_string(),
      Self::Unknown => "Unknown".to_string(),
    }
  }
}

/// A Unifi device adopted on the controller.
#[derive(Debug)]
pub struct Device {
  /// Internal ID
  pub id: String,
  /// Device human-readable name
  pub name: String,
  /// Hardware model
  pub model: String,
  /// MAC address
  pub mac: MacAddr,
  /// IP address
  pub ip: Option<IpAddr>,
  /// Firmware version running on the device
  pub version: String,
  /// Is an firmware upgrade available?
  pub upgradable: bool,
  /// State the device is currently in
  pub state: DeviceState,
  /// Current state of the device
  pub uptime: Duration,
  /// Number of bytes received by the device
  pub rx_bytes: u64,
  /// number of bytes sent by the device
  pub tx_bytes: u64,
}
