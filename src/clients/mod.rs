mod list;
mod state;

use std::net::IpAddr;

use chrono::NaiveDateTime;
use macaddr::MacAddr;

use crate::Unified;

pub enum ClientRef<'r> {
    Id(&'r str),
    Mac(&'r str),
    Ip(&'r str),
}

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
