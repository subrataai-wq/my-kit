//! Empty state placeholder component.

use dioxus::prelude::*;

#[component]
pub fn EmptyState(
    title: String,
    subtitle: Option<String>,
    action: Option<Element>,
) -> Element {
    rsx! {
        div {
            style: "text-align: center; padding: 48px 20px; border: 2px dashed var(--border); border-radius: var(--radius-lg); background: var(--bg-surface);",
            h3 {
                style: "font-size: 16px; font-weight: 600; margin: 0 0 6px 0; color: var(--text-main);",
                "{title}"
            }
            if let Some(sub) = subtitle {
                p {
                    style: "font-size: 13px; color: var(--text-muted); margin: 0 0 16px 0;",
                    "{sub}"
                }
            }
            if let Some(act) = action {
                div {
                    style: "margin-top: 12px;",
                    {act}
                }
            }
        }
    }
}

