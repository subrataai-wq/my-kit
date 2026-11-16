//! Debounce helper for rate-limiting rapid inputs. Zero external dependencies and web safe.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Debouncer {
    delay_ms: u64,
    last_trigger_ms: Option<u64>,
}

impl Debouncer {
    pub fn new(delay_ms: u64) -> Self {
        Self {
            delay_ms,
            last_trigger_ms: None,
        }
    }

    /// Check if enough time has elapsed since the last execution.
    pub fn should_execute(&mut self, current_time_ms: u64) -> bool {
        if let Some(last) = self.last_trigger_ms {
            if current_time_ms.saturating_sub(last) >= self.delay_ms {
                self.last_trigger_ms = Some(current_time_ms);
                true
            } else {
                false
            }
        } else {
            self.last_trigger_ms = Some(current_time_ms);
            true
        }
    }

    pub fn reset(&mut self) {
        self.last_trigger_ms = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debounce_rate_limits() {
        let mut d = Debouncer::new(300);
        assert!(d.should_execute(1000));
        assert!(!d.should_execute(1100));
        assert!(!d.should_execute(1299));
        assert!(d.should_execute(1300));
        assert!(!d.should_execute(1350));
    }
}

