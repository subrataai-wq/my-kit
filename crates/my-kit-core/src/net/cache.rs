//! Deterministic in-memory TTL cache with explicit time passing. Safe across all targets including wasm.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct Entry<V> {
    value: V,
    expires_at: u64,
}

#[derive(Debug, Clone, Default)]
pub struct TtlCache<K: Eq + Hash, V> {
    store: HashMap<K, Entry<V>>,
}

impl<K: Eq + Hash, V> TtlCache<K, V> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// Insert an item with an explicit unix timestamp expiration time `expires_at`.
    pub fn insert(&mut self, key: K, value: V, expires_at: u64) {
        self.store.insert(key, Entry { value, expires_at });
    }

    /// Get item if present and not expired at `now`.
    pub fn get(&self, key: &K, now: u64) -> Option<&V> {
        let entry = self.store.get(key)?;
        if entry.expires_at >= now {
            Some(&entry.value)
        } else {
            None
        }
    }

    /// Remove any entries that have expired relative to `now`.
    pub fn prune(&mut self, now: u64) {
        self.store.retain(|_, entry| entry.expires_at >= now);
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key).map(|e| e.value)
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_evicts_expired_cleanly() {
        let mut cache = TtlCache::new();
        cache.insert("token", "secret_123", 100);

        // Before expiry
        assert_eq!(cache.get(&"token", 50), Some(&"secret_123"));
        assert_eq!(cache.get(&"token", 100), Some(&"secret_123"));

        // After expiry
        assert_eq!(cache.get(&"token", 101), None);

        // Pruning removes it completely
        cache.prune(101);
        assert_eq!(cache.len(), 0);
    }
}

