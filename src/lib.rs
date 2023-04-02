//! # timewarp
//! NLP library for parsing English and German natural language into dates and times.
//! Leverages [pest](https://pest.rs) for parsing human readable-dates and times.
//!
//!
//! Should parse
//! ```rust
//! use timewarp::Direction::*;
//! use timewarp::{date_matcher, Doy};
//!
//! // Fri 2023-03-17
//! let today = Doy::from_ymd(2023, 3, 17);
//! // Date as used in German (d.m.y)
//! assert_eq!(
//!     date_matcher(today, StartTime, "22.1.23").unwrap(),
//!     "2023-01-22"
//! );
//! assert_eq!(
//!     date_matcher(today, StartTime, "22.1.").unwrap(),
//!     "2023-01-22"
//! );
//! // Date as common for english-speaker m/d/y
//! assert_eq!(
//!     date_matcher(today, StartTime, "3/16/2023").unwrap(),
//!     "2023-03-16"
//! );
//! // Date written in ISO
//! assert_eq!(
//!     date_matcher(today, StartTime, "2023-03-16").unwrap(),
//!     "2023-03-16"
//! );
//!
//! assert_eq!(
//!     date_matcher(today, EndTime, "last monday").unwrap(),
//!     "2023-03-13"
//! );
//! assert_eq!(
//!     date_matcher(today, StartTime, "tuesday").unwrap(),
//!     "2023-03-14"
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "tuesday").unwrap(),
//!     "2023-03-21"
//! );
//! assert_eq!(
//!     date_matcher(today, StartTime, "letzten donnerstag").unwrap(),
//!     "2023-03-16"
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "last friday").unwrap(),
//!     "2023-03-10"
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "nächsten Fr").unwrap(),
//!     "2023-03-24"
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "coming Thu").unwrap(),
//!     "2023-03-23"
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "übernächsten Donnerstag").unwrap(),
//!     "2023-03-30"
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "nächster Mo").unwrap(),
//!     "2023-03-20"
//! );
//! assert_eq!(
//!     date_matcher(today, EndTime, "vorletzter mo").unwrap(),
//!     "2023-03-06"
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
pub use doy::Doy;
pub use error::TimeWarpError;
pub use month_of_year::Month;
