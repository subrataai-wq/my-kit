//! my-kit-ui: Dioxus 0.7 component kit, design tokens, and reactive hooks.

pub mod state;
pub mod ui;

pub const MAIN_CSS: &str = include_str!("../assets/main.css");

pub mod prelude {
    pub use super::state::prelude::*;
    pub use super::ui::prelude::*;
}
