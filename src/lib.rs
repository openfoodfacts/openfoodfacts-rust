// TODO: Reqwest is curretly configured in blocking mode. To support both blocking and
// non-blocking modes one needs to use conditional complilation ?
use std::env::consts::OS;
use std::error::Error;
use reqwest::blocking::{Client, Response};
use reqwest::header;
use url::{Url, ParseError};
// use serde::ser::{Serialize, Serializer};

mod search;

// Builder --------------------------------------------------------------------

// (username, password)
// TODO: Need derive ?
#[derive(Debug, PartialEq)]
struct Auth(String, String);

/// The Open Food Facts API client builder.
/// 
/// # Examples
/// 
/// ```ignore
/// let off = Off:new().locale("fr").build()?;
/// ```
pub struct Off {
    /// The default locale. Should be a country code or "world".
    locale: String,
    /// The authentication credentials. Optional. Only needed for write operations.
    auth: Option<Auth>,
    /// The User-Agent header value to send on each Off request. Optional.
    user_agent: Option<String>
}

impl Off {
    /// Create a new builder with defaults:
    /// 
    /// * The default locale is set to "world".
    /// * No authentication credentials
    /// * The user agent is set to
    ///   `OffRustClient - {OS name} - Version {version} - {github repo URL}`
    pub fn new() -> Self {
        Self {
            locale: "world".to_string(),
            auth: None,
            // TODO: Get version and URL from somewhere else ?
            user_agent: Some(format!(
                "OffRustClient - {} - Version {} - {}",
                OS, "alpha", "https://github.com/openfoodfacts/openfoodfacts-rust"
            ))
        }
    }

    /// Set the default locale.
    pub fn locale(mut self, value: &str) -> Self {
        self.locale = value.to_string();
        self
    }

    /// Set the authentication credentials.
    pub fn auth(mut self, username: &str, password: &str) -> Self {
        self.auth = Some(Auth(username.to_string(), password.to_string()));
        self
    }

    // TODO: Give full usr agent string or allow parameters:
    // appname, platform, version, url
    /// Set the user agent.
    pub fn user_agent(mut self, value: &str) -> Self {
        self.user_agent = Some(value.to_string());
        self
    }

    /// Create a new OffClient with the current builder options.
    /// After build() is called, the builder object is invalid.
    pub fn build(self) -> Result<OffClient, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        if let Some(user_agent) = self.user_agent {
            headers.insert(header::USER_AGENT,
                           header::HeaderValue::from_str(&user_agent).unwrap());
        }
        if let Some(auth) = self.auth {
            // TODO: Needs to be encoded !
            let basic_auth = format!("Basic {}:{}", auth.0, auth.1);
            headers.insert(reqwest::header::AUTHORIZATION,
                           reqwest::header::HeaderValue::from_str(&basic_auth).unwrap());
        }
        // Build the reqwest client.
        let mut cb = Client::builder();
        if !headers.is_empty() {
            cb = cb.default_headers(headers);
        }
        Ok(OffClient {
            locale: self.locale,
            client: cb.build()?
        })
    }
}


// Off client -----------------------------------------------------------------


/// The OFF API client, created using the Off() builder.
/// 
/// All methods return a OffResult object,
pub struct OffClient {
    /// The default locale to use when no locale is given in a method call.
    /// Always the lowercase alpha-2 ISO3166 code.
    locale: String,
    /// The uderlying reqwest client.
    // TODO: not sure if it is possible to use blocking and non-blocking clients
    // transparently.
    client: Client
}

/// The return type of all OffClient methods.
type OffResult = Result<Response, Box<dyn Error>>;


