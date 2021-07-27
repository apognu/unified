use std::{net::IpAddr, str::FromStr};

use chrono::NaiveDateTime;
use macaddr::MacAddr;
use serde::Deserialize;

use crate::{
    clients::{Client, ClientRef},
    unified::{Method, Response},
    Unified, UnifiedError,
};

#[derive(Deserialize)]
struct RemoteClient {
    #[serde(rename = "_id")]
    id: String,
    name: Option<String>,
    mac: String,
    oui: String,
    hostname: Option<String>,
    ip: Option<String>,
    #[serde(rename = "1x_identity")]
    identity: Option<String>,
    last_seen: Option<i64>,
    is_wired: bool,
    is_guest: bool,
    authorized: bool,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    #[serde(rename = "wired-rx_bytes", default)]
    pub wired_rx_bytes: u64,
    #[serde(rename = "wired-tx_bytes", default)]
    pub wired_tx_bytes: u64,
}

impl Unified {
    pub async fn clients(&self, site: &str) -> Result<Vec<Client<'_>>, UnifiedError> {
        let response = self
            .request(Method::Get, &format!("/api/s/{}/stat/sta", site))
            .send()
            .await?
            .json::<Response<Vec<RemoteClient>>>()
            .await?;

        let clients = response
            .data
            .into_iter()
            .map(|client| {
                let seen = client
                    .last_seen
                    .map(|ts| NaiveDateTime::from_timestamp(ts, 0));

                Client {
                    unified: self,
                    site: site.to_string(),

                    id: client.id,
                    name: client.name,
                    mac: MacAddr::from_str(&client.mac).unwrap(),
                    oui: client.oui,
                    hostname: client.hostname,
                    ip: client.ip.and_then(|ip| IpAddr::from_str(&ip).ok()),
                    identity: client.identity,
                    last_seen: seen,
                    wired: client.is_wired,
                    guest: client.is_guest,
                    authorized: client.authorized,
                    rx_bytes: client.rx_bytes,
                    tx_bytes: client.tx_bytes,
                    wired_rx_bytes: client.wired_rx_bytes,
                    wired_tx_bytes: client.wired_tx_bytes,
                }
            })
            .collect();

        Ok(clients)
    }

    pub async fn client(
        &self,
        site: &str,
        client_ref: ClientRef<'_>,
    ) -> Result<Option<Client<'_>>, UnifiedError> {
        let mac = match client_ref {
            ClientRef::Mac(mac) => {
                Some(MacAddr::from_str(mac).map_err(|_| UnifiedError::InvalidMacAddress)?)
            }
            _ => None,
        };

        let ip = match client_ref {
            ClientRef::Ip(ip) => {
                Some(IpAddr::from_str(ip).map_err(|_| UnifiedError::InvalidIpAddress)?)
            }
            _ => None,
        };

        Ok(self
            .clients(&site)
            .await?
            .into_iter()
            .find(|client| match client_ref {
                ClientRef::Id(id) => client.id == id,
                ClientRef::Mac(_) => mac.map(|mac| client.mac == mac).unwrap_or_default(),
                ClientRef::Ip(_) => client.ip == ip,
            }))
    }
}
