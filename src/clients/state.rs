use serde_json::json;

use crate::{clients::Client, unified::Method, UnifiedError};

impl<'c> Client<'c> {
    pub async fn block(&self) -> Result<(), UnifiedError> {
        self.set_blocked(true).await
    }

    pub async fn unblock(&self) -> Result<(), UnifiedError> {
        self.set_blocked(false).await
    }

    pub async fn kick(&self) -> Result<(), UnifiedError> {
        self.unified
            .request(Method::Post, &format!("/api/s/{}/cmd/stamgr", self.site))
            .json(&json!({ "cmd": "kick-sta", "mac": self.mac.to_string() }))
            .send()
            .await?;

        Ok(())
    }

    async fn set_blocked(&self, blocked: bool) -> Result<(), UnifiedError> {
        let command = if blocked { "block-sta" } else { "unblock-sta" };

        self.unified
            .request(Method::Post, &format!("/api/s/{}/cmd/stamgr", self.site))
            .json(&json!({ "cmd": command, "mac": self.mac.to_string() }))
            .send()
            .await?;

        Ok(())
    }
}
