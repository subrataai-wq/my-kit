//! Authentication token session and claims management.

pub use compact_str::CompactString;
pub type CompactStr = CompactString;
use thiserror::Error;

#[cfg(feature = "auth")]
pub use secrecy::{ExposeSecret, SecretString};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// Strongly typed authentication domain errors.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AuthError {
    #[error("Session expired at timestamp {expired_at}")]
    Expired { expired_at: u64 },
    #[error("Invalid token signature or formatting")]
    InvalidToken,
    #[error("Missing required claim: {0}")]
    MissingClaim(CompactStr),
    #[error("User does not have required role: {0}")]
    InsufficientPermissions(CompactStr),
}

impl From<AuthError> for crate::base::result::AppError {
    fn from(err: AuthError) -> Self {
        crate::base::result::AppError::invalid(err.to_string())
    }
}

/// User claims embedded in or associated with an authenticated session.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Claims {
    pub user_id: String,
    pub email: Option<String>,
    pub roles: Vec<CompactStr>,
    #[cfg(feature = "json")]
    pub metadata: Option<serde_json::Value>,
}

impl Claims {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            email: None,
            roles: Vec::new(),
            #[cfg(feature = "json")]
            metadata: None,
        }
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn with_role(mut self, role: impl Into<CompactStr>) -> Self {
        self.roles.push(role.into());
        self
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles
            .iter()
            .any(|r| r.as_str().eq_ignore_ascii_case(role))
    }
}

/// An authenticated session with access token, optional refresh token, and expiration timestamp.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct TokenSession {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: u64,
    pub claims: Claims,
}

impl TokenSession {
    pub fn new(access_token: impl Into<String>, expires_at: u64, claims: Claims) -> Self {
        Self {
            access_token: access_token.into(),
            refresh_token: None,
            expires_at,
            claims,
        }
    }

    pub fn with_refresh_token(mut self, token: impl Into<String>) -> Self {
        self.refresh_token = Some(token.into());
        self
    }

    /// Returns the access token wrapped in a `SecretString` to guarantee redaction in debug logs.
    #[cfg(feature = "auth")]
    pub fn secret_access_token(&self) -> SecretString {
        SecretString::from(self.access_token.clone())
    }

    /// Returns the refresh token wrapped in a `SecretString` if present.
    #[cfg(feature = "auth")]
    pub fn secret_refresh_token(&self) -> Option<SecretString> {
        self.refresh_token
            .as_ref()
            .map(|t| SecretString::from(t.clone()))
    }

    /// Checks if the token is expired given the current unix timestamp in seconds.
    pub fn is_expired(&self, now_unix: u64) -> bool {
        now_unix >= self.expires_at
    }

    /// Returns remaining seconds until expiration, or 0 if already expired.
    pub fn expires_in(&self, now_unix: u64) -> u64 {
        self.expires_at.saturating_sub(now_unix)
    }

    /// Formats the access token as an HTTP Authorization header value.
    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }

    /// Returns true if the session claims contain the specified role.
    pub fn has_role(&self, role: &str) -> bool {
        self.claims.has_role(role)
    }

    /// Returns true if the session claims contain any of the specified roles.
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }
}

pub mod prelude {
    pub use super::{AuthError, Claims, TokenSession};
    #[cfg(feature = "auth")]
    pub use super::{ExposeSecret, SecretString};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_session_expiration() {
        let claims = Claims::new("user_123")
            .with_email("dev@studio.local")
            .with_role("admin");
        let session = TokenSession::new("token_abc", 1000, claims);

        assert!(!session.is_expired(999));
        assert!(session.is_expired(1000));
        assert!(session.is_expired(1001));

        assert_eq!(session.expires_in(900), 100);
        assert_eq!(session.expires_in(1050), 0);
    }

    #[test]
    fn test_claims_roles_and_header() {
        let claims = Claims::new("user_456")
            .with_role("Editor")
            .with_role("Viewer");
        let session = TokenSession::new("xyz789", 2000, claims);

        assert!(session.has_role("editor"));
        assert!(session.has_role("VIEWER"));
        assert!(!session.has_role("admin"));
        assert!(session.has_any_role(&["admin", "editor"]));

        assert_eq!(session.auth_header(), "Bearer xyz789");
    }

    #[test]
    fn test_secrecy_redacts_in_debug_logs() {
        let claims = Claims::new("user_123");
        let session = TokenSession::new("sensitive_jwt_token", 1000, claims);
        let secret = session.secret_access_token();

        let debug_str = format!("{:?}", secret);
        assert!(debug_str.contains("[REDACTED]"));
        assert!(!debug_str.contains("sensitive_jwt_token"));
        assert_eq!(secret.expose_secret(), "sensitive_jwt_token");
    }

    #[test]
    fn test_auth_errors() {
        let err = AuthError::Expired { expired_at: 12345 };
        assert!(err.to_string().contains("12345"));
        let app_err: crate::base::result::AppError = err.into();
        assert_eq!(app_err.kind(), crate::base::result::ErrorKind::Invalid);
    }
}
