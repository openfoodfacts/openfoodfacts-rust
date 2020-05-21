// TODO: Reqwest is curretly configured in blokcing mode. To support both blocking and
// non-blocking modes one needs to use conditional complilation ?
//
use std::env::consts::OS;
use std::error::Error;
use reqwest::blocking::{Client, Response};
use reqwest::header;
use url::{Url, ParseError};

// Builder --------------------------------------------------------------------

struct Options {
  locale: String,
  auth: Option<Auth>,
  user_agent: Option<String>
}

// (username, password)
#[derive(Debug, PartialEq)]
struct Auth(String, String);

pub struct Builder {
  options: Options
}

impl Builder {
    pub fn locale(mut self, value: &str) -> Self {
        self.options.locale = value.to_string();
        self
    }

    pub fn auth(mut self, username: &str, password: &str) -> Self {
        self.options.auth = Some(Auth(username.to_string(), password.to_string()));
        self
    }

    // TODO: Give full usr agent string or allow parameters:
    // appname, platform, version, url
    pub fn user_agent(mut self, value: &str) -> Self {
        self.options.user_agent = Some(value.to_string());
        self
    }

    // Create a new Off client with the current Builder options.
    // After build() is called, the Builder object is invalid.
    pub fn build(self) -> Result<Off, reqwest::Error> {
      let mut headers = header::HeaderMap::new();
      if let Some(user_agent) = self.options.user_agent {
        headers.insert(header::USER_AGENT,
                       header::HeaderValue::from_str(&user_agent).unwrap());
      }
      // TODO: Set Authentication header if Auth given.
      // if let Some(auth) = self.options.auth {
      //     headers.insert(reqwest::header::AUTHORIZATION,
      //                  reqwest::header::HeaderValue::from_str("Basic name:pwd"));
      // }
      let mut cb = Client::builder();
      if !headers.is_empty() {
        cb = cb.default_headers(headers);
      }
      Ok(Off {
          locale: self.options.locale,
          client: cb.build()?
      })
    }

    // Create a new Builder
    fn new() -> Self {
        Self {
            options: Options {
                locale: "world".to_string(),
                auth: None,
                // TODO: Get version and URL from somewhere else ?
                user_agent: Some(format!(
                    "OffRustClient - {} - Version {} - {}",
                    OS, "alpha", "https://github.com/openfoodfacts/openfoodfacts-rust"
                ))
            }
        }
    }
}

// Return a new Off builder with the default options.
pub fn client() -> Result<Off, reqwest::Error> {
    Builder::new().build()
}


// Return a Builder inititalzed with defaults.
pub fn builder() -> Builder {
    Builder::new()
}


// Off client -----------------------------------------------------------------


// TODO: Is there a way to get the str out of taxonomy::Taxonomy without
// having to use .0 ?

pub mod taxonomy {
  pub struct Taxonomy(pub &'static str);

  macro_rules! taxonomy {
    ($c:ident, $n:expr) => {pub const $c: Taxonomy = Taxonomy($n);}
  }

  taxonomy!(ADDITIVES, "additives");
  taxonomy!(ADDITIVE_CLASSES, "additives_classes");
  taxonomy!(ALLERGENS, "allergens");
  taxonomy!(BRANDS, "brands");
  taxonomy!(COUNTRIES, "countries");
  taxonomy!(INGREDIENTS, "ingredients");
  taxonomy!(INGREDIENT_ANALYSIS, "ingredients-analysis");  // Note the '-'
  taxonomy!(LANGUAGES, "languages");
  taxonomy!(NOVA_GROUPS, "nova_groups");
  taxonomy!(NUTRIENT_LEVELS, "nutrient_levels");
  taxonomy!(PRODUCT_STATES, "states");
}


pub mod facet {
  pub struct Facet(pub &'static str);

  macro_rules! facet {
    ($c:ident, $n:expr) => {pub const $c: Facet = Facet($n);}
  }

  facet!(ADDITIVES, "additives");
  facet!(ALLERGENS, "allergens");
  facet!(BRANDS, "brands");
  facet!(COUNTRIES, "countries");
  facet!(INGREDIENTS, "ingredients");
  facet!(INGREDIENT_ANALYSIS, "ingredients-analysis");  // Note the '-'
  facet!(LANGUAGES, "languages");
  facet!(PRODUCT_STATES, "states");
  facet!(LABELS, "labels");
}


pub struct Off {
    locale: String,           // The default locale
    client: Client
}


type OffResult = Result<Response, Box<dyn Error>>;


// All functions will return a Result object
// page and locale should be optional.
impl Off {
    // Get a taxonomy.
    pub fn taxonomy(&self, taxonomy: &taxonomy::Taxonomy) -> OffResult {
      let base = self.base_url(Some("world"))?;
      let url = base.join(&format!("data/taxonomies/{}.json", taxonomy.0))?;
      let response = self.client.get(url).send()?;
      Ok(response)
    }

