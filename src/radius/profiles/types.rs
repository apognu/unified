/// RADIUS profile configured on the controller.
///
/// # Limitations
///
/// As of now, this interface only allows listing pre-existing profiles.
#[derive(Debug)]
pub struct RadiusProfile {
  /// Internal ID
  pub id: String,
  /// Human-readable name for the RADIUS profile
  pub name: String,
}
