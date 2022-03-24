use std::convert::Infallible;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Contains a country code (`cc`) and an optional language code (`lc`).
///
/// * Country code must be lowercase `ISO 3166-1` code or the special value "world".
/// * Language code must be lowercase `ISO 639-1` code.
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
/// ```
///
/// * Create a locale from a string with the formats "{cc}" or "{cc}-{lc}".
///
/// ```ignore
/// let cc_only = Locale::from_str("fr");
/// let cc_lc = Locale::from_str("fr-ca");
/// ```
///
/// Locales can be converted into a string "{cc}" or "{cc}-{lc}" with `Locale::to_string()`.
#[derive(Debug, PartialEq)]
pub struct Locale {
    pub cc: String,
    pub lc: Option<String>,
}

impl Locale {
    /// Returns a new Locale object with the given country code and optional language code.
    /// If country code is empty returns default locale instead.
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

impl FromStr for Locale {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_assert!(!s.is_empty());
        let mut split = s.split('-');
        let cc = split.next();
        let lc = split.next().filter(|t| !t.is_empty());
        Ok(Self::new(cc.unwrap(), lc))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Locale;

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
            let locale = Locale::from_str(code).unwrap();
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
