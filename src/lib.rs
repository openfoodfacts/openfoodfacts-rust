use std::env::consts::OS;
use std::error::Error;
use reqwest::blocking::{Client, Response};
use reqwest::header;
use url::{Url, ParseError};

mod search;

use crate::search::Query;

const VERSION: &str = "alpha";


// Builder --------------------------------------------------------------------

// (username, password)
#[derive(Debug, PartialEq)]
struct Auth(String, String);

/// The Open Food Facts API client builder.
///
/// # Examples
///
/// ```ignore
/// let off = Off:new("v0").locale("fr").build()?;
/// ```
pub struct Off {
    /// The API version to use. One of "v0" or "v2".
    version: String,
    /// The default locale. A country code, a "{country code}-{language code}"
    /// pair or "world".
    ///
    /// * Country codes must be lowercase ISO 3166-1 codes.
    /// * Language codes must be lowercase ISO 639-1 codes.
    locale: String,
    /// The authentication credentials. Optional. Only needed for write operations.
    auth: Option<Auth>,
    /// The User-Agent header value to send on each Off request. Optional.
    /// If not given, use the default user agent.
    user_agent: Option<String>
}

impl Off {
    /// Create a new builder with defaults:
    ///
    /// * The default locale is set to "world".
    /// * No authentication credentials
    /// * The user agent is set to
    ///   `OffRustClient - {OS name} - Version {lib version} - {github repo URL}`
    ///
    /// # Arguments
    ///
    /// * version - The API version to use. One of "v0", "v2".
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            locale: "world".to_string(),
            auth: None,
            // TODO: Get version and URL from somewhere else ?
            user_agent: Some(format!(
                "OffRustClient - {} - Version {} - {}",
                OS, VERSION, "https://github.com/openfoodfacts/openfoodfacts-rust"
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

    /// Set the user agent string.
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
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
            version: self.version,
            locale: self.locale,
            client: cb.build()?
        })
    }
}


// Client ---------------------------------------------------------------------


// Output formats
pub enum Format {
    Json,
    Xml
}

// Sorting criteria
pub enum Sorting {
    Popularity,
    ProductName,
    CreatedDate,
    LastModifiedDate
}

// TODO: Collect all output parameters in a separate struct ?
//  locale could be added to the output set.
// output = Output::new() -> defaults: format=Format::Json, sort_by=Sorting::Popularity
// outout = Output{format: , sort_by} ?
// output.format(Format)
// output.sort_by(Sorting),

// format: Default is JSON -> method call parameter
// pub fn format(& mut self, format: Format) -> & mut Self {
//     self.params.insert(String::from(match format {
//         Json => "json",
//         Xml => "xml"
//     }), Value::Bool(true));
//     self
// }

// sorting
// pub fn sort_by(& mut self, sorting: Sorting) -> & mut Self {
//     self.params.insert(String::from("sort_by"), Value::String(match sorting {
//         Popularity => String::from("unique_scans_n"),
//         ProductName => String::from("product_name"),
//         CreatedDate => String::from("created_t"),
//         LastModifiedDate => String::from("last_modified_t")
//     }));
//     self
// }

// TODO: page and page_size
// pagination = Pagination::new()
// pagination = Pagination{page, page_size} ?
// pagination.page(N)
// pagination.next_page()
// pagination.prev_page()
// pagination.page_size()


// TODO: fields
// TODO: nocache

/// The OFF API client, created using the Off() builder.
///
/// All methods return a OffResult object.
///
/// The OffClient owns a reqwest::Client object. One single OffClient should
/// be used per application.
pub struct OffClient {
    /// The API version.
    version: String,
    /// The default locale to use when no locale is given in a method call.
    locale: String,
    /// The uderlying reqwest client. TODO: Make a ref.
    client: Client
}

/// The return type of all OffClient methods.
type OffResult = Result<Response, Box<dyn Error>>;


// page and locale should be optional.
// JSON response data can be deserialized into untyped JSON values
// with response.json::<HashMap::<String, serde_json::Value>>()
//
// If a pair <cc>-<lc> is given, the name of the /category/ segment
// will be localized. VERFIY THIS. If one gives a language code, it
// should be possible to pass the localized segment name in an optional
// parameter?


impl OffClient {
    // ------------------------------------------------------------------------
    // Metadata
    // ------------------------------------------------------------------------

