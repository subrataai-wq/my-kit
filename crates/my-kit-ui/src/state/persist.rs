//! Reactive persistent state hook integrated with KeyValueStore and NoticeManager.

use dioxus::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

#[cfg(not(target_arch = "wasm32"))]
use my_kit_core::platform::paths::app_data_dir;
#[cfg(not(target_arch = "wasm32"))]
use my_kit_core::platform::storage::FileStorage;
use my_kit_core::platform::storage::KeyValueStore;
#[cfg(target_arch = "wasm32")]
use my_kit_core::platform::storage::LocalStorage;

use crate::state::notice::use_notices;

#[cfg(not(target_arch = "wasm32"))]
fn get_default_store() -> Option<Box<dyn KeyValueStore>> {
    let storage_dir = app_data_dir("studio_app").join("storage");
    FileStorage::new(&storage_dir)
        .ok()
        .map(|s| Box::new(s) as Box<dyn KeyValueStore>)
}

#[cfg(target_arch = "wasm32")]
fn get_default_store() -> Option<Box<dyn KeyValueStore>> {
    Some(Box::new(LocalStorage::new()) as Box<dyn KeyValueStore>)
}

/// Reactive hook that initializes state from local storage, auto-syncs mutations,
/// and reports any I/O errors directly through `NoticeManager`.
pub fn use_persistent<T>(key: &'static str, init: impl FnOnce() -> T) -> Signal<T>
where
    T: Serialize + DeserializeOwned + Clone + PartialEq + 'static,
{
    let notices = use_notices();
    let storage = get_default_store();

    let initial_value = storage
        .as_ref()
        .and_then(|s| s.get(key).ok().flatten())
        .and_then(|raw| serde_json::from_str::<T>(&raw).ok())
        .unwrap_or_else(init);

    let signal = use_signal(|| initial_value);

    // Reactively watch signal changes and persist to disk/local storage
    use_effect(move || {
        let current_val = (signal)();
        if let Some(store) = &storage {
            match serde_json::to_string(&current_val) {
                Ok(serialized) => {
                    if let Err(e) = store.set(key, &serialized) {
                        notices.error(format!("Failed to save '{key}': {e}"));
                    }
                }
                Err(e) => {
                    notices.error(format!("Serialization error for '{key}': {e}"));
                }
            }
        }
    });

    signal
}
