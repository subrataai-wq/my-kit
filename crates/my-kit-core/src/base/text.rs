//! Text helpers.

/// Cut text to `max_chars` letters and add `…` if it was cut.
pub fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    if max_chars == 0 {
        return String::new();
    }
    let mut out: String = s.chars().take(max_chars - 1).collect();
    out.push('…');
    out
}

/// Split text into words. Splits on spaces, `_`, `-`, and on `aB` changes.
pub fn split_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut cur = String::new();
    let mut prev_lower = false;
    for c in s.chars() {
        if !c.is_alphanumeric() {
            if !cur.is_empty() {
                words.push(std::mem::take(&mut cur));
            }
            prev_lower = false;
            continue;
        }
        if c.is_uppercase() && prev_lower && !cur.is_empty() {
            words.push(std::mem::take(&mut cur));
        }
        cur.push(c);
        prev_lower = c.is_lowercase();
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    words
}

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// `"Hello, World!"` gives `"hello-world"`. Good for URLs and file names.
pub fn slugify(s: &str) -> String {
    split_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn snake_case(s: &str) -> String {
    split_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

pub fn kebab_case(s: &str) -> String {
    slugify(s)
}

pub fn camel_case(s: &str) -> String {
    let words = split_words(s);
    let mut out = String::new();
    for (i, w) in words.iter().enumerate() {
        let w = w.to_lowercase();
        out.push_str(&if i == 0 { w } else { capitalize(&w) });
    }
    out
}

pub fn pascal_case(s: &str) -> String {
    split_words(s)
        .iter()
        .map(|w| capitalize(&w.to_lowercase()))
        .collect()
}

/// `"the quick fox"` gives `"The Quick Fox"`.
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|w| capitalize(&w.to_lowercase()))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Hide the start of a secret. `mask("1234567890", 4)` gives `"******7890"`.
pub fn mask(s: &str, visible_end: usize) -> String {
    let total = s.chars().count();
    let keep = visible_end.min(total);
    let hidden = total - keep;
    let tail: String = s.chars().skip(hidden).collect();
    format!("{}{}", "*".repeat(hidden), tail)
}

/// `pluralize(1, "item", "items")` gives `"1 item"`.
pub fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(slugify("  Hello, World!  "), "hello-world");
        assert_eq!(snake_case("userFirstName"), "user_first_name");
        assert_eq!(snake_case("User First-Name"), "user_first_name");
        assert_eq!(camel_case("user_first_name"), "userFirstName");
        assert_eq!(pascal_case("user first name"), "UserFirstName");
        assert_eq!(title_case("the QUICK fox"), "The Quick Fox");
        assert_eq!(kebab_case("Some Title"), "some-title");
    }

    #[test]
    fn cutting_and_counting() {
        assert_eq!(truncate("hello world", 8), "hello w…");
        assert_eq!(truncate("hi", 8), "hi");
        assert_eq!(truncate("hello", 0), "");
        assert_eq!(truncate("नमस्ते दुनिया", 5).chars().count(), 5);
        assert_eq!(collapse_whitespace(" a   b \n c "), "a b c");
        assert_eq!(word_count("one two  three"), 3);
        assert!(is_blank(" \t\n"));
    }

    #[test]
    fn misc() {
        assert_eq!(mask("1234567890", 4), "******7890");
        assert_eq!(mask("12", 4), "12");
        assert_eq!(pluralize(1, "item", "items"), "1 item");
        assert_eq!(pluralize(3, "item", "items"), "3 items");
        assert_eq!(capitalize(""), "");
    }
}

