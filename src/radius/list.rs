use serde::Deserialize;

use crate::{
    radius::{RadiusUser, RadiusUserRef},
    unified::{Method, Response},
    Unified, UnifiedError,
};

#[derive(Deserialize)]
struct RemoteRadiusUser {
    #[serde(rename = "_id")]
    id: String,
    name: String,
    vlan: String,
    x_password: String,
}

impl Unified {
    pub async fn radius_users(&self, site: &str) -> Result<Vec<RadiusUser<'_>>, UnifiedError> {
        let response = self
            .request(Method::Get, &format!("/api/s/{}/rest/account", site))
            .send()
            .await?
            .json::<Response<Vec<RemoteRadiusUser>>>()
            .await?;

        let users = response
            .data
            .into_iter()
            .map(|user| RadiusUser {
                unified: self,
                site: site.to_string(),

                id: user.id,
                name: user.name,
                vlan: user.vlan,
                password: user.x_password,
            })
            .collect();

        Ok(users)
    }

    pub async fn radius_user(
        &self,
        site: &str,
        user_ref: RadiusUserRef<'_>,
    ) -> Result<Option<RadiusUser<'_>>, UnifiedError> {
        Ok(self
            .radius_users(&site)
            .await?
            .into_iter()
            .find(|user| match user_ref {
                RadiusUserRef::Id(id) => user.id == id,
                RadiusUserRef::Name(name) => user.name == name,
            }))
    }
}
