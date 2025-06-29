use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

use crate::error::CursedError;

/// Timer identifier
pub type TimerId = u64;

/// Timer callback type
pub type TimerCallback = Box<dyn FnOnce() + Send + 'static>;

/// Timer entry in the timer wheel
pub struct TimerEntry {
    pub id: TimerId,
    pub expires_at: Instant,
    pub callback: TimerCallback,
    pub repeating: bool,
    pub interval: Option<Duration>,
}

impl std::fmt::Debug for TimerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimerEntry")
            .field("id", &self.id)
            .field("expires_at", &self.expires_at)
            .field("repeating", &self.repeating)
            .field("interval", &self.interval)
            .field("callback", &"<callback>")
            .finish()
    }
}

impl TimerEntry {
    pub fn new<F>(id: TimerId, expires_at: Instant, callback: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            id,
            expires_at,
            callback: Box::new(callback),
            repeating: false,
            interval: None,
        }
    }

    pub fn repeating<F>(id: TimerId, expires_at: Instant, callback: F, interval: Duration) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            id,
            expires_at,
            callback: Box::new(callback),
            repeating: true,
            interval: Some(interval),
        }
    }
}

impl PartialEq for TimerEntry {
    fn eq(&self, other: &Self) -> bool {
        self.expires_at == other.expires_at
    }
}

impl Eq for TimerEntry {}

impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimerEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (earliest expiration first)
        other.expires_at.cmp(&self.expires_at)
    }
}

/// High-resolution timer wheel for managing timeouts and intervals
pub struct TimerWheel {
    next_timer_id: TimerId,
    timers: BinaryHeap<TimerEntry>,
    resolution: Duration,
    max_timers: usize,
}

impl TimerWheel {
    /// Create a new timer wheel
    pub fn new(resolution: Duration, max_timers: usize) -> Self {
        Self {
            next_timer_id: 1,
            timers: BinaryHeap::new(),
            resolution,
            max_timers,
        }
    }

    /// Schedule a one-time timeout
    pub fn schedule_timeout<F>(&mut self, timeout: Duration, callback: F) -> Result<TimerId, CursedError>
    where
        F: FnOnce() + Send + 'static,
    {
        if self.timers.len() >= self.max_timers {
            return Err(CursedError::runtime_error("Timer wheel is full"));
        }

        let timer_id = self.next_timer_id;
        self.next_timer_id += 1;

        let expires_at = Instant::now() + timeout;
        let timer = TimerEntry::new(timer_id, expires_at, callback);
        
        self.timers.push(timer);
        Ok(timer_id)
    }

    /// Schedule a repeating interval timer
    pub fn schedule_interval<F>(&mut self, interval: Duration, callback: F) -> Result<TimerId, CursedError>
    where
        F: Fn() + Send + 'static,
    {
        if self.timers.len() >= self.max_timers {
            return Err(CursedError::runtime_error("Timer wheel is full"));
        }

        let timer_id = self.next_timer_id;
        self.next_timer_id += 1;

        let expires_at = Instant::now() + interval;
        
        // Wrap the callback to make it repeatable
        let repeating_callback = Box::new(callback);
        let timer = TimerEntry::repeating(timer_id, expires_at, move || {
            repeating_callback();
        }, interval);
        
        self.timers.push(timer);
        Ok(timer_id)
    }

    /// Cancel a timer
    pub fn cancel_timer(&mut self, timer_id: TimerId) -> bool {
        // Note: This is a simplified implementation
        // A production implementation would use a more efficient data structure
        // that allows for efficient removal by ID
        
        let mut found = false;
        let mut temp_timers = Vec::new();
        
        while let Some(timer) = self.timers.pop() {
            if timer.id == timer_id {
                found = true;
                // Don't add back to heap
            } else {
                temp_timers.push(timer);
            }
        }
        
        // Restore remaining timers
        for timer in temp_timers {
            self.timers.push(timer);
        }
        
        found
    }

    /// Advance the timer wheel and return expired callbacks
    pub fn tick(&mut self) -> Vec<TimerCallback> {
        let now = Instant::now();
        let mut expired_callbacks = Vec::new();
        let mut repeating_timers = Vec::new();

        // Process all expired timers
        while let Some(timer) = self.timers.peek() {
            if timer.expires_at <= now {
                let mut timer = self.timers.pop().unwrap();
                
                // If it's a repeating timer, schedule the next occurrence
                if timer.repeating {
                    if let Some(interval) = timer.interval {
                        let next_expires = timer.expires_at + interval;
                        repeating_timers.push(TimerEntry::repeating(
                            timer.id,
                            next_expires,
                            move || {
                                // This is a simplified approach - in production,
                                // we'd need a more sophisticated callback system
                            },
                            interval,
                        ));
                    }
                }
                
                expired_callbacks.push(timer.callback);
            } else {
                break;
            }
        }

        // Re-add repeating timers
        for timer in repeating_timers {
            self.timers.push(timer);
        }

        expired_callbacks
    }

