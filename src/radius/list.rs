use std::str::FromStr;

use num_traits::{FromPrimitive, ToPrimitive};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
  radius::{RadiusUser, RadiusUserRef},
  Unified, UnifiedError,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RemoteRadiusUser {
  #[serde(skip_serializing, rename = "_id")]
  pub(crate) id: String,
  pub(crate) name: String,
  #[serde(rename = "x_password")]
  pub(crate) password: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) vlan: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) tunnel_type: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) tunnel_medium_type: Option<u16>,
}

impl From<RadiusUser<'_>> for RemoteRadiusUser {
  fn from(user: RadiusUser) -> RemoteRadiusUser {
    RemoteRadiusUser {
      id: user.id,
      name: user.name,
      password: user.password,
      vlan: user.vlan.map(|vlan| vlan.to_string()),
      tunnel_type: user.tunnel_type.and_then(|tt| ToPrimitive::to_u16(&tt)),
      tunnel_medium_type: user.tunnel_medium_type.and_then(|tmt| ToPrimitive::to_u16(&tmt)),
    }
  }
}

impl Unified {
  /// List all configured RADIUS users.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///
  /// # Example
  ///
  /// ```
  /// let users = unifi.users("default").await?;
  /// ```
  pub async fn radius_users(&self, site: &str) -> Result<Vec<RadiusUser<'_>>, UnifiedError> {
    let response: Vec<RemoteRadiusUser> = self.request(Method::GET, &format!("/api/s/{}/rest/account", site)).query().await?;

    let users = response
      .into_iter()
      .map(|user| RadiusUser {
        unified: self,
        site: site.to_string(),

        id: user.id,
        name: user.name,
        vlan: user.vlan.map(|vlan| u16::from_str(&vlan).ok()).flatten(),
        password: user.password,
        tunnel_type: user.tunnel_type.and_then(FromPrimitive::from_u16),
        tunnel_medium_type: user.tunnel_medium_type.and_then(FromPrimitive::from_u16),
      })
      .collect();

    Ok(users)
  }

  /// Find a specific RADIUS user by the provided ref.
  ///
  /// RADIUS users can by looked by ID and username. The attribute to use is
  /// selected according to the variant of [`RadiusUserRef`] provided to the
  /// function.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///  * `user_ref` - Attribute and value to use to look up the RADIUS user
  ///
  /// # Example
  ///
  /// ```
  /// let user = unifi.user("default", ClientRef::Name("joe.shmoe@acme.corp")).await?;
  /// ```
  pub async fn radius_user(&self, site: &str, user_ref: RadiusUserRef<'_>) -> Result<Option<RadiusUser<'_>>, UnifiedError> {
    Ok(self.radius_users(&site).await?.into_iter().find(|user| match user_ref {
      RadiusUserRef::Id(id) => user.id == id,
      RadiusUserRef::Name(name) => user.name == name,
    }))
  }
}
