//! Cross-platform standard directories and path sanitization.

use std::path::{Path, PathBuf};

/// Get standard application config directory (e.g., `%APPDATA%\<app>` on Windows, `~/.config/<app>` on Linux/macOS).
pub fn app_config_dir(app_name: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            return PathBuf::from(appdata).join(app_name);
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join(app_name);
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join(app_name);
        } else if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(".config").join(app_name);
        }
    }
    std::env::temp_dir().join(app_name).join("config")
}

/// Get standard application data directory (e.g., `%LOCALAPPDATA%\<app>` on Windows).
pub fn app_data_dir(app_name: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            return PathBuf::from(local).join(app_name);
        } else if let Some(appdata) = std::env::var_os("APPDATA") {
            return PathBuf::from(appdata).join(app_name);
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join(app_name);
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
            return PathBuf::from(xdg).join(app_name);
        } else if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(".local").join("share").join(app_name);
        }
    }
    std::env::temp_dir().join(app_name).join("data")
}

/// Sanitize filename by stripping illegal characters (`/`, `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|`).
pub fn sanitize_filename(name: &str) -> String {
    const FORBIDDEN: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0'];
    let sanitized: String = name
        .chars()
        .map(|c| if FORBIDDEN.contains(&c) { '_' } else { c })
        .collect();
    let trimmed = sanitized.trim_matches(|c| c == ' ' || c == '.');
    if trimmed.is_empty() {
        "file".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Normalize and canonicalize path, safely stripping Windows verbatim prefix `\\?\`.
pub fn normalize_path(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let canonical = std::fs::canonicalize(path.as_ref())?;
    #[cfg(target_os = "windows")]
    {
        let path_str = canonical.to_string_lossy();
        if let Some(stripped) = path_str.strip_prefix(r"\\?\") {
            return Ok(PathBuf::from(stripped));
        }
    }
    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitization_removes_illegal_characters() {
        assert_eq!(sanitize_filename("my:file*name?.txt"), "my_file_name_.txt");
        assert_eq!(sanitize_filename("..."), "file");
        assert_eq!(sanitize_filename("valid_name.json"), "valid_name.json");
    }

    #[test]
    fn paths_return_non_empty() {
        let config = app_config_dir("prelude_test");
        assert!(config.to_string_lossy().contains("prelude_test"));

        let data = app_data_dir("prelude_test");
        assert!(data.to_string_lossy().contains("prelude_test"));
    }

    #[test]
    fn normalize_current_dir() {
        let cur = normalize_path(".").unwrap();
        assert!(cur.is_absolute());
        #[cfg(target_os = "windows")]
        assert!(!cur.to_string_lossy().starts_with(r"\\?\"));
    }
}

