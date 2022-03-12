use crate::types::Params;
use std::fmt::{self, Display, Formatter};

/// Sorting criteria
#[derive(Debug)]
pub enum SortBy {
    Popularity,
    ProductName,
    CreatedDate,
    LastModifiedDate,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let sort = match self {
            Self::Popularity => "unique_scans_n",
            Self::ProductName => "product_name",
            Self::CreatedDate => "created_t",
            Self::LastModifiedDate => "last_modified_t",
        };
        write!(f, "{}", sort)
    }
}

// The internal representation of a search parameter value.
enum Value {
    String(String),
    Number(u32),
    None,
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

/// Implemented by Search objects.
pub trait SearchParams {
    fn params(&self) -> Params;
}

/// The search query builder. Produces an object implementing the trait SearchParams
/// that can be passed to OffClient::search(). The constructor expects the ApiVersion
/// number, used to select which implementation to return.
pub struct SearchBuilder<S> {
    params: Vec<(String, Value)>,
    state: S,
}

// ----------------------------------------------------------------------------
// Search V0
// ----------------------------------------------------------------------------

/// Search parameters.
///
/// # Examples
///
/// ```ignore
/// let query = SearchParamsV0::new()
///     .criteria("categories", "contains", "cereals")
///     .criteria("label", "contains", "kosher")
///     .ingredient("additives", "without"),
///     .nutrient("energy", "lt", 500);
/// ```
#[derive(Default)]
pub struct SearchStateV0 {
    criteria_index: u32,
    nutrient_index: u32,
    // TODO: Aso V2 ?
    sort_by: Option<SortBy>,
}

pub type SearchBuilderV0 = SearchBuilder<SearchStateV0>;

impl Default for SearchBuilderV0 {
    fn default() -> Self {
        Self {
            params: Vec::new(),
            state: SearchStateV0::default(),
        }
    }
}

impl SearchBuilderV0 {
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a criteria query parameter.
    ///
    /// Produces a triplet of pairs
    ///
    /// ```ignore
    /// tagtype_N=<criteria>
    /// tag_contains_N=<op>
    /// tag_N=<value>
    /// ```
    ///
    /// # Arguments
    ///
    /// * criteria - A valid criteria name. See the [`API docs`].
    /// * op - One of "contains" or "does_not_contain".
    /// * value - The searched criteria value.
    ///
    /// [`API docs`]: https://openfoodfacts.github.io/api-documentation/#5Filtering
    pub fn criteria(&mut self, criteria: &str, op: &str, value: &str) -> &mut Self {
        self.state.criteria_index += 1;
        self.params.push((
            format!("tagtype_{}", self.state.criteria_index),
            Value::from(criteria),
        ));
        self.params.push((
            format!("tag_contains_{}", self.state.criteria_index),
            Value::from(op),
        ));
        self.params.push((
            format!("tag_{}", self.state.criteria_index),
            Value::from(value),
        ));
        self
    }

    /// Define an ingredient query parameter.
    ///
    /// Produces a pair
    ///
    /// `<ingredient>=<value>`
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
                _ => Value::from(value),
            },
        ));
        self
    }

    /// Define a nutrient (a.k.a nutriment in the API docs) search parameters.
    ///
    /// Produces a triplet of pairs
    ///
    /// ```ignore
    /// nutriment_N=<nutriment>
    /// nutriment_compare_N=<op>
    /// nutriment_value_N=<quantity>
    /// ```
    /// # Arguments
    ///
    /// * nutrient - The nutrient name. See the [`API docs`].
    /// * op - The comparation operation to perform. One of "lt", "lte", "gt", "gte",
    ///        "eq".
    /// * value - The value to compare.
    ///
    /// [`API docs`]: https://openfoodfacts.github.io/api-documentation/#5Filtering
    pub fn nutrient(&mut self, nutriment: &str, op: &str, value: u32) -> &mut Self {
        self.state.nutrient_index += 1;
        self.params.push((
            format!("nutriment_{}", self.state.nutrient_index),
            Value::from(nutriment),
        ));
        self.params.push((
            format!("nutriment_compare_{}", self.state.nutrient_index),
            Value::from(op),
        ));
        self.params.push((
            format!("nutriment_value_{}", self.state.nutrient_index),
            Value::from(value),
        ));
        self
    }

    /// Set/clear the sorting order.
    pub fn sort_by(&mut self, sort_by: Option<SortBy>) -> &mut Self {
        self.state.sort_by = sort_by;
        self
    }
}

