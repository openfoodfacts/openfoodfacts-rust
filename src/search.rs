// Search parameters.
mod search {
    use std::iter::IntoIterator;


    /// Defines a criteria search parameter.
    ///
    /// Serializes to a triplet of pairs
    ///
    /// tagtype_N=<name>
    /// tag_contains_N=<op>
    /// tag_N=<value>
    ///
    /// The index N is calculated by the SearchParams serializer and indicates the number
    /// of this CriteriaParam in the SearchParams vector.
    #[derive(Debug)]
    pub struct CriteriaParam {
        /// A valid criteria name.
        pub name: String,
        // One of "contains" or "does_not_contain".
        pub op: String,
        // The searched criteria value.
        pub value: String
    }


    /// Defines an ingredient search parameter. 
    ///
    /// Serializes to a pair
    /// 
    /// <name>=<value>
    ///
    #[derive(Debug)]
    pub struct IngredientParam {
        /// One of "additives", "ingredients_from_palm_oil",
        /// "ingredients_that_may_be_from_palm_oil", "ingredients_from_or_that_may_be_from_palm_oil"
        pub name: String,
        // One of "with", "without", "indifferent".
        /// If <name> is "additives", the values "with", "without" and "indiferent" are converted to
        /// "with_additives", "without_additives" and "indifferent_additives" respectively.
        pub value: String
    }

    /// Defines a nutriment search parameters.
    ///
    /// Serializes to a triplet of pairs
    ///
    /// nutriment_N=<name>
    /// nutriment_compare_N=<op>
    /// nutriment_value_N=<quantity>
    ///
    /// TODO: Unit of nutriment_value_N (gr, mg)?
    /// The index N is calculated by the SearchParams serializer and indicates the number
    /// of this NutrimentParam in the SearchParams vector.

    #[derive(Debug)]
    pub struct NutrimentParam {
        /// A valid nutriment values (as returned by the "nutriments" taxonomy).
        pub name: String,
        /// One of "lt", "lte", "gt", "gte" or "eq".
        pub op: String,
        // The nutriment quantity.
        pub value: u32
    }

    // Other search parameters.

    // TODO:
    // format: Default is JSON -> method call parameter
    // page and page_size -> method call parameter
    // sort_by: String or Enum (Popularity, Product name, Add date, Edit date) 
    //  -> method call parameter 

    /// A search parameter of any of the valid types.
    pub enum SearchParam {
        Criteria(CriteriaParam),
        Ingredient(IngredientParam),
        Nutriment(NutrimentParam)
    }


    /// The collection of SearchParam objects involved in a particular search.
    ///
    /// Serializes to an HTTP query string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let search_params = SearchParams::new()
    ///     .criteria("categories", "contains", "cereals")
    ///     .criteria("label", "contains", "kosher")
    ///     .ingredient("additives", "without"),
    ///     .nutriment("energy", "lt", 500);
    /// ```
    pub struct SearchParams(Vec<SearchParam>);

    impl SearchParams {
        pub fn new() -> Self {
            Self(Vec::new())
        }

        pub fn criteria(& mut self, criteria: &str, op: &str, value: &str) -> & mut Self {
            self.0.push(SearchParam::Criteria(CriteriaParam {
                name: String::from(criteria),
                op: String::from(op),
                value: String::from(value)
            }));
            self
        }
    
        pub fn ingredient(& mut self, ingredient: &str, value: &str) -> & mut Self {
            self.0.push(SearchParam::Ingredient(IngredientParam {
                name: String::from(ingredient),
                value: match ingredient {
                    "additives" => format!("{}_additives", value),
                    _ => String::from(value)
                }
            }));
            self
        }

        pub fn nutriment(& mut self, nutriment: &str, op: &str, value: u32) -> & mut Self {
            self.0.push(SearchParam::Nutriment(NutrimentParam {
                name: String::from(nutriment),
                op: String::from(op),
                value: value
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

    // TODO: Use UrlEncodedSerializer ?
    // impl Serialize for SearchParams {
    //   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //   where
    //       S: Serializer,
    //   {
    //       // The simplest implementation is to convert each SeachParam in an array of
    //       // pairs name=value
    //       // and call serde_urlencoded::to_string() on it.
    //       Ok("name=value")
    //   }
    // }
}   // end module


#[cfg(test)]
mod tests {
    use super::search::*;

    #[test]
    fn test_search_params() {
        let mut search_params = SearchParams::new();
        search_params.criteria("brands", "contains", "Nestlé")
                     .criteria("categories", "does_not_contain", "cheese")
                     .ingredient("additives", "without")
                     .ingredient("ingredients_that_may_be_from_palm_oil", "indifferent")
                     .nutriment("fiber", "lt", 500)
                     .nutriment("salt", "gt", 100);
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
    
        if let SearchParam::Criteria(brands) = spi.next().unwrap() {
            assert_eq!(brands.name, "brands");
            assert_eq!(brands.op, "contains");
            assert_eq!(brands.value, "Nestlé");
        }
        else {
            panic!("Not a CriteriaParam")
        }

        spi.next();   // CATEGORIES

        if let SearchParam::Ingredient(ingredient) = spi.next().unwrap() {
            assert_eq!(ingredient.name, "additives");
            assert_eq!(ingredient.value, "without_additives");
        }
        else {
            panic!("Not an Ingredient")
        }

        if let SearchParam::Ingredient(ingredient) = spi.next().unwrap() {
            assert_eq!(ingredient.name, "ingredients_that_may_be_from_palm_oil");
            assert_eq!(ingredient.value, "indifferent");
        }
        else {
            panic!("Not an Ingredient")
        }

        if let SearchParam::Nutriment(nutriment) = spi.next().unwrap() {
            assert_eq!(nutriment.name, "fiber");
            assert_eq!(nutriment.op, "lt");
            assert_eq!(nutriment.value, 500);
        }
        else {
            panic!("Not an Nutriment")
        }
    }
}