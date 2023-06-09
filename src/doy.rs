use crate::day_of_week::DayOfWeek;
use crate::month_of_year::Month;
use crate::DayOfWeek::{Sun, Thu};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Sub};
use std::time::SystemTime;

/// Day Of Year. Helper-class to easily calculate dates.
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
        let doyffset = offset % Self::DAY;
        let doy = 1 + ((offset - doyffset) / Self::DAY) as i32;
        Self { year, doy }
    }

    /// creates a new Doy, by the give `dayOfYear` and the `year`.
    /// 1 = 1. Jan, 32 = 1. Feb, 0 = 31. Dec year - 1  
    pub fn new(doy: i32, year: i32) -> Self {
        let max_doy = 365 + Self::is_leapyear(year) as i32;
        if doy < 1 {
            Self::new(365 + Self::is_leapyear(year - 1) as i32 + doy, year - 1)
        } else if doy > max_doy {
            Self::new(doy - max_doy, year + 1)
        } else {
            Self { doy, year }
        }
    }

    #[inline]
    fn day_per_month(year: i32) -> Vec<i32> {
        let leap = Self::is_leapyear(year) as i32;
        vec![31, 28 + leap, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    }

    /// Creates a Doy from `year`, `month` and `day`.
    pub fn from_ymd(year: i32, m: i32, d: i32) -> Self {
        assert!(m > 0 && m < 13, "Month has to be in 1..12");
        let doy = Self::day_per_month(year)
            .iter()
            .take(m as usize - 1)
            .sum::<i32>()
            + d;
        Self::new(doy, year)
    }

    /// Creates a Doy for the Monday of the given week (iso 8601)
    pub fn from_week(year: i32, week: i32) -> Self {
        assert!(week > 0 && week < 54, "Week has to be in 1..53");
        let weekday = Self::new(4, year).day_of_week();
        let doy = match weekday {
            Sun => Thu as i32 - 6,
            _ => Thu as i32 + 1 - (weekday as i32),
        } + (week - 1) * 7;
        Self::new(doy, year)
    }

    /// Is the given `year` a leap-year?
    #[inline]
    pub fn is_leapyear(year: i32) -> bool {
        year % 4 == 0 && year % 100 != 0
    }

    /// Is this year a leap-year?.
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

impl Display for Doy {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), Error> {
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

impl std::cmp::PartialOrd for Doy {
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
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let m = &format!("Tried to convert: '{value}'");
        Ok(if &value[4..5] == "-" {
            Self::from_ymd(
                i32::from_str(&value[0..4]).expect(m),
                i32::from_str(&value[5..7]).expect(m),
                i32::from_str(&value[8..10]).expect(m),
            )
        } else {
            Self::from_ymd(
                i32::from_str(&value[0..4]).expect(m),
                i32::from_str(&value[4..6]).expect(m),
                i32::from_str(&value[6..8]).expect(m),
            )
        })
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
    /// The start-date of this DaySpan (inclusive)
    pub fn start(&self) -> Doy {
        use Tempus::*;
        match *self {
            Moment(d) => d,
            Interval(d, _) => d,
        }
    }

    /// The end-date of this DaySpan (exclusive)
    pub fn end(&self) -> Doy {
        use Tempus::*;
        match *self {
            Moment(d) => d + 1,
            Interval(_, e) => e,
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
