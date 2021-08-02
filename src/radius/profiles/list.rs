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
  /// List all configured RADIUS profile.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///
  /// # Example
  ///
  /// ```
  /// let profiles = unifi.radius_profiles("default").await;
  /// ```
  pub async fn radius_profiles(&self, site: &str) -> Result<Vec<RadiusProfile>, UnifiedError> {
    let response: Vec<RemoteRadiusProfile> = self.request(Method::GET, &format!("/api/s/{}/rest/radiusprofile", site)).query().await?;

    let profiles = response.into_iter().map(|network| RadiusProfile { id: network.id, name: network.name }).collect();

    Ok(profiles)
  }
}
