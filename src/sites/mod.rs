mod list;

/// Representation of the attribute used to select a site.
pub enum SiteRef<'r> {
  Id(&'r str),
  Name(&'r str),
  Description(&'r str),
}

/// A location where Unifi hardware operates.
#[derive(Debug)]
pub struct Site {
  pub id: String,
  pub name: String,
  pub description: String,
  pub alarms: u64,
  pub health: SiteHealth,
}

/// Various data about overall health of a Unifi site.
#[derive(Debug, Default)]
pub struct SiteHealth {
  pub www: bool,
  pub wan: bool,
  pub lan: bool,
  pub wlan: bool,
  pub vpn: bool,
}
