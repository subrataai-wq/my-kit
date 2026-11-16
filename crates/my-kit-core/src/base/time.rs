//! Time helpers. Std only, no date crate.

use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub fn now_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(target_arch = "wasm32")]
pub fn now_unix_secs() -> u64 {
    0
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now_unix_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

#[cfg(target_arch = "wasm32")]
pub fn now_unix_millis() -> u128 {
    0
}

/// Measure how long something takes.
#[cfg(not(target_arch = "wasm32"))]
pub struct Stopwatch(std::time::Instant);

#[cfg(not(target_arch = "wasm32"))]
impl Stopwatch {
    pub fn start() -> Self {
        Self(std::time::Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }

    pub fn restart(&mut self) -> Duration {
        let e = self.0.elapsed();
        self.0 = std::time::Instant::now();
        e
    }
}

#[cfg(target_arch = "wasm32")]
pub struct Stopwatch;

#[cfg(target_arch = "wasm32")]
impl Stopwatch {
    pub fn start() -> Self {
        Self
    }

    pub fn elapsed(&self) -> Duration {
        Duration::ZERO
    }

    pub fn restart(&mut self) -> Duration {
        Duration::ZERO
    }
}

/// `3725 s` gives `"1h 2m 5s"`. Under one second gives `"250ms"`.
pub fn format_duration(d: Duration) -> String {
    let total = d.as_secs();
    if total == 0 {
        return if d.subsec_millis() > 0 {
            format!("{}ms", d.subsec_millis())
        } else {
            "0s".into()
        };
    }
    let (days, h, m, s) = (
        total / 86_400,
        total % 86_400 / 3600,
        total % 3600 / 60,
        total % 60,
    );
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if h > 0 {
        parts.push(format!("{h}h"));
    }
    if m > 0 {
        parts.push(format!("{m}m"));
    }
    if s > 0 {
        parts.push(format!("{s}s"));
    }
    parts.join(" ")
}

/// Read text like `"1h30m"`, `"2d"`, `"45s"`, `"250ms"`. Returns `None` if it is not valid.
pub fn parse_duration(input: &str) -> Option<Duration> {
    let s = input.trim().to_lowercase();
    let mut total_ms: u128 = 0;
    let mut num = String::new();
    let mut any = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            num.push(c);
            continue;
        }
        if c.is_whitespace() {
            continue;
        }
        let n: u128 = num.parse().ok()?;
        num.clear();
        let unit_ms: u128 = match c {
            'd' => 86_400_000,
            'h' => 3_600_000,
            's' => 1000,
            'm' => {
                if chars.peek() == Some(&'s') {
                    chars.next();
                    1
                } else {
                    60_000
                }
            }
            _ => return None,
        };
        total_ms += n * unit_ms;
        any = true;
    }
    if !num.is_empty() || !any {
        return None;
    }
    Some(Duration::from_millis(total_ms as u64))
}

/// `human_ago(7200)` gives `"2 hours ago"`. Input is seconds passed.
pub fn human_ago(secs: u64) -> String {
    let (n, unit) = match secs {
        0..=59 => return "just now".into(),
        60..=3599 => (secs / 60, "minute"),
        3600..=86_399 => (secs / 3600, "hour"),
        _ => (secs / 86_400, "day"),
    };
    format!("{n} {unit}{} ago", if n == 1 { "" } else { "s" })
}

/// Unix seconds to `"YYYY-MM-DDTHH:MM:SSZ"` (UTC).
pub fn unix_to_iso8601(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

// Days since 1970-01-01 to (year, month, day). Algorithm by Howard Hinnant.
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting() {
        assert_eq!(format_duration(Duration::from_secs(3725)), "1h 2m 5s");
        assert_eq!(format_duration(Duration::from_millis(250)), "250ms");
        assert_eq!(format_duration(Duration::ZERO), "0s");
        assert_eq!(format_duration(Duration::from_secs(90_000)), "1d 1h");
    }

    #[test]
    fn parsing() {
        assert_eq!(parse_duration("1h30m"), Some(Duration::from_secs(5400)));
        assert_eq!(parse_duration("250ms"), Some(Duration::from_millis(250)));
        assert_eq!(parse_duration("2d"), Some(Duration::from_secs(172_800)));
        assert_eq!(parse_duration("10"), None);
        assert_eq!(parse_duration("h"), None);
        assert_eq!(parse_duration(""), None);
        assert_eq!(parse_duration("5x"), None);
    }

    #[test]
    fn ago() {
        assert_eq!(human_ago(10), "just now");
        assert_eq!(human_ago(60), "1 minute ago");
        assert_eq!(human_ago(7200), "2 hours ago");
        assert_eq!(human_ago(86_400 * 3), "3 days ago");
    }

    #[test]
    fn iso_dates() {
        assert_eq!(unix_to_iso8601(0), "1970-01-01T00:00:00Z");
        assert_eq!(unix_to_iso8601(1_700_000_000), "2023-11-14T22:13:20Z");
        assert_eq!(unix_to_iso8601(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(unix_to_iso8601(-1), "1969-12-31T23:59:59Z");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn clock_works() {
        assert!(now_unix_secs() > 1_700_000_000);
        let sw = Stopwatch::start();
        assert!(sw.elapsed() < Duration::from_secs(5));
    }
}

