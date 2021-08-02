use reqwest::Method;

use crate::{
  networks::{builder::NetworkBuilder, types::*},
  Unified, UnifiedError,
};

impl<'n> Network<'n> {
  pub fn builder(unified: &'n Unified, site: &str, name: &str, purpose: NetworkPurpose, group: NetworkGroup<'n>) -> NetworkBuilder<'n> {
    NetworkBuilder {
      network: Network {
        unified,
        site: site.to_string(),

        id: String::new(),
        name: name.to_string(),
        enabled: true,

        purpose,
        group,

        subnet: None,
        domain: None,

        vlan_enabled: false,
        vlan: None,

        dhcp: None,
        vpn: None,
      },
    }
  }

  pub async fn create(self) -> Result<(), UnifiedError> {
    let body: RemoteNetwork = self.clone().into();

    self
      .unified
      .request::<Vec<RemoteNetwork>>(Method::POST, &format!("/api/s/{}/rest/networkconf", self.site))
      .map(|r| r.json(&body))
      .query()
      .await?;

    Ok(())
  }

  pub async fn delete(self) -> Result<(), UnifiedError> {
    self
      .unified
      .request::<Vec<()>>(Method::DELETE, &format!("/api/s/{}/rest/networkconf/{}", self.site, self.id))
      .query()
      .await?;

    Ok(())
  }
}
