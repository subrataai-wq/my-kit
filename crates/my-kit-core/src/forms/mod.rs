//! Form state and validation engine for input validation.

use crate::base::validate::{is_email, is_username};
pub use compact_str::CompactString;
pub type CompactStr = CompactString;

/// Represents a validated form field with tracking for user interaction and validation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field<T> {
    pub value: T,
    pub error: Option<CompactStr>,
    pub touched: bool,
    pub dirty: bool,
}

impl<T: Default> Default for Field<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Field<T> {
    pub fn new(initial_value: T) -> Self {
        Self {
            value: initial_value,
            error: None,
            touched: false,
            dirty: false,
        }
    }

    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
        self.dirty = true;
        self.error = None;
    }

    pub fn touch(&mut self) {
        self.touched = true;
    }

    pub fn set_error(&mut self, err: impl Into<CompactStr>) {
        self.error = Some(err.into());
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn is_valid(&self) -> bool {
        self.error.is_none()
    }
}

/// Composable validation helper functions for form fields.
pub struct Validation;

impl Validation {
    pub fn required(value: &str, message: impl Into<CompactStr>) -> Option<CompactStr> {
        if value.trim().is_empty() {
            Some(message.into())
        } else {
            None
        }
    }

    pub fn min_length(
        value: &str,
        min: usize,
        message: impl Into<CompactStr>,
    ) -> Option<CompactStr> {
        if value.chars().count() < min {
            Some(message.into())
        } else {
            None
        }
    }

    pub fn max_length(
        value: &str,
        max: usize,
        message: impl Into<CompactStr>,
    ) -> Option<CompactStr> {
        if value.chars().count() > max {
            Some(message.into())
        } else {
            None
        }
    }

    pub fn email(value: &str, message: impl Into<CompactStr>) -> Option<CompactStr> {
        if !is_email(value) {
            Some(message.into())
        } else {
            None
        }
    }

    pub fn username(
        value: &str,
        min: usize,
        max: usize,
        message: impl Into<CompactStr>,
    ) -> Option<CompactStr> {
        if !is_username(value, min, max) {
            Some(message.into())
        } else {
            None
        }
    }

    pub fn numeric_range(
        value: f64,
        min: f64,
        max: f64,
        message: impl Into<CompactStr>,
    ) -> Option<CompactStr> {
        if value < min || value > max {
            Some(message.into())
        } else {
            None
        }
    }

    pub fn matches<F: Fn(&str) -> bool>(
        value: &str,
        predicate: F,
        message: impl Into<CompactStr>,
    ) -> Option<CompactStr> {
        if !predicate(value) {
            Some(message.into())
        } else {
            None
        }
    }
}

pub mod prelude {
    pub use super::{Field, Validation};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_state_transitions() {
        let mut field = Field::new(String::new());
        assert_eq!(field.value, "");
        assert!(!field.dirty);
        assert!(!field.touched);
        assert!(field.is_valid());

        field.set("Hello".to_string());
        assert_eq!(field.value, "Hello");
        assert!(field.dirty);

        field.set_error("Invalid text");
        assert!(!field.is_valid());
        assert_eq!(field.error.as_deref(), Some("Invalid text"));

        field.clear_error();
        assert!(field.is_valid());
    }

    #[test]
    fn test_validation_rules() {
        assert_eq!(
            Validation::required("", "Required"),
            Some("Required".into())
        );
        assert_eq!(
            Validation::required("   ", "Required"),
            Some("Required".into())
        );
        assert_eq!(Validation::required("user", "Required"), None);

        assert_eq!(
            Validation::min_length("abc", 5, "Min 5"),
            Some("Min 5".into())
        );
        assert_eq!(Validation::min_length("abcdef", 5, "Min 5"), None);

        assert_eq!(
            Validation::max_length("abcdef", 5, "Max 5"),
            Some("Max 5".into())
        );
        assert_eq!(Validation::max_length("abc", 5, "Max 5"), None);

        assert_eq!(
            Validation::email("invalid", "Bad email"),
            Some("Bad email".into())
        );
        assert_eq!(Validation::email("test@example.com", "Bad email"), None);

        assert_eq!(
            Validation::numeric_range(5.0, 10.0, 20.0, "Out of range"),
            Some("Out of range".into())
        );
        assert_eq!(
            Validation::numeric_range(15.0, 10.0, 20.0, "Out of range"),
            None
        );
    }

    #[test]
    fn test_compact_str_inlined_without_heap() {
        let err = Validation::required("", "Required").unwrap();
        assert!(!err.is_heap_allocated());
        assert_eq!(err.as_str(), "Required");
    }
}
