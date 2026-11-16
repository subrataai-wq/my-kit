//! One simple error type for the whole app, plus `.context("...")`.

use std::error::Error as StdError;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("not found")]
    NotFound,
    #[error("invalid")]
    Invalid,
    #[error("io error")]
    Io,
    #[error("parse error")]
    Parse,
    #[error("permission denied")]
    Permission,
    #[error("network error")]
    Network,
    #[error("error")]
    Other,
}

fn format_source(source: Option<&(dyn StdError + Send + Sync + 'static)>) -> String {
    match source {
        Some(s) => format!(" (caused by: {s})"),
        None => String::new(),
    }
}

#[derive(Error, Debug)]
#[error("{kind}: {message}{}", format_source(.source.as_deref()))]
pub struct AppError {
    kind: ErrorKind,
    message: String,
    #[source]
    source: Option<Box<dyn StdError + Send + Sync>>,
}

impl AppError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::NotFound, message)
    }

    pub fn invalid(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Invalid, message)
    }

    pub fn io(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Io, message)
    }

    pub fn with_source(mut self, source: impl StdError + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}


impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        let kind = match e.kind() {
            std::io::ErrorKind::NotFound => ErrorKind::NotFound,
            std::io::ErrorKind::PermissionDenied => ErrorKind::Permission,
            _ => ErrorKind::Io,
        };
        AppError::new(kind, e.to_string()).with_source(e)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::new(ErrorKind::Parse, e.to_string()).with_source(e)
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(e: std::num::ParseFloatError) -> Self {
        AppError::new(ErrorKind::Parse, e.to_string()).with_source(e)
    }
}

/// Add a short message to any `Result` or `Option`.
pub trait Context<T> {
    fn context(self, message: impl Into<String>) -> AppResult<T>;
    fn with_context<F: FnOnce() -> String>(self, f: F) -> AppResult<T>;
}

impl<T, E: StdError + Send + Sync + 'static> Context<T> for Result<T, E> {
    fn context(self, message: impl Into<String>) -> AppResult<T> {
        self.map_err(|e| AppError::new(ErrorKind::Other, message).with_source(e))
    }

    fn with_context<F: FnOnce() -> String>(self, f: F) -> AppResult<T> {
        self.map_err(|e| AppError::new(ErrorKind::Other, f()).with_source(e))
    }
}

impl<T> Context<T> for Option<T> {
    fn context(self, message: impl Into<String>) -> AppResult<T> {
        self.ok_or_else(|| AppError::not_found(message))
    }

    fn with_context<F: FnOnce() -> String>(self, f: F) -> AppResult<T> {
        self.ok_or_else(|| AppError::not_found(f()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_on_result_keeps_source() {
        let r: Result<i32, _> = "abc".parse::<i32>();
        let e = r.context("reading age").unwrap_err();
        assert_eq!(e.message(), "reading age");
        assert!(e.to_string().contains("caused by"));
        assert!(StdError::source(&e).is_some());
    }

    #[test]
    fn context_on_option_is_not_found() {
        let e = None::<u8>.context("user missing").unwrap_err();
        assert_eq!(e.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn io_error_maps_kind() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "x");
        assert_eq!(AppError::from(io).kind(), ErrorKind::NotFound);
    }
}

