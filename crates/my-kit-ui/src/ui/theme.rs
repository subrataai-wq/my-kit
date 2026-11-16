//! Theme management using CSS variables and context provider.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// Provides or consumes the reactive application theme mode.
pub fn use_theme() -> Signal<ThemeMode> {
    use_context_provider(|| Signal::new(ThemeMode::Light))
}

