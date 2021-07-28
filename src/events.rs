use chrono::NaiveDateTime;
use reqwest::Method;
use serde::Deserialize;

use crate::{unified::Response, Unified, UnifiedError};

#[derive(Debug)]
pub struct Event {
  pub time: NaiveDateTime,
  pub subsystem: String,
  pub device: Option<String>,
  pub ssid: Option<String>,
  pub message: String,
}

#[derive(Deserialize)]
struct RemoteEvent {
  time: i64,
  subsystem: Option<String>,
  hostname: Option<String>,
  guest: Option<String>,
  sw_name: Option<String>,
  ap_name: Option<String>,
  gw_name: Option<String>,
  ssid: Option<String>,
  #[serde(rename = "msg")]
  message: String,
}

impl Unified {
  pub async fn events(&self, site: &str, limit: Option<u64>) -> Result<Vec<Event>, UnifiedError> {
    let url = match limit {
      Some(limit) => format!("/api/s/{}/stat/event?_limit={}", site, limit),
      None => format!("/api/s/{}/stat/event", site),
    };

    let response = self.request(Method::GET, &url).send().await?.json::<Response<Vec<RemoteEvent>>>().await?;

    let events = response
      .data
      .into_iter()
      .map(|event| {
        let device = event.hostname.or(event.guest).or(event.sw_name).or(event.ap_name).or(event.gw_name);

        let subsystem = event.subsystem.map(|subsystem| subsystem.to_uppercase()).unwrap_or_else(|| "AUTH".to_string());

        Event {
          time: NaiveDateTime::from_timestamp(event.time / 1000, 0),
          device,
          subsystem,
          ssid: event.ssid,
          message: event.message,
        }
      })
      .collect();

    Ok(events)
  }
}
