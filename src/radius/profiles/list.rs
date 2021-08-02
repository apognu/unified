use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{radius::profiles::types::RadiusProfile, Unified, UnifiedError};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteRadiusProfile {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
}

impl Unified {
  pub async fn radius_profiles(&self, site: &str) -> Result<Vec<RadiusProfile<'_>>, UnifiedError> {
    let response: Vec<RemoteRadiusProfile> = self.request(Method::GET, &format!("/api/s/{}/rest/radiusprofile", site)).query().await?;

    let profiles = response
      .into_iter()
      .map(|network| RadiusProfile {
        unified: self,
        site: site.to_string(),

        id: network.id,
        name: network.name,
      })
      .collect();

    Ok(profiles)
  }
}
