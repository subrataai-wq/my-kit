//! Small math helpers.

/// Keep `value` between `lo` and `hi`. Works for numbers of any type.
pub fn clamp<T: PartialOrd>(value: T, lo: T, hi: T) -> T {
    if value < lo {
        lo
    } else if value > hi {
        hi
    } else {
        value
    }
}

/// Point between `a` and `b`. `t = 0.0` gives `a`, `t = 1.0` gives `b`.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Move a number from one range to another. Example: 5 in 0..10 becomes 50 in 0..100.
pub fn map_range(value: f64, in_lo: f64, in_hi: f64, out_lo: f64, out_hi: f64) -> f64 {
    if (in_hi - in_lo).abs() < f64::EPSILON {
        return out_lo;
    }
    out_lo + (value - in_lo) * (out_hi - out_lo) / (in_hi - in_lo)
}

/// `part` as a percent of `total`. Gives `0.0` when total is zero.
pub fn percent(part: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        part / total * 100.0
    }
}

pub fn round_to(value: f64, places: u32) -> f64 {
    let factor = 10f64.powi(places as i32);
    (value * factor).round() / factor
}

pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() <= epsilon
}

/// Divide without panic. `None` when `b` is zero.
pub fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = v.len() / 2;
    Some(if v.len().is_multiple_of(2) {
        (v[mid - 1] + v[mid]) / 2.0
    } else {
        v[mid]
    })
}

pub fn min_max(values: &[f64]) -> Option<(f64, f64)> {
    let mut it = values.iter().copied();
    let first = it.next()?;
    Some(it.fold((first, first), |(lo, hi), x| (lo.min(x), hi.max(x))))
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(clamp(15, 0, 10), 10);
        assert_eq!(clamp(-1.5, 0.0, 1.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(percent(1.0, 4.0), 25.0);
        assert_eq!(percent(1.0, 0.0), 0.0);
        assert_eq!(round_to(12.3456, 2), 12.35);
        assert!(approx_eq(0.1 + 0.2, 0.3, 1e-9));
        assert_eq!(safe_div(1.0, 0.0), None);
    }

    #[test]
    fn stats() {
        assert_eq!(mean(&[1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(&[4.0, 1.0, 2.0, 3.0]), Some(2.5));
        assert_eq!(min_max(&[3.0, -1.0, 7.0]), Some((-1.0, 7.0)));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn integers() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(0, 6), 0);
    }
}
