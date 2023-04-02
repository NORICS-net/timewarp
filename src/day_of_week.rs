use self::DayOfWeek::*;
use super::date_matcher::Rule;

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
    pub(crate) fn from_day_of_week(rule: Rule) -> Self {
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

    /// calculates the amount of days this DayOfWeek is before the `other` one.
    pub fn days_before(&self, other: Self) -> i32 {
        let today = if *self as i32 > other as i32 {
            *self as i32
        } else {
            *self as i32 + 7
        };
        today - other as i32
    }
}

impl From<i32> for DayOfWeek {
    fn from(value: i32) -> Self {
        match value % 7 {
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

#[cfg(test)]
mod should {

    #[test]
    fn calc_days_before() {
        use super::DayOfWeek::*;
        assert_eq!(1, Sat.days_before(Fri));
        assert_eq!(6, Fri.days_before(Sat));
        assert_eq!(7, Sat.days_before(Sat));
    }
}
