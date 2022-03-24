#![allow(dead_code)]
use reqwest::blocking::Client;
use reqwest::header;
use std::env::consts::OS;

mod client;
mod locale;
mod output;
mod search;
mod types;

/// Re-exports
pub use crate::client::{OffClient, OffResult};
pub use crate::locale::Locale;
pub use crate::output::Output;
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
pub struct Off {
    version: ApiVersion,
    // The default locale.
    locale: Locale,
    // Optional. Only needed for write operations.
    auth: Option<Auth>,
    // The User-Agent header value to send on each Off request. Optional.
    // If not given, use the default user agent.
    user_agent: Option<String>,
}

impl Off {
    /// Create a new builder with defaults:
    ///
    /// * The default locale is set to `Locale::default()`.
    /// * No authentication credentials
    /// * The user agent is set to
    ///   `OffRustClient - {OS name} - Version {lib version} - {github repo URL}`
    ///
    /// # Arguments
    ///
    /// * version - The API version to use.
    pub fn new(version: ApiVersion) -> Self {
        Self {
            version,
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

    /// Create a new OffClient with the current builder options.
    /// After build() is called, the builder object is invalid.
    pub fn build(self) -> Result<OffClient, reqwest::Error> {
        // Default headers
        let mut headers = header::HeaderMap::new();
        if let Some(auth) = self.auth {
            // TODO: Needs to be encoded !
            let basic_auth = format!("Basic {}:{}", auth.0, auth.1);
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&basic_auth).unwrap(),
            );
        }
        let mut cb = Client::builder();
        if !headers.is_empty() {
            cb = cb.default_headers(headers);
        }
        if let Some(user_agent) = self.user_agent {
            cb = cb.user_agent(user_agent);
        }
        // TODO: gzip compression
        // TODO: Timeouts
        Ok(OffClient {
            version: self.version,
            locale: self.locale,
            client: cb.build()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let builder = Off::new(ApiVersion::V0);
        assert_eq!(builder.version, ApiVersion::V0);
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
        let builder = Off::new(ApiVersion::V0)
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
