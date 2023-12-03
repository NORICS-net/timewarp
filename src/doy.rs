use crate::day_of_week::DayOfWeek;
use crate::error::parse_error;
use crate::month_of_year::Month;
use crate::DayOfWeek::{Sun, Thu};
use crate::TimeWarpError;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::ops::{Add, Sub};
use std::time::SystemTime;

/// Day Of Year. Helper-class to easily calculate dates.
#[must_use]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Doy {
    pub year: i32,
    pub doy: i32,
}

impl Doy {
    pub const SECOND: u128 = 1000;
    pub const MINUTE: u128 = Self::SECOND * 60;
    pub const HOUR: u128 = Self::MINUTE * 60;
    pub const DAY: u128 = Self::HOUR * 24;
    pub const YEAR: u128 = Self::DAY * 365 + Self::HOUR * 6;

    /// returns the Doy representing today.
    pub fn today() -> Self {
        let millis = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self::from_millis(millis)
    }

    /// converts milliseconds from POSIX time or Epoch time to Doy.
    pub fn from_millis(millis: u128) -> Self {
        let offset = millis % Self::YEAR;
        let year = 1970 + ((millis - offset) / Self::YEAR) as i32;
        let doy_offset = offset % Self::DAY;
        let doy = 1 + ((offset - doy_offset) / Self::DAY) as i32;
        Self { year, doy }
    }

    /// Creates a new Doy, by the give `dayOfYear` and the `year`.
    /// 1 = 1. Jan, 32 = 1. Feb, 0 = 31. Dec (year - 1)  
    pub fn new(doy: i32, year: i32) -> Self {
        let max_doy = 365 + i32::from(Self::is_leapyear(year));
        if doy < 1 {
            Self::new(365 + i32::from(Self::is_leapyear(year - 1)) + doy, year - 1)
        } else if doy > max_doy {
            Self::new(doy - max_doy, year + 1)
        } else {
            Self { year, doy }
        }
    }

    #[inline]
    fn day_per_month(year: i32) -> Vec<i32> {
        let leap = Self::is_leapyear(year) as i32;
        vec![31, 28 + leap, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    }

    /// Creates a Doy from `year`, `month` and `day`.
    ///
    /// # Panics
    /// panics if `month` is not in 1..12
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Self {
        assert!(month > 0 && month < 13, "Month has to be in 1..12");
        let day_of_year = Self::day_per_month(year)
            .iter()
            .take(month as usize - 1)
            .sum::<i32>()
            + day;
        Self::new(day_of_year, year)
    }

    /// Creates a Doy for the Monday of the given week (iso 8601)
    ///
    /// # Panics
    /// panics if `week` is not in 1..53
    pub fn from_week(year: i32, week: i32) -> Self {
        assert!(week > 0 && week < 54, "Week has to be in 1..53");
        // weekday of 4th, Jan.
        let weekday = Self::new(4, year).day_of_week();
        let day_of_year = match weekday {
            Sun => Thu as i32 - 6,
            _ => Thu as i32 + 1 - (weekday as i32),
        } + (week - 1) * 7;
        Self::new(day_of_year, year)
    }

    /// Is the given `year` a leap-year?
    #[inline]
    pub fn is_leapyear(year: i32) -> bool {
        year % 4 == 0 && year % 100 != 0
    }

    /// Is this year a leap-year?
    pub fn leapyear(&self) -> bool {
        Self::is_leapyear(self.year)
    }

    /// converts a *day of year* to `mmdd`.
    fn as_date(&self) -> (i32, i32) {
        let mut doy = self.doy;
        let mut m = 1;
        for ds in Self::day_per_month(self.year) {
            if doy <= ds {
                return (m, doy);
            }
            m += 1;
            doy -= ds;
        }
        (-1, -1)
    }

    /// returns this doy in iso-format `yyyy-mm-dd`.
    pub fn as_iso_date(&self) -> String {
        format!("{self:#}")
    }

    /// Day of Week
    #[inline]
    pub fn day_of_week(&self) -> DayOfWeek {
        let y = self.year % 100;
        let y_off = y + (y / 4) + 6 - self.leapyear() as i32;
        DayOfWeek::from(y_off + self.doy)
    }

    /// The ISO 8601 Weeks start with Monday and end on Sunday. The first week of the year always
    /// contains January 4th. And the first Thursday is always in the first week of the year.
    ///
    /// returns the week in iso-8601-format: `yyyy`-W`ww`
    pub fn iso8601week(&self) -> String {
        let dof = self.day_of_week();
        let thu = match dof {
            Sun => *self + Thu as i32 - 7,
            _ => *self + Thu as i32 - (dof as i32),
        };
        let kw = (thu.doy + 6) / 7;
        format!("{}-W{kw:02}", thu.year)
    }

    /// Returns the day of month.
    pub fn day_of_month(&self) -> i32 {
        self.as_date().1
    }

    /// Returns just the `Month`.
    pub fn month(&self) -> Month {
        Month::from(self.as_date().0)
    }
}

impl From<Doy> for String {
    fn from(doy: Doy) -> Self {
        doy.to_string()
    }
}

impl From<u128> for Doy {
    fn from(value: u128) -> Self {
        Doy::from_millis(value)
    }
}

impl Display for Doy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let (month, day) = self.as_date();
        let year = self.year;
        if f.alternate() {
            write!(f, "{year:04}-{month:02}-{day:02}")
        } else {
            write!(f, "{year:04}{month:02}{day:02}")
        }
    }
}

