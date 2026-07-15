//! Julian Day conversion utilities.
//!
//! M4 implements only Julian Day conversion and validation. It does not
//! introduce lunisolar calendar rules or astronomical position algorithms.

mod algorithms;
mod conversion;
mod validation;

pub use conversion::{gregorian_to_jd, jd_to_gregorian, weekday, Weekday};
pub use validation::is_leap_year;
