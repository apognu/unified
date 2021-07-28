use chrono::NaiveDateTime;
use reqwest::Method;
use serde::Deserialize;

use crate::{Unified, UnifiedError};

#[derive(Debug)]
pub struct Alert {
  pub time: NaiveDateTime,
  pub subsystem: String,
  pub device: Option<String>,
  pub message: String,
  pub archived: bool,
}

#[derive(Deserialize)]
struct RemoteAlert {
  time: i64,
  subsystem: String,
  sw_name: Option<String>,
  ap_name: Option<String>,
  gw_name: Option<String>,
  #[serde(rename = "msg")]
  message: String,
  archived: bool,
}

impl Unified {
  pub async fn alerts(&self, site: &str, limit: Option<u64>) -> Result<Vec<Alert>, UnifiedError> {
    let url = match limit {
      Some(limit) => format!("/api/s/{}/stat/alarm?_limit={}", site, limit),
      None => format!("/api/s/{}/stat/alarm", site),
    };

    let response: Vec<RemoteAlert> = self.request(Method::GET, &url).send().await?;

    let events = response
      .into_iter()
      .map(|alert| {
        let device = alert.sw_name.or(alert.ap_name).or(alert.gw_name);

        Alert {
          time: NaiveDateTime::from_timestamp(alert.time / 1000, 0),
          device,
          subsystem: alert.subsystem.to_uppercase(),
          message: alert.message,
          archived: alert.archived,
        }
      })
      .collect();

    Ok(events)
  }
}
