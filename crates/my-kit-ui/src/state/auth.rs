//! Authentication state provider and consumer hooks.

use dioxus::prelude::*;
use my_kit_core::auth::TokenSession;

/// Initializes the global authentication context. Call this once at the app root component.
pub fn use_init_auth(initial_session: Option<TokenSession>) -> Signal<Option<TokenSession>> {
    use_context_provider(|| Signal::new(initial_session))
}

/// Accesses the global authentication context.
pub fn use_auth() -> Signal<Option<TokenSession>> {
    use_context::<Signal<Option<TokenSession>>>()
}

/// Returns true if an authenticated session is active and not expired.
pub fn use_is_authenticated() -> bool {
    let auth = use_auth();
    auth().is_some()
}

/// Returns true if the authenticated session has the specified role.
pub fn use_has_role(role: &str) -> bool {
    let auth = use_auth();
    auth().as_ref().is_some_and(|s| s.has_role(role))
}
