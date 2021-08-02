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
  pub authorized: bool,
  pub rx_bytes: u64,
  pub tx_bytes: u64,
  #[serde(rename = "wired-rx_bytes", default)]
  pub wired_rx_bytes: u64,
  #[serde(rename = "wired-tx_bytes", default)]
  pub wired_tx_bytes: u64,
}

/// Representation of the attribute used to select a client.
pub enum ClientRef<'r> {
  Id(&'r str),
  Mac(&'r str),
  Ip(&'r str),
}

/// A client connected to the network.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Client<'c> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'c Unified,
  pub(crate) site: String,

  pub id: String,
  pub name: Option<String>,
  pub mac: MacAddr,
  pub oui: String,
  pub hostname: Option<String>,
  pub ip: Option<IpAddr>,
  pub identity: Option<String>,
  pub last_seen: Option<NaiveDateTime>,
  pub wired: bool,
  pub guest: bool,
  pub authorized: bool,
  pub rx_bytes: u64,
  pub tx_bytes: u64,
  pub wired_rx_bytes: u64,
  pub wired_tx_bytes: u64,
}
