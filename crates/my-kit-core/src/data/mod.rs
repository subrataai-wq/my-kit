//! Data manipulation, parsing, querying, and pagination utilities.

pub mod csv;
#[cfg(feature = "json")]
pub mod json;
pub mod paginate;
pub mod search;
pub mod sort;

pub mod prelude {
    pub use super::csv::{parse_csv, to_csv};
    #[cfg(feature = "json")]
    pub use super::json::{from_file, from_str, to_file, to_string, to_string_pretty};
    pub use super::paginate::Pagination;
    pub use super::search::{filter_by, fuzzy_match, fuzzy_score};
    pub use super::sort::{natural_cmp, sort_by_key, sort_natural, SortOrder};
}

