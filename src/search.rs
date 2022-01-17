use std::fmt::{self, Display, Formatter};
use crate::types::{ApiVersion, Params};

/// Sorting criteria
#[derive(Debug)]
pub enum SortBy {
    Popularity,
    ProductName,
    CreatedDate,
    LastModifiedDate
}

impl Display for SortBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let sort = match self {
            Self::Popularity => "unique_scans_n",
            Self::ProductName => "product_name",
            Self::CreatedDate => "created_t",
            Self::LastModifiedDate => "last_modified_t"
        };
        write!(f, "{}", sort)
    }
}

/// Implemented by Search objects.
pub trait SearchParams {
    fn params(&self) -> Params;
}

/// The search query builder. Produces an object implementing the trait SearchParams
/// that can be passed to OffClient::search(). The constructor expects the ApiVersion
/// number, used to select which implementation to return.
pub struct Search;

impl Search {
    pub fn new(version: ApiVersion) -> impl SearchParams {
        match version {
            ApiVersion::V0 => SearchParamsV0::new(),
            _ => panic!("Not implemented")
            // TODO: ApiVersion::V2 => SearchParamsV2::new(),
        }
    }
}


// The internal representation of a search parameter value.
enum Value {
    String(String),
    Number(u32),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Number(value)
    }
}

/// Search parameters for V0.
///
/// # Examples
///
/// ```ignore
/// let query = Query::new()
///     .criteria("categories", "contains", "cereals")
///     .criteria("label", "contains", "kosher")
///     .ingredient("additives", "without"),
///     .nutriment("energy", "lt", 500);
/// ```
/// TODO: Rename as Filters ?
pub struct SearchParamsV0 {
    params: Vec<(String, Value)>,
    criteria_index: u32,
    nutriment_index: u32,
    sort_by: Option<SortBy>
}

// TODO: Use refs for strings ?
impl SearchParamsV0 {
    /// Create a new, empty search parameters.
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            criteria_index: 0,
            nutriment_index: 0,
            sort_by: None
        }
    }

    /// Define a criteria query parameter.
    ///
    /// Produces a triplet of pairs
    ///
    /// ```ignore
    /// tagtype_N=<name>
    /// tag_contains_N=<op>
    /// tag_N=<value>
    /// ```
    ///
    /// # Arguments
    ///
    /// * name - A valid criteria name. See openfoodfacts API docs.
    /// * op - One of "contains" or "does_not_contain".
    /// * value - The searched criteria value.
    pub fn criteria(&mut self, criteria: &str, op: &str, value: &str) -> &mut Self {
        self.criteria_index += 1;
        self.params.push((format!("tagtype_{}", self.criteria_index), Value::from(criteria)));
        self.params.push((format!("tag_contains_{}", self.criteria_index), Value::from(op)));
        self.params.push((format!("tag_{}", self.criteria_index), Value::from(value)));
        self
    }

    /// Define an ingredient query parameter.
    ///
    /// Produces a pair
    ///
    /// `<name>=<value>`
    ///
    /// # Arguments
    ///
    /// * ingredient - One of:
    ///     - "additives"
    ///     - "ingredients_from_palm_oil",
    ///     - "ingredients_that_may_be_from_palm_oil",
    ///     - "ingredients_from_or_that_may_be_from_palm_oil".
    // * value: One of "with", "without", "indifferent".
    ///
    /// If `ingredient` is "additives", the values "with", "without" and "indiferent"
    /// are converted to "with_additives", "without_additives" and "indifferent_additives"
    /// respectively.
    pub fn ingredient(&mut self, ingredient: &str, value: &str) -> &mut Self {
        self.params.push((
            String::from(ingredient),
            match ingredient {
                "additives" => Value::from(format!("{}_additives", value)),
                _ => Value::from(value)
            }
        ));
        self
    }

    /// Define a nutriment search parameters.
    ///
    /// Produces a triplet of pairs
    ///
    /// ```ignore
    /// nutriment_N=<name>
    /// nutriment_compare_N=<op>
    /// nutriment_value_N=<quantity>
    /// ```
    /// # Arguments
    ///
    /// * nutriment - The nutriment name. See the openfoodfacts API docs.
    /// * op - The comparation operation to perform. One of "lt", "lte", "gt", "gte",
    ///        "eq".
    /// * value - The value to compare.
    pub fn nutriment(&mut self, nutriment: &str, op: &str, value: u32) -> &mut Self {
        self.nutriment_index += 1;
        self.params.push((format!("nutriment_{}", self.nutriment_index), Value::from(nutriment)));
        self.params.push((format!("nutriment_compare_{}", self.nutriment_index), Value::from(op)));
        self.params.push((format!("nutriment_value_{}", self.nutriment_index), Value::from(value)));
        self
    }

    /// Set/clear the sorting order.
    pub fn sort_by(&mut self, sort_by: Option<SortBy>) -> &mut Self {
        self.sort_by = sort_by;
        self
    }
}

