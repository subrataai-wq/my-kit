//! my-kit-core: Headless core primitives, algorithms, forms, media, and auth for the studio.

pub use compact_str::{self, CompactString};
pub type CompactStr = compact_str::CompactString;
pub use thiserror;
pub use ulid::{self, Ulid};

pub mod base;
pub mod checksum;
pub mod data;
pub mod net;
pub mod platform;

#[cfg(feature = "forms")]
pub mod forms;

#[cfg(feature = "media")]
pub mod media;

#[cfg(feature = "auth")]
pub mod auth;

pub mod prelude {
    pub use compact_str::CompactString;
    pub use super::CompactStr;
    pub use ulid::Ulid;
    pub use super::base::prelude::*;
    pub use super::checksum::prelude::*;
    pub use super::data::prelude::*;
    pub use super::net::prelude::*;
    pub use super::platform::prelude::*;
    #[cfg(feature = "forms")]
    pub use super::forms::prelude::*;
    #[cfg(feature = "media")]
    pub use super::media::prelude::*;
    #[cfg(feature = "auth")]
    pub use super::auth::prelude::*;
}


