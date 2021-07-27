mod list;
mod state;

use crate::Unified;

/// Representation of the attribute used to select a RADIUS user.
pub enum RadiusUserRef<'r> {
  Id(&'r str),
  Name(&'r str),
}

/// A RADIUS user configured in your RADIUS profile.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct RadiusUser<'ru> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'ru Unified,
  pub(crate) site: String,

  pub id: String,
  pub name: String,
  pub vlan: String,
  pub password: String,
}
