use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use reqwest::blocking::{Client, Response};
use url::{Url, ParseError};

use crate::output::{Locale, Output};
use crate::search::Query;

/// Supported API versions.
///
/// ApiVersion::to_string() produces the API version string "v{version number}".
/// ApiVersion::from(string) produces the corresponding ApiVersion enum value
/// from a string "v{version number}". Returns `fmt::Error` if the version number
/// is invalid.
#[derive(Debug, PartialEq)]
pub enum ApiVersion {
    V0,
    V2
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let version = match self {
            ApiVersion::V0 => "v0",
            ApiVersion::V2 => "v2"
        };
        write!(f, "{}", version)
    }
}

impl FromStr for ApiVersion {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_assert!(s.len() > 0);
        match s {
            "v0" => { Ok(ApiVersion::V0) },
            "v2" => { Ok(ApiVersion::V2) },
            _    => { Err(fmt::Error) }
        }
    }
}

/// The OFF API client, created using the Off builder.
///
/// All methods return a OffResult object.
///
/// The OffClient owns a reqwest::Client object. One single OffClient should
/// be used per application.
#[derive(Debug)]
pub struct OffClient {
    // The API version.
    pub(crate) version: ApiVersion,
    // The default locale to use when no locale is given in a method call.
    pub(crate) locale: Locale,
    // The uderlying reqwest client. TODO: Make a ref ?
    pub(crate) client: Client
}

/// The return type of all OffClient methods.
pub type OffResult = Result<Response, Box<dyn Error>>;

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
    /// Taxomonies support only the locale "world".
    ///
    /// # Arguments
    ///
    /// * taxonomy - The taxonomy name. One of the following:
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
        let base_url = self.base_url(Some(Locale::default()))?;   // force world locale.
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
    /// # Arguments
    ///
    /// * facet - Thefacet name. One of the following:
    ///     - additives
    ///     - allergens
    ///     - brands
    ///     - countries
    ///     - ingredients
    ///     - languages
    ///     - states
    /// * output - Optional output parameters. This call only supports the locale
    ///     parameter.
    pub fn facet(&self, facet: &str, output: Option<Output>) -> OffResult {
        let base_url = self.base_url(output.and_then(|o| o.locale))?;
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
    /// Categories support only the locale "world".
    pub fn categories(&self) -> OffResult {
        let base_url = self.base_url(Some(Locale::default()))?;   // force world locale.
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
    /// * output - Optional output parameters. This call only supports the locale
    ///     and paginazion parameters.
    pub fn products_by_category(&self, category: &str, output: Option<Output>) -> OffResult {
        let base_url = self.base_url(output.and_then(|o| o.locale))?;
        let url = base_url.join(&format!("category/{}.json", category))?;
        // TODO: add query params
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
    /// * output - Optional output parameters. This call only supports the locale
    ///     and paginazion parameters.
    pub fn products_with_additive(&self, additive: &str, output: Option<Output>) -> OffResult {
        let base_url = self.base_url(output.and_then(|o| o.locale))?;
        let url = base_url.join(&format!("additive/{}.json", additive))?;
        // TODO: Add query params
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
    /// * output - Optional output parameters. This call only supports the locale
    ///     and paginazion parameters.
    pub fn products_in_state(&self, state: &str, output: Option<Output>) -> OffResult {
        let base_url = self.base_url(output.and_then(|o| o.locale))?;
        let url = base_url.join(&format!("state/{}.json", state))?;
        // TODO: Add query params
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
    /// * output - Optional output parameters. This call only supports the locale
    ///     and fields parameters.
    pub fn product_by_barcode(&self, barcode: &str, output: Option<Output>) -> OffResult {
        let api_url = self.api_url(output.and_then(|o| o.locale))?;
        let url = api_url.join(&format!("product/{}", barcode))?;
        // TODO: Add query params
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
    /// * output - Optional output parameters. This call only supports the locale
    ///     and pagination parameters. TODO: Verify
    pub fn product_nutrients(&self, barcode: &str, output: Option<Output>) -> OffResult {
        let api_url = self.base_url(output.and_then(|o| o.locale))?;
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
    /// * output - Optional output parameters. This call only supports the locale
    ///     and fields parameters. TODO: Also pagination ?
    ///
    pub fn search_by_barcode(&self, barcodes: &str, fields: Option<&str>, output: Option<Output>) -> OffResult {
        let api_url = self.api_url(output.and_then(|o| o.locale))?;
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
    fn base_url(&self, locale: Option<Locale>) -> Result<Url, ParseError> {
        let url = format!("https://{}.openfoodfacts.org/",
                          locale.map_or(self.locale.to_string(), |l| l.to_string()));
        Url::parse(&url)
    }

    /// Return the API URL with the given locale.
    fn api_url(&self, locale: Option<Locale>) -> Result<Url, ParseError> {
        let base = self.base_url(locale)?;
        base.join(&format!("api/{}/", self.version))
    }

    /// Return the search URL with the given locale.
    fn search_url(&self, locale: Option<Locale>) -> Result<Url, ParseError> {
        let base = self.base_url(locale)?;
        base.join("cgi/search.pl")
    }
}

#[cfg(test)]
mod tests_api_version {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(ApiVersion::V0.to_string(), String::from("v0"));
        assert_eq!(ApiVersion::V2.to_string(), String::from("v2"));
    }

    #[test]
    fn from_str() {
        assert_eq!(ApiVersion::from_str("v0").unwrap(), ApiVersion::V0);
        assert_eq!(ApiVersion::from_str("v2").unwrap(), ApiVersion::V2);
        assert_eq!(ApiVersion::from_str("v666").unwrap_err(), fmt::Error);
    }
}

#[cfg(test)]
mod tests_client {
    use crate::Off;
    use super::*;

    #[test]
    fn base_url_default() {
        let off = Off::new(ApiVersion::V0).build().unwrap();
        assert_eq!(off.base_url(None).unwrap().as_str(),
                    "https://world.openfoodfacts.org/");
    }

    #[test]
    fn base_url_locale() {
        let off = Off::new(ApiVersion::V0).build().unwrap();
        assert_eq!(off.base_url(Some(Locale::new().country("gr"))).unwrap().as_str(),
                    "https://gr.openfoodfacts.org/");
    }

    #[test]
    fn api_url() {
        let off_v0 = Off::new(ApiVersion::V0).build().unwrap();
        assert_eq!(off_v0.api_url(None).unwrap().as_str(),
                    "https://world.openfoodfacts.org/api/v0/");

        let off_v2 = Off::new(ApiVersion::V2).build().unwrap();
        assert_eq!(off_v2.api_url(None).unwrap().as_str(),
                    "https://world.openfoodfacts.org/api/v2/");
        }

    #[test]
    fn client_search_url() {
        let off = Off::new(ApiVersion::V0).build().unwrap();
        assert_eq!(off.search_url(Some(Locale::new().country("gr"))).unwrap().as_str(),
                    "https://gr.openfoodfacts.org/cgi/search.pl");
    }
}
