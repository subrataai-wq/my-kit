//! Read settings from environment variables and simple `.env` files.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum EnvError {
    Missing(String),
    Invalid { key: String, value: String },
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::Missing(k) => write!(f, "environment variable `{k}` is not set"),
            EnvError::Invalid { key, value } => {
                write!(f, "environment variable `{key}` has a bad value: `{value}`")
            }
        }
    }
}

impl std::error::Error for EnvError {}

/// Value of `key`, or `None` if missing or not valid text.
pub fn get(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

pub fn get_or(key: &str, default: &str) -> String {
    get(key).unwrap_or_else(|| default.to_string())
}

pub fn require(key: &str) -> Result<String, EnvError> {
    get(key).ok_or_else(|| EnvError::Missing(key.to_string()))
}

/// Read and convert to any type (`u16`, `f64`, `bool`, ...). Error if missing or bad.
pub fn require_parse<T: FromStr>(key: &str) -> Result<T, EnvError> {
    let value = require(key)?;
    value.trim().parse::<T>().map_err(|_| EnvError::Invalid {
        key: key.to_string(),
        value,
    })
}

/// Read and convert, use `default` if missing or bad.
pub fn parse_or<T: FromStr>(key: &str, default: T) -> T {
    get(key)
        .and_then(|v| v.trim().parse::<T>().ok())
        .unwrap_or(default)
}

/// `1`, `true`, `yes`, `on` are true. `0`, `false`, `no`, `off` are false.
pub fn get_bool(key: &str, default: bool) -> bool {
    match get(key).map(|v| v.trim().to_lowercase()).as_deref() {
        Some("1" | "true" | "yes" | "on") => true,
        Some("0" | "false" | "no" | "off") => false,
        _ => default,
    }
}

/// Turn the text of a `.env` file into (key, value) pairs. Pure function, no side effects.
pub fn parse_dotenv(content: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let key = k.trim();
        if key.is_empty() {
            continue;
        }
        let mut value = v.trim();
        for q in ['"', '\''] {
            if value.len() >= 2 && value.starts_with(q) && value.ends_with(q) {
                value = &value[1..value.len() - 1];
                break;
            }
        }
        out.push((key.to_string(), value.to_string()));
    }
    out
}

/// Load a `.env` file into the process. Variables that are already set are kept.
/// Returns how many variables were newly set. Call this once at start-up.
pub fn load_dotenv(path: impl AsRef<std::path::Path>) -> std::io::Result<usize> {
    let content = std::fs::read_to_string(path)?;
    let mut count = 0;
    for (k, v) in parse_dotenv(&content) {
        if std::env::var_os(&k).is_none() {
            // SAFETY: load_dotenv is invoked during start-up before worker threads spawn
            unsafe {
                std::env::set_var(&k, v);
            }
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dotenv_parsing() {
        let text = "# note\nA=1\nexport B = \"two words\"\nC='x'\n\nbad line\n=nokey\n";
        let v = parse_dotenv(text);
        assert_eq!(
            v,
            vec![
                ("A".into(), "1".into()),
                ("B".into(), "two words".into()),
                ("C".into(), "x".into())
            ]
        );
    }

    #[test]
    fn typed_reads() {
        unsafe {
            std::env::set_var("UK_TEST_PORT", " 8080 ");
            std::env::set_var("UK_TEST_BAD", "abc");
        }
        assert_eq!(require_parse::<u16>("UK_TEST_PORT"), Ok(8080));
        assert!(matches!(
            require_parse::<u16>("UK_TEST_BAD"),
            Err(EnvError::Invalid { .. })
        ));
        assert_eq!(parse_or::<u16>("UK_TEST_MISSING_X", 3), 3);
        assert!(matches!(
            require("UK_TEST_MISSING_X"),
            Err(EnvError::Missing(_))
        ));
    }

    #[test]
    fn bool_reads() {
        unsafe {
            std::env::set_var("UK_TEST_FLAG", "Yes");
            std::env::set_var("UK_TEST_FLAG2", "off");
        }
        assert!(get_bool("UK_TEST_FLAG", false));
        assert!(!get_bool("UK_TEST_FLAG2", true));
        assert!(get_bool("UK_TEST_FLAG_NONE", true));
    }
}
