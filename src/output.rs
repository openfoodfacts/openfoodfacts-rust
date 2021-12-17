use std::convert::Infallible;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Locale. A country code (`cc`) and an optional
/// language code (`lc`).
///
/// * Country codes must be lowercase ISO 3166-1 codes
///   or the special value "world".
/// * Language codes must be lowercase ISO 639-1 codes.
///
/// Can be converted to a string "{cc}" or "{cc}-{lc}".
/// Can be constructed from a string with the same formats.
#[derive(Debug, PartialEq)]
pub struct Locale {
    pub cc: String,
    pub lc: Option<String>
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            cc: String::from("world"),
            lc: None
        }
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
        debug_assert!(s.len() > 0);
        let mut split = s.split("-");
        let cc = split.next();
        let lc = split.next();
        Ok(Self::new().country(cc.unwrap()).language(lc))
    }
}

impl Locale {
    /// Create a new Locale object with the defaults
    /// cc: "world", lc: None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the country code.
    pub fn country(mut self, cc: &str) -> Self {
        self.cc = String::from(cc);
        self
    }

    /// Set or clear the language code.
    pub fn language(mut self, lc: Option<&str>) -> Self {
        self.lc = lc.map(|s| String::from(s));
        self
    }
}

/// General output parameters. Not all API methods support all parameters.
/// None values indicate that the parameter will be excluded from
/// the query parameters list.
#[derive(Debug, Default)]
pub struct Output {
    pub(crate) locale: Option<Locale>,
    pub(crate) page: Option<usize>,
    pub(crate) page_size: Option<usize>,
    pub(crate)fields: Option<String>,
    pub(crate) no_cache: Option<bool>
}

impl Output {
    /// Create a new Output object with defaults (all fields None).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set or clear the locale.
    pub fn locale(mut self, value: Option<Locale>) -> Self {
        self.locale = value;
        self
    }

    /// Set or clear the current page.
    pub fn page(mut self, value: Option<usize>) -> Self {
        self.page = value;
        self
    }

    /// Set or clear the page size.
    pub fn page_size(mut self, value: Option<usize>) -> Self {
        self.page_size = value;
        self
    }

    /// Set or clear the fields list. A comma-separated list of field names.
    /// Some("") is equivalent to None.
    pub fn fields(mut self, value: Option<&str>) -> Self {
        self.fields = value.and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) });
        self
    }

    /// Set of clear the no cache flag. Note that the values `Some(false)`
    /// and `None` would produce different results. The former would produce
    /// the query parameter `nocache=false` while the later would produce no
    /// query parameter.
    pub fn no_cache(mut self, value: Option<bool>) -> Self {
        self.no_cache = value;
        self
    }
}

#[cfg(test)]
mod tests_locale {
    use super::*;

    #[test]
    fn default() {
        let locale = Locale::new();
        assert_eq!(locale.cc, String::from("world"));
        assert_eq!(locale.lc, None);
    }

    #[test]
    fn country() {
        let locale = Locale::new().country("fr");
        assert_eq!(locale.cc, String::from("fr"));
        assert_eq!(locale.lc, None);
    }

    #[test]
    fn language() {
        let locale = Locale::new().language(Some("fr"));
        assert_eq!(locale.cc, String::from("world"));
        assert_eq!(locale.lc, Some(String::from("fr")));
    }

    #[test]
    fn options() {
        let mut locale = Locale::new().country("fr").language(Some("ca"));
        assert_eq!(locale.cc, String::from("fr"));
        assert_eq!(locale.lc, Some(String::from("ca")));

        locale = locale.language(None);
        assert_eq!(locale.lc, None);
    }

    #[test]
    fn to_string() {
        let locale = Locale::new().country("fr").language(Some("ca"));
        assert_eq!(locale.to_string(), String::from("fr-ca"));
    }

    #[test]
    fn from_str() {
        let locale_1 = Locale::from_str("fr").unwrap();
        assert_eq!(locale_1.cc, String::from("fr"));
        assert_eq!(locale_1.lc, None);

        let locale_2 = Locale::from_str("fr-ca").unwrap();
        assert_eq!(locale_2.cc, String::from("fr"));
        assert_eq!(locale_2.lc, Some(String::from("ca")));

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
        let output = Output::new();
        assert_eq!(output.locale, None);
        assert_eq!(output.page, None);
        assert_eq!(output.page_size, None);
        assert_eq!(output.fields, None);
        assert_eq!(output.no_cache, None);
    }

    #[test]
    fn locale() {
        let mut output = Output::new().locale(Some(Locale::new()));
        assert_eq!(output.locale, Some(Locale::new()));

        output = output.locale(None);
        assert_eq!(output.locale, None);
    }

    #[test]
    fn pagination() {
        let mut output = Output::new().page(Some(0));
        assert_eq!(output.page, Some(0));
        assert_eq!(output.page_size, None);

        output = output.page_size(Some(20));
        assert_eq!(output.page, Some(0));
        assert_eq!(output.page_size, Some(20));

        output = output.page(None);
        assert_eq!(output.page, None);
        assert_eq!(output.page_size, Some(20));

        output = output.page_size(None);
        assert_eq!(output.page, None);
        assert_eq!(output.page_size, None);
    }

    #[test]
    fn fields() {
        let mut output = Output::new().fields(Some(""));
        assert_eq!(output.fields, None);

        output = output.fields(Some("a,b,c"));
        assert_eq!(output.fields, Some(String::from("a,b,c")));

        output = output.fields(None);
        assert_eq!(output.fields, None);
    }

    #[test]
    fn no_cache() {
        let mut output = Output::new().no_cache(Some(true));
        assert_eq!(output.no_cache, Some(true));

        output = output.no_cache(None);
        assert_eq!(output.no_cache, None);
    }
}
