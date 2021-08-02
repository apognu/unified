use std::{net::IpAddr, str::FromStr, time::Duration};

use macaddr::MacAddr;
use num_traits::FromPrimitive;
use reqwest::Method;

use crate::{devices::types::*, Unified, UnifiedError};

impl Unified {
  /// List all adopted device on the given site.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///
  /// # Example
  ///
  /// ```
  /// let devices = unifi.devices("default").await?;
  /// ```
  pub async fn devices(&self, site: &str) -> Result<Vec<Device>, UnifiedError> {
    let response: Vec<RemoteDevice> = self.request(Method::GET, &format!("/api/s/{}/stat/device", site)).query().await?;

    let devices = response
      .into_iter()
      .map(|device| {
        let ip = if !device.network_table.is_empty() {
          device
            .network_table
            .into_iter()
            .find_map(|network| if network.persistent { IpAddr::from_str(&network.ip).ok() } else { None })
        } else {
          IpAddr::from_str(&device.ip).ok()
        };

        let state = FromPrimitive::from_u32(device.state).unwrap_or(DeviceState::Unknown);

        Device {
          id: device.id,
          name: device.name,
          model: device.model,
          mac: MacAddr::from_str(&device.mac).unwrap(),
          ip,
          version: device.version,
          upgradable: device.upgradable,
          state,
          uptime: Duration::from_secs(device.uptime),
          rx_bytes: device.rx_bytes,
          tx_bytes: device.tx_bytes,
        }
      })
      .collect();

    Ok(devices)
  }

  /// Find a specific adopted device by the provided ref.
  ///
  /// Devices can by looked by ID, MAC address and IP address. The attribute
  /// to use is selected according to the variant of [`DeviceRef`] provided to
  /// the function.
  ///
  /// # Arguments
  ///
  ///  * `site` - Name of the site to use
  ///  * `device_ref` - Attribute and value to use to look up the device
  ///
  /// # Example
  ///
  /// ```
  /// let device = unifi.device("default", DeviceRef::Ip("1.2.3.4")).await?;
  /// ```
  pub async fn device(&self, site: &str, device_ref: DeviceRef<'_>) -> Result<Option<Device>, UnifiedError> {
    let mac = match device_ref {
      DeviceRef::Mac(mac) => Some(MacAddr::from_str(mac).map_err(|_| UnifiedError::InvalidMacAddress)?),
      _ => None,
    };

    let ip = match device_ref {
      DeviceRef::Ip(ip) => Some(IpAddr::from_str(ip).map_err(|_| UnifiedError::InvalidIpAddress)?),
      _ => None,
    };

    Ok(self.devices(site).await?.into_iter().find(|device| match device_ref {
      DeviceRef::Id(id) => device.id == id,
      DeviceRef::Mac(_) => mac.map(|mac| device.mac == mac).unwrap_or_default(),
      DeviceRef::Ip(_) => device.ip == ip,
    }))
  }
}
