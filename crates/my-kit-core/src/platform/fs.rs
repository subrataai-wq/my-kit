//! File and folder helpers.
//! Note: these do not work in a browser (web / wasm). Use them for desktop and mobile builds.

use std::io;
use std::path::{Path, PathBuf};

pub fn ensure_dir(path: impl AsRef<Path>) -> io::Result<()> {
    std::fs::create_dir_all(path)
}

pub fn read_text(path: impl AsRef<Path>) -> io::Result<String> {
    std::fs::read_to_string(path)
}

/// Safe write: writes to a temp file first, then renames it.
/// If the app crashes in the middle, the old file is not broken.
pub fn write_atomic(path: impl AsRef<Path>, contents: &[u8]) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    std::fs::write(&tmp, contents)?;
    std::fs::rename(&tmp, path)
}

/// Lowercase extension without the dot. `"a/b/Photo.PNG"` gives `Some("png")`.
pub fn extension(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
}

pub fn file_stem(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref()
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_string)
}

pub fn is_hidden(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('.'))
        .unwrap_or(false)
}

/// All files inside `dir`, sorted. Set `recursive` to go into sub folders.
pub fn list_files(dir: impl AsRef<Path>, recursive: bool) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    collect(dir.as_ref(), recursive, &mut out)?;
    out.sort();
    Ok(out)
}

fn collect(dir: &Path, recursive: bool, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            if recursive {
                collect(&path, recursive, out)?;
            }
        } else {
            out.push(path);
        }
    }
    Ok(())
}

/// Total size of all files in a folder, in bytes.
pub fn dir_size(dir: impl AsRef<Path>) -> io::Result<u64> {
    let mut total = 0;
    for f in list_files(dir, true)? {
        total += std::fs::metadata(f)?.len();
    }
    Ok(total)
}

/// `1536` gives `"1.5 KB"`.
pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.1} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("utils_kit_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    #[test]
    fn sizes() {
        assert_eq!(human_size(10), "10 B");
        assert_eq!(human_size(1536), "1.5 KB");
        assert_eq!(human_size(5 * 1024 * 1024), "5.0 MB");
    }

    #[test]
    fn names() {
        assert_eq!(extension("a/b/Photo.PNG"), Some("png".into()));
        assert_eq!(
            file_stem("a/b/report.final.txt"),
            Some("report.final".into())
        );
        assert!(is_hidden("x/.env"));
        assert!(!is_hidden("x/env"));
    }

    #[test]
    fn write_and_list() {
        let dir = temp_dir("write_and_list");
        write_atomic(dir.join("a/b.txt"), b"hello").unwrap();
        write_atomic(dir.join("c.txt"), b"hi").unwrap();
        assert_eq!(read_text(dir.join("a/b.txt")).unwrap(), "hello");
        assert_eq!(list_files(&dir, false).unwrap().len(), 1);
        assert_eq!(list_files(&dir, true).unwrap().len(), 2);
        assert_eq!(dir_size(&dir).unwrap(), 7);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
