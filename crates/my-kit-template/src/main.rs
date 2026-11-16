use dioxus::prelude::*;
use my_kit_core::prelude::*;
use my_kit_ui::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut theme = use_theme();
    let notices = use_notices();
    let now = use_now();
    let mut auth = use_init_auth(None);

    let mut name_field = use_signal(|| Field::new(String::new()));
    let mut email_field = use_signal(|| Field::new(String::new()));

    let on_submit_form = move |_| {
        let name_val = name_field().value;
        let email_val = email_field().value;

        let name_err = Validation::required(&name_val, "Name is required")
            .or_else(|| Validation::min_length(&name_val, 3, "Name must be at least 3 characters"));
        let email_err = Validation::required(&email_val, "Email is required")
            .or_else(|| Validation::email(&email_val, "Please enter a valid email address"));

        name_field.write().error = name_err;
        email_field.write().error = email_err;

        if name_field().is_valid() && email_field().is_valid() {
            notices.success(format!("Form submitted successfully for {name_val}!"));
            name_field.set(Field::new(String::new()));
            email_field.set(Field::new(String::new()));
        } else {
            notices.error("Please fix the validation errors before submitting.");
        }
    };

    let toggle_auth = move |_| {
        if auth().is_some() {
            auth.set(None);
            notices.info("Signed out successfully.");
        } else {
            let claims = Claims::new("usr_demo_01")
                .with_email("founder@studio.local")
                .with_role("admin");
            let session = TokenSession::new("token_simulated_jwt_abc123", now() + 3600, claims);
            auth.set(Some(session));
            notices.success("Signed in as Administrator!");
        }
    };

    let sample_dim = Dimensions::new(1920, 1080);
    let sample_box = Dimensions::new(400, 400);
    let fitted = fit_contain(sample_dim, sample_box);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }

        main {
            "data-theme": theme().as_str(),
            style: "min-height: 100vh; background: var(--bg-app); color: var(--text-main); font-family: var(--font-sans); padding: 32px 16px;",

            div { style: "max-width: 720px; margin: 0 auto; display: flex; flex-direction: column; gap: 24px;",
                // Header Bar
                header { style: "display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--border); padding-bottom: 16px;",
                    div {
                        h1 { style: "font-size: 24px; font-weight: 700; margin: 0;", "Startup Studio Engine" }
                        p { style: "color: var(--text-muted); font-size: 13px; margin: 4px 0 0 0;",
                            "Rust 2024 · Dioxus 0.7 · Unified Web & Desktop"
                        }
                    }
                    div { style: "display: flex; gap: 8px; align-items: center;",
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            onclick: move |_| theme.set(theme().toggle()),
                            if theme() == ThemeMode::Light { "🌙 Dark" } else { "☀️ Light" }
                        }
                        Button {
                            variant: if auth().is_some() { ButtonVariant::Danger } else { ButtonVariant::Primary },
                            size: ButtonSize::Sm,
                            onclick: toggle_auth,
                            if auth().is_some() { "Sign Out" } else { "Sign In (Admin)" }
                        }
                    }
                }

                // Notices Toast Area
                for notice in notices.list() {
                    Banner {
                        level: notice.level,
                        message: notice.message,
                        on_dismiss: move |_| notices.dismiss(&notice.id),
                    }
                }

                // Section 1: Reactive Form Engine
                section { style: "background: var(--bg-surface); padding: 20px; border-radius: var(--radius-lg); border: 1px solid var(--border);",
                    h2 { style: "font-size: 18px; margin: 0 0 16px 0;", "1. Reactive Form & Validation Engine" }

                    FormField {
                        label: "Founder Name".to_string(),
                        error: name_field().error,
                        hint: "Minimum 3 characters".to_string(),
                        TextInput {
                            value: name_field().value,
                            placeholder: "e.g., Alex Carter".to_string(),
                            has_error: name_field().error.is_some(),
                            oninput: move |e: FormEvent| {
                                name_field.write().value = e.value();
                                name_field.write().error = None;
                            },
                        }
                    }

                    FormField {
                        label: "Work Email".to_string(),
                        error: email_field().error,
                        TextInput {
                            value: email_field().value,
                            placeholder: "founder@client.com".to_string(),
                            input_type: "email".to_string(),
                            has_error: email_field().error.is_some(),
                            oninput: move |e: FormEvent| {
                                email_field.write().value = e.value();
                                email_field.write().error = None;
                            },
                        }
                    }

                    div { style: "margin-top: 16px;",
                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::Md,
                            onclick: on_submit_form,
                            "Validate & Submit"
                        }
                    }
                }

                // Section 2: Media Toolkit
                section { style: "background: var(--bg-surface); padding: 20px; border-radius: var(--radius-lg); border: 1px solid var(--border);",
                    h2 { style: "font-size: 18px; margin: 0 0 12px 0;", "2. Media & Dimensions Toolkit" }
                    p { style: "color: var(--text-muted); font-size: 14px; margin: 0 0 12px 0;",
                        "Calculates responsive aspect ratios and container containment math for galleries and storefronts:"
                    }
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                        Badge { variant: BadgeVariant::Primary, text: "Original: 1920x1080 (16:9)".to_string() }
                        Badge { variant: BadgeVariant::Default, text: "Max Box: 400x400".to_string() }
                        Badge { variant: BadgeVariant::Success, text: format!("Fitted: {}x{}", fitted.width, fitted.height) }
                    }
                }

                // Section 3: Protected View (Auth Guard)
                section { style: "background: var(--bg-surface); padding: 20px; border-radius: var(--radius-lg); border: 1px solid var(--border);",
                    h2 { style: "font-size: 18px; margin: 0 0 12px 0;", "3. Role-Based Auth Guard" }
                    AuthGuard {
                        required_role: "admin".to_string(),
                        div { style: "padding: 16px; background: var(--success-bg-subtle); border: 1px solid var(--success-border-subtle); border-radius: var(--radius-md); color: var(--success-text);",
                            p { style: "font-weight: 600; margin: 0 0 4px 0;", "✓ Secure Admin Dashboard Unlocked" }
                            p { style: "font-size: 13px; margin: 0;",
                                "Authenticated as {auth().as_ref().map_or(\"Unknown\", |s| &s.claims.user_id)} (Session active: now={now()})"
                            }
                        }
                    }
                }
            }
        }
    }
}
