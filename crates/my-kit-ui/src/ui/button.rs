//! Accessible button component with variant and size presets.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Outline,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let var_cls = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Outline => "btn-outline",
        ButtonVariant::Ghost => "btn-ghost",
    };
    let size_cls = match size {
        ButtonSize::Sm => "btn-sm",
        ButtonSize::Md => "btn-md",
        ButtonSize::Lg => "btn-lg",
    };

    rsx! {
        button {
            class: "btn {var_cls} {size_cls}",
            disabled,
            onclick: move |e| {
                if !disabled
                    && let Some(handler) = onclick
                {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}
