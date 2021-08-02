use crate::Unified;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RemoteApGroup {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
}

/// Group of related wireless Access Points
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct ApGroup<'g> {
  #[derivative(Debug = "ignore")]
  pub(crate) unified: &'g Unified,
  pub(crate) site: String,

  /// Internal ID
  pub id: String,
  /// Name for the access point group
  pub name: String,
}
