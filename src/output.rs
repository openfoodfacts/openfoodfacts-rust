use std::vec::Vec;

use crate::locale::Locale;
use crate::types::Params;

/// General output parameters. Not all API methods support all parameters.
/// None values indicate that the parameter will be excluded from
/// the query parameters.
///
/// # Constructors
///
/// There is only the `default()` construtor to build an empty output object:
///
/// ```ignore
/// let output = Locale::default();
/// ```
/// Output objects are better created using the struct update syntax:
///
/// ```ignore
/// let output = Output {
///     locale: Some(Locale::new("fr", None)),
///     page: Some(1),
///     ..Output::default()
/// }
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
    /// Return an array of pairs ("name", "value") denoting query parameters.
    ///
    /// # Arguments
    ///
    /// * names - A sequence of parameter names. These match the names of the
    ///     fields in the Output structure (i.e. "page" refers to the `Output::page`)
    ///     field).
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
    fn default() {
        let output = Output::default();
        assert_eq!(output.locale, None);
        assert_eq!(output.page, None);
        assert_eq!(output.page_size, None);
        assert_eq!(output.fields, None);
        assert_eq!(output.nocache, None);
    }

    #[test]
    fn params() {
        let output = Output {
            page: Some(1),
            page_size: Some(20),
            ..Output::default()
        };
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
