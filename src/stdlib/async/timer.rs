/// Async timer operations for CURSED stdlib
use std::time::{Duration, Instant};
use crate::runtime::r#async::{Future, delay as runtime_delay, timeout as runtime_timeout};
use std::future::Future as StdFuture;
use crate::stdlib::r#async::{AsyncError, AsyncResult};

/// Sleep for the specified duration
pub async fn sleep(duration: Duration) {
    runtime_delay(duration).await
}

/// Sleep until the specified instant
pub async fn sleep_until(deadline: Instant) {
    let now = Instant::now();
    if deadline > now {
        sleep(deadline - now).await
    }
}

/// Create an interval timer that fires repeatedly
pub fn interval(duration: Duration) -> Interval {
    Interval::new(duration)
}

/// Create a future that completes after the specified duration
pub fn after(duration: Duration) -> impl Future<Output = ()> {
    runtime_delay(duration)
}

/// Interval timer that fires repeatedly
pub struct Interval {
    duration: Duration,
    next_deadline: Instant,
    cancelled: bool,
}

impl Interval {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            next_deadline: Instant::now() + duration,
            cancelled: false,
        }
    }

    /// Tick the interval (wait for next fire)
    pub async fn tick(&mut self) -> Result<(), AsyncError> {
        if self.cancelled {
            return Err(AsyncError::Runtime("Interval cancelled".to_string()));
        }

        let now = Instant::now();
        if now < self.next_deadline {
            sleep(self.next_deadline - now).await;
        }

        // Update next deadline
        self.next_deadline = Instant::now() + self.duration;
        Ok(())
    }

    /// Cancel the interval
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    /// Check if the interval is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Reset the interval with a new duration
    pub fn reset(&mut self, duration: Duration) {
        self.duration = duration;
        self.next_deadline = Instant::now() + duration;
        self.cancelled = false;
    }
}

/// Timeout wrapper for futures
pub struct Timeout<F> {
    future: Option<F>,
    timeout_duration: Duration,
}

impl<F> Timeout<F>
where
    F: Future,
{
    pub fn new(duration: Duration, future: F) -> Self {
        Self {
            future: Some(future),
            timeout_duration: duration,
        }
    }
}

impl<F> Future for Timeout<F>
where
    F: Future,
{
    type Output = Result<F::Output, AsyncError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if let Some(future) = &mut this.future {
            // Use the runtime timeout wrapper
            let timeout_future = runtime_timeout(this.timeout_duration, unsafe {
                std::pin::Pin::new_unchecked(future)
            });

            let mut timeout_future = Box::pin(timeout_future);
            match timeout_future.as_mut().poll(cx) {
                std::task::Poll::Ready(Ok(result)) => {
                    this.future = None;
                    std::task::Poll::Ready(Ok(result))
                }
                std::task::Poll::Ready(Err(err)) => {
                    this.future = None;
                    std::task::Poll::Ready(Err(err.into()))
                }
                std::task::Poll::Pending => std::task::Poll::Pending,
            }
        } else {
            std::task::Poll::Ready(Err(AsyncError::Runtime("Future already completed".to_string())))
        }
    }
}

// Implement standard Future trait for Timeout to support .await syntax
impl<F> StdFuture for Timeout<F>
where
    F: Future,
{
    type Output = Result<F::Output, AsyncError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        // Delegate to the custom Future implementation
        Future::poll(self, cx)
    }
}

/// Timeout utilities
pub mod timeout_utils {
    use super::*;

    /// Add a timeout to any future
    pub fn with_timeout<F>(duration: Duration, future: F) -> Timeout<F>
    where
        F: Future,
    {
        Timeout::new(duration, future)
    }

    /// Race a future against a timeout
    pub async fn race_timeout<F, T>(
        duration: Duration,
        future: F,
    ) -> Result<T, AsyncError>
    where
        F: Future<Output = T>,
    {
        with_timeout(duration, future).await
    }

