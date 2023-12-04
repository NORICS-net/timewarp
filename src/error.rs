use self::TimeWarpError::ParseError;
use crate::date_matcher::Rule;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum TimeWarpError {
    ParseError(String),
}

impl From<pest::error::Error<Rule>> for TimeWarpError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        ParseError(value.to_string())
    }
}

impl From<ParseIntError> for TimeWarpError {
    fn from(value: ParseIntError) -> Self {
        ParseError(value.to_string())
    }
}

impl std::error::Error for TimeWarpError {}

impl<T> From<Result<T, TimeWarpError>> for TimeWarpError
where
    T: Debug,
{
    fn from(value: Result<T, TimeWarpError>) -> Self {
        value.unwrap_err()
    }
}

impl Display for TimeWarpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ParseError(str) => write!(f, "{str}"),
        }
    }
}

#[inline]
pub fn parse_error<T>(str: impl Into<String>) -> Result<T, TimeWarpError> {
    Err::<T, TimeWarpError>(ParseError(str.into()))
}
