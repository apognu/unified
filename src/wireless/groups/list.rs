use reqwest::Method;

use crate::{wireless::groups::types::*, Unified, UnifiedError};

impl Unified {
  pub async fn ap_groups(&self, site: &str) -> Result<Vec<ApGroup<'_>>, UnifiedError> {
    let response: Vec<RemoteApGroup> = self.request(Method::GET, &format!("/v2/api/site/{}/apgroups", site)).query_v2().await?;

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