impl SearchParams for SearchParamsV0 {
    fn params(&self) -> Params {
        let mut added: Vec<&str> = Vec::new();  // Vec<T = &str>
        let mut params: Params = Vec::new();
        for (name, value) in &self.params {
            if !added.contains(&name.as_str()) {
                let v = match value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string()
                };
                params.push((&name, v));
                added.push(&name);
            }
        }
        if let Some(ref s) = self.sort_by {
            params.push(("sort_by", s.to_string()));
        }
        // Adds the 'action' and 'json' parameter. TODO: Should be done in client::search() ?
        params.push(("action", String::from("process")));
        params.push(("json", true.to_string()));
        params
    }
}

pub struct SearchParamsV2;

impl SearchParamsV2 {
    pub fn new() -> Self {
        SearchParamsV2 {}
    }
}

impl SearchParams for SearchParamsV2 {
    fn params(&self) -> Params {
        Params::new()
    }
}

#[cfg(test)]
mod tests_sort_by {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(SortBy::Popularity.to_string(), String::from("unique_scans_n"));
        assert_eq!(SortBy::ProductName.to_string(), String::from("product_name"));
        assert_eq!(SortBy::CreatedDate.to_string(), String::from("created_t"));
        assert_eq!(SortBy::LastModifiedDate.to_string(), String::from("last_modified_t"));
    }
}

#[cfg(test)]
mod tests_search {
    use super::*;

    #[test]
    fn search_params() {
        let mut search = SearchParamsV0::new();
        search.criteria("brands", "contains", "Nestlé")
              .criteria("categories", "does_not_contain", "cheese")
              .ingredient("additives", "without")
              .ingredient("ingredients_that_may_be_from_palm_oil", "indifferent")
              .nutriment("fiber", "lt", 500)
              .nutriment("salt", "gt", 100);


        let params = search.params();
        assert_eq!(&params, &[
            ("tagtype_1", String::from("brands")),
            ("tag_contains_1", String::from("contains")),
            ("tag_1", String::from("Nestlé")),
            ("tagtype_2", String::from("categories")),
            ("tag_contains_2", String::from("does_not_contain")),
            ("tag_2", String::from("cheese")),
            ("additives", String::from("without_additives")),
            ("ingredients_that_may_be_from_palm_oil", String::from("indifferent")),
            ("nutriment_1", String::from("fiber")),
            ("nutriment_compare_1", String::from("lt")),
            ("nutriment_value_1", String::from("500")),
            ("nutriment_2", String::from("salt")),
            ("nutriment_compare_2", String::from("gt")),
            ("nutriment_value_2", String::from("100")),
            ("action", String::from("process")),
            ("json", String::from("true"))
        ]);
    }

    /*
    #[test]
    fn search() {
        // Get a list of products by barcodes (v2 only). Supports locale and fields.
        let output = Output {
            locale: Some(Locale::new("fr", None)),
            fields: Some("code,product_name"),
            ..Output::default()
        };
        client.search(Query::products("p1,p2"), output);

        // Search for French breakfast cereals with no additives nor palm oil and a great Nutriscore (A) (v0)
        // Supports locale and sort,
        client.search(Query::filter().criteria("categories", "contains", "breakfast_cereals")
                                     .criteria("nutrition_grades", "contains", "A")
                                     .ingredient("ingredients_from_palm_oil", "without")
                                     .ingredient("additives", "without"), output);

        // With sorting.
        output.sort_by = Some(SortBy::Popularity);
        client.search(Query::filter().criteria("categories", "contains", "breakfast_cereals")
                                     .criteria("nutrition_grades", "contains", "A")
                                     .ingredient("ingredients_from_palm_oil", "without")
                                     .ingredient("additives", "without")
                                     .nutriment("salt", "gt", 100), output);

        // Search V2
    }
    */
}
