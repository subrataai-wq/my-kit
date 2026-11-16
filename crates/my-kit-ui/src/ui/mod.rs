//! Presentation components styled with CSS variables.

pub mod auth_guard;
pub mod badge;
pub mod banner;
pub mod button;
pub mod empty_state;
pub mod form_input;
pub mod modal;
pub mod progress_bar;
pub mod theme;

pub mod prelude {
    pub use super::auth_guard::AuthGuard;
    pub use super::badge::{Badge, BadgeVariant};
    pub use super::banner::Banner;
    pub use super::button::{Button, ButtonSize, ButtonVariant};
    pub use super::empty_state::EmptyState;
    pub use super::form_input::{FormField, TextInput};
    pub use super::modal::Modal;
    pub use super::progress_bar::ProgressBar;
    pub use super::theme::{ThemeMode, use_theme};
}
