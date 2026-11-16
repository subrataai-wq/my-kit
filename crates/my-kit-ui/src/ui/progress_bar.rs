//! Visual progress bar component driven by Progress metrics.

use dioxus::prelude::*;
use my_kit_core::base::collections::Progress;

#[component]
pub fn ProgressBar(progress: Progress) -> Element {
    let pct = progress.percent;
    rsx! {
        div {
            style: "width: 100%; margin-bottom: 16px;",
            div {
                style: "display: flex; justify-content: space-between; font-size: 12px; color: var(--text-muted); font-weight: 600; margin-bottom: 6px;",
                span { "Completed: {progress.completed}/{progress.total}" }
                span { "{pct}%" }
            }
            div {
                style: "background: var(--border); border-radius: var(--radius-full); height: 8px; overflow: hidden;",
                div {
                    style: "background: var(--primary); height: 100%; width: {pct}%; transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);",
                }
            }
        }
    }
}

