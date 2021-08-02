use crate::Unified;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RemoteApGroup {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct ApGroup<'g> {
  #[derivative(Debug = "ignore")]
  pub unified: &'g Unified,
  pub site: String,

  pub id: String,
  pub name: String,
}
