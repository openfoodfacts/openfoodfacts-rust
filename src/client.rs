use reqwest::blocking::{Client as HttpClient, Response as HttpResponse};
use url::{ParseError, Url};

use crate::output::{Locale, Output};
use crate::search::SearchParams;
use crate::types::{ApiVersion, Params};

/// The return type of all OffClient methods.
pub type OffResult = Result<HttpResponse, Box<dyn std::error::Error>>;

// API version methods.
trait VersionInfo {
    fn version(&self) -> ApiVersion {
        panic!("Not implemented");
    }

    fn api_url(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        panic!("Not implemented");
    }

    fn search_url(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        panic!("Not implemented");
    }
}

// The OFF API client data.
//
// All methods return a OffResult object.
//
// The OffClient owns a reqwest::Client object. One single OffClient should
// be used per application.
#[derive(Debug)]
struct OffClient {
    // The default locale to use when no locale is given in a method call.
    pub(crate) locale: Locale,
    // The uderlying reqwest client. TODO: Make a ref ?
    pub(crate) client: HttpClient,
}

impl OffClient {
    // Notes:
    //
    // * The 'cc' and 'lc' query parmeters are not supported. The country and
    //   language are always selected via the subdomain.
    // * Only JSON calls are supported.

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
    fn taxonomy(&self, taxonomy: &str) -> OffResult {
        let base_url = self.base_url_world()?; // force world locale.
        let url = base_url.join(&format!("data/taxonomies/{}.json", taxonomy))?;
        self.get(url, None)
    }

    /// Get the given facet.
    ///
    /// # OFF API request
    ///
    /// `GET https://{locale}.openfoodfacts.org/{facet}.json`
    ///
    /// # Arguments
    ///
    /// * facet - The facet type name. One of the following:
    ///     - additives
    ///     - allergens
    ///     - brands
    ///     - countries
    ///     - entry-dates
    ///     - ingredients
    ///     - labels
    ///     - languages
    ///     - packaging
    ///     - purchase-places
    ///     - states
    ///     - stores
    ///     - traces
    ///     The name may be given in english or localized, i.e. additives (world), additifs (fr).
    /// * output - Optional output parameters. This call supports only the locale,
    ///     pagination, fields and nocache parameters.
    fn facet(&self, facet: &str, output: Option<Output>) -> OffResult {
        // Borrow output and extract Option<&Locale>
        let base_url = self.base_url(output.as_ref().and_then(|o| o.locale.as_ref()))?;
        let url = base_url.join(&format!("{}.json", facet))?;
        let params = output.map(|o| o.params(&["page", "page_size", "fields", "nocache"]));
        self.get(url, params.as_ref())
    }

    /// Get all the categories.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://world.openfoodfacts.org/categories.json
    /// ```
    ///
    /// # Arguments
    ///
    /// * output - Optional output parameters. This call supports only the locale parameter.
    fn categories(&self, output: Option<Output>) -> OffResult {
        let base_url = self.base_url(output.as_ref().and_then(|o| o.locale.as_ref()))?;
        let url = base_url.join("categories.json")?;
        self.get(url, None)
    }

    /// Get the nutrients by country.
    ///
    /// # OFF API request
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/cgi/nutrients.pl
    /// ```
    ///
    /// # Arguments
    ///
    /// * output - Optional output parameter. This call supports only the locale
    ///   parameter.
    fn nutrients(&self, output: Option<Output>) -> OffResult {
        let cgi_url = self.cgi_url(output.as_ref().and_then(|o| o.locale.as_ref()))?;
        let url = cgi_url.join("nutrients.pl")?;
        self.get(url, None)
    }

    /// Get all products for the given facet or category.
    ///
    /// # OFF API request
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/{facet}/{value}.json
    /// ```
    ///
    /// # Arguments
    ///
    /// * what - A facet name or "category". The facet name is always the singular name
    ///     of the face type name (i.e. brands -> brand, entry-dates -> entry-date, etc).
    ///     The facet name or the "category" literal may be given either in english or
    ///     localized, i.e. additives (world), additifs (fr), category (world), categorie (fr).
    /// * id - The localized id of the facet or category. The IDs are returned by calls
    ///     to the corresponding `facet(<facet_type>)` or `categories()` endpoint. For example,
    ///     the IDs for the `entry-date` facet are returned by the call `facet("entry-dates")`.
    /// * output - Optional output parameters. This call supports the locale, pagination
    ///     and fields parameters.
    fn products_by(&self, what: &str, id: &str, output: Option<Output>) -> OffResult {
        let base_url = self.base_url(output.as_ref().and_then(|o| o.locale.as_ref()))?;
        let url = base_url.join(&format!("{}/{}.json", what, id))?;
        let params = output.map(|o| o.params(&["page", "page_size", "fields"]));
        self.get(url, params.as_ref())
    }

