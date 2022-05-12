use std::net::IpAddr;

use chrono::NaiveDateTime;
use macaddr::MacAddr;
use serde::Deserialize;

use crate::Unified;

#[derive(Deserialize)]
pub(super) struct RemoteClient {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: Option<String>,
  pub mac: String,
  pub oui: String,
  pub hostname: Option<String>,
  pub ip: Option<String>,
  #[serde(rename = "1x_identity")]
  pub identity: Option<String>,
  pub last_seen: Option<i64>,
  pub is_wired: bool,
  pub is_guest: bool,
  #[serde(default)]
  pub authorized: bool,
  #[serde(default)]
  pub rx_bytes: u64,
  #[serde(default)]
  pub tx_bytes: u64,
  #[serde(rename = "wired-rx_bytes", default)]
  pub wired_rx_bytes: u64,
  #[serde(rename = "wired-tx_bytes", default)]
  pub wired_tx_bytes: u64,
}

/// Representation of the attribute used to select a client.
pub enum ClientRef<'r> {
  /// Client is selected by its internal ID
  Id(&'r str),
  /// Client is selected by its MAC address
  Mac(&'r str),
  /// Client is selected by its IP address
  Ip(&'r str),
}

/// A client connected to the network.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Client<'c> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'c Unified,
  pub(crate) site: String,

  /// Internal ID
  pub id: String,
  /// Name (alias) of the client
  pub name: Option<String>,
  /// MAC address
  pub mac: MacAddr,
  /// Vendor name
  pub oui: String,
  /// Hostname
  pub hostname: Option<String>,
  /// IP address
  pub ip: Option<IpAddr>,
  /// 802.1x identity (if applicable)
  pub identity: Option<String>,
  /// Date and time the client was last connected to the network
  pub last_seen: Option<NaiveDateTime>,
  /// Is the client connected through a wired connection?
  pub wired: bool,
  /// Is the client a guest?
  pub guest: bool,
  /// If the client is a guest, are they authorized to communicate over the network?
  pub authorized: bool,
  /// Number of bytes received by the client
  pub rx_bytes: u64,
  /// Number of bytes sent by the client
  pub tx_bytes: u64,
  /// Number of bytes received by the client over a wired connection
  pub wired_rx_bytes: u64,
  /// Number of bytes sent by the client over a wired connection
  pub wired_tx_bytes: u64,
}
