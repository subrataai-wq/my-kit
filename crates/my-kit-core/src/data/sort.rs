//! Natural alphanumeric sorting and multi-criteria sort utilities.

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn is_ascending(&self) -> bool {
        matches!(self, Self::Ascending)
    }
}

/// Natural alphanumeric comparison where numeric segments are compared by numerical value.
/// Example: `"file2.txt"` < `"file10.txt"`.
pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    let mut a_chars = a.chars().peekable();
    let mut b_chars = b.chars().peekable();

    while let (Some(&ac), Some(&bc)) = (a_chars.peek(), b_chars.peek()) {
        if ac.is_ascii_digit() && bc.is_ascii_digit() {
            let a_num = extract_number(&mut a_chars);
            let b_num = extract_number(&mut b_chars);
            match a_num.cmp(&b_num) {
                Ordering::Equal => continue,
                other => return other,
            }
        } else {
            let ac_lower = ac.to_lowercase().next().unwrap_or(ac);
            let bc_lower = bc.to_lowercase().next().unwrap_or(bc);
            match ac_lower.cmp(&bc_lower) {
                Ordering::Equal => {
                    a_chars.next();
                    b_chars.next();
                }
                other => return other,
            }
        }
    }

    a.len().cmp(&b.len())
}

fn extract_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> u64 {
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            s.push(c);
            chars.next();
        } else {
            break;
        }
    }
    s.parse::<u64>().unwrap_or(0)
}

/// Sort a slice in place by a key extractor with explicit SortOrder.
pub fn sort_by_key<T, K: Ord, F: Fn(&T) -> K>(items: &mut [T], key_fn: F, order: SortOrder) {
    items.sort_by(|a, b| {
        let ka = key_fn(a);
        let kb = key_fn(b);
        if order.is_ascending() {
            ka.cmp(&kb)
        } else {
            kb.cmp(&ka)
        }
    });
}

/// Sort a slice in place using natural alphanumeric string comparison.
pub fn sort_natural<T, F: Fn(&T) -> &str>(items: &mut [T], key_fn: F, order: SortOrder) {
    items.sort_by(|a, b| {
        let ka = key_fn(a);
        let kb = key_fn(b);
        if order.is_ascending() {
            natural_cmp(ka, kb)
        } else {
            natural_cmp(kb, ka)
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_sorting_orders_numbers_correctly() {
        assert_eq!(natural_cmp("item2", "item10"), Ordering::Less);
        assert_eq!(natural_cmp("item10", "item2"), Ordering::Greater);
        assert_eq!(natural_cmp("item2", "item2"), Ordering::Equal);
        assert_eq!(natural_cmp("v1.2.9", "v1.2.10"), Ordering::Less);
    }

    #[test]
    fn sort_natural_slice() {
        let mut files = vec!["file10.txt", "file1.txt", "file2.txt", "file20.txt"];
        sort_natural(&mut files, |s| s, SortOrder::Ascending);
        assert_eq!(
            files,
            vec!["file1.txt", "file2.txt", "file10.txt", "file20.txt"]
        );

        sort_natural(&mut files, |s| s, SortOrder::Descending);
        assert_eq!(
            files,
            vec!["file20.txt", "file10.txt", "file2.txt", "file1.txt"]
        );
    }
}
