//! Abstract key-value storage implementations for local persistence.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

use crate::base::result::{AppError, AppResult};

/// Strongly typed persistence errors.
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Storage key not found: {0}")]
    NotFound(String),
    #[error("Internal storage lock was poisoned: {0}")]
    LockPoisoned(String),
    #[error("Storage I/O failure: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization failure: {0}")]
    Serialization(String),
    #[error("Storage engine unavailable in current context (e.g., Worker/SSR)")]
    Unavailable,
}

impl From<StorageError> for AppError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::NotFound(msg) => AppError::not_found(msg),
            StorageError::Io(e) => AppError::from(e),
            other => AppError::invalid(other.to_string()),
        }
    }
}


#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

#[cfg(not(target_arch = "wasm32"))]
use crate::base::result::Context;
#[cfg(not(target_arch = "wasm32"))]
use crate::platform::fs;
#[cfg(not(target_arch = "wasm32"))]
use crate::platform::paths::sanitize_filename;

/// Generic Key-Value persistence contract.
pub trait KeyValueStore: Send + Sync {
    fn get(&self, key: &str) -> AppResult<Option<String>>;
    fn set(&self, key: &str, value: &str) -> AppResult<()>;
    fn remove(&self, key: &str) -> AppResult<()>;
    fn clear(&self) -> AppResult<()>;
}

/// In-memory storage backed by `Arc<RwLock<HashMap>>`.
#[derive(Debug, Default, Clone)]
pub struct MemoryStorage {
    map: Arc<RwLock<HashMap<String, String>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl KeyValueStore for MemoryStorage {
    fn get(&self, key: &str) -> AppResult<Option<String>> {
        let lock = self
            .map
            .read()
            .map_err(|_| StorageError::LockPoisoned("MemoryStorage read lock".into()))?;
        Ok(lock.get(key).cloned())
    }

    fn set(&self, key: &str, value: &str) -> AppResult<()> {
        let mut lock = self
            .map
            .write()
            .map_err(|_| StorageError::LockPoisoned("MemoryStorage write lock".into()))?;
        lock.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn remove(&self, key: &str) -> AppResult<()> {
        let mut lock = self
            .map
            .write()
            .map_err(|_| StorageError::LockPoisoned("MemoryStorage write lock".into()))?;
        lock.remove(key);
        Ok(())
    }

    fn clear(&self) -> AppResult<()> {
        let mut lock = self
            .map
            .write()
            .map_err(|_| StorageError::LockPoisoned("MemoryStorage write lock".into()))?;
        lock.clear();
        Ok(())
    }
}

/// Persistent file storage storing each key as an atomic file inside a directory.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct FileStorage {
    root_dir: PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileStorage {
    pub fn new(root_dir: impl AsRef<Path>) -> AppResult<Self> {
        let root = root_dir.as_ref().to_path_buf();
        fs::ensure_dir(&root).context("Failed to initialize storage directory")?;
        Ok(Self { root_dir: root })
    }

    fn key_path(&self, key: &str) -> PathBuf {
        let safe_name = format!("{}.val", sanitize_filename(key));
        self.root_dir.join(safe_name)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl KeyValueStore for FileStorage {
    fn get(&self, key: &str) -> AppResult<Option<String>> {
        let path = self.key_path(key);
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_text(&path).context("Failed to read storage key")?;
        Ok(Some(content))
    }

    fn set(&self, key: &str, value: &str) -> AppResult<()> {
        let path = self.key_path(key);
        fs::write_atomic(&path, value.as_bytes()).context("Failed to persist storage key")?;
        Ok(())
    }

    fn remove(&self, key: &str) -> AppResult<()> {
        let path = self.key_path(key);
        if path.exists() {
            std::fs::remove_file(&path).context("Failed to remove storage key")?;
        }
        Ok(())
    }

    fn clear(&self) -> AppResult<()> {
        if self.root_dir.exists() {
            for entry in std::fs::read_dir(&self.root_dir).context("Reading storage directory")? {
                let entry = entry.context("Storage entry error")?;
                let path = entry.path();
                if path.is_file() {
                    let _ = std::fs::remove_file(path);
                }
            }
        }
        Ok(())
    }
}

/// Browser `localStorage` wrapper with panic-free context checking.
///
/// NOTE: LocalStorage is limited to ~5 MB per origin. Suitable for tokens and settings.
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Default, Clone, Copy)]
pub struct LocalStorage;

#[cfg(target_arch = "wasm32")]
impl LocalStorage {
    pub fn new() -> Self {
        Self
    }

    fn get_storage() -> Result<web_sys::Storage, StorageError> {
        let window = web_sys::window().ok_or(StorageError::Unavailable)?;
        window
            .local_storage()
            .map_err(|_| StorageError::Unavailable)?
            .ok_or(StorageError::Unavailable)
    }
}

#[cfg(target_arch = "wasm32")]
impl KeyValueStore for LocalStorage {
    fn get(&self, key: &str) -> AppResult<Option<String>> {
        let storage = Self::get_storage()?;
        storage
            .get_item(key)
            .map_err(|_| StorageError::Unavailable.into())
    }

    fn set(&self, key: &str, value: &str) -> AppResult<()> {
        let storage = Self::get_storage()?;
        storage
            .set_item(key, value)
            .map_err(|_| StorageError::Unavailable.into())
    }

    fn remove(&self, key: &str) -> AppResult<()> {
        let storage = Self::get_storage()?;
        storage
            .remove_item(key)
            .map_err(|_| StorageError::Unavailable.into())
    }

    fn clear(&self) -> AppResult<()> {
        let storage = Self::get_storage()?;
        storage
            .clear()
            .map_err(|_| StorageError::Unavailable.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_storage_crud() {
        let store = MemoryStorage::new();
        assert_eq!(store.get("foo").unwrap(), None);

        store.set("foo", "bar").unwrap();
        assert_eq!(store.get("foo").unwrap(), Some("bar".into()));

        store.remove("foo").unwrap();
        assert_eq!(store.get("foo").unwrap(), None);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn file_storage_crud() {
        let tmp = std::env::temp_dir().join(format!("test_store_{}", std::process::id()));
        let store = FileStorage::new(&tmp).unwrap();

        assert_eq!(store.get("alpha").unwrap(), None);
        store.set("alpha", "123").unwrap();
        assert_eq!(store.get("alpha").unwrap(), Some("123".into()));

        store.remove("alpha").unwrap();
        assert_eq!(store.get("alpha").unwrap(), None);

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