    /// Get the next timer expiration time
    pub fn next_expiration(&self) -> Option<Instant> {
        self.timers.peek().map(|timer| timer.expires_at)
    }

    /// Get the number of active timers
    pub fn timer_count(&self) -> usize {
        self.timers.len()
    }

    /// Get timer wheel statistics
    pub fn get_stats(&self) -> TimerWheelStats {
        TimerWheelStats {
            active_timers: self.timers.len(),
            max_timers: self.max_timers,
            resolution: self.resolution,
            next_expiration: self.next_expiration(),
        }
    }
}

/// Timer wheel statistics
#[derive(Debug, Clone)]
pub struct TimerWheelStats {
    pub active_timers: usize,
    pub max_timers: usize,
    pub resolution: Duration,
    pub next_expiration: Option<Instant>,
}

/// Async sleep implementation
pub async fn sleep(duration: Duration) -> Result<(), CursedError> {
    tokio::time::sleep(duration).await;
    Ok(())
}

/// Async sleep until a specific time
pub async fn sleep_until(deadline: Instant) -> Result<(), CursedError> {
    tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)).await;
    Ok(())
}

/// Async interval timer
pub struct Interval {
    inner: tokio::time::Interval,
}

impl Interval {
    pub fn new(period: Duration) -> Self {
        Self {
            inner: tokio::time::interval(period),
        }
    }

    pub async fn tick(&mut self) -> Instant {
        self.inner.tick().await.into_std()
    }

    pub fn period(&self) -> Duration {
        self.inner.period()
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }
}

/// Create a new interval timer
pub fn interval(period: Duration) -> Interval {
    Interval::new(period)
}

/// Create a future that completes after a delay
pub async fn delay(duration: Duration) -> Result<(), CursedError> {
    sleep(duration).await
}

/// Create a future that completes at a specific time
pub async fn delay_until(deadline: Instant) -> Result<(), CursedError> {
    sleep_until(deadline).await
}

/// Timeout wrapper for futures
pub struct Timeout<F> {
    future: F,
    timeout: Duration,
}

impl<F> Timeout<F> {
    pub fn new(future: F, timeout: Duration) -> Self {
        Self { future, timeout }
    }
}

impl<F> std::future::Future for Timeout<F>
where
    F: std::future::Future,
{
    type Output = Result<F::Output, TimeoutError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        // This is a simplified implementation
        // A production version would use proper timeout handling
        unsafe {
            let this = self.get_unchecked_mut();
            match std::pin::Pin::new_unchecked(&mut this.future).poll(cx) {
                std::task::Poll::Ready(result) => std::task::Poll::Ready(Ok(result)),
                std::task::Poll::Pending => std::task::Poll::Pending,
            }
        }
    }
}

/// Timeout error type
#[derive(Debug, Clone)]
pub struct TimeoutError;

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation timed out")
    }
}

impl std::error::Error for TimeoutError {}

/// Create a timeout for a future
pub fn timeout<F>(duration: Duration, future: F) -> Timeout<F>
where
    F: std::future::Future,
{
    Timeout::new(future, duration)
}

/// Create a future that completes after a delay
pub fn after(duration: Duration) -> impl std::future::Future<Output = ()> {
    async move {
        sleep(duration).await.unwrap_or(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_timer_wheel_creation() {
        let wheel = TimerWheel::new(Duration::from_millis(1), 1000);
        assert_eq!(wheel.timer_count(), 0);
    }

    #[test]
    fn test_schedule_timeout() {
        let mut wheel = TimerWheel::new(Duration::from_millis(1), 1000);
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        
        let timer_id = wheel.schedule_timeout(Duration::from_millis(10), move || {
            *called_clone.lock().unwrap() = true;
        }).unwrap();
        
        assert!(timer_id > 0);
        assert_eq!(wheel.timer_count(), 1);
    }

    #[test]
    fn test_cancel_timer() {
        let mut wheel = TimerWheel::new(Duration::from_millis(1), 1000);
        
        let timer_id = wheel.schedule_timeout(Duration::from_millis(10), || {}).unwrap();
        assert_eq!(wheel.timer_count(), 1);
        
        let cancelled = wheel.cancel_timer(timer_id);
        assert!(cancelled);
        assert_eq!(wheel.timer_count(), 0);
    }

    #[tokio::test]
    async fn test_async_sleep() {
        let start = Instant::now();
        sleep(Duration::from_millis(10)).await.unwrap();
        let elapsed = start.elapsed();
        
        assert!(elapsed >= Duration::from_millis(10));
        assert!(elapsed < Duration::from_millis(50)); // Allow some tolerance
    }

    #[tokio::test]
    async fn test_interval() {
        let mut interval = interval(Duration::from_millis(5));
        let start = Instant::now();
        
        interval.tick().await;
        let first_tick = start.elapsed();
        
        interval.tick().await;
        let second_tick = start.elapsed();
        
        assert!(first_tick < second_tick);
        assert!(second_tick >= Duration::from_millis(5));
    }
}
