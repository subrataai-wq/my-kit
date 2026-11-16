//! Substring and fuzzy search matching algorithms with zero-copy references.

/// Case-insensitive fuzzy subsequence match.
/// Checks if all characters of `pattern` appear in `target` in order.
pub fn fuzzy_match(pattern: &str, target: &str) -> bool {
    let mut pattern_chars = pattern.chars().flat_map(char::to_lowercase);
    let mut next_char = pattern_chars.next();

    for c in target.chars().flat_map(char::to_lowercase) {
        if let Some(target_char) = next_char {
            if c == target_char {
                next_char = pattern_chars.next();
            }
        } else {
            break;
        }
    }

    next_char.is_none()
}

/// Score a fuzzy match (higher score indicates higher relevance).
/// Rewards consecutive matches and matches at word starts.
pub fn fuzzy_score(pattern: &str, target: &str) -> Option<i64> {
    if pattern.is_empty() {
        return Some(0);
    }

    let p_lower: Vec<char> = pattern.to_lowercase().chars().collect();
    let t_lower: Vec<char> = target.to_lowercase().chars().collect();

    let mut score = 0i64;
    let mut p_idx = 0;
    let mut consecutive = 0;

    for (t_idx, &tc) in t_lower.iter().enumerate() {
        if p_idx < p_lower.len() && tc == p_lower[p_idx] {
            score += 10 + (consecutive * 5);
            consecutive += 1;

            if t_idx == 0 || !t_lower[t_idx - 1].is_alphanumeric() {
                score += 20; // Word boundary bonus
            }
            p_idx += 1;
        } else {
            consecutive = 0;
        }
    }

    if p_idx == p_lower.len() {
        Some(score)
    } else {
        None
    }
}

/// Filter an item collection by fuzzy query, returning zero-copy references.
pub fn filter_by<'a, T, F: Fn(&T) -> &str>(items: &'a [T], query: &str, accessor: F) -> Vec<&'a T> {
    if query.trim().is_empty() {
        return items.iter().collect();
    }
    items
        .iter()
        .filter(|item| fuzzy_match(query, accessor(item)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_matching() {
        assert!(fuzzy_match("fb", "foobar"));
        assert!(fuzzy_match("ob", "foobar"));
        assert!(!fuzzy_match("bf", "foobar"));
        assert!(fuzzy_match("", "anything"));
    }

    #[test]
    fn fuzzy_scoring() {
        let score_prefix = fuzzy_score("pre", "Prelude Desktop");
        let score_subsequence = fuzzy_score("pdt", "Prelude Desktop");
        assert!(score_prefix.is_some());
        assert!(score_subsequence.is_some());
        assert!(score_prefix.unwrap() > score_subsequence.unwrap());
    }

    #[test]
    fn filter_by_returns_references() {
        let items = vec!["Rust", "Python", "Ruby", "TypeScript"];
        let matched = filter_by(&items, "ru", |s| s);
        assert_eq!(matched, vec![&"Rust", &"Ruby"]);
    }
}

