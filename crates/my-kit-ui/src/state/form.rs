//! Reactive form state management hook.

use dioxus::prelude::*;

/// Creates a reactive form state container.
pub fn use_form<T: 'static>(init: impl FnOnce() -> T) -> Signal<T> {
    use_signal(init)
}

