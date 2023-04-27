//! # timewarp
//! NLP library for parsing English and German natural language into dates and times.
//! Leverages [pest](https://pest.rs) for parsing human readable-dates and times.
//!
//!
//! Should parse
//! ```rust
//! use timewarp::Direction::*;
//! use timewarp::{date_matcher, Doy, Tempus};
//!
//! // Fri 2023-03-17
//! let today = Doy::from_ymd(2023, 3, 17);
//! // Date as used in German (d.m.y)
//! assert_eq!(
//!     date_matcher(today, From, "22.1.23").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 1, 22))
//! );
//! assert_eq!(
//!     date_matcher(today, From, "22.1.").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 1, 22))
//! );
//! // Date as common for english-speaker m/d/y
//! assert_eq!(
//!     date_matcher(today, From, "3/16/2023").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 16))
//! );
//! // Date written in ISO
//! assert_eq!(
//!     date_matcher(today, From, "2023-03-16").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 16))
//! );
//!
//! assert_eq!(
//!     date_matcher(today, To, "last monday").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 13))
//! );
//! assert_eq!(
//!     date_matcher(today, From, "tuesday").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 14))
//! );
//! assert_eq!(
//!     date_matcher(today, To, "tuesday").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 21))
//! );
//! assert_eq!(
//!     date_matcher(today, From, "letzten donnerstag").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 16))
//! );
//! assert_eq!(
//!     date_matcher(today, To, "last friday").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 10))
//! );
//! assert_eq!(
//!     date_matcher(today, To, "nächsten Fr").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 24))
//! );
//! assert_eq!(
//!     date_matcher(today, To, "coming Thu").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 23))
//! );
//! assert_eq!(
//!     date_matcher(today, To, "übernächsten Donnerstag").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 30))
//! );
//! assert_eq!(
//!     date_matcher(today, To, "nächster Mo").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 20))
//! );
//! assert_eq!(
//!     date_matcher(today, To, "vorletzter mo").unwrap(),
//!     Tempus::Moment(Doy::from_ymd(2023, 3, 6))
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
pub use doy::{Doy, Tempus};
pub use error::TimeWarpError;
pub use month_of_year::Month;