    // Get a facet.
    pub fn facet(&self, facet: &facet::Facet, locale: Option<&str>) -> OffResult {
      let base = self.base_url(locale)?;
      let url = base.join(&format!("{}.json", facet.0))?;
      let response = self.client.get(url).send()?;
      Ok(response)
    }

    // Get categories.
    pub fn categories(&self, locale: Option<&str>) -> OffResult {
      let base = self.base_url(locale)?;
      let url = base.join("categories.json")?;
      let response = self.client.get(url).send()?;
      Ok(response)
    }

    // Get category
    pub fn category(&self, category: &str, locale: Option<&str>) -> OffResult {
      let base = self.base_url(locale)?;
      let url = base.join(&format!("category/{}.json", category))?;
      let response = self.client.get(url).send()?;
      Ok(response)
    }

    // Get product by barcode.
    pub fn product(&self, barcode: &str, page: Option<u32>, locale: Option<&str>) {
    }

    // Search products.
    pub fn search(&self) {
    }

    // Other
    // Get products by additive
    // Get products by category
    // Get product by product state

    fn base_url(&self, locale: Option<&str>) -> Result<Url, ParseError> {
        let url = format!("https://{}.openfoodfacts.org/", locale.unwrap_or(&self.locale));
        Url::parse(&url)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Get a Builder with default options.
    #[test]
    fn test_builder_default_options() {
        let builder = builder();
        assert_eq!(builder.options.locale, "world");
        assert_eq!(builder.options.auth, None);
        assert_eq!(builder.options.user_agent, Some(format!(
            "OffRustClient - {} - Version {} - {}",
            OS, "alpha", "https://github.com/openfoodfacts/openfoodfacts-rust"
        )));
    }

    // Set Builder options.
    #[test]
    fn test_builder_with_options() {
        let builder = builder().locale("gr")
                           .auth("user", "pwd")
                           .user_agent("user agent");
        assert_eq!(builder.options.locale, "gr");
        assert_eq!(builder.options.auth,
                   Some(Auth("user".to_string(), "pwd".to_string())));
        assert_eq!(builder.options.user_agent, Some("user agent".to_string()));
    }

    // Get base URL with default locale
    #[test]
    fn test_off_base_url_default() {
        let off = client().unwrap();
        assert_eq!(off.base_url(None).unwrap().as_str(), "https://world.openfoodfacts.org/");
    }

    // Get base URL with given locale
    #[test]
    fn test_off_base_url_locale() {
        let off = client().unwrap();
        assert_eq!(off.base_url(Some("gr")).unwrap().as_str(), "https://gr.openfoodfacts.org/");
    }

    #[test]
    fn test_taxonomy_const() {
      assert_eq!(taxonomy::ALLERGENS.0, "allergens");
    }

    #[test]
    fn test_facet_const() {
      assert_eq!(facet::LABELS.0, "labels");
    }

    #[test]
    fn test_get_taxonomy() {
      let off = client().unwrap();
      let response = off.taxonomy(&taxonomy::NOVA_GROUPS).unwrap();
      assert_eq!(response.url().as_str(),
                 "https://world.openfoodfacts.org/data/taxonomies/nova_groups.json");
      assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_get_facet_default() {
    let off = client().unwrap();
    let response = off.facet(&facet::BRANDS, None).unwrap();  // None : defaut locale (world)
    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/brands.json");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_get_facet_locale() {
    let off = client().unwrap();
    let response = off.facet(&facet::BRANDS, Some("gr")).unwrap();
    assert_eq!(response.url().as_str(), "https://gr.openfoodfacts.org/brands.json");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_get_categories_default() {
    let off = client().unwrap();
    let response = off.categories(None).unwrap();   // None : defaut locale (world)
    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/categories.json");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_get_categories_locale() {
    let off = client().unwrap();
    let response = off.categories(Some("gr")).unwrap();
    assert_eq!(response.url().as_str(), "https://gr.openfoodfacts.org/categories.json");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_get_category_default() {
    let off = client().unwrap();
    let response = off.category("cheeses", None).unwrap();   // None : defaut locale (world)
    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/category/cheeses.json");
    assert_eq!(response.status().is_success(), true);
  }

  // Fails: Probably the category name is returned in the locale language ??
  #[test]
  fn test_get_category_locale() {
    let off = client().unwrap();
    let response = off.category("cheeses", Some("gr")).unwrap();
    assert_eq!(response.url().as_str(), "https://gr.openfoodfacts.org/category/cheeses.json");
    assert_eq!(response.status().is_success(), true);
  }
}
