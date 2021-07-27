use serde_json::json;

use crate::{unified::Method, wireless::WirelessNetwork, UnifiedError};

impl<'wn> WirelessNetwork<'wn> {
    pub async fn enable(&self) -> Result<(), UnifiedError> {
        self.set_state(true).await
    }

    pub async fn disable(&self) -> Result<(), UnifiedError> {
        self.set_state(false).await
    }

    async fn set_state(&self, state: bool) -> Result<(), UnifiedError> {
        self.unified
            .request(
                Method::Put,
                &format!("/api/s/{}/rest/wlanconf/{}", self.site, self.id),
            )
            .json(&json!({ "enabled": state }))
            .send()
            .await?;

        Ok(())
    }
}
