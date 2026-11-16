//! Pill badge component for statuses and categories.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
}

#[component]
pub fn Badge(
    text: String,
    #[props(default)] variant: BadgeVariant,
) -> Element {
    let cls = match variant {
        BadgeVariant::Default => "badge badge-default",
        BadgeVariant::Primary => "badge badge-primary",
        BadgeVariant::Success => "badge badge-success",
        BadgeVariant::Warning => "badge badge-warning",
        BadgeVariant::Danger => "badge badge-danger",
    };

    rsx! {
        span {
            class: "{cls}",
            "{text}"
        }
    }
}

