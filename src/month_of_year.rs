use self::Month::{Apr, Aug, Dec, Feb, Jan, Jul, Jun, Mar, May, Nov, Oct, Sep, Unknown};
use crate::date_matcher::Rule;

/// Names of months - shorted to 3-letter-identifier.
#[must_use]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Month {
    Jan = 1,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
    #[default]
    Unknown = 255,
}

impl Month {
    pub(crate) fn from_month(rule: Rule) -> Self {
        match rule {
            Rule::january => Jan,
            Rule::february => Feb,
            Rule::march => Mar,
            Rule::april => Apr,
            Rule::may => May,
            Rule::june => Jun,
            Rule::july => Jul,
            Rule::august => Aug,
            Rule::september => Sep,
            Rule::october => Oct,
            Rule::november => Nov,
            Rule::december => Dec,
            _ => Unknown,
        }
    }

    /// calculates the amount of month this month is before the `other` one.
    pub fn month_before(&self, other: Self) -> i32 {
        let today = if *self as i32 > other as i32 {
            *self as i32
        } else {
            *self as i32 + 12
        };
        today - other as i32
    }

    /// Increments the month by `add`. An `add = 1` returns the next month. A `-2` returns the pre-last.
    pub fn inc(&self, add: i32) -> Self {
        From::from(*self as i32 + add)
    }
}

impl From<i32> for Month {
    fn from(value: i32) -> Self {
        match value % 12 {
            1 => Jan,
            2 => Feb,
            3 => Mar,
            4 => Apr,
            5 => May,
            6 => Jun,
            7 => Jul,
            8 => Aug,
            9 => Sep,
            10 => Oct,
            11 => Nov,
            0 => Dec,
            _ => Unknown,
        }
    }
}

#[cfg(test)]
mod should {
    use crate::month_of_year::Month::{Dec, Jan, Jul, May};

    #[test]
    fn calc_month_before() {
        assert_eq!(May.month_before(Jul), 10);
        assert_eq!(Jul.month_before(May), 2);
    }

    #[test]
    fn inc_months() {
        assert_eq!(May.inc(2), Jul);
        assert_eq!(Dec.inc(1), Jan);
        assert_eq!(Dec.inc(7), Jul);
        assert_eq!(Dec.inc(-5), Jul);
    }
}