    // ------------------------------------------------------------------------
    // Read
    // ------------------------------------------------------------------------

    /// Get the nutrition facts of the given product.
    ///
    /// # OFF API request
    ///
    /// ```ignore
    /// GET https://{locale}.openfoodfacts.org/api/{version}/product/{barcode}
    /// ```
    ///
    /// # Arguments
    ///
    /// * barcode - The product barcode.
    /// * output - Optional output parameters. This call only supports the locale
    ///     and fields parameters.
    fn product(&self, barcode: &str, output: Option<Output>) -> OffResult {
        let api_url = self.api_url(output.as_ref().and_then(|o| o.locale.as_ref()))?;
        let url = api_url.join(&format!("product/{}", barcode))?;
        let params = output.map(|o| o.params(&["fields"]));
        self.get(url, params.as_ref())
    }

    // ------------------------------------------------------------------------
    // Search
    // ------------------------------------------------------------------------

    fn search(&self, search: impl SearchParams, output: Option<Output>) -> OffResult {
        let url = self.search_url(output.as_ref().and_then(|o| o.locale.as_ref()))?;
        let mut params = search.params();
        if let Some(output_params) = output.map(|o| o.params(&["fields"])) {
            params.extend(output_params);
        }
        self.get(url, Some(&params))
    }

    // ------------------------------------------------------------------------
    // Write
    // ------------------------------------------------------------------------

    // TODO

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

    // Return the base URL with the locale given in Output::locale. If Output is None
    // or Output::locale is None, use the client's default locale.
    fn base_url(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        self.base_url_locale(locale)
    }

    // Return the base URL with the "world" locale.
    fn base_url_world(&self) -> Result<Url, ParseError> {
        self.base_url_locale(Some(&Locale::default()))
    }

    // Return the CGI URL with the locale given in Output::locale.
    fn cgi_url(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        let base = self.base_url(locale)?;
        base.join("cgi/")
    }

    // Return the base URL with the given locale. If locale is None, return the
    // client's default locale.
    fn base_url_locale(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        let url = format!(
            "https://{}.openfoodfacts.org/",
            locale.map_or(self.locale.to_string(), |l| l.to_string())
        );
        Url::parse(&url)
    }

    // Build and send a GET request.
    fn get(&self, url: Url, params: Option<&Params>) -> OffResult {
        let mut rb = self.client.get(url);
        if let Some(p) = params {
            rb = rb.query(p);
        }
        let response = rb.send()?;
        Ok(response)
    }
}

impl VersionInfo for OffClient {}

pub struct OffClientV0 {
    client: OffClient,
}

impl OffClientV0 {
    pub(crate) fn new(locale: Locale, client: HttpClient) -> Self {
        Self {
            client: OffClient { locale, client },
        }
    }

    pub fn taxonomy(&self, taxonomy: &str) -> OffResult {
        self.client.taxonomy(taxonomy)
    }

    pub fn facet(&self, facet: &str, output: Option<Output>) -> OffResult {
        self.client.facet(facet, output)
    }

    pub fn categories(&self, output: Option<Output>) -> OffResult {
        self.client.categories(output)
    }

    pub fn nutrients(&self, output: Option<Output>) -> OffResult {
        self.client.nutrients(output)
    }

    pub fn products_by(&self, what: &str, id: &str, output: Option<Output>) -> OffResult {
        self.client.products_by(what, id, output)
    }

    pub fn product(&self, barcode: &str, output: Option<Output>) -> OffResult {
        self.client.product(barcode, output)
    }

    pub fn search(&self, search: impl SearchParams, output: Option<Output>) -> OffResult {
        self.client.search(search, output)
    }
}

impl VersionInfo for OffClientV0 {
    fn version(&self) -> ApiVersion {
        ApiVersion::V0
    }

    fn api_url(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        let base = self.client.base_url(locale)?;
        base.join(&format!("api/{}/", self.version()))
    }

