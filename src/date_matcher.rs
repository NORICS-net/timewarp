use self::DayOfWeek::*;
use super::TimeWarpError;
use crate::error::parse_error;
use pest::iterators::Pairs;
use pest::Parser;
use std::str::FromStr;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DayOfWeek {
    Sun = 0,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    #[default]
    Unknown = 255,
}

impl DayOfWeek {
    pub fn from_day_of_week(rule: Rule) -> Self {
        match rule {
            Rule::monday => Mon,
            Rule::tuesday => Tue,
            Rule::wednesday => Wed,
            Rule::thursday => Thu,
            Rule::friday => Fri,
            Rule::saturday => Sat,
            Rule::sunday => Sun,
            _ => Unknown,
        }
    }
}

impl From<i32> for DayOfWeek {
    fn from(value: i32) -> Self {
        match value {
            0 => Sun,
            1 => Mon,
            2 => Tue,
            3 => Wed,
            4 => Thu,
            5 => Fri,
            6 => Sat,
            _ => Unknown,
        }
    }
}

#[derive(Parser, Debug, Default)]
#[grammar = "date_matcher.pest"]
struct DateMatcher;

fn yy_mm_dd(pairs: Pairs<'_, Rule>) -> Result<String, TimeWarpError> {
    let mut yy = 0;
    let mut mm = 0;
    let mut dd = 0;
    for pair in pairs {
        match pair.as_rule() {
            Rule::yyyy => yy = i32::from_str(pair.as_str())?,
            Rule::mm => mm = i32::from_str(pair.as_str())?,
            Rule::dd => dd = i32::from_str(pair.as_str())?,
            _ => {
                println!("Found more than expected: {:?}", pair)
            }
        };
    }
    if yy < 100 {
        yy = 2000 + yy;
    }
    Ok(format!("{yy:04}-{mm:02}-{dd:02}"))
}

pub fn date_matcher(date: impl Into<String>) -> Result<String, TimeWarpError> {
    let text = date.into();

    for pair in DateMatcher::parse(Rule::date_matcher, &text)?
        .next()
        .unwrap()
        .into_inner()
    {
        match pair.as_rule() {
            Rule::date_iso | Rule::date_en | Rule::date_de => return yy_mm_dd(pair.into_inner()),
            _ => println!("{:?}", pair),
        };
    }
    parse_error("Nothing found")
}

#[cfg(test)]
mod should {
    use super::date_matcher;

    #[test]
    fn find_yesterday() {
        assert_eq!(date_matcher("yesterday").unwrap(), "2023-01-22");
    }

    #[test]
    fn parse_date() {
        assert_eq!(date_matcher("22.01.2023").unwrap(), "2023-01-22");
        assert_eq!(date_matcher("22.1.23").unwrap(), "2023-01-22");
        assert_eq!(date_matcher("3/16/2023").unwrap(), "2023-03-16");
        assert_eq!(date_matcher("2023-03-16").unwrap(), "2023-03-16");
        assert_eq!(date_matcher("    23-03-16  ").unwrap(), "2023-03-16");
    }
}
