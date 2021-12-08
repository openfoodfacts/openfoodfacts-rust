/// Search paramters.
use std::collections::HashMap;
use serde::ser::{Serialize, Serializer};

/// The value of a search parameter
enum Value {
    String(String),
    Number(u32),
}

/// The collection of query parameters involved in a search.
///
/// Serializes to an HTTP query string.
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
pub struct Query {
    params: HashMap<String, Value>,
    criteria_index: u32,
    nutriment_index: u32
}

impl Query {
    /// Create a new, empty query parameters.
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            criteria_index: 0,
            nutriment_index: 0
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
    pub fn criteria(& mut self, criteria: &str, op: &str, value: &str) -> & mut Self {
        self.criteria_index += 1;
        self.params.insert(format!("tagtype_{}", self.criteria_index),
                            Value::String(String::from(criteria)));
        self.params.insert(format!("tag_contains_{}", self.criteria_index),
                            Value::String(String::from(op)));
        self.params.insert(format!("tag_{}", self.criteria_index),
                            Value::String(String::from(value)));
        self.criteria_index += 1;
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
    pub fn ingredient(& mut self, ingredient: &str, value: &str) -> & mut Self {
        self.params.insert(String::from(ingredient),
                            Value::String(match ingredient {
                                "additives" => format!("{}_additives", value),
                                _ => String::from(value)
                            }));
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
    pub fn nutriment(& mut self, nutriment: &str, op: &str, value: u32) -> & mut Self {
        self.nutriment_index += 1;
        self.params.insert(format!("nutriment_{}", self.nutriment_index),
                            Value::String(String::from(nutriment)));
        self.params.insert(format!("nutriment_compare_{}", self.nutriment_index),
                            Value::String(String::from(op)));
        self.params.insert(format!("nutriment_value_{}", self.nutriment_index),
                            Value::Number(value));
        self.nutriment_index += 1;
        self
    }
}


#[cfg(test)]
mod tests {
    // use super::search::*;

    // #[test]
    // fn test_search_params() {
    //     let mut search_params = Query::new();
    //     search_params.criteria("brands", "contains", "Nestlé")
    //                  .criteria("categories", "does_not_contain", "cheese")
    //                  .ingredient("additives", "without")
    //                  .ingredient("ingredients_that_may_be_from_palm_oil", "indifferent")
    //                  .nutriment("fiber", "lt", 500)
    //                  .nutriment("salt", "gt", 100);
    //     // Other: sort by, , product_name, created, last_modified,
    //     // builder.sort_by();
    //     // builder.created(date);
    //     // builder.last_modified(date);
    //     // builder.pagination(page, page_size);
    //     // // TODO: Use content types ?
    //     // builder.format(Json);
    //     // builder.format(Xml);
    //     // TODO: unique_scans_<n> ?
    //     // page, page_size, format(json, Xml).

    //     let mut spi = search_params.into_iter();
    
    //     if let SearchParam::Criteria(brands) = spi.next().unwrap() {
    //         assert_eq!(brands.name, "brands");
    //         assert_eq!(brands.op, "contains");
    //         assert_eq!(brands.value, "Nestlé");
    //     }
    //     else {
    //         panic!("Not a CriteriaParam")
    //     }

    //     spi.next();   // CATEGORIES

    //     if let SearchParam::Ingredient(ingredient) = spi.next().unwrap() {
    //         assert_eq!(ingredient.name, "additives");
    //         assert_eq!(ingredient.value, "without_additives");
    //     }
    //     else {
    //         panic!("Not an Ingredient")
    //     }

    //     if let SearchParam::Ingredient(ingredient) = spi.next().unwrap() {
    //         assert_eq!(ingredient.name, "ingredients_that_may_be_from_palm_oil");
    //         assert_eq!(ingredient.value, "indifferent");
    //     }
    //     else {
    //         panic!("Not an Ingredient")
    //     }

    //     if let SearchParam::Nutriment(nutriment) = spi.next().unwrap() {
    //         assert_eq!(nutriment.name, "fiber");
    //         assert_eq!(nutriment.op, "lt");
    //         assert_eq!(nutriment.value, 500);
    //     }
    //     else {
    //         panic!("Not an Nutriment")
    //     }
    // }
}