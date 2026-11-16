//! Accessible dialog modal with keyboard and backdrop dismissal.

use dioxus::prelude::*;

#[component]
pub fn Modal(
    open: bool,
    title: String,
    on_close: EventHandler<()>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        div {
            class: "modal-backdrop",
            tabindex: 0,
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    on_close.call(());
                }
            },
            onclick: move |_| {
                on_close.call(());
            },
            div {
                class: "modal-card",
                role: "dialog",
                aria_modal: "true",
                onclick: move |e| {
                    e.stop_propagation();
                },
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; border-bottom: 1px solid var(--border); padding-bottom: 12px;",
                    h2 {
                        style: "font-size: 18px; font-weight: 600; margin: 0; color: var(--text-main);",
                        "{title}"
                    }
                    button {
                        class: "btn btn-ghost btn-sm",
                        style: "font-size: 16px; padding: 2px 8px; line-height: 1;",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }
                div {
                    {children}
                }
            }
        }
    }
}

