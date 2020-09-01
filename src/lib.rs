// TODO: Reqwest is curretly configured in blokcing mode. To support both blocking and
// non-blocking modes one needs to use conditional complilation ?
use std::iter::IntoIterator;
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
// TODO: Need derive ?
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
    if let Some(auth) = self.options.auth {
      let basic_auth = format!("Basic {}:{}", auth.0, auth.1);
      headers.insert(reqwest::header::AUTHORIZATION,
                     reqwest::header::HeaderValue::from_str(&basic_auth).unwrap());
    }
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


// Return a Builder inititalzed with defaults.
pub fn builder() -> Builder {
  Builder::new()
}


// Return a new Off client initalized with defaults.
pub fn client() -> Result<Off, reqwest::Error> {
  builder().build()
}


// Off client -----------------------------------------------------------------


// TODO: Is there a way to get the str out of taxonomy::Taxonomy without
// having to use .0 ?
// TODO: Support language in subdomain:
//  locale can be <country> (world, gr, fr) or <country>-<language> (gr-en)

// Taxonomy names
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


// Facet names
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

// Search criteria names
pub mod criteria {
  // https://wiki.openfoodfacts.org/API/Read/Search
  // search_terms: list of terms
  // format: Always JSON
  // page_size: usize
  // sort_by: String or Enum (Popularity, Product name, Add date, Edit date, Completness)
  // tags : A collection of tuples (name, op, value) ? -> tag_0 ... tag_N
  // nutrients: A collection of tuples (name, op, value) ? -> nutriment_0 ... nutriment_N
  // Zero or more pairs (name, value)
  // All names parameters search_terms .. nutrient0 can in fact be given using the pair
  // form.

  // A search consists of one or more search criteria, defined as a triplet (name, contains, value)
  // where are are strings. <name> can be any values of tag_type. contains is a boolean and value
  // is a string.
  //
  // Criteria are "serialized" into query parameters as:
  //
  //  tagtype_<n>=<name>&tag_contains_<n>=<contains>&tag_<n>=<value>
  //
  // where <n> is the index of the criteria in the criteria list (starts at 0).
  //
  // TODO: Could be changed to
  //
  // Criteria(CriteriaName, op, value)
  //
  //  - where CriteriaName and op are Enums
  //
  // OR
  // 
  // Criteria(CriteriaName).op(value)
  //
  pub struct Criteria(pub &'static str);

  macro_rules! criteria {
    ($c:ident, $n:expr) => {pub const $c: Criteria = Criteria($n);}
  }

  criteria!(BRANDS, "brands");
  criteria!(CATEGORIES, "categories");
  criteria!(PACKAGING, "packaging");
  criteria!(LABELS, "labels");
  criteria!(ORIGINS, "origins");
  criteria!(MANUFACTURING_PLACES, "manufacturing_places");
  criteria!(EMB_CODES, "emb_codes");
  criteria!(PURCHASE_PLACES, "purchase_places");
  criteria!(STORES, "stores");
  criteria!(COUNTRIES, "countries");
  criteria!(ADDITIVES, "additives");
  criteria!(ALLERGENS, "allergens");
  criteria!(TRACES, "traces");
  criteria!(NUTRITION_GRADES, "nutrition_grades");
  criteria!(STATES, "states");
}

// Search param types

#[derive(Debug)]
pub struct CriteriaParam {
  name: String,
  op: String,
  value: String
}

#[derive(Debug)]
pub struct IngredientParam {
  name: String,
  value: String
}

#[derive(Debug)]
pub struct NutrimentParam {
  name: String,
  op: String,
  value: String
}

#[derive(Debug)]
pub struct NameParam {
  name: String,
}

// Implementation using Enum

pub enum SearchParam {
  Criteria(CriteriaParam),
  Ingredient(IngredientParam),
  Nutriment(NutrimentParam),
  Name(NameParam)
}

// TODO: Implemenrt using Traits. All search param types will support Serde serialization into
// query strings.
// In order to be stored into the Vector, we need a trait represeting the common type.

// Types:
//  - CriteriaParam (name, op, value)
//  - IngredientParam (name, value)
//  - NutrimentParam (name, op, value)
//  - NameParam (name)
// All types implement the serde_urlencoded::Serializer trait


pub struct SearchParams(Vec<SearchParam>);

// Builder for seartch params. Produces a collection of objects supporting serialization.
impl SearchParams {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn criteria(& mut self, criteria: &criteria::Criteria, op: &str, value: &str) -> &Self {
    self.0.push(SearchParam::Criteria(CriteriaParam {
      name: String::from(criteria.0),
      op: String::from(op),
      value: String::from(value)
    }));
    self
  }

  pub fn ingredient(& mut self, ingredient: &str, value: &str) -> &Self {
    self.0.push(SearchParam::Ingredient(IngredientParam {
      name: String::from(ingredient),
      value: String::from(value)
    }));
    self
  }

  pub fn nutriment(& mut self, nutriment: &str, op: &str, value: usize) -> &Self {
    self.0.push(SearchParam::Nutriment(NutrimentParam {
      name: String::from(nutriment),
      op: String::from(op),
      value: value.to_string()
    }));
    self
  }

  pub fn name(&mut self, name: &str) -> &Self {
    self.0.push(SearchParam::Name(NameParam {
      name: String::from(name),
    }));
    self
  }
}

impl IntoIterator for SearchParams {
  type Item = SearchParam;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}


pub struct Off {
  locale: String,           // The default locale
  client: Client
}


type OffResult = Result<Response, Box<dyn Error>>;


// All functions will return a Result object
// page and locale should be optional.
// JSON response data can be deserialized into untyped JSON values
// with response.json::<HashMap::<String, serde_json::Value>>()
impl Off {
  // Get a taxonomy.
  pub fn taxonomy(&self, taxonomy: &taxonomy::Taxonomy) -> OffResult {
    let base_url = self.base_url(Some("world"))?;
    let url = base_url.join(&format!("data/taxonomies/{}.json", taxonomy.0))?;
    let response = self.client.get(url).send()?;
    Ok(response)
  }

