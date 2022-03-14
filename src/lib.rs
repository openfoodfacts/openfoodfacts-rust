#![allow(dead_code)]
pub use crate::client::{HttpClient, OffClient, OffResult};
pub use crate::output::{Locale, Output};
use crate::types::{Version, V0, V2};
use std::env::consts::OS;

mod client;
mod output;
mod search;
mod types;

/// The version of this library.
pub const VERSION: &str = "alpha";

/// Return a builder to build an OffClient supporting the API V0.
///
/// ```
/// use openfoodfacts as off;
/// let client = off::v0().locale(off::Locale::new("fr", None)).build().unwrap();
/// ```
pub fn v0() -> OffBuilder<V0> {
    OffBuilder::new(V0 {})
}

/// Return a builder to build an OffClient supporting the API V2.
///
/// ```
/// use openfoodfacts as off;
/// let client = off::v2().locale(off::Locale::new("fr", None)).build().unwrap();
/// ```
pub fn v2() -> OffBuilder<V2> {
    OffBuilder::new(V2 {})
}

// Authentication tuple (username, password).
#[derive(Debug, PartialEq)]
struct Auth(String, String);

/// The Open Food Facts API client builder.
#[derive(Debug)]
pub struct OffBuilder<V> {
    // The version marker
    v: V,
    // The default locale.
    locale: Locale,
    // Optional. Only needed for write operations.
    auth: Option<Auth>,
    // The User-Agent header value to send on each request. Optional.
    // If not given, use the default user agent.
    user_agent: Option<String>,
}

impl<V> OffBuilder<V>
where
    V: Version + Copy,
{
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

    /// Create a new OffClient for the <V> version of the API, with the current
    /// builder options.
    /// After build() is called, the builder object is invalid.
    pub fn build(self) -> Result<OffClient<V>, reqwest::Error> {
        let client = self.build_http_client()?;
        Ok(OffClient::new(self.v, self.locale, client))
    }

    // Create a new builder with defaults:
    //
    // * The default locale is set to `Locale::default()`.
    // * No authentication credentials
    // * The user agent is set to
    //   `OffRustClient - {OS name} - Version {lib version} - {github repo URL}`
    //
    // # Arguments:
    //
    // * v: A version marker object.
    fn new(v: V) -> Self {
        Self {
            v,
            locale: Locale::default(),
            auth: None,
            // TODO: Get version and URL from somewhere else ?
            user_agent: Some(format!(
                "OffRustClient - {} - Version {} - {}",
                OS, VERSION, "https://github.com/openfoodfacts/openfoodfacts-rust"
            )),
        }
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
        let builder = v0();
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
        let builder = v0()
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
