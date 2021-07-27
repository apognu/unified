use cookie::Cookie;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};

use crate::UnifiedError;

#[derive(Serialize)]
struct Credentials {
    username: String,
    password: String,
    remember: bool,
}

#[derive(Deserialize)]
pub(crate) struct Response<T> {
    pub(crate) data: T,
}

pub enum Scheme {
    Http,
    Https,
}

impl Scheme {
    fn as_str(&self) -> &'static str {
        match self {
            &Self::Http => "http",
            &Self::Https => "https",
        }
    }
}

/// Handle to an authenticated connection to a Unifi controller
pub struct Unified {
    scheme: Scheme,
    host: String,
    token: String,
}

pub(crate) enum Method {
    Get,
    Post,
    Put,
}

impl Unified {
    /// Creates a Unified handle with a separately acquired token
    pub fn from_token(scheme: Scheme, host: &str, token: &str) -> Unified {
        Unified {
            scheme,
            host: host.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn from_credentials(
        scheme: Scheme,
        host: &str,
        username: &str,
        password: &str,
    ) -> Result<Unified, UnifiedError> {
        let host = host.to_string();
        let credentials = Credentials {
            username: username.to_string(),
            password: password.to_string(),
            remember: true,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}://{}/api/login", scheme.as_str(), host))
            .json(&credentials)
            .send()
            .await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(UnifiedError::InvalidCredentials);
        }
        if response.status() != StatusCode::OK {
            return Err(UnifiedError::Unknown);
        }

        let cookies = response
            .headers()
            .get_all("set-cookie")
            .into_iter()
            .map(|cookie| Cookie::parse(cookie.to_str().unwrap_or_default()).ok())
            .flatten()
            .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
            .collect::<Vec<String>>();

        Ok(Unified {
            scheme,
            host,
            token: cookies.join("; "),
        })
    }

    pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let client = Client::new();
        let url = format!("{}://{}{}", self.scheme.as_str(), self.host, path);

        let builder = match method {
            Method::Get => client.get(&url),
            Method::Post => client.post(&url),
            Method::Put => client.put(&url),
        };

        builder.header("cookie", &self.token)
    }
}
