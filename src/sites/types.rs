use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct RemoteSite {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  #[serde(rename = "desc")]
  pub description: String,
  pub num_new_alarms: u64,
  pub health: Vec<RemoteSiteHealth>,
}

#[derive(Deserialize)]
pub(super) struct RemoteSiteHealth {
  pub subsystem: String,
  pub status: String,
}

/// Representation of the attribute used to select a site.
pub enum SiteRef<'r> {
  /// Select the site by its internal ID
  Id(&'r str),
  /// Select the site by its slug
  Name(&'r str),
  /// Select the site by its human-friendly name (description)
  Description(&'r str),
}

/// A location where Unifi hardware operates.
#[derive(Debug)]
pub struct Site {
  /// Internal ID
  pub id: String,
  /// Site slug ID (called "name" in the controller)
  pub name: String,
  /// Site description (called "description" in the controller)
  pub description: String,
  /// Number of active alerts for the network
  pub alarms: u64,
  /// Health information about the network
  pub health: SiteHealth,
}

/// Various data about overall health of a Unifi site.
#[derive(Debug, Default)]
pub struct SiteHealth {
  /// State of the Internet access
  pub www: bool,
  /// State of the upstream WAN connection
  pub wan: bool,
  /// State of the LAN network
  pub lan: bool,
  /// State of the wireless network
  pub wlan: bool,
  /// State of your VPN networks
  pub vpn: bool,
}
