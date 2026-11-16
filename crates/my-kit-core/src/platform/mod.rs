//! Platform and operating system utilities.

pub mod env;
pub mod fs;
pub mod paths;
pub mod storage;

pub mod prelude {
    pub use super::env::{get, get_bool, get_or, load_dotenv, parse_dotenv, parse_or, require, require_parse};
    pub use super::fs::{dir_size, ensure_dir, extension, file_stem, human_size, is_hidden, list_files, read_text, write_atomic};
    pub use super::paths::{app_config_dir, app_data_dir, normalize_path, sanitize_filename};
    #[cfg(not(target_arch = "wasm32"))]
    pub use super::storage::FileStorage;
    #[cfg(target_arch = "wasm32")]
    pub use super::storage::LocalStorage;
    pub use super::storage::{KeyValueStore, MemoryStorage};
}
