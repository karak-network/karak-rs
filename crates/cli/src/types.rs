use serde::{Deserialize, Serialize};
use url::Url as Url_;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Url(Url_);

impl Default for Url {
    fn default() -> Self {
        Url(Url_::parse("http://localhost:8545").unwrap())
    }
}

impl std::str::FromStr for Url {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Url(Url_::parse(s)?))
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Url_> for Url {
    fn from(url: Url_) -> Self {
        Url(url)
    }
}

impl From<Url> for Url_ {
    fn from(url: Url) -> Self {
        url.0
    }
}

impl AsRef<Url_> for Url {
    fn as_ref(&self) -> &Url_ {
        &self.0
    }
}