macro_rules! gen_calcs {
    ($($key:ident),+) => {
    $(
        impl Add<$key> for Doy {
            type Output = Doy;

            fn add(self, rhs: $key) -> Self::Output {
                Doy::new(self.doy + rhs as i32, self.year)
            }
        }

        impl Sub<$key> for Doy {
            type Output = Doy;

            fn sub(self, rhs: $key) -> Self::Output {
                Doy::new(self.doy - rhs as i32, self.year)
            }
        }
    )+
    }
}

gen_calcs!(i8, i16, i32, i64, u8, u16, u32, u64);

impl PartialOrd for Doy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let p = if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        };
        Some(p)
    }

    fn lt(&self, other: &Self) -> bool {
        self.year < other.year || (self.year == other.year && self.doy < other.doy)
    }

    fn le(&self, other: &Self) -> bool {
        self.year < other.year || (self.year == other.year && self.doy <= other.doy)
    }

    fn gt(&self, other: &Self) -> bool {
        self.year > other.year || (self.year == other.year && self.doy > other.doy)
    }

    fn ge(&self, other: &Self) -> bool {
        self.year > other.year || (self.year == other.year && self.doy >= other.doy)
    }
}

impl TryFrom<&str> for Doy {
    type Error = TimeWarpError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let err = |e: ParseIntError| -> Result<i32, TimeWarpError> {
            parse_error(format!("Error converting into number: '{value}'\n{e}",))
        };
        let y = i32::from_str(&value[0..4]).map_err(err)?;
        let (m, d) = if value.len() == 10 && &value[4..5] == "-" && &value[7..8] == "-" {
            (
                i32::from_str(&value[5..7]).map_err(err)?,
                i32::from_str(&value[8..10]).map_err(err)?,
            )
        } else if value.len() == 8 {
            (
                i32::from_str(&value[4..6]).map_err(err)?,
                i32::from_str(&value[6..8]).map_err(err)?,
            )
        } else {
            return parse_error(format!("Wrong date-format: '{value}'"));
        };
        if m < 1 || m > 12 {
            return parse_error(format!("Month out of range 0..12: '{m}'"));
        }
        let days_in_month = Self::day_per_month(y).as_slice()[(m - 1) as usize];
        if d < 1 || d > days_in_month {
            return parse_error(format!(
                "Days exceeded in month {m} '{d}' ({days_in_month})"
            ));
        }
        Ok(Self::from_ymd(y, m, d))
    }
}

/// A timespan in whole days.
///
#[derive(Debug, Eq, PartialEq)]
pub enum Tempus {
    Moment(Doy),
    Interval(Doy, Doy),
}

impl Tempus {
    /// The start-date of this `DaySpan` (inclusive).
    pub fn start(&self) -> Doy {
        match *self {
            Tempus::Moment(d) | Tempus::Interval(d, _) => d,
        }
    }

    /// The end-date of this `DaySpan` (exclusive).
    pub fn end(&self) -> Doy {
        match *self {
            Tempus::Moment(d) => d + 1,
            Tempus::Interval(_, e) => e,
        }
    }
}

#[cfg(test)]
mod should {
    use crate::day_of_week::DayOfWeek::*;
    use crate::doy::Doy;
    use crate::month_of_year::Month;
    use std::convert::TryFrom;

    #[test]
    fn try_from() {
        assert_eq!(
            "2018-01-01",
            Doy::try_from("2018-01-01").unwrap().as_iso_date()
        );
        assert!(Doy::try_from("2018-13-01").is_err());
        assert!(Doy::try_from("2018-02-29").is_err());
        assert!(Doy::try_from("20180431").is_err());
        assert!(Doy::try_from("2018/04/15").is_err());
    }

    #[test]
    fn from_week_of_year() {
        assert_eq!("2018-01-01", Doy::from_week(2018, 1).as_iso_date());
        assert_eq!("2018-12-31", Doy::from_week(2019, 1).as_iso_date());
        assert_eq!("2019-12-30", Doy::from_week(2020, 1).as_iso_date());
        assert_eq!("2021-01-04", Doy::from_week(2021, 1).as_iso_date());
        assert_eq!("2022-01-03", Doy::from_week(2022, 1).as_iso_date());
    }

