//! Quick checks for form input.

/// Basic email check: one `@`, text before it, a dot in the domain, no spaces.
pub fn is_email(s: &str) -> bool {
    if s.chars().any(char::is_whitespace) || s.len() > 254 {
        return false;
    }
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && !domain.contains('@')
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !domain.contains("..")
}

/// Starts with `http://` or `https://` and has a host.
pub fn is_http_url(s: &str) -> bool {
    let rest = s.strip_prefix("https://").or_else(|| s.strip_prefix("http://"));
    match rest {
        Some(r) => {
            let host = r.split(['/', '?', '#']).next().unwrap_or("");
            !host.is_empty() && !s.chars().any(char::is_whitespace)
        }
        None => false,
    }
}

/// Letters, numbers and `_` only, length from `min` to `max`.
pub fn is_username(s: &str, min: usize, max: usize) -> bool {
    let n = s.chars().count();
    n >= min && n <= max && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn in_range<T: PartialOrd>(value: T, lo: T, hi: T) -> bool {
    value >= lo && value <= hi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emails() {
        assert!(is_email("a@b.com"));
        assert!(is_email("first.last+tag@mail.example.org"));
        assert!(!is_email("a@b"));
        assert!(!is_email("@b.com"));
        assert!(!is_email("a b@c.com"));
        assert!(!is_email("a@@b.com"));
        assert!(!is_email("a@b..com"));
    }

    #[test]
    fn urls() {
        assert!(is_http_url("https://example.com/path?x=1"));
        assert!(is_http_url("http://localhost:8080"));
        assert!(!is_http_url("ftp://example.com"));
        assert!(!is_http_url("https://"));
        assert!(!is_http_url("https://a b.com"));
    }

    #[test]
    fn others() {
        assert!(is_username("sub_rata1", 3, 16));
        assert!(!is_username("ab", 3, 16));
        assert!(!is_username("bad name", 3, 16));
        assert!(is_hex("DeadBeef09"));
        assert!(!is_hex(""));
        assert!(in_range(5, 1, 5));
    }
}

