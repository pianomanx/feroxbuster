//! contains all of feroxbuster's filters
use std::any::Any;
use std::fmt::Debug;

use crate::ferox_response::FeroxResponse;
use crate::traits::{FeroxFilter, FeroxSerialize};

pub use self::container::FeroxFilters;
pub use self::lines::LinesFilter;
pub use self::regex::RegexFilter;
pub use self::similarity::SimilarityFilter;
pub use self::size::SizeFilter;
pub use self::status_code::StatusCodeFilter;
pub use self::wildcard::WildcardFilter;
pub use self::words::WordsFilter;

mod wildcard;
mod status_code;
mod words;
mod lines;
mod size;
mod regex;
mod similarity;
mod container;
#[cfg(test)]
mod tests;
