use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct Url(url::Url);

impl Url {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn domain(&self) -> &str {
        // Guaranteed to work if self was parsed with `FromStr`:
        self.try_domain().unwrap()
    }

    pub fn path_segments(&self) -> impl Iterator<Item = &str> {
        PathSegments(self.0.path_segments())
    }

    fn try_domain(&self) -> Result<&str, ParseError> {
        self.0.domain().ok_or(ParseError::NoHost)
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Url {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = Url(url::Url::from_str(s)?);
        u.try_domain()?;
        Ok(u)
    }
}

#[derive(Debug, derive_more::From)]
pub enum ParseError {
    Url(url::ParseError),
    NoHost,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseError::*;

        match self {
            Url(e) => e.fmt(f),
            NoHost => write!(f, "URL is missing required host domain"),
        }
    }
}

pub struct PathSegments<'a>(Option<std::str::Split<'a, char>>);

impl<'a> Iterator for PathSegments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().and_then(|it| it.next())
    }
}