// page and locale should be optional.
// JSON response data can be deserialized into untyped JSON values
// with response.json::<HashMap::<String, serde_json::Value>>()
impl OffClient {
    /// Get the given taxonomy.
    /// 
    /// # OFF API request
    ///
    /// `GET https://world.openfoodfacts.org/data/taxonomies/{taxonomy}.json`
    ///
    /// Taxomonies support only the locale "world". The default client locale
    /// is ignored.
    /// 
    /// # Arguments
    /// 
    /// * `taxonomy` - A string with the taxonomy name.
    pub fn taxonomy(&self, taxonomy: &str) -> OffResult {
        let base_url = self.base_url(Some("world"))?;
        let url = base_url.join(&format!("data/taxonomies/{}.json", taxonomy))?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    /// Get the given facet.
    ///
    /// # OFF API request
    ///
    /// `GET https://{locale}.openfoodfacts.org/{facet}.json`
    ///
    /// * `facet` - A string with the taxonomy name.
    /// * `locale`- Optional locale. Should contain only a country code.
    ///             If missing, uses the default client locale.
    pub fn facet(&self, facet: &str, locale: Option<&str>) -> OffResult {
        let base_url = self.base_url(locale)?;
        let url = base_url.join(&format!("{}.json", facet))?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    /// Get all the categories.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://world.openfoodfacts.org/categories.json
    /// ```
    ///
    /// Categories support only the locale "world". The default client locale
    /// is ignored.
    pub fn categories(&self) -> OffResult {
        let base_url = self.base_url(Some("world"))?;
        let url = base_url.join("categories.json")?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    // TODO:
    //
    //  - products_by_category
    //  - products_with_additive
    //  - products_in_state
    //
    // If a pair <cc>-<lc> is given, the name of the /category/ segment
    // will be localized. VERFIY THIS. If one gives a language code, it
    // should be possible to pass the localized segment name in an optional
    // parameter?

    /// Get all products belonging to the given category.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/category/{category}.json
    /// ```
    ///
    /// # Arguments
    ///
    /// * `category`- The category name.
    /// * `locale`- Optional locale. May contain a country code or a pair
    ///             <country code>-<language code>. If missing, uses the default
    ///             client locale.
    pub fn products_by_category(&self, category: &str, locale: Option<&str>) -> OffResult {
        let base_url = self.base_url(locale)?;
        let url = base_url.join(&format!("category/{}.json", category))?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    /// Get all products containing the given additive.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/additive/{additive}.json
    /// ```
    ///
    /// # Arguments
    ///
    /// * `additive`- The additive name.
    /// * `locale`- Optional locale. May contain a country code or a pair
    ///             <country code>-<language code>. If missing, uses the default
    ///             client locale.
    pub fn products_with_additive(&self, additive: &str, locale: Option<&str>) -> OffResult {
        let base_url = self.base_url(locale)?;
        let url = base_url.join(&format!("additive/{}.json", additive))?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    /// Get all products in the given state.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/state/{state}.json
    /// ```
    ///
    /// # Arguments
    ///
    /// * `state`- The state name.
    /// * `locale`- Optional locale. May contain a country code or a pair
    ///             <country code>-<language code>. If missing, uses the default
    ///             client locale.
    pub fn products_in_state(&self, state: &str, locale: Option<&str>) -> OffResult {
        let base_url = self.base_url(locale)?;
        let url = base_url.join(&format!("state/{}.json", state))?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    /// Get a product by barcode.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/api/v0/product/{barcode}
    /// ```
    ///
    /// # Arguments
    ///
    /// * `barcode` - A string with the product barcode.
    /// * `locale`- Optional locale. Should contain only a country code TODO: VERIFY THIS.
    ///             If missing, uses the default client locale.
    pub fn product_by_barcode(&self, barcode: &str, locale: Option<&str>) -> OffResult {
        let api_url = self.api_url(locale)?;
        let url = api_url.join(&format!("product/{}", barcode))?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    // Search queries ---------------------------------------------------------

    // Search products.
    // TODO: Serialization
    // Option 1
    //  qparams = SearchParams::to_array() -> &[] returns an array of tuples. 
    //  The default serde_urlencoded::to_string() does the actual serialization
    //  as expected by self.client.get(search_url).query(qparams).send()?;
    //
    // Option 2
    //  SearchParams implement Serialize, which builds the array and returns
    //  serde_urlencoded::to_string().
    // pub fn search(&self, params: &SearchParams, page: usize, locale: Option<&str>) -> OffResult {
    //   let search_url = self.search_url(locale)?;
    //   let response = self.client.get(search_url).query(params).send()?;
    //   Ok(response)
    // }

    // TODO: Other ?
    // Get products by additive
    // Get products by category
    // Get product by product state

    fn base_url(&self, locale: Option<&str>) -> Result<Url, ParseError> {
        let url = format!("https://{}.openfoodfacts.org/", locale.unwrap_or(&self.locale));
        Url::parse(&url)
      }

    fn api_url(&self, locale: Option<&str>) -> Result<Url, ParseError> {
        let base = self.base_url(locale)?;
        base.join("api/v0/")
    }

    fn search_url(&self, locale: Option<&str>) -> Result<Url, ParseError> {
        let base = self.base_url(locale)?;
        base.join("cgi/search.pl")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    // **
    // Builder
    // **

    // Get a Builder with default options.
    #[test]
    fn test_builder_default_options() {
        let builder = Off::new();
        assert_eq!(builder.locale, "world");
        assert_eq!(builder.auth, None);
        assert_eq!(builder.user_agent, Some(format!(
            "OffRustClient - {} - Version {} - {}",
            OS, "alpha", "https://github.com/openfoodfacts/openfoodfacts-rust"
        )));
    }

    // Set Builder options.
    #[test]
    fn test_builder_with_options() {
        let builder = Off::new().locale("gr")
                                 .auth("user", "pwd")
                                 .user_agent("user agent");
        assert_eq!(builder.locale, "gr");
        assert_eq!(builder.auth,
                   Some(Auth("user".to_string(), "pwd".to_string())));
        assert_eq!(builder.user_agent, Some("user agent".to_string()));
    }

    // Get base URL with default locale
    #[test]
    fn test_client_base_url_default() {
        let off = Off::new().build().unwrap();
        assert_eq!(off.base_url(None).unwrap().as_str(),
                   "https://world.openfoodfacts.org/");
    }

    // Get base URL with given locale
    #[test]
    fn test_client_base_url_locale() {
        let off = Off::new().build().unwrap();
        assert_eq!(off.base_url(Some("gr")).unwrap().as_str(),
                   "https://gr.openfoodfacts.org/");
    }

    // Get API URL
    #[test]
    fn test_client_api_url() {
        let off = Off::new().build().unwrap();
        assert_eq!(off.api_url(None).unwrap().as_str(),
                   "https://world.openfoodfacts.org/api/v0/");
    }

    // Get search URL
    #[test]
    fn test_client_search_url() {
        let off = Off::new().build().unwrap();
        assert_eq!(off.search_url(Some("gr")).unwrap().as_str(),
                   "https://gr.openfoodfacts.org/cgi/search.pl");
    }

    #[test]
    fn test_client_taxonomy() {
        let off = Off::new().build().unwrap();
        let response = off.taxonomy("nova_groups").unwrap();
        assert_eq!(response.url().as_str(),
                   "https://world.openfoodfacts.org/data/taxonomies/nova_groups.json");
        assert_eq!(response.status().is_success(), true);
    }

    #[test]
    fn test_client_taxonomy_not_found() {
        let off = Off::new().build().unwrap();
        let response = off.taxonomy("not_found").unwrap();
        assert_eq!(response.url().as_str(),
                   "https://world.openfoodfacts.org/data/taxonomies/not_found.json");
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_client_facet() {
        let off = Off::new().build().unwrap();
        let response = off.facet("brands", Some("gr")).unwrap();
        assert_eq!(response.url().as_str(), "https://gr.openfoodfacts.org/brands.json");
        assert_eq!(response.status().is_success(), true);
    }

    #[test]
    fn test_client_categories() {
        let off = Off::new().build().unwrap();
        let response = off.categories().unwrap();
        assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/categories.json");
        assert_eq!(response.status().is_success(), true);
    }

    #[test]
    fn test_client_products_by_category() {
        let off = Off::new().build().unwrap();
        let response = off.products_by_category("cheeses", None).unwrap();
        assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/category/cheeses.json");
        assert_eq!(response.status().is_success(), true);
    }

    #[test]
    fn test_client_products_with_additive() {
        let off = Off::new().build().unwrap();
        let response = off.products_with_additive("e322-lecithins", None).unwrap();
        assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/additive/e322-lecithins.json");
        assert_eq!(response.status().is_success(), true);
    }

    #[test]
    fn test_client_products_in_state() {
        let off = Off::new().build().unwrap();
        let response = off.products_in_state("empty", None).unwrap();
        assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/state/empty.json");
        assert_eq!(response.status().is_success(), true);
    }

    #[test]
    fn test_client_product_by_barcode() {
        let off = Off::new().build().unwrap();
        let response = off.product_by_barcode("069000019832", None).unwrap();  // Diet Pepsi
        assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/api/v0/product/069000019832");
        assert_eq!(response.status().is_success(), true);
    }

    // Use/keep as example.
    //
    // use std::collections::HashMap;
    // use serde_json::Value;
    //
    // #[test]
    // fn test_off_json() {
    //   let off = client().unwrap();
    //   let response = off.category("cheeses", Some("gr")).unwrap();
    //   println!("JSON: {:?}", response.json::<HashMap::<String, Value>>().unwrap());
    // }
}