impl SearchParams for SearchBuilderV0 {
    fn params(&self) -> Params {
        let mut params: Params = Vec::new();
        for (name, value) in &self.params {
            let v = match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::None => {
                    continue;
                }
            };
            params.push((name, v));
        }
        if let Some(ref s) = self.state.sort_by {
            params.push(("sort_by", s.to_string()));
        }
        // Adds the 'action' and 'json' parameter. TODO: Should be done in client::search() ?
        params.push(("action", String::from("process")));
        params.push(("json", true.to_string()));
        params
    }
}

// ----------------------------------------------------------------------------
// Search V2
// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct SearchStateV2 {
    sort_by: Option<SortBy>,
}

pub type SearchBuilderV2 = SearchBuilder<SearchStateV2>;

impl Default for SearchBuilderV2 {
    fn default() -> Self {
        Self {
            params: Vec::new(),
            state: SearchStateV2::default(),
        }
    }
}

impl SearchBuilderV2 {
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a criteria query parameter.
    ///
    /// Produces pairs
    ///
    /// ```ignore
    /// <criteria>_tags=<value>
    /// ```
    ///
    /// or
    ///
    /// ```ignore
    /// <criteria>_tags_<lc>= <value>
    /// ```
    ///
    /// if a language code has been given.
    ///
    /// # Arguments
    ///
    /// * criteria - A valid criteria name. See the [`API docs`].
    /// * value - The criteria value. Use comma for AND, colon for OR and tilde for NOT.
    ///     See the [`Search V2 API docs`].
    /// * lc: Optional language code.
    ///
    /// [`openfoodfacts API docs`]: https://openfoodfacts.github.io/api-documentation/#5Filtering
    /// [`Search V2 API docs`]: https://wiki.openfoodfacts.org/Open_Food_Facts_Search_API_Version_2
    pub fn criteria(&mut self, criteria: &str, value: &str, lc: Option<&str>) -> &mut Self {
        if let Some(lc) = lc {
            self.params
                .push((format!("{}_tags_{}", criteria, lc), Value::from(value)));
        } else {
            self.params
                .push((format!("{}_tags", criteria), Value::from(value)));
        }
        self
    }

    /// Define a condition on a nutrient.
    ///
    /// Produces a pair
    ///
    /// ```ignore
    /// <nutrient>_<unit>=<value>
    /// ```
    ///
    /// if `op` is "=", otherwise produces a non-valued parameter
    ///
    /// ```ignore
    /// <nutient>_<unit><op><value>
    /// ```
    ///
    /// # Arguments
    ///
    /// * nutrient - The nutrient name. See the [`API docs`].
    /// * unit - One of the "100g" or "serving".
    /// * op - A comparison operator. One of  '=', '<', '>', `<=', '=>`.
    ///     See the [`Search V2 API docs`].
    /// * value - The value to compare.
    ///
    /// TODO: Verify the <= and => operators.
    ///
    /// [`API docs`]: https://openfoodfacts.github.io/api-documentation/#5Filtering
    /// [`Search V2 API docs`]: https://wiki.openfoodfacts.org/Open_Food_Facts_Search_API_Version_2
    pub fn nutrient(&mut self, nutrient: &str, unit: &str, op: &str, value: u32) -> &mut Self {
        let param = match op {
            "=" => (format!("{}_{}", nutrient, unit), Value::from(value)),
            // The name and value becomes the param name. TODO: Check HTTP specs if <, >, etc supported
            // in query params in place of =.
            _ => (format!("{}_{}{}{}", nutrient, unit, op, value), Value::None),
        };
        self.params.push(param);
        self
    }

