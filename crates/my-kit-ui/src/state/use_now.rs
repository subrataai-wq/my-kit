//! Live reactive clock hook.

use dioxus::prelude::*;
use my_kit_core::base::time::now_unix_secs;

/// Returns a signal that holds the current unix timestamp.
pub fn use_now() -> Signal<u64> {
    let mut now = use_signal(now_unix_secs);
    use_hook(move || {
        spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                now.set(now_unix_secs());
            }
        });
    });
    now
}
