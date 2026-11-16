//! Collection utilities and list helpers. Zero external dependencies.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Split a slice into chunks of `size`.
/// If `size == 0` or slice is empty, returns an empty Vec without panicking.
pub fn chunk<T: Clone>(slice: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 || slice.is_empty() {
        return Vec::new();
    }
    slice.chunks(size).map(|c| c.to_vec()).collect()
}

/// Deduplicate items while preserving their first occurrence order.
pub fn dedupe<T: Clone + Eq + Hash>(items: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in items {
        if seen.insert(item.clone()) {
            out.push(item.clone());
        }
    }
    out
}

/// Split items into two collections based on a predicate: (matching, not_matching).
pub fn partition<T, F: Fn(&T) -> bool>(items: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>) {
    let mut trues = Vec::new();
    let mut falses = Vec::new();
    for item in items {
        if predicate(&item) {
            trues.push(item);
        } else {
            falses.push(item);
        }
    }
    (trues, falses)
}

/// Count occurrences of each distinct element.
pub fn frequency_map<T: Clone + Eq + Hash>(items: &[T]) -> HashMap<T, usize> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
}

/// Toggle or mutate the first matching element in a mutable slice.
/// Returns true if an item was found and modified.
pub fn toggle_by<T, F: Fn(&T) -> bool>(
    items: &mut [T],
    predicate: F,
    toggle_fn: impl FnOnce(&mut T),
) -> bool {
    if let Some(item) = items.iter_mut().find(|i| predicate(i)) {
        toggle_fn(item);
        true
    } else {
        false
    }
}

/// Remove the first element matching `key_fn(item) == key`.
/// Returns true if an item was removed.
pub fn remove_by_id<T, K: PartialEq, F: Fn(&T) -> K>(
    items: &mut Vec<T>,
    key: &K,
    key_fn: F,
) -> bool {
    if let Some(idx) = items.iter().position(|item| &key_fn(item) == key) {
        items.remove(idx);
        true
    } else {
        false
    }
}

/// Find a reference to the first element where `key_fn(item) == key`.
pub fn find_by_id<'a, T, K: PartialEq, F: Fn(&T) -> K>(
    items: &'a [T],
    key: &K,
    key_fn: F,
) -> Option<&'a T> {
    items.iter().find(|item| &key_fn(item) == key)
}

/// Progress metrics for tasks or checklist items.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Progress {
    pub total: usize,
    pub completed: usize,
    pub percent: f64,
}

impl Progress {
    pub fn new(total: usize, completed: usize) -> Self {
        let completed = completed.min(total);
        let percent = if total == 0 {
            0.0
        } else {
            ((completed as f64 / total as f64) * 100.0 * 10.0).round() / 10.0
        };
        Self {
            total,
            completed,
            percent,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.total > 0 && self.completed >= self.total
    }

    pub fn remaining(&self) -> usize {
        self.total.saturating_sub(self.completed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunking_handles_zero_and_values() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(chunk(&data, 0), Vec::<Vec<i32>>::new());
        assert_eq!(chunk(&data, 2), vec![vec![1, 2], vec![3, 4], vec![5]]);
        assert_eq!(chunk::<i32>(&[], 3), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn dedupe_preserves_order() {
        let input = vec![3, 1, 3, 2, 1, 4];
        assert_eq!(dedupe(&input), vec![3, 1, 2, 4]);
    }

    #[test]
    fn partition_works() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let (even, odd) = partition(nums, |n| n % 2 == 0);
        assert_eq!(even, vec![2, 4, 6]);
        assert_eq!(odd, vec![1, 3, 5]);
    }

    #[test]
    fn frequency_map_counts() {
        let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
        let counts = frequency_map(&words);
        assert_eq!(counts.get(&"apple"), Some(&3));
        assert_eq!(counts.get(&"banana"), Some(&2));
        assert_eq!(counts.get(&"cherry"), Some(&1));
    }

    #[derive(Clone, PartialEq, Debug)]
    struct Item {
        id: String,
        active: bool,
    }

    #[test]
    fn toggle_and_remove_by_id() {
        let mut items = vec![
            Item {
                id: "a".into(),
                active: false,
            },
            Item {
                id: "b".into(),
                active: true,
            },
        ];

        let found = toggle_by(&mut items, |i| i.id == "a", |i| i.active = !i.active);
        assert!(found);
        assert!(items[0].active);

        let removed = remove_by_id(&mut items, &"a".to_string(), |i| i.id.clone());
        assert!(removed);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "b");

        let missing = remove_by_id(&mut items, &"xyz".to_string(), |i| i.id.clone());
        assert!(!missing);
    }

    #[test]
    fn progress_calculation() {
        let p = Progress::new(4, 2);
        assert_eq!(p.percent, 50.0);
        assert_eq!(p.remaining(), 2);
        assert!(!p.is_complete());

        let complete = Progress::new(5, 5);
        assert!(complete.is_complete());
        assert_eq!(complete.percent, 100.0);

        let empty = Progress::new(0, 0);
        assert_eq!(empty.percent, 0.0);
        assert!(!empty.is_complete());
    }
}

