//! Form input components styled with semantic CSS tokens.

use dioxus::prelude::*;
use my_kit_core::CompactStr;

#[derive(Props, Clone, PartialEq)]
pub struct FormFieldProps {
    #[props(default)]
    pub label: Option<String>,
    #[props(default)]
    pub error: Option<CompactStr>,
    #[props(default)]
    pub hint: Option<String>,
    pub children: Element,
}

/// Layout wrapper that presents a label, input control, optional hint, and validation error.
#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    rsx! {
        div { class: "form-field",
            if let Some(lbl) = &props.label {
                label { class: "form-label", "{lbl}" }
            }
            {props.children}
            if let Some(err) = &props.error {
                span { class: "form-error", "{err}" }
            } else if let Some(hint) = &props.hint {
                span { class: "form-hint", "{hint}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TextInputProps {
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub name: Option<String>,
    #[props(default = "text".to_string())]
    pub input_type: String,
    pub value: String,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub has_error: bool,
    pub oninput: EventHandler<FormEvent>,
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
}

/// Standard text input with focus ring and error state styling.
#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    let error_class = if props.has_error { " input-error" } else { "" };
    let class = format!("input-base{error_class}");

    rsx! {
        input {
            class: "{class}",
            r#type: "{props.input_type}",
            value: "{props.value}",
            placeholder: props.placeholder.as_deref().unwrap_or(""),
            disabled: props.disabled,
            oninput: move |e| props.oninput.call(e),
            onblur: move |e| {
                if let Some(blur_handler) = &props.onblur {
                    blur_handler.call(e);
                }
            },
        }
    }
}

