use std::str::FromStr;

use ipnet::IpNet;
use serde::Deserialize;

use crate::{
    networks::{Network, NetworkRef},
    unified::{Method, Response},
    Unified, UnifiedError,
};

#[derive(Deserialize)]
pub struct RemoteNetwork {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(default = "crate::util::is_true")]
    pub enabled: bool,
    pub purpose: String,
    pub ip_subnet: Option<String>,
    pub domain_name: Option<String>,
    pub vlan: Option<String>,
}

impl Unified {
    pub async fn networks(&self, site: &str) -> Result<Vec<Network>, UnifiedError> {
        let response = self
            .request(Method::Get, &format!("/api/s/{}/rest/networkconf", site))
            .send()
            .await?
            .json::<Response<Vec<RemoteNetwork>>>()
            .await?;

        let networks = response
            .data
            .into_iter()
            .map(|network| {
                let subnet = network
                    .ip_subnet
                    .map(|ip| IpNet::from_str(&ip).ok())
                    .flatten();

                Network {
                    id: network.id,
                    name: network.name,
                    enabled: network.enabled,
                    subnet,
                    purpose: network.purpose,
                    domain: network.domain_name,
                    vlan: network.vlan.map(|vlan| u16::from_str(&vlan).ok()).flatten(),
                }
            })
            .collect();

        Ok(networks)
    }

    pub async fn network(
        &self,
        site: &str,
        network_ref: NetworkRef<'_>,
    ) -> Result<Option<Network>, UnifiedError> {
        let subnet = match network_ref {
            NetworkRef::Subnet(subnet) => IpNet::from_str(subnet).ok(),
            _ => None,
        };

        Ok(self
            .networks(site)
            .await?
            .into_iter()
            .find(|network| match network_ref {
                NetworkRef::Id(id) => network.id == id,
                NetworkRef::Name(name) => network.name == name,
                NetworkRef::Subnet(_) => network.subnet == subnet,
                NetworkRef::Domain(domain) => network
                    .domain
                    .as_ref()
                    .map(|dom| dom == domain)
                    .unwrap_or_default(),
            }))
    }
}
