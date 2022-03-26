use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};

/// A pair country code (`cc`) and optional language code (`lc`).
///
/// * The country code must be lowercase `ISO 3166-1` code or the
///   special value "world".
/// * The language code must be lowercase `ISO 639-1` code.
///
/// # Constructors
///
/// * Create the default Locale (cc = "world" and lc = None).
///
/// ```
/// use openfoodfacts as off;
///
/// let locale = off::Locale::default();
/// ```
///
/// * Create a locale with the given country code and optional language code.
///
/// ```
/// use openfoodfacts as off;
///
/// let cc_only = off::Locale::new("fr", None);
/// let cc_lc = off::Locale::new("fr", Some("ca"));
/// ```
///
/// * Create a locale from a string with the formats "{cc}" or "{cc}-{lc}".
///
/// ```
/// use openfoodfacts as off;
///
/// let cc_only = off::Locale::from("fr");
/// let cc_lc = off::Locale::from("fr-ca");
/// ```
///
/// Locales can be converted into a String "{cc}" or "{cc}-{lc}" with
/// [Locale::to_string()].
#[derive(Debug, PartialEq)]
pub struct Locale {
    pub cc: String,
    pub lc: Option<String>,
}

impl Locale {
    /// Returns a new Locale object with the given country code (`cc`) and optional
    /// language code (`lc`). If the country code is the empty string, returns the
    /// default locale.
    pub fn new(cc: &str, lc: Option<&str>) -> Self {
        let cc = String::from(cc);
        if cc.is_empty() {
            Locale::default()
        } else {
            Self {
                cc,
                lc: lc.map(String::from),
            }
        }
    }
}

impl Default for Locale {
    /// Returns a new Locale object with "world" country code and empty language code.
    fn default() -> Self {
        Self::new("world", None)
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.lc.is_some() {
            write!(f, "{}-{}", self.cc, self.lc.as_ref().unwrap())
        } else {
            write!(f, "{}", self.cc)
        }
    }
}

impl From<&'_ str> for Locale {
    fn from(s: &str) -> Self {
        debug_assert!(!s.is_empty());
        let mut split = s.split('-');
        let cc = split.next();
        let lc = split.next().filter(|t| !t.is_empty());
        Self::new(cc.unwrap(), lc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let locale = Locale::default();
        assert_eq!(locale.cc, "world");
        assert_eq!(locale.lc, None);
    }

    #[test]
    fn new_with_country_code() {
        let locale = Locale::new("en", None);
        assert_eq!(locale.cc, "en");
        assert_eq!(locale.lc, None);
    }

    #[test]
    fn new_with_empty_country_code() {
        let locale = Locale::new("", Some("us"));
        assert_eq!(locale.cc, "world");
        assert_eq!(locale.lc, None);
    }

    #[test]
    fn new_with_country_code_and_language_code() {
        let locale = Locale::new("en", Some("us"));
        assert_eq!(locale.cc, "en");
        assert_eq!(locale.lc.unwrap(), "us");
    }

    #[test]
    fn from_str() {
        let input = ["en", "en-", "en-us", "-", "-us"];
        let output = [
            ("en", None),
            ("en", None),
            ("en", Some("us".to_string())),
            ("world", None),
            ("world", None),
        ];
        for (i, code) in input.iter().enumerate() {
            let locale = Locale::from(*code);
            assert_eq!(locale.cc, output[i].0);
            assert_eq!(locale.lc, output[i].1);
        }
    }

    #[test]
    fn to_string() {
        let locale_cc = Locale::new("fr", None);
        assert_eq!(locale_cc.to_string(), String::from("fr"));

        let locale_cc_lc = Locale::new("fr", Some("ca"));
        assert_eq!(locale_cc_lc.to_string(), String::from("fr-ca"));
    }
}
