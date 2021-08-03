use reqwest::Method;

use crate::{http::ApiV2, wireless::groups::types::*, Unified, UnifiedError};

impl Unified {
  /// List pre-existing Access Point groups.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///
  /// # Example
  ///
  /// ```
  /// let groups = unifi.ap_groups("default").await?;
  /// ```
  ///
  /// TODO: use API v2
  pub async fn ap_groups(&self, site: &str) -> Result<Vec<ApGroup<'_>>, UnifiedError> {
    let response = self.request::<ApiV2<Vec<RemoteApGroup>>>(Method::GET, &format!("/v2/api/site/{}/apgroups", site)).query().await?;

    let groups = response
      .into_iter()
      .map(|group| ApGroup {
        unified: self,
        site: site.to_string(),

        id: group.id,
        name: group.name,
      })
      .collect();

    Ok(groups)
  }
}
