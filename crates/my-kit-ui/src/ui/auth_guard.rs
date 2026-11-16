//! Authentication guard component for conditionally protecting views.

use crate::state::auth::use_auth;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthGuardProps {
    #[props(default)]
    pub required_role: Option<String>,
    #[props(default)]
    pub fallback: Option<Element>,
    pub children: Element,
}

/// Conditionally renders child content only if the user is authenticated and possesses the required role.
#[component]
pub fn AuthGuard(props: AuthGuardProps) -> Element {
    let auth = use_auth();
    let session_opt = auth();

    if let Some(session) = session_opt {
        if let Some(role) = &props.required_role
            && !session.has_role(role)
        {
            return props.fallback.unwrap_or_else(|| {
                rsx! {
                    div { class: "auth-guard auth-unauthorized",
                        p { "Access Restricted: You lack the required role to view this content." }
                    }
                }
            });
        }
        props.children
    } else {
        props.fallback.unwrap_or_else(|| {
            rsx! {
                div { class: "auth-guard auth-unauthenticated",
                    p { "Authentication required: Please sign in to view this section." }
                }
            }
        })
    }
}
