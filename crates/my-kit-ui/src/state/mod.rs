//! Reactive state hooks for Dioxus 0.7.

pub mod auth;
pub mod debounce;
pub mod form;
pub mod notice;
pub mod persist;
pub mod use_now;

pub mod prelude {
    pub use super::auth::{use_auth, use_has_role, use_init_auth, use_is_authenticated};
    pub use super::debounce::Debouncer;
    pub use super::form::use_form;
    pub use super::notice::{Notice, NoticeLevel, NoticeManager, use_notices};
    pub use super::persist::use_persistent;
    pub use super::use_now::use_now;
}
