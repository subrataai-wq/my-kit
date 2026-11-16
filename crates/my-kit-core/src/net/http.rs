//! URL query parameter and HTTP utilities.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }
}

/// Helper for constructing URL query parameter strings.
#[derive(Debug, Default, Clone)]
pub struct QueryParams {
    params: BTreeMap<String, String>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self {
            params: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn build(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }
        let encoded: Vec<String> = self
            .params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        format!("?{}", encoded.join("&"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_param_building() {
        let mut q = QueryParams::new();
        q.add("page", "1").add("limit", "25");
        assert_eq!(q.build(), "?limit=25&page=1");
    }
}

