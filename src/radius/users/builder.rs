use crate::radius::users::types::*;

/// Helper builder to create new RADIUS users.
///
/// Should be created through the [RadiusUser::builder] method.
///
/// # Example
///
/// ```
/// let user =
///    RadiusUser::builder(&unified, "default", "joe.shmoe", "superpassword");
///      .vlan(20)
///      .tunnel_type(TunnelType::VirtualLan)
///      .tunnel_medium_type(TunnelMediumType::Ethernet802)
///      .build();
/// ```
pub struct RadiusUserBuilder<'ru> {
  pub(crate) user: RadiusUser<'ru>,
}

impl<'ru> RadiusUserBuilder<'ru> {
  /// Place the user into a specific VLAN
  ///
  /// # Arguments
  ///
  ///  * `vlan` - VLAN identifier
  pub fn vlan(mut self, vlan: u16) -> RadiusUserBuilder<'ru> {
    self.user.vlan = Some(vlan);
    self
  }

  /// Set the high-level tunnel type for connections used by this user.
  ///
  /// See [TunnelType].
  ///
  /// # Arguments
  ///
  ///  * `tunnel_type` - Tunnel type
  pub fn tunnel_type(mut self, tunnel_type: TunnelType) -> RadiusUserBuilder<'ru> {
    self.user.tunnel_type = Some(tunnel_type);
    self
  }

  /// Set the low-level tunnel type for connections used by this user.
  ///
  /// See [TunnelMediumType].
  ///
  /// # Arguments
  ///
  ///  * `tunnel_medium_type` - Tunnel medium type
  pub fn tunnel_medium_type(mut self, tunnel_medium_type: TunnelMediumType) -> RadiusUserBuilder<'ru> {
    self.user.tunnel_medium_type = Some(tunnel_medium_type);
    self
  }

  /// Finalize the builder and get a [RadiusUser] to be created.
  pub fn build(self) -> RadiusUser<'ru> {
    self.user
  }
}
