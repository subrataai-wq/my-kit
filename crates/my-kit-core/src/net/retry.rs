//! Asynchronous retry policy with exponential backoff.

use std::future::Future;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RetryPolicy {
    pub max_attempts: usize,
    pub initial_delay_ms: u64,
    pub backoff_factor: f64,
    pub max_delay_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            backoff_factor: 2.0,
            max_delay_ms: 2000,
        }
    }
}

/// Execute an async operation with exponential backoff retry.
pub async fn retry_async<T, E, F, Fut>(policy: &RetryPolicy, mut op: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    let mut delay_ms = policy.initial_delay_ms;

    loop {
        attempt += 1;
        match op().await {
            Ok(val) => return Ok(val),
            Err(e) if attempt >= policy.max_attempts => return Err(e),
            Err(_) => {
                #[cfg(feature = "async-timer")]
                {
                    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                }
                let next_delay = (delay_ms as f64 * policy.backoff_factor) as u64;
                delay_ms = next_delay.min(policy.max_delay_ms);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_policy_defaults() {
        let p = RetryPolicy::default();
        assert_eq!(p.max_attempts, 3);
        assert_eq!(p.initial_delay_ms, 100);
    }
}