  // Get a facet.
  pub fn facet(&self, facet: &facet::Facet, locale: Option<&str>) -> OffResult {
    let base_url = self.base_url(locale)?;
    let url = base_url.join(&format!("{}.json", facet.0))?;
    let response = self.client.get(url).send()?;
    Ok(response)
  }

  // Get categories.
  pub fn categories(&self, locale: Option<&str>) -> OffResult {
    let base_url = self.base_url(locale)?;
    let url = base_url.join("categories.json")?;
    let response = self.client.get(url).send()?;
    Ok(response)
  }

  // Get category
  pub fn category(&self, category: &str, locale: Option<&str>) -> OffResult {
    let base_url = self.base_url(locale)?;
    let url = base_url.join(&format!("category/{}.json", category))?;
    let response = self.client.get(url).send()?;
    Ok(response)
  }

  // Get product by barcode.
  pub fn product(&self, barcode: &str, locale: Option<&str>) -> OffResult {
    let api_url = self.api_url(locale)?;
    let url = api_url.join(&format!("product/{}", barcode))?;
    let response = self.client.get(url).send()?;
    Ok(response)
  }

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

  // **
  // Builder
  // **

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
    assert_eq!(off.base_url(None).unwrap().as_str(),
               "https://world.openfoodfacts.org/");
  }

  // Get base URL with given locale
  #[test]
  fn test_off_base_url_locale() {
    let off = client().unwrap();
    assert_eq!(off.base_url(Some("gr")).unwrap().as_str(),
               "https://gr.openfoodfacts.org/");
  }

  // Get API URL
  #[test]
  fn test_off_api_url() {
    let off = client().unwrap();
    assert_eq!(off.api_url(None).unwrap().as_str(),
              "https://world.openfoodfacts.org/api/v0/");
  }

  // Get search URL
  #[test]
  fn test_off_search_url() {
    let off = client().unwrap();
    assert_eq!(off.search_url(Some("gr")).unwrap().as_str(),
               "https://gr.openfoodfacts.org/cgi/search.pl");
  }

  #[test]
  fn test_off_taxonomy_const() {
    assert_eq!(taxonomy::ALLERGENS.0, "allergens");
  }

  #[test]
  fn test_off_facet_const() {
    assert_eq!(facet::LABELS.0, "labels");
  }

  #[test]
  fn test_off_get_taxonomy() {
    let off = client().unwrap();
    let response = off.taxonomy(&taxonomy::NOVA_GROUPS).unwrap();
    assert_eq!(response.url().as_str(),
               "https://world.openfoodfacts.org/data/taxonomies/nova_groups.json");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_off_get_facet() {
    let off = client().unwrap();
    let response = off.facet(&facet::BRANDS, Some("gr")).unwrap();
    assert_eq!(response.url().as_str(), "https://gr.openfoodfacts.org/brands.json");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_off_get_categories() {
    let off = client().unwrap();
    let response = off.categories(Some("gr")).unwrap();   // None : defaut locale (world)
    assert_eq!(response.url().as_str(), "https://gr.openfoodfacts.org/categories.json");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_off_get_category() {
    let off = client().unwrap();
    let response = off.category("cheeses", None).unwrap();
    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/category/cheeses.json");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_off_get_product() {
    let off = client().unwrap();
    let response = off.product("069000019832", None).unwrap();  // Diet Pepsi
    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/api/v0/product/069000019832");
    assert_eq!(response.status().is_success(), true);
  }

  #[test]
  fn test_off_search_params() {
    let mut search_params = SearchParams::new();
    search_params.criteria(&criteria::BRANDS, "contains", "Nestlé");
    search_params.criteria(&criteria::CATEGORIES, "does_not_contain", "cheese");
    search_params.ingredient("additives", "without");
    search_params.ingredient("ingredients_that_may_be_from_palm_oil", "indifferent");
    search_params.nutriment("fiber", "lt", 500);
    search_params.nutriment("salt", "gt", 100);
    search_params.name("name");
    // Other: sort by, , product_name, created, last_modified,
    // builder.sort_by();
    // builder.created(date);
    // builder.last_modified(date);
    // builder.pagination(page, page_size);
    // // TODO: Use content types ?
    // builder.format(Json);
    // builder.format(Xml);
    // TODO: unique_scans_<n> ?
    // page, page_size, format(json, Xml).

    let mut spi = search_params.into_iter();
  
    let brands = spi.next().unwrap();
    assert_eq!(brands.name, criteria::BRANDS.0);
    assert_eq!(brands.op, "contains");
    assert_eq!(brands.value, "Nestlé");

    spi.next();   // CATEGORIES

    let ingredient = spi.next().unwrap();
    assert_eq!(ingredient.name, "additives");
    assert_eq!(ingredient.op, "");
    assert_eq!(ingredient.value, "without");

    spi.next();   // ingredients_that_may_be_from_palm_oil

    let nutriment = spi.next().unwrap();
    assert_eq!(nutriment.name, "fiber");
    assert_eq!(nutriment.op, "lt");
    assert_eq!(nutriment.value, "500");

    spi.next();   // salt

    let product_name = spi.next().unwrap();
    assert_eq!(product_name.name, "name");
    assert_eq!(product_name.op, "");
    assert_eq!(product_name.value, "");


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
