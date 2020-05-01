use std::env::consts::OS;
use reqwest;
use reqwest::header;


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
      let mut cb = reqwest::Client::builder();
      if !headers.is_empty() {
        cb = cb.default_headers(headers);
      }
      Ok(Off {
          locale: self.options.locale,
          client: cb.build()?
      })
    }

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


pub fn client() -> Result<Off, reqwest::Error> {
    Builder::new().build()
}


pub fn builder() -> Builder {
    Builder::new()
}


// Off client -----------------------------------------------------------------

pub enum Taxonomy {
    Additives,
    AdditiveClasses,    // TODO: alias for NovaGroups ?
    Allergens,
    Brands,
    Categories,         // TODO: Mentioned in the API docs but no example given
    Countries,
    Ingredients,
    IngredientsAnalysis,
    Languages,
    NovaGroups,
    NutrientLevels,
    ProductStates,
}

pub enum Facet {
    Additives,
    Allergens,
    Brands,
    Countries,
    Ingredients,
    IngredientsAnalysis,
    Languages,
    ProductStates,
}

pub struct Off {
    locale: String,           // The default locale
    client: reqwest::Client
}

// All functions will return a Result object
// page and locale should be optional.
impl Off {
    // Get a taxonomy.
    pub fn taxonomy(&self, taxonomy: Taxonomy) {

    }

    // Get a facet.
    pub fn facet(&self, facet: Facet, locale: Option<&str>) {

    }

    // Get categories ??
    pub fn categories(&self) {}

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

    // TODO: Use a macro instead ?
    fn base_url(&self, locale: Option<&str>) -> String {
        format!("https://{}.openfoodfacts.org", locale.unwrap_or(&self.locale))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // Get a Builder with default options.
    #[test]
    fn builder_default_options() {
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
    fn builder_with_options() {
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
    fn off_base_url_default() {
        let off = client().unwrap();
        assert_eq!(off.base_url(None), "https://world.openfoodfacts.org");
    }

    // Get base URL with given locale
    #[test]
    fn off_base_url_locale() {
        let off = client().unwrap();
        assert_eq!(off.base_url(Some("gr")), "https://gr.openfoodfacts.org");
    }
}