    /// Convenience method to add a nutrient condition per 100 grams.
    pub fn nutrient_100g(&mut self, nutrient: &str, op: &str, value: u32) -> &mut Self {
        self.nutrient(nutrient, "100g", op, value)
    }

    /// Convenience method to add a nutrient condition per serving.
    pub fn nutrient_serving(&mut self, nutrient: &str, op: &str, value: u32) -> &mut Self {
        self.nutrient(nutrient, "serving", op, value)
    }

    /// TODO: Supported ?
    /// Set/clear the sorting order.
    pub fn sort_by(&mut self, sort_by: Option<SortBy>) -> &mut Self {
        self.state.sort_by = sort_by;
        self
    }
}

impl SearchParams for SearchBuilderV2 {
    fn params(&self) -> Params {
        let mut params: Params = Vec::new();
        for (name, value) in &self.params {
            let v = match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::None => String::new(), // The empty string
            };
            params.push((name, v));
        }
        if let Some(ref s) = self.state.sort_by {
            params.push(("sort_by", s.to_string()));
        }
        params
    }
}

#[cfg(test)]
mod tests_sort_by {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(
            SortBy::Popularity.to_string(),
            String::from("unique_scans_n")
        );
        assert_eq!(
            SortBy::ProductName.to_string(),
            String::from("product_name")
        );
        assert_eq!(SortBy::CreatedDate.to_string(), String::from("created_t"));
        assert_eq!(
            SortBy::LastModifiedDate.to_string(),
            String::from("last_modified_t")
        );
    }
}

#[cfg(test)]
mod tests_search_v0 {
    use super::*;

    #[test]
    fn search_params() {
        let mut search = SearchBuilderV0::new();
        search
            .criteria("brands", "contains", "Nestlé")
            .criteria("categories", "does_not_contain", "cheese")
            .ingredient("additives", "without")
            .ingredient("ingredients_that_may_be_from_palm_oil", "indifferent")
            .nutrient("fiber", "lt", 500)
            .nutrient("salt", "gt", 100);

        let params = search.params();
        assert_eq!(
            &params,
            &[
                ("tagtype_1", String::from("brands")),
                ("tag_contains_1", String::from("contains")),
                ("tag_1", String::from("Nestlé")),
                ("tagtype_2", String::from("categories")),
                ("tag_contains_2", String::from("does_not_contain")),
                ("tag_2", String::from("cheese")),
                ("additives", String::from("without_additives")),
                (
                    "ingredients_that_may_be_from_palm_oil",
                    String::from("indifferent")
                ),
                ("nutriment_1", String::from("fiber")),
                ("nutriment_compare_1", String::from("lt")),
                ("nutriment_value_1", String::from("500")),
                ("nutriment_2", String::from("salt")),
                ("nutriment_compare_2", String::from("gt")),
                ("nutriment_value_2", String::from("100")),
                ("action", String::from("process")),
                ("json", String::from("true"))
            ]
        );
    }
}

#[cfg(test)]
mod tests_search_v2 {
    use super::*;

    #[test]
    fn search_params() {
        let mut search = SearchBuilderV2::new();
        search
            .criteria("brands", "Nestlé", Some("fr"))
            .criteria("categories", "-cheese", None)
            // TODO ?
            //              .ingredient("additives", "without")
            //              .ingredient("ingredients_that_may_be_from_palm_oil", "indifferent")
            .nutrient_100g("fiber", "<", 500)
            .nutrient_serving("salt", "=", 100);

        let params = search.params();
        assert_eq!(
            &params,
            &[
                ("brands_tags_fr", String::from("Nestlé")),
                ("categories_tags", String::from("-cheese")),
                // TODO
                //            ("additives", String::from("without_additives")),
                //            ("ingredients_that_may_be_from_palm_oil", String::from("indifferent")),
                ("fiber_100g<500", String::new()),
                ("salt_serving", String::from("100")),
            ]
        );
    }
}
