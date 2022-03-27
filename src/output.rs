use std::vec::Vec;

use crate::locale::Locale;
use crate::types::Params;

/// General output parameters. Not all API methods support all parameters.
/// None values indicate that the parameter will be excluded from the
/// query parameters.
///
/// ```
/// use openfoodfacts::{self as off, Locale, Output};
///
/// let output = Output::new()
///     .locale(Locale::new("fr", None))
///     .page(1);
/// assert_eq!(output.locale.unwrap().cc, String::from("fr"));
/// assert_eq!(output.page.unwrap(), 1);
/// ```
#[derive(Debug, Default)]
pub struct Output {
    pub locale: Option<Locale>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub fields: Option<&'static str>,
    pub nocache: Option<bool>,
}

impl Output {
    /// Creates a new Output object with defaults (all None).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the locale field.
    pub fn locale(mut self, locale: Locale) -> Self {
        self.locale = Some(locale);
        self
    }

    /// Sets the page and page_size fields.
    pub fn pagination(self, page: usize, page_size: usize) -> Self {
        self.page(page).page_size(page_size)
    }
    /// Sets the page field.
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the page field.
    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Sets the fields field. Must be a str slice with comma-separated field names.
    /// Sets fields to None if the slice is empty.
    pub fn fields(mut self, fields: &'static str) -> Self {
        self.fields = Some(fields).filter(|t| !t.is_empty());
        self
    }

    /// Sets the nocache field.
    pub fn nocache(mut self, nocache: bool) -> Self {
        self.nocache = Some(nocache);
        self
    }

    /// Returns an array of pairs ("name", "value") representing query parameters.
    /// If `names` is given, it must be a sequence of parameter names. These match
    /// the names of the fields in the Output structure (i.e. "page" refers to the
    /// `Output::page` field).
    ///
    /// Note that:
    ///
    /// * The `locale` name is ignored.
    /// * Fields with value `None` are ignored.
    /// * Repeated names are ignored.
    /// * Callers should only request the parameters that are supported by the target
    ///   API call.
    pub fn params<'a>(&self, names: &[&'a str]) -> Params<'a> {
        let mut added: Vec<&str> = Vec::new();
        let mut params: Params = Vec::new();
        for name in names {
            if !added.contains(name) {
                let value = match *name {
                    "page" => self.page.map(|v| v.to_string()),
                    "page_size" => self.page_size.map(|v| v.to_string()),
                    "fields" => self.fields.map(|v| v.to_string()),
                    "nocache" => self.nocache.map(|v| v.to_string()),
                    _ => None,
                };
                if let Some(v) = value {
                    params.push((name, v));
                    added.push(name);
                }
            }
        }
        params
    }
}

#[cfg(test)]
mod tests_output {
    use super::*;

    #[test]
    fn defaults() {
        let output = Output::new();
        assert_eq!(output.locale, None);
        assert_eq!(output.page, None);
        assert_eq!(output.page_size, None);
        assert_eq!(output.fields, None);
        assert_eq!(output.nocache, None);
    }

    #[test]
    fn locale() {
        let output = Output::new().locale(Locale::from("fr"));
        assert_eq!(output.locale, Some(Locale::from("fr")));
    }

    #[test]
    fn pagination() {
        let output = Output::new().pagination(1, 20);
        assert_eq!(output.page, Some(1));
        assert_eq!(output.page_size, Some(20));
    }

    #[test]
    fn page_page_size() {
        let output = Output::new().page(1).page_size(20);

        assert_eq!(output.page, Some(1));
        assert_eq!(output.page_size, Some(20));
    }

    #[test]
    fn fields() {
        let output = Output::new().fields("a,b,c");
        assert_eq!(output.fields, Some("a,b,c"));
    }

    #[test]
    fn no_cache() {
        let output = Output::new().nocache(true);
        assert_eq!(output.nocache, Some(true));
    }

    #[test]
    fn params() {
        let output = Output::new().pagination(1, 20);

        let params = output.params(&["page", "page_size"]);
        assert_eq!(
            &params,
            &[
                ("page", String::from("1")),
                ("page_size", String::from("20"))
            ]
        );
    }
}