    #[test]
    fn into_week_of_year() {
        // 1. Rule: 4th of January is always in W01
        assert_eq!("2018-W01", Doy::from_ymd(2018, 1, 4).iso8601week());
        assert_eq!("2019-W01", Doy::from_ymd(2019, 1, 4).iso8601week());
        assert_eq!("2020-W01", Doy::from_ymd(2020, 1, 4).iso8601week());
        assert_eq!("2021-W01", Doy::from_ymd(2021, 1, 4).iso8601week());
        assert_eq!("2022-W01", Doy::from_ymd(2022, 1, 4).iso8601week());
        assert_eq!("2023-W01", Doy::from_ymd(2023, 1, 4).iso8601week());
        assert_eq!("2026-W01", Doy::from_ymd(2026, 1, 4).iso8601week());

        assert_eq!("2018-W01", Doy::from_ymd(2018, 1, 1).iso8601week());
        assert_eq!("2019-W01", Doy::from_ymd(2019, 1, 1).iso8601week());
        assert_eq!("2020-W53", Doy::from_ymd(2021, 1, 1).iso8601week());
        assert_eq!("2021-W52", Doy::from_ymd(2022, 1, 1).iso8601week());

        assert_eq!("2018-W26", Doy::from_ymd(2018, 7, 1).iso8601week());
        assert_eq!("2019-W27", Doy::from_ymd(2019, 7, 1).iso8601week());
        assert_eq!("2020-W27", Doy::from_ymd(2020, 7, 1).iso8601week());
        assert_eq!("2021-W26", Doy::from_ymd(2021, 7, 1).iso8601week());
    }

    #[test]
    fn day_of_month() {
        let test = Doy::from_ymd(2018, 4, 13);
        assert_eq!(test.as_iso_date(), "2018-04-13");
        assert_eq!(test.month(), Month::Apr);

        let test = Doy::from_ymd(2018, 3, 6);
        assert_eq!(test.as_iso_date(), "2018-03-06");
        assert_eq!(test.month(), Month::Mar);
    }

    #[test]
    fn create_by_doy_year() {
        let proof = Doy::new(-7, 2020);
        let test = Doy::new(358, 2019);
        assert_eq!(test, proof);
        let proof = Doy::new(-1, 2020);
        assert_eq!("20191230", proof.to_string());
        let proof = Doy::new(-1, 2021);
        assert_eq!("20201230", proof.to_string());
    }

    #[test]
    fn return_leapyear() {
        assert!(Doy::new(1, 2020).leapyear());
        assert!(!Doy::new(1, 2018).leapyear());
        assert!(!Doy::new(1, 2000).leapyear());
    }

    #[test]
    fn convert_to_string() {
        assert_eq!("20201225", Doy::new(360, 2020).to_string());
        assert_eq!("20181225", Doy::new(359, 2018).to_string());
    }

    #[test]
    fn calc_day_of_week() {
        assert_eq!(Wed, Doy::new(31, 2018).day_of_week());
        assert_eq!(Thu, Doy::new(31, 2019).day_of_week());
        assert_eq!(Fri, Doy::new(31, 2020).day_of_week());
        // Wochentag vom 1. Weihnachtstag 25.12.
        assert_eq!(Tue, Doy::new(359, 2018).day_of_week());
        assert_eq!(Fri, Doy::new(360, 2020).day_of_week());
        assert_eq!(Sat, Doy::new(359, 2021).day_of_week());
    }

    #[test]
    fn create_via_try_from() {
        assert_eq!("20200229", Doy::from_ymd(2020, 2, 29).to_string());
        assert_eq!("19990814", Doy::from_ymd(1999, 8, 14).to_string());
        let d = "20240721";
        assert_eq!(d, &Doy::try_from(d).unwrap().to_string());
    }

    #[test]
    fn order_gt_or_lt() {
        let a = Doy::new(112, 2020);
        let b = Doy::new(225, 2020);
        let c = Doy::new(85, 2021);

        assert!(a < b);
        assert!(c > a);
        assert!(b < c);
        assert!(a >= a);
        assert!(b <= c);
    }

    #[test]
    fn add_i32() {
        let d = Doy::new(15, 2020) + 2;
        assert_eq!(Doy::new(17, 2020), d);
    }

    #[test]
    fn from_millis() {
        assert_eq!("20230317", Doy::from_millis(1679086777511).to_string());
        assert_eq!("20230101", Doy::from_millis(1672570315000).to_string());
        assert_eq!("20181231", Doy::from_millis(1546253515000).to_string());
    }
}
