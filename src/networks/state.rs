use reqwest::Method;

use crate::{
  http::ApiV1NoData,
  networks::{builder::NetworkBuilder, types::*},
  Unified, UnifiedError,
};

impl<'n> Network<'n> {
  /// Create a builder for a network.
  ///
  /// # Arguments
  ///
  ///  * `site`    - Name of the site to use
  ///  * `name`    - Name of the network
  ///  * `purpose` - Type of network
  ///  * `group`   - Physical interface for this network
  ///
  /// # Example
  ///
  /// ```
  /// let network = Network::builder(&unifi, "default", "ACME - Employees", NetworkPurpose::Corporate, NetworkGroup::Lan("LAN1".to_string()))
  ///   .build();
  /// ```
  pub fn builder(unified: &'n Unified, site: &str, name: &str) -> NetworkBuilder<'n> {
    NetworkBuilder {
      network: Network {
        unified,
        site: site.to_string(),

        id: String::new(),
        name: name.to_string(),
        enabled: true,

        purpose: NetworkPurpose::Invalid,
        group: NetworkGroup::Invalid,

        subnet: None,
        domain: None,

        vlan_enabled: false,
        vlan: None,

        dhcp: None,
        vpn: None,
      },
    }
  }

  /// Create a network.
  ///
  /// # Example
  ///
  /// ```
  /// Network::builder(&unifi, "default", "Employees", NetworkPurpose::Corporate, NetworkGroup::Lan("LAN1".to_string()))
  ///   .build()
  ///   .create()
  ///   .await?;
  /// ```
  pub async fn create(self) -> Result<(), UnifiedError> {
    let body: RemoteNetwork = self.clone().into();

    self
      .unified
      .request::<ApiV1NoData>(Method::POST, &format!("/api/s/{}/rest/networkconf", self.site))
      .map(|r| r.json(&body))
      .query()
      .await?;

    Ok(())
  }

  /// Update the network.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(mut network) = unifi.network("default", NetworkRef::Name("ACME - Employees")).await? {
  ///   network.domain = Some("employees.acme.corp");
  ///   network.update().await?;
  /// }
  /// ```
  pub async fn update(self) -> Result<(), UnifiedError> {
    let body: RemoteNetwork = self.clone().into();

    self
      .unified
      .request::<ApiV1NoData>(Method::PUT, &format!("/api/s/{}/rest/networkconf/{}", self.site, self.id))
      .map(|r| r.json(&body))
      .query()
      .await?;

    Ok(())
  }

  /// Delete the network.
  ///
  /// # Example
  ///
  /// ```
  /// if let Some(network) = unifi.network("default", NetworkRef::Name("ACME - Employees")).await? {
  ///   network.delete().await?;
  /// }
  /// ```
  pub async fn delete(self) -> Result<(), UnifiedError> {
    self
      .unified
      .request::<ApiV1NoData>(Method::DELETE, &format!("/api/s/{}/rest/networkconf/{}", self.site, self.id))
      .query()
      .await?;

    Ok(())
  }
}
