#![allow(dead_code)]
use reqwest::blocking::Client as HttpClient;
use std::env::consts::OS;

mod client;
mod output;
mod search;
mod types;

/// Re-exports
pub use crate::client::{OffClientV0, OffClientV2, OffResult};
pub use crate::output::{Locale, Output};
pub use crate::search::Search;
pub use crate::types::ApiVersion;

/// The version of this library.
pub const VERSION: &str = "alpha";

// Authentication tuple (username, password).
#[derive(Debug, PartialEq)]
struct Auth(String, String);

/// The Open Food Facts API client builder.
///
/// # Examples
///
/// ```ignore
/// let off = Off:new(ApiVersion::V0).locale(Locale::new().country("fr")).build()?;
/// ```
#[derive(Debug)]
pub struct OffBuilder {
    // The default locale.
    locale: Locale,
    // Optional. Only needed for write operations.
    auth: Option<Auth>,
    // The User-Agent header value to send on each Off request. Optional.
    // If not given, use the default user agent.
    user_agent: Option<String>,
}

impl OffBuilder {
    /// Create a new builder with defaults:
    ///
    /// * The default locale is set to `Locale::default()`.
    /// * No authentication credentials
    /// * The user agent is set to
    ///   `OffRustClient - {OS name} - Version {lib version} - {github repo URL}`
    pub fn new() -> Self {
        Self {
            locale: Locale::default(),
            auth: None,
            // TODO: Get version and URL from somewhere else ?
            user_agent: Some(format!(
                "OffRustClient - {} - Version {} - {}",
                OS, VERSION, "https://github.com/openfoodfacts/openfoodfacts-rust"
            )),
        }
    }

    /// Set the default locale.
    pub fn locale(mut self, value: Locale) -> Self {
        self.locale = value;
        self
    }

    /// Set the authentication credentials.
    pub fn auth(mut self, username: &str, password: &str) -> Self {
        self.auth = Some(Auth(username.to_string(), password.to_string()));
        self
    }

    /// Set the user agent string.
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    /// Create a new OffClient for the V0 of the API, with the current
    /// builder options.
    /// After build_v0() is called, the builder object is invalid.
    pub fn build_v0(self) -> Result<OffClientV0, reqwest::Error> {
        let client = self.build_http_client()?;
        Ok(OffClientV0::new(self.locale, client))
    }

    /// Create a new OffClient for the V2 of the API, with the current
    /// builder options.
    /// After build_v2() is called, the builder object is invalid.
    pub fn build_v2(self) -> Result<OffClientV2, reqwest::Error> {
        let client = self.build_http_client()?;
        Ok(OffClientV2::new(self.locale, client))
    }

    fn build_http_client(&self) -> reqwest::Result<HttpClient> {
        // Default headers
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(ref auth) = self.auth {
            // TODO: Needs to be encoded !
            let basic_auth = format!("Basic {}:{}", auth.0, auth.1);
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&basic_auth).unwrap(),
            );
        }
        let mut cb = HttpClient::builder();
        if !headers.is_empty() {
            cb = cb.default_headers(headers);
        }
        if let Some(ref user_agent) = self.user_agent {
            cb = cb.user_agent(user_agent);
        }
        // TODO: gzip compression
        // TODO: Timeouts
        cb.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let builder = OffBuilder::new();
        assert_eq!(builder.locale, Locale::default());
        assert_eq!(builder.auth, None);
        assert_eq!(
            builder.user_agent,
            Some(format!(
                "OffRustClient - {} - Version {} - {}",
                OS, "alpha", "https://github.com/openfoodfacts/openfoodfacts-rust"
            ))
        );
    }

    #[test]
    fn options() {
        let builder = OffBuilder::new()
            .locale(Locale::new("gr", None))
            .auth("user", "pwd")
            .user_agent("user agent");
        assert_eq!(builder.locale, Locale::new("gr", None));
        assert_eq!(
            builder.auth,
            Some(Auth(String::from("user"), String::from("pwd")))
        );
        assert_eq!(builder.user_agent, Some(String::from("user agent")));
    }
}
