use serde::Deserialize;

use crate::{
    sites::{Site, SiteHealth, SiteRef},
    unified::{Method, Response},
    Unified, UnifiedError,
};

#[derive(Deserialize)]
struct RemoteSite {
    #[serde(rename = "_id")]
    id: String,
    name: String,
    #[serde(rename = "desc")]
    description: String,
    num_new_alarms: u64,
    health: Vec<RemoteSiteHealth>,
}

#[derive(Deserialize)]
struct RemoteSiteHealth {
    subsystem: String,
    status: String,
}

impl Unified {
    pub async fn sites(&self) -> Result<Vec<Site>, UnifiedError> {
        let response = self
            .request(Method::Get, "/api/stat/sites")
            .send()
            .await?
            .json::<Response<Vec<RemoteSite>>>()
            .await?;

        let sites = response
            .data
            .into_iter()
            .map(|site| {
                let health =
                    site.health
                        .into_iter()
                        .fold(SiteHealth::default(), |mut status, subsystem| {
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

    pub async fn site(&self, site_ref: SiteRef<'_>) -> Result<Option<Site>, UnifiedError> {
        Ok(self.sites().await?.into_iter().find(|site| match site_ref {
            SiteRef::Id(id) => site.id == id,
            SiteRef::Name(name) => site.name == name,
            SiteRef::Description(description) => site.description == description,
        }))
    }
}