    /// Get the given taxonomy. Taxonomies are static JSON files.
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
    /// * `taxonomy` - The taxonomy name. One of the following:
    ///     - additives
    ///     - allergens
    ///     - additives_classes (*)
    ///     - brands
    ///     - countries
    ///     - ingredients
    ///     - ingredients_analysis (*)
    ///     - languages
    ///     - nova_groups (*)
    ///     - nutrient_levels (*)
    ///     - states
    /// (*) Only taxomomy. There is no facet equivalent.
    pub fn taxonomy(&self, taxonomy: &str) -> OffResult {
        let base_url = self.base_url(Some("world"))?;   // force world locale.
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
    /// * `facet` - Thefacet name. One of the following:
    ///     - additives
    ///     - allergens
    ///     - brands
    ///     - countries
    ///     - ingredients
    ///     - languages
    ///     - states
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
        let base_url = self.base_url(Some("world"))?;   // force world locale.
        let url = base_url.join("categories.json")?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

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
    /// TODO: Add pagination.
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
    /// TODO: Add pagination.
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
    /// TODO: Add pagination.
    pub fn products_in_state(&self, state: &str, locale: Option<&str>) -> OffResult {
        let base_url = self.base_url(locale)?;
        let url = base_url.join(&format!("state/{}.json", state))?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    // ------------------------------------------------------------------------
    // Read
    // ------------------------------------------------------------------------

    /// Get the nutrition facts of the given product.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/api/v0/product/{barcode}
    /// ```
    ///
    /// Not clear how this differs from the get products by barcodes (products()) call
    /// below.
    ///
    /// # Arguments
    ///
    /// * `barcode` - The product barcode.
    /// * `locale`- Optional locale. Should contain only a country code TODO: VERIFY THIS.
    ///             If missing, uses the default client locale.
    /// TODO: Support fields.
    pub fn product_by_barcode(&self, barcode: &str, locale: Option<&str>) -> OffResult {
        let api_url = self.api_url(locale)?;
        let url = api_url.join(&format!("product/{}", barcode))?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    /// Get the nutrients of the given product.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/cgi/nutients.pl
    /// ```
    ///
    /// # Arguments
    ///
    /// * `barcode` - The product barcode.
    /// * `id` - TBC: using `ingredients_fr` as in API docs.
    ///     TODO: This should probably be `ingredients_{language}.
    ///     {language} can be extracted from {locale}; if not given,
    ///     `en` should be used.
    /// * `process_image` -  TBC: using `1` as in API docs.
    /// * `ocr_engine` - TBC: using `google_cloud_vision ` as in API docs.
    /// * `locale`- Optional locale. Should contain only a country code TODO: VERIFY THIS.
    ///             If missing, uses the default client locale.
    pub fn product_nutrients(&self, barcode: &str, locale: Option<&str>) -> OffResult {
        let api_url = self.base_url(locale)?;
        let url = api_url.join("cgi/nutrients.pl")?;
        let response = self.client.get(url).query(&[
                ("code", barcode),
                ("id", "ingredients_fr"),
                ("process_image", "1"),
                ("ocr_engine", "google_cloud_vision")
            ]).send()?;
        Ok(response)
    }

    // ------------------------------------------------------------------------
    // Write
    // ------------------------------------------------------------------------

    // TODO

    // ------------------------------------------------------------------------
    // Search
    // ------------------------------------------------------------------------

    /// Search products by barcode.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/api/v0/search
    /// ```
    ///
    /// See also `product_by_barcode()` above.
    ///
    /// # Arguments
    ///
    /// * `barcodes` - A string with comma-separated barcodes.
    /// * `fields` - Some(str) with a string with comma-separated fields or `*``
    ///              or None. Both `*` and None return all fields.
    ///
    pub fn search_by_barcode(&self, barcodes: &str, fields: Option<&str>, locale: Option<&str>) -> OffResult {
        let api_url = self.api_url(locale)?;
        let url = api_url.join("search")?;
        let response = self.client.get(url).query(&[
            ("code", barcodes),
            ("fields", match fields {
                Some("*") | None => "",
                _ => fields.unwrap()
            })
        ]).send()?;
        Ok(response)
    }

    /// Search using filters.
    pub fn search(&self, query: Query) {
        // TODO
    }

    // TODO: Serialization
    // Option 1
    //  qparams = SearchParams::to_array() -> &[] returns an array of tuples.
    //  The default serde_urlencoded::to_string() does the actual serialization
    //  as expected by self.client.get(search_url).query(qparams).send()?;
    //
    // Option 2
    //  SearchParams implement Serialize, which builds the array and returns
    //  serde_urlencoded::to_string().
    // pub fn search(&self, params: &SearchParams, output: &OutputParams, page: &Pagination, locale: Option<&str>) -> OffResult {
    //   let search_url = self.search_url(locale)?;
    //   let response = self.client.get(search_url).query(params).send()?;
    //   Ok(response)
    // }

    /// Return the base URL with the given locale.
    fn base_url(&self, locale: Option<&str>) -> Result<Url, ParseError> {
        let url = format!("https://{}.openfoodfacts.org/", locale.unwrap_or(&self.locale));
        Url::parse(&url)
    }

    /// Return the API URL with the given locale.
    fn api_url(&self, locale: Option<&str>) -> Result<Url, ParseError> {
        let base = self.base_url(locale)?;
        base.join(&format!("api/{}/", self.version))
    }

    /// Return the search URL with the given locale.
    fn search_url(&self, locale: Option<&str>) -> Result<Url, ParseError> {
        let base = self.base_url(locale)?;
        base.join("cgi/search.pl")
    }

    /// Return the country code part of the locale or the empty string.
    fn country_code(locale: &str) -> &str {
        Self::_split_locale(locale, 0)
    }

    /// Return the language code part of the locale or the empty string.
    fn language_code(locale: &str) -> &str {
        Self::_split_locale(locale, 1)
    }

    #[inline]
    fn _split_locale(locale: &str, n: usize) -> &str {
        if let Some(v) =  locale.split("-").nth(n) {
            v
        } else {
            ""
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Get a Builder with default options.
    #[test]
    fn builder_default_options() {
        let builder = Off::new("v0");
        assert_eq!(builder.version, "v0");
        assert_eq!(builder.locale, "world");
        assert_eq!(builder.auth, None);
        assert_eq!(builder.user_agent, Some(format!(
            "OffRustClient - {} - Version {} - {}",
            OS, "alpha", "https://github.com/openfoodfacts/openfoodfacts-rust"
        )));
    }

    // Set Builder options.
    #[test]
    fn builder_with_options() {
        let builder = Off::new("v0").locale("gr")
                                 .auth("user", "pwd")
                                 .user_agent("user agent");
        assert_eq!(builder.locale, "gr");
        assert_eq!(builder.auth,
                   Some(Auth("user".to_string(), "pwd".to_string())));
        assert_eq!(builder.user_agent, Some("user agent".to_string()));
    }

    // Get base URL with default locale
    #[test]
    fn client_base_url_default() {
        let off = Off::new("v0").build().unwrap();
        assert_eq!(off.base_url(None).unwrap().as_str(),
                   "https://world.openfoodfacts.org/");
    }

    // Get base URL with given locale
    #[test]
    fn client_base_url_locale() {
        let off = Off::new("v0").build().unwrap();
        assert_eq!(off.base_url(Some("gr")).unwrap().as_str(),
                   "https://gr.openfoodfacts.org/");
    }

    // Get API URL
    #[test]
    fn client_api_url() {
        let off_v0 = Off::new("v0").build().unwrap();
        assert_eq!(off_v0.api_url(None).unwrap().as_str(),
                   "https://world.openfoodfacts.org/api/v0/");

        let off_v2 = Off::new("v2").build().unwrap();
        assert_eq!(off_v2.api_url(None).unwrap().as_str(),
                  "https://world.openfoodfacts.org/api/v2/");
        }

    // Get search URL
    #[test]
    fn client_search_url() {
        let off = Off::new("v0").build().unwrap();
        assert_eq!(off.search_url(Some("gr")).unwrap().as_str(),
                   "https://gr.openfoodfacts.org/cgi/search.pl");
    }

    // Extract country code
    #[test]
    fn client_country_code() {
        assert_eq!(OffClient::country_code(""), "");
        assert_eq!(OffClient::country_code("-fr"), "");
        assert_eq!(OffClient::country_code("fr"), "fr");
    }

    // Extract language code
    #[test]
    fn client_language_code() {
        assert_eq!(OffClient::language_code(""), "");
        assert_eq!(OffClient::language_code("fr"), "");
        assert_eq!(OffClient::language_code("fr-"), "");
        assert_eq!(OffClient::language_code("-fr"), "fr");
        assert_eq!(OffClient::language_code("fr-ca"), "ca");
    }
}
