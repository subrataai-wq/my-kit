//! Banner alert component for status notifications.

use crate::state::notice::NoticeLevel;
use dioxus::prelude::*;

#[component]
pub fn Banner(
    level: NoticeLevel,
    message: String,
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    let cls = match level {
        NoticeLevel::Info => "banner banner-info",
        NoticeLevel::Success => "banner banner-success",
        NoticeLevel::Warning => "banner banner-warning",
        NoticeLevel::Error => "banner banner-error",
    };

    rsx! {
        div {
            class: "{cls}",
            span { "{message}" }
            if let Some(dismiss) = on_dismiss {
                button {
                    class: "btn btn-ghost btn-sm",
                    style: "padding: 2px 6px; font-weight: bold;",
                    onclick: move |_| dismiss.call(()),
                    "×"
                }
            }
        }
    }
}