    /// Apply timeout with custom error message
    pub async fn timeout_with_message<F, T>(
        duration: Duration,
        future: F,
        message: &str,
    ) -> Result<T, AsyncError>
    where
        F: Future<Output = T>,
    {
        match with_timeout(duration, future).await {
            Ok(result) => Ok(result),
            Err(AsyncError::Timeout) => Err(AsyncError::Runtime(message.to_string())),
            Err(other) => Err(other),
        }
    }
}

/// Deadline utilities
pub mod deadline {
    use super::*;

    /// Create a deadline from now + duration
    pub fn from_now(duration: Duration) -> Instant {
        Instant::now() + duration
    }

    /// Create a deadline at a specific instant
    pub fn at(instant: Instant) -> Instant {
        instant
    }

    /// Check if a deadline has passed
    pub fn has_passed(deadline: Instant) -> bool {
        Instant::now() >= deadline
    }

    /// Get time remaining until deadline
    pub fn time_remaining(deadline: Instant) -> Option<Duration> {
        let now = Instant::now();
        if deadline > now {
            Some(deadline - now)
        } else {
            None
        }
    }

    /// Sleep until deadline
    pub async fn sleep_until_deadline(deadline: Instant) {
        if let Some(duration) = time_remaining(deadline) {
            sleep(duration).await;
        }
    }
}

/// Rate limiting utilities
pub mod rate_limit {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Simple rate limiter
    pub struct RateLimiter {
        max_per_interval: usize,
        interval: Duration,
        tokens: Arc<Mutex<usize>>,
        last_refill: Arc<Mutex<Instant>>,
    }

    impl RateLimiter {
        pub fn new(max_per_interval: usize, interval: Duration) -> Self {
            Self {
                max_per_interval,
                interval,
                tokens: Arc::new(Mutex::new(max_per_interval)),
                last_refill: Arc::new(Mutex::new(Instant::now())),
            }
        }

        /// Wait for a token (rate limit)
        pub async fn acquire(&self) -> Result<(), AsyncError> {
            loop {
                {
                    let now = Instant::now();
                    let mut last_refill = self.last_refill.lock().unwrap();
                    let mut tokens = self.tokens.lock().unwrap();

                    // Refill tokens if interval has passed
                    if now.duration_since(*last_refill) >= self.interval {
                        *tokens = self.max_per_interval;
                        *last_refill = now;
                    }

                    // Try to consume a token
                    if *tokens > 0 {
                        *tokens -= 1;
                        return Ok(());
                    }
                }

                // Wait a bit before trying again
                sleep(Duration::from_millis(10)).await;
            }
        }

        /// Try to acquire a token without waiting
        pub fn try_acquire(&self) -> bool {
            let now = Instant::now();
            let mut last_refill = self.last_refill.lock().unwrap();
            let mut tokens = self.tokens.lock().unwrap();

            // Refill tokens if interval has passed
            if now.duration_since(*last_refill) >= self.interval {
                *tokens = self.max_per_interval;
                *last_refill = now;
            }

            // Try to consume a token
            if *tokens > 0 {
                *tokens -= 1;
                true
            } else {
                false
            }
        }
    }

    impl Clone for RateLimiter {
        fn clone(&self) -> Self {
            Self {
                max_per_interval: self.max_per_interval,
                interval: self.interval,
                tokens: self.tokens.clone(),
                last_refill: self.last_refill.clone(),
            }
        }
    }
}

pub use timeout_utils::*;
pub use deadline::*;
pub use rate_limit::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_creation() {
        let mut interval = Interval::new(Duration::from_millis(100));
        assert!(!interval.is_cancelled());
        
        interval.cancel();
        assert!(interval.is_cancelled());
    }

    #[test]
    fn test_deadline_utils() {
        let deadline = from_now(Duration::from_millis(100));
        assert!(!has_passed(deadline));
        assert!(time_remaining(deadline).is_some());
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(5, Duration::from_secs(1));
        
        // Should be able to acquire initially
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        
        // Should be empty now
        assert!(!limiter.try_acquire());
    }
}
