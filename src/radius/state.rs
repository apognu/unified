use crate::{radius::RadiusUser, unified::Method, UnifiedError};

impl<'ru> RadiusUser<'ru> {
  /// Delete the RADIUS user.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(user) = unifi.radius_user("default", RadiusUserRef::Name("joe.shmoe@acme.corp")).await? {
  ///   user.delete().await?;
  /// }
  /// ```
  pub async fn delete(&self) -> Result<(), UnifiedError> {
    self.unified.request(Method::Delete, &format!("/api/s/{}/rest/account/{}", self.site, self.id)).send().await?;

    Ok(())
  }
}
