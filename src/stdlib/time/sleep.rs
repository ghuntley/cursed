/// Sleep, timing, and delay utilities
// use crate::stdlib::time::error::{TimeError, TimeResult, time_error, system_time_error};
// use crate::stdlib::time::duration::Duration;
// use crate::stdlib::time::datetime::{DateTime, Instant};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::CursedError;

/// Sleep for the specified duration
pub fn sleep(duration: Duration) -> TimeResult<()> {
    if duration.is_negative() {
        return Err(time_error("Cannot sleep for negative duration"));
    let std_duration = std::time::Duration::new(
    );
    
    thread::sleep(std_duration);
    Ok(())
/// Sleep for the specified number of milliseconds
pub fn sleep_millis(milliseconds: u64) -> TimeResult<()> {
    let duration = Duration::from_milliseconds(milliseconds as i64);
    sleep(duration)
/// Sleep for the specified number of microseconds
pub fn sleep_micros(microseconds: u64) -> TimeResult<()> {
    let duration = Duration::from_microseconds(microseconds as i64);
    sleep(duration)
/// Sleep for the specified number of nanoseconds
pub fn sleep_nanos(nanoseconds: u64) -> TimeResult<()> {
    let duration = Duration::from_nanoseconds(nanoseconds as i64);
    sleep(duration)
/// Sleep until the specified datetime
pub fn sleep_until(target: DateTime) -> TimeResult<()> {
//     let now = crate::stdlib::time::datetime::now()?;
//     let duration = crate::stdlib::time::duration::duration_between(now, target);
    
    if duration.is_negative() {
        return Err(time_error("Target time is in the past"));
    sleep(duration)
/// Execute a function with a timeout
pub fn timeout<F, T>(duration: Duration, f: F) -> TimeResult<Option<T>>
where
{
    use std::sync::mpsc;
    
    if duration.is_negative() {
        return Err(time_error("Timeout duration cannot be negative"));
    let (sender, receiver) = mpsc::channel();
    
    // Spawn a thread to execute the function
    thread::spawn(move || {
        let result = f();
        let _ = sender.send(result); // Ignore send errors (timeout may have occurred)
    });
    
    // Wait for result with timeout
    let std_duration = std::time::Duration::new(
    );
    
    match receiver.recv_timeout(std_duration) {
    }
}

/// Delay execution (alias for sleep)
pub fn delay(duration: Duration) -> TimeResult<()> {
    sleep(duration)
/// Timer for measuring elapsed time and triggering events
#[derive(Debug)]
pub struct Timer {
impl Timer {
    /// Create a new timer
    pub fn new() -> Self {
        Timer {
        }
    }
    
    /// Create a timer with a specific duration
    pub fn with_duration(duration: Duration) -> Self {
        Timer {
        }
    }
    
    /// Create a named timer
    pub fn with_name(name: String) -> Self {
        Timer {
        }
    }
    
    /// Create a named timer with duration
    pub fn with_name_and_duration(name: String, duration: Duration) -> Self {
        Timer {
        }
    }
    
    /// Get elapsed time since timer creation
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    /// Check if the timer has expired (if duration was set)
    pub fn is_expired(&self) -> bool {
        if let Some(duration) = self.duration {
            self.elapsed() >= duration
        } else {
            false
        }
    }
    
    /// Get remaining time (if duration was set)
    pub fn remaining(&self) -> Option<Duration> {
        if let Some(duration) = self.duration {
            let elapsed = self.elapsed();
            if elapsed < duration {
                Some(duration.subtract(&elapsed).unwrap_or(Duration::from_seconds(0)))
            } else {
                Some(Duration::from_seconds(0))
            }
        } else {
            None
        }
    }
    
    /// Reset the timer
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    /// Wait until the timer expires
    pub fn wait(&self) -> TimeResult<()> {
        if let Some(remaining) = self.remaining() {
            if !remaining.is_zero() {
                sleep(remaining)?;
            }
        }
        Ok(())
    /// Get timer name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    /// Set timer name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// Stopwatch for precise time measurements
#[derive(Debug)]
pub struct Stopwatch {
impl Stopwatch {
    /// Create a new stopwatch
    pub fn new() -> Self {
        Stopwatch {
        }
    }
    
    /// Create a named stopwatch
    pub fn with_name(name: String) -> Self {
        Stopwatch {
        }
    }
    
    /// Start the stopwatch
    pub fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.is_running = true;
        }
    }
    
    /// Stop the stopwatch
    pub fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_time {
                self.accumulated_time = self.accumulated_time.add(&start.elapsed()).unwrap_or(self.accumulated_time);
            }
            self.is_running = false;
            self.start_time = None;
        }
    }
    
    /// Reset the stopwatch
    pub fn reset(&mut self) {
        self.start_time = None;
        self.accumulated_time = Duration::from_seconds(0);
        self.is_running = false;
        self.laps.clear();
    /// Restart the stopwatch (reset and start)
    pub fn restart(&mut self) {
        self.reset();
        self.start();
    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        let current_time = if self.is_running {
            if let Some(start) = self.start_time {
                start.elapsed()
            } else {
                Duration::from_seconds(0)
            }
        } else {
            Duration::from_seconds(0)
        
        self.accumulated_time.add(&current_time).unwrap_or(self.accumulated_time)
    /// Record a lap time
    pub fn lap(&mut self) -> Duration {
        let lap_time = self.elapsed();
        self.laps.push(lap_time);
        lap_time
    /// Get all lap times
    pub fn laps(&self) -> &[Duration] {
        &self.laps
    /// Get the number of laps
    pub fn lap_count(&self) -> usize {
        self.laps.len()
    /// Check if stopwatch is running
    pub fn is_running(&self) -> bool {
        self.is_running
    /// Get stopwatch name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    /// Set stopwatch name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    /// Get average lap time
    pub fn average_lap_time(&self) -> Option<Duration> {
        if self.laps.is_empty() {
            None
        } else {
            let total: Duration = self.laps.iter().fold(Duration::from_seconds(0), |acc, lap| {
                acc.add(lap).unwrap_or(acc)
            });
            total.divide(self.laps.len() as i64).ok()
        }
    }
    
    /// Get fastest lap time
    pub fn fastest_lap(&self) -> Option<Duration> {
        self.laps.iter().min().copied()
    /// Get slowest lap time
    pub fn slowest_lap(&self) -> Option<Duration> {
        self.laps.iter().max().copied()
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

/// High-precision system time in nanoseconds
pub fn system_time_nanos() -> TimeResult<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .map_err(|e| system_time_error(&format!("Failed to get system time: {}", e)))
/// Wait for a condition to become true with timeout
pub fn wait_for<F>(mut condition: F, timeout_duration: Duration, check_interval: Duration) -> TimeResult<bool>
where
{
    let start = Instant::now();
    
    while start.elapsed() < timeout_duration {
        if condition() {
            return Ok(true);
        }
        sleep(check_interval)?;
    Ok(false)
/// Execute a function and return the result with execution time
pub fn time_function<F, T>(f: F) -> TimeResult<(T, Duration)>
where
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    Ok((result, elapsed))
/// Measure execution time of a function multiple times
pub fn benchmark_function<F, T>(f: F, iterations: usize) -> TimeResult<(Vec<T>, Vec<Duration>, Duration)>
where
{
    let mut results = Vec::with_capacity(iterations);
    let mut times = Vec::with_capacity(iterations);
    let start_total = Instant::now();
    
    for _ in 0..iterations {
        let (result, time) = time_function(&f)?;
        results.push(result);
        times.push(time);
    let total_time = start_total.elapsed();
    Ok((results, times, total_time))
/// Retry a function with delays between attempts
pub fn retry_with_delay<F, T, E>(
) -> Result<T, E>
where
{
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        
        match f() {
            Err(e) => {
                if attempts >= max_attempts {
                    return Err(e);
                // Sleep before next attempt (ignore sleep errors)
                let _ = sleep(delay_between_attempts);
            }
        }
    }
}

