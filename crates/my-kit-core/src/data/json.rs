//! JSON serialization and file persistence helpers.

use crate::base::result::{AppResult, Context};
use crate::platform::fs;
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

pub fn to_string_pretty<T: Serialize>(val: &T) -> AppResult<String> {
    serde_json::to_string_pretty(val).context("Failed to serialize to pretty JSON")
}

pub fn to_string<T: Serialize>(val: &T) -> AppResult<String> {
    serde_json::to_string(val).context("Failed to serialize to JSON")
}

pub fn from_str<T: DeserializeOwned>(s: &str) -> AppResult<T> {
    serde_json::from_str(s).context("Failed to deserialize from JSON string")
}

pub fn to_file<T: Serialize>(path: impl AsRef<Path>, val: &T) -> AppResult<()> {
    let json = to_string_pretty(val)?;
    fs::write_atomic(path, json.as_bytes()).context("Failed to write JSON file")?;
    Ok(())
}

pub fn from_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> AppResult<T> {
    let raw = fs::read_text(path).context("Failed to read JSON file")?;
    from_str(&raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
    struct User {
        id: u64,
        name: String,
    }

    #[test]
    fn json_roundtrip() {
        let u = User {
            id: 42,
            name: "Prelude".into(),
        };
        let s = to_string(&u).unwrap();
        let parsed: User = from_str(&s).unwrap();
        assert_eq!(u, parsed);
    }
}
