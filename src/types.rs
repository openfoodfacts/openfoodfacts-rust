use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::vec::Vec;

/// Query parameters. A vector of pairs (name, value) where
/// boh name and value are strings. Params objects are produced
/// by the Output and Search objects.
pub type Params<'a> = Vec<(&'a str, String)>;

/// Supported API versions.
///
/// ApiVersion::to_string() produces the API version string "v{version number}".
/// ApiVersion::from(string) produces the corresponding ApiVersion enum value
/// from a string "v{version number}". Returns `fmt::Error` if the version number
/// is invalid.
#[derive(Debug, PartialEq)]
pub enum ApiVersion {
    V0,
    V2,
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let version = match self {
            Self::V0 => "v0",
            Self::V2 => "v2",
        };
        write!(f, "{}", version)
    }
}

impl FromStr for ApiVersion {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_assert!(!s.is_empty());
        match s {
            "v0" => Ok(Self::V0),
            "v2" => Ok(Self::V2),
            _ => Err(fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests_api_version {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(ApiVersion::V0.to_string(), String::from("v0"));
        assert_eq!(ApiVersion::V2.to_string(), String::from("v2"));
    }

    #[test]
    fn from_str() {
        use std::str::FromStr;

        assert_eq!(ApiVersion::from_str("v0").unwrap(), ApiVersion::V0);
        assert_eq!(ApiVersion::from_str("v2").unwrap(), ApiVersion::V2);
        assert_eq!(ApiVersion::from_str("v666").unwrap_err(), std::fmt::Error);
    }
}
