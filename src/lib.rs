//! # timewarp
//! NLP library for parsing English and German natural language into dates and times.
//! Leverages [pest](https://pest.rs) for parsing human readable-dates and times.
//!
//!
//! Should parse
//! ```rust
//! use timewarp::Direction::*;
//! use timewarp::{date_matcher, Doy, DaySpan};
//!
//! // Fri 2023-03-17
//! let today = Doy::from_ymd(2023, 3, 17);
//! // Date as used in German (d.m.y)
//! assert_eq!(
//!     date_matcher(today, StartTime, "22.1.23").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 1, 22))
//! );
//! assert_eq!(
//!     date_matcher(today, StartTime, "22.1.").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 1, 22))
//! );
//! // Date as common for english-speaker m/d/y
//! assert_eq!(
//!     date_matcher(today, StartTime, "3/16/2023").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 16))
//! );
//! // Date written in ISO
//! assert_eq!(
//!     date_matcher(today, StartTime, "2023-03-16").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 16))
//! );
//!
//! assert_eq!(
//!     date_matcher(today, EndTime, "last monday").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 13))
//! );
//! assert_eq!(
//!     date_matcher(today, StartTime, "tuesday").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 14))
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "tuesday").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 21))
//! );
//! assert_eq!(
//!     date_matcher(today, StartTime, "letzten donnerstag").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 16))
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "last friday").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 10))
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "nächsten Fr").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 24))
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "coming Thu").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 23))
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "übernächsten Donnerstag").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 30))
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "nächster Mo").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 20))
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "vorletzter mo").unwrap(),
//!     DaySpan::Doy(Doy::from_ymd(2023, 3, 6))
//! );
//! ```

#[macro_use]
extern crate pest_derive;

mod date_matcher;
mod day_of_week;
mod doy;
mod error;
mod month_of_year;

pub use date_matcher::{date_matcher, Direction};
pub use day_of_week::DayOfWeek;
pub use doy::{DaySpan, Doy};
pub use error::TimeWarpError;
pub use month_of_year::Month;
