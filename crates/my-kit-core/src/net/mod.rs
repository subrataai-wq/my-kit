//! Network resilience, caching, and request utilities.

pub mod cache;
pub mod http;
pub mod retry;

pub mod prelude {
    pub use super::cache::TtlCache;
    pub use super::http::{HttpMethod, QueryParams};
    pub use super::retry::{retry_async, RetryPolicy};
}

