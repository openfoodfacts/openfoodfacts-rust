/// General output parameters. Not all API methods support all parameters.
/// None values indicate that the parameter will be excluded from
/// the query parameters list.
#[derive(Debug, Default)]
pub struct Output {
    /// Country and language codes.
    locale: Option<String>,
    /// Pagination (0 ... n-1)
    page: Option<usize>,
    page_size: Option<usize>,
    /// Output fields.
    fields: Option<String>,
    /// Skip caching for facets
    no_cache: Option<bool>
}

impl Output {
    /// Create a new Output object with defaults.
    fn new() -> Self {
        Self::default()
    }

    /// Set the locale.
    pub fn locale(mut self, value: Option<&str>) -> Self {
        self.locale = value.map(|s| s.to_string());
        self
    }

    /// Set the current page.
    pub fn page(mut self, value: Option<usize>) -> Self {
        self.page = value;
        self
    }

    /// Set the page size.
    pub fn page_size(mut self, value: Option<usize>) -> Self {
        self.page_size = value;
        self
    }

    /// Set the fields. A comma-separated list of field names.
    /// An empty string is equivalent to None.
    pub fn fields(mut self, value: Option<&str>) -> Self {
        self.fields = value.and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) });
        self
    }

    pub fn no_cache(mut self, value: Option<bool>) -> Self {
        self.no_cache = value;
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let output = Output::new();
        assert_eq!(output.locale, None);
        assert_eq!(output.page, None);
        assert_eq!(output.page_size, None);
        assert_eq!(output.fields, None);
        assert_eq!(output.no_cache, None);
    }

    #[test]
    fn locale() {
        let mut output = Output::new().locale(Some("world"));
        assert_eq!(output.locale, Some(String::from("world")));

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
