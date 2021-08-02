use reqwest::Method;

use crate::{sites::types::*, Unified, UnifiedError};

impl Unified {
  /// List all configured sites on the controller.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///
  /// # Example
  ///
  /// ```
  /// let sites = unifi.sites().await?;
  /// ```
  pub async fn sites(&self) -> Result<Vec<Site>, UnifiedError> {
    let response: Vec<RemoteSite> = self.request(Method::GET, "/api/stat/sites").query().await?;

    let sites = response
      .into_iter()
      .map(|site| {
        let health = site.health.into_iter().fold(SiteHealth::default(), |mut status, subsystem| {
          match subsystem.subsystem.as_str() {
            "www" => status.www = subsystem.status == "ok",
            "wan" => status.wan = subsystem.status == "ok",
            "lan" => status.lan = subsystem.status == "ok",
            "wlan" => status.wlan = subsystem.status == "ok",
            "vpn" => status.vpn = subsystem.status == "ok",
            _ => {}
          }

          status
        });

        Site {
          id: site.id,
          name: site.name,
          description: site.description,
          alarms: site.num_new_alarms,
          health,
        }
      })
      .collect();

    Ok(sites)
  }

  /// Find a specific site by the provided ref.
  ///
  /// Sites can by looked by ID, name and description. The attribute to use is
  /// selected according to the variant of [`SiteRef`] provided to the
  /// function.
  ///
  /// # Arguments
  ///
  ///  * `site_ref` - Attribute and value to use to look up the site
  ///
  /// # Example
  ///
  /// ```
  /// let client = unifi.site(SiteRef::Name("default")).await?;
  /// ```
  pub async fn site(&self, site_ref: SiteRef<'_>) -> Result<Option<Site>, UnifiedError> {
    Ok(self.sites().await?.into_iter().find(|site| match site_ref {
      SiteRef::Id(id) => site.id == id,
      SiteRef::Name(name) => site.name == name,
      SiteRef::Description(description) => site.description == description,
    }))
  }
}