    fn search_url(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        // Return the API URL with the locale given in Output::locale.
        let cgi_url = self.client.cgi_url(locale)?;
        cgi_url.join("search.pl")
    }
}

pub struct OffClientV2 {
    client: OffClient,
}

impl OffClientV2 {
    pub(crate) fn new(locale: Locale, client: HttpClient) -> Self {
        Self {
            client: OffClient { locale, client },
        }
    }

    pub fn search(&self, search: impl SearchParams, output: Option<Output>) -> OffResult {
        self.client.search(search, output)
    }

    // TODO: V2 only ?
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
    pub fn search_by_barcode(&self, barcodes: &str, output: Option<Output>) -> OffResult {
        // Borrow output and extract Option<&Locale>
        let url = self.search_url(output.as_ref().and_then(|o| o.locale.as_ref()))?;
        let mut params = Params::new();
        params.push(("code", String::from(barcodes)));
        if let Some(output_params) = output.map(|o| o.params(&["fields"])) {
            params.extend(output_params);
        }
        self.client.get(url, Some(&params))
    }
}

impl VersionInfo for OffClientV2 {
    fn version(&self) -> ApiVersion {
        ApiVersion::V2
    }

    fn api_url(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        let base = self.client.base_url(locale)?;
        base.join(&format!("api/{}/", self.version()))
    }

    fn search_url(&self, locale: Option<&Locale>) -> Result<Url, ParseError> {
        // Return the API URL with the locale given in Output::locale.
        let api_url = self.api_url(locale)?;
        api_url.join("search")
    }
}

#[cfg(test)]
mod tests_off_client {
    use super::*;
    use crate::OffBuilder;

    #[test]
    fn base_url_default() {
        let off = OffBuilder::new().build_v0().unwrap();
        assert_eq!(
            off.client.base_url(None).unwrap().as_str(),
            "https://world.openfoodfacts.org/"
        );
    }

    #[test]
    fn base_url_locale() {
        let off = OffBuilder::new().build_v0().unwrap();
        assert_eq!(
            off.client
                .base_url(Some(&Locale::new("gr", None)))
                .unwrap()
                .as_str(),
            "https://gr.openfoodfacts.org/"
        );
    }

    #[test]
    fn base_url_world() {
        let off = OffBuilder::new()
            .locale(Locale::new("gr", None))
            .build_v0()
            .unwrap();
        assert_eq!(
            off.client.base_url_world().unwrap().as_str(),
            "https://world.openfoodfacts.org/"
        );
    }

    #[test]
    fn client_cgi_url() {
        let off = OffBuilder::new().build_v0().unwrap();
        assert_eq!(
            off.client
                .cgi_url(Some(&Locale::new("gr", None)))
                .unwrap()
                .as_str(),
            "https://gr.openfoodfacts.org/cgi/"
        );
    }
}

#[cfg(test)]
mod tests_version_info {
    use super::*;
    use crate::OffBuilder;

    #[test]
    fn version() {
        let off_v0 = OffBuilder::new().build_v0().unwrap();
        assert_eq!(off_v0.version(), ApiVersion::V0);

        let off_v2 = OffBuilder::new().build_v2().unwrap();
        assert_eq!(off_v2.version(), ApiVersion::V2);
    }

    #[test]
    fn api_url() {
        let off_v0 = OffBuilder::new().build_v0().unwrap();

        assert_eq!(
            off_v0.api_url(None).unwrap().as_str(),
            "https://world.openfoodfacts.org/api/v0/"
        );

        let off_v2 = OffBuilder::new().build_v2().unwrap();
        assert_eq!(
            off_v2
                .api_url(Some(&Locale::new("gr", None)))
                .unwrap()
                .as_str(),
            "https://gr.openfoodfacts.org/api/v2/"
        );
    }

    #[test]
    fn search_url() {
        let off_v0 = OffBuilder::new().build_v0().unwrap();

        assert_eq!(
            off_v0.search_url(None).unwrap().as_str(),
            "https://world.openfoodfacts.org/cgi/search.pl"
        );

        let off_v2 = OffBuilder::new().build_v2().unwrap();
        assert_eq!(
            off_v2
                .search_url(Some(&Locale::new("gr", None)))
                .unwrap()
                .as_str(),
            "https://gr.openfoodfacts.org/api/v2/search"
        );
    }
}
