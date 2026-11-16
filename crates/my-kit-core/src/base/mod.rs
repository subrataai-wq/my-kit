//! Core foundational utilities with zero external dependencies.

pub mod collections;
pub mod id;
pub mod math;
pub mod result;
pub mod text;
pub mod time;
pub mod validate;

pub mod prelude {
    pub use super::collections::{chunk, dedupe, find_by_id, frequency_map, partition, remove_by_id, toggle_by, Progress};
    pub use super::id::{random_range, random_u64, short_id, unique_id};
    pub use super::math::{clamp, lerp, map_range, percent, round_to};
    pub use super::result::{AppError, AppResult, Context, ErrorKind};
    pub use super::text::{capitalize, collapse_whitespace, is_blank, pluralize, truncate};
    pub use super::time::{format_duration, human_ago, now_unix_secs, parse_duration, unix_to_iso8601};
    pub use super::validate::{in_range, is_email, is_http_url, is_username};
}

