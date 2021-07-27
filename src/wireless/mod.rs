mod list;
mod state;

use crate::Unified;

pub enum WirelessNetworkRef<'r> {
    Id(&'r str),
    Ssid(&'r str),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct WirelessNetwork<'wn> {
    #[derivative(Debug = "ignore")]
    pub(crate) unified: &'wn Unified,
    pub(crate) site: String,

    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub security: String,
    pub wpa: WirelessNetworkWpa,
    pub vlan: Option<u16>,
}

#[derive(Debug)]
pub struct WirelessNetworkWpa {
    mode: String,
    encryption: String,
}
