use std::convert::Infallible;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::vec::Vec;

use crate::types::Params;

/// Locale. A country code (`cc`) and an optional
/// language code (`lc`).
///
/// * Country codes must be lowercase ISO 3166-1 codes
///   or the special value "world".
/// * Language codes must be lowercase ISO 639-1 codes.
///
/// # Constructors
///
/// * Create the default Locale (cc = "world" and lc = None).
///
/// ```ignore
/// let locale = Locale::default();
/// ```
///
/// * Create a locale with the given country code and optional language code.
///
/// ```ignore
/// let cc_only = Locale::new("fr", None);
/// let cc_lc = Locale::new("fr", Some("ca"));
///
/// * Create a locale from a string with the formats "{cc}" or "{cc}-{lc}".
///
/// ```ignore
/// let cc_only = Locale::from_str("fr");
/// let cc_lc = Locale::from_str("fr-ca");
/// ```
///
/// Locales can be converted int a string "{cc}" or "{cc}-{lc}" with `Locale::to_string()`.
#[derive(Debug, PartialEq)]
pub struct Locale {
    pub cc: String,
    pub lc: Option<String>,
}

impl Default for Locale {
    fn default() -> Self {
        Self::new("world", None)
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.lc.is_some() {
            write!(f, "{}-{}", self.cc, self.lc.as_ref().unwrap())
        } else {
            write!(f, "{}", self.cc)
        }
    }
}

impl FromStr for Locale {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_assert!(!s.is_empty());
        let mut split = s.split('-');
        let cc = split.next();
        let lc = split.next();
        Ok(Self::new(cc.unwrap(), lc))
    }
}

impl Locale {
    /// Create a new Locale object with the given country code and optional language code.
    pub fn new(cc: &str, lc: Option<&str>) -> Self {
        Self {
            cc: String::from(cc),
            lc: lc.map(String::from),
        }
    }
}

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
mod tests_locale {
    use super::*;

    #[test]
    fn default() {
        let locale = Locale::default();
        assert_eq!(locale.cc, String::from("world"));
        assert_eq!(locale.lc, None);
    }

    #[test]
    fn country() {
        let locale = Locale::new("fr", None);
        assert_eq!(locale.cc, String::from("fr"));
        assert_eq!(locale.lc, None);
    }

    #[test]
    fn country_language() {
        let locale = Locale::new("fr", Some("ca"));
        assert_eq!(locale.cc, String::from("fr"));
        assert_eq!(locale.lc, Some(String::from("ca")));
    }

    #[test]
    fn to_string() {
        let locale_cc = Locale::new("fr", None);
        assert_eq!(locale_cc.to_string(), String::from("fr"));

        let locale_cc_lc = Locale::new("fr", Some("ca"));
        assert_eq!(locale_cc_lc.to_string(), String::from("fr-ca"));
    }

    #[test]
    fn from_str() {
        let locale_1 = Locale::from_str("fr").unwrap();
        assert_eq!(locale_1.cc, String::from("fr"));
        assert_eq!(locale_1.lc, None);

        let locale_2 = Locale::from_str("fr-ca").unwrap();
        assert_eq!(locale_2.cc, String::from("fr"));
        assert_eq!(locale_2.lc, Some(String::from("ca")));

        // Malformed strings are interpreted as (invalid) country codes.
        let locale_3 = Locale::from_str("fr_ca").unwrap();
        assert_eq!(locale_3.cc, String::from("fr_ca"));
        assert_eq!(locale_3.lc, None);
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
