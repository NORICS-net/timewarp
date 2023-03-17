//! # timewarp
//! NLP library for parsing English and German natural language into dates and times.
//! Leverages [pest](https://pest.rs) for parsing human readable-dates and times.
//!
//!
//! Should parse
//! ```rust
//!
//! ```

#[macro_use]
extern crate pest_derive;

mod date_matcher;
mod doy;
mod error;

pub use date_matcher::date_matcher;
pub use error::TimeWarpError;
