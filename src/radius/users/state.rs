use reqwest::Method;

use crate::{
  http::ApiV1NoData,
  radius::users::{builder::RadiusUserBuilder, types::*},
  Unified, UnifiedError,
};

impl<'ru> RadiusUser<'ru> {
  /// Create a new RadiusUser builder.
  ///
  /// # Arguments
  ///
  ///  * `unified`  - instance of Unified
  ///  * `site`     - Unifi site where the user should be created
  ///  * `name`     - Username for the new user
  ///  * `password` - Password for the new user
  ///
  /// # Example
  ///
  /// ```
  /// let user = RadiusUser::builder(&unifi, "default", "joe.shmoe", "superpassword");
  /// ```
  pub fn builder(unified: &'ru Unified, site: &str, name: &str, password: &str) -> RadiusUserBuilder<'ru> {
    RadiusUserBuilder {
      user: RadiusUser {
        unified,
        site: site.to_string(),
        id: String::new(),
        name: name.to_string(),
        password: password.to_string(),
        vlan: None,
        tunnel_type: None,
        tunnel_medium_type: None,
      },
    }
  }

  /// Create a new RADIUS user.
  ///
  /// # Example
  ///
  /// ```
  /// RadiusUser::builder(&unifi, "default", "joe.shmoe", "hispassword").build().create().await?;
  /// ```
  pub async fn create(self) -> Result<(), UnifiedError> {
    let body: RemoteRadiusUser = self.clone().into();

    self
      .unified
      .request::<ApiV1NoData>(Method::POST, &format!("/api/s/{}/rest/account", self.site))
      .map(|r| r.json(&body))
      .query()
      .await?;

    Ok(())
  }

  /// Update a RADIUS user.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(mut user) = unifi.radius_user("default", RadiusUserRef::Name("joe.shmoe@acme.corp")).await? {
  ///   user.password = "newpassword".to_string();
  ///   user.update().await?;
  /// }
  /// ```
  pub async fn update(self) -> Result<(), UnifiedError> {
    let body: RemoteRadiusUser = self.clone().into();

    self
      .unified
      .request::<ApiV1NoData>(Method::PUT, &format!("/api/s/{}/rest/account/{}", self.site, self.id))
      .map(|r| r.json(&body))
      .query()
      .await?;

    Ok(())
  }

  /// Delete the RADIUS user.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(user) = unifi.radius_user("default", RadiusUserRef::Name("joe.shmoe@acme.corp")).await? {
  ///   user.delete().await?;
  /// }
  /// ```
  pub async fn delete(self) -> Result<(), UnifiedError> {
    self
      .unified
      .request::<ApiV1NoData>(Method::DELETE, &format!("/api/s/{}/rest/account/{}", self.site, self.id))
      .query()
      .await?;

    Ok(())
  }
}
