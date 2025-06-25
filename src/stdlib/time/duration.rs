/// Duration and time arithmetic functionality
// use crate::stdlib::time::error::{TimeError, TimeResult, duration_error, arithmetic_overflow_error};
// use crate::stdlib::time::datetime::DateTime;
use crate::error::CursedError;

/// Represents a duration of time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Duration {
    seconds: i64,
    nanoseconds: i32, // 0 to 999,999,999
}

impl Duration {
    /// Create a new duration
    pub fn new(seconds: i64, nanoseconds: i32) -> TimeResult<Self> {
        if nanoseconds < 0 || nanoseconds >= 1_000_000_000 {
            return Err(duration_error("Nanoseconds must be between 0 and 999,999,999"));
        }
        
        Ok(Duration { seconds, nanoseconds })
    }
    
    /// Create duration from seconds
    pub fn from_seconds(seconds: i64) -> Self {
        Duration { seconds, nanoseconds: 0 }
    }
    
    /// Create duration from milliseconds
    pub fn from_milliseconds(milliseconds: i64) -> Self {
        let seconds = milliseconds / 1000;
        let nanos = ((milliseconds % 1000) * 1_000_000) as i32;
        Duration { seconds, nanoseconds: nanos }
    }
    
    /// Create duration from microseconds
    pub fn from_microseconds(microseconds: i64) -> Self {
        let seconds = microseconds / 1_000_000;
        let nanos = ((microseconds % 1_000_000) * 1000) as i32;
        Duration { seconds, nanoseconds: nanos }
    }
    
    /// Create duration from nanoseconds
    pub fn from_nanoseconds(nanoseconds: i64) -> Self {
        let seconds = nanoseconds / 1_000_000_000;
        let nanos = (nanoseconds % 1_000_000_000) as i32;
        Duration { seconds, nanoseconds: nanos }
    }
    
    /// Get total seconds (including fractional part)
    pub fn total_seconds(&self) -> i64 {
        self.seconds
    }
    
    /// Get total seconds as f64 (including nanoseconds)
    pub fn total_seconds_f64(&self) -> f64 {
        self.seconds as f64 + (self.nanoseconds as f64 / 1_000_000_000.0)
    }
    
    /// Get total milliseconds
    pub fn total_milliseconds(&self) -> i64 {
        self.seconds * 1000 + (self.nanoseconds / 1_000_000) as i64
    }
    
    /// Get total microseconds
    pub fn total_microseconds(&self) -> i64 {
        self.seconds * 1_000_000 + (self.nanoseconds / 1000) as i64
    }
    
    /// Get total nanoseconds
    pub fn total_nanoseconds(&self) -> i64 {
        self.seconds * 1_000_000_000 + self.nanoseconds as i64
    }
    
    /// Get the seconds component
    pub fn seconds(&self) -> i64 {
        self.seconds
    }
    
    /// Get the nanoseconds component
    pub fn nanoseconds(&self) -> i32 {
        self.nanoseconds
    }
    
    /// Check if duration is zero
    pub fn is_zero(&self) -> bool {
        self.seconds == 0 && self.nanoseconds == 0
    }
    
    /// Check if duration is positive
    pub fn is_positive(&self) -> bool {
        self.seconds > 0 || (self.seconds == 0 && self.nanoseconds > 0)
    }
    
    /// Check if duration is negative
    pub fn is_negative(&self) -> bool {
        self.seconds < 0
    }
    
    /// Get absolute value of duration
    pub fn abs(&self) -> Self {
        if self.is_negative() {
            Duration {
                seconds: -self.seconds,
                nanoseconds: if self.nanoseconds == 0 { 0 } else { 1_000_000_000 - self.nanoseconds },
            }
        } else {
            *self
        }
    }
    
    /// Add two durations
    pub fn add(&self, other: &Duration) -> TimeResult<Self> {
        let total_nanos = self.nanoseconds as i64 + other.nanoseconds as i64;
        let extra_seconds = total_nanos / 1_000_000_000;
        let remaining_nanos = (total_nanos % 1_000_000_000) as i32;
        
        let total_seconds = self.seconds
            .checked_add(other.seconds)
            .and_then(|s| s.checked_add(extra_seconds))
            .ok_or_else(|| arithmetic_overflow_error("duration addition", "seconds overflow"))?;
        
        Ok(Duration {
            seconds: total_seconds,
            nanoseconds: remaining_nanos,
        })
    }
    
    /// Subtract two durations
    pub fn subtract(&self, other: &Duration) -> TimeResult<Self> {
        let total_nanos = self.nanoseconds as i64 - other.nanoseconds as i64;
        let (extra_seconds, remaining_nanos) = if total_nanos < 0 {
            (-1, (total_nanos + 1_000_000_000) as i32)
        } else {
            (0, total_nanos as i32)
        };
        
        let total_seconds = self.seconds
            .checked_sub(other.seconds)
            .and_then(|s| s.checked_add(extra_seconds))
            .ok_or_else(|| arithmetic_overflow_error("duration subtraction", "seconds overflow"))?;
        
        Ok(Duration {
            seconds: total_seconds,
            nanoseconds: remaining_nanos,
        })
    }
    
    /// Multiply duration by a scalar
    pub fn multiply(&self, factor: i64) -> TimeResult<Self> {
        let total_nanos = self.total_nanoseconds()
            .checked_mul(factor)
            .ok_or_else(|| arithmetic_overflow_error("duration multiplication", "nanoseconds overflow"))?;
        
        Ok(Self::from_nanoseconds(total_nanos))
    }
    
    /// Divide duration by a scalar
    pub fn divide(&self, divisor: i64) -> TimeResult<Self> {
        if divisor == 0 {
            return Err(duration_error("Cannot divide duration by zero"));
        }
        
        let total_nanos = self.total_nanoseconds() / divisor;
        Ok(Self::from_nanoseconds(total_nanos))
    }
    
    /// Convert to human-readable string
    pub fn humanize(&self) -> String {
        if self.is_zero() {
            return "0 seconds".to_string();
        }
        
        let mut parts = Vec::new();
        let mut remaining = self.abs();
        
        // Days
        if remaining.seconds >= 86400 {
            let days = remaining.seconds / 86400;
            parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
            remaining.seconds %= 86400;
        }
        
        // Hours
        if remaining.seconds >= 3600 {
            let hours = remaining.seconds / 3600;
            parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
            remaining.seconds %= 3600;
        }
        
        // Minutes
        if remaining.seconds >= 60 {
            let minutes = remaining.seconds / 60;
            parts.push(format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" }));
            remaining.seconds %= 60;
        }
        
        // Seconds
        if remaining.seconds > 0 || remaining.nanoseconds > 0 {
            if remaining.nanoseconds == 0 {
                parts.push(format!("{} second{}", remaining.seconds, if remaining.seconds == 1 { "" } else { "s" }));
            } else {
                let total_seconds = remaining.seconds as f64 + (remaining.nanoseconds as f64 / 1_000_000_000.0);
                parts.push(format!("{:.3} seconds", total_seconds));
            }
        }
        
        let result = if parts.len() == 1 {
            parts[0].clone()
        } else if parts.len() == 2 {
            format!("{} and {}", parts[0], parts[1])
        } else {
            let last = parts.pop().unwrap();
            format!("{}, and {}", parts.join(", "), last)
        };
        
        if self.is_negative() {
            format!("-{}", result)
        } else {
            result
        }
    }
}

// Convenience functions for creating durations

/// Create duration from seconds
pub fn seconds(seconds: i64) -> Duration {
    Duration::from_seconds(seconds)
}

/// Create duration from minutes
pub fn minutes(minutes: i64) -> Duration {
    Duration::from_seconds(minutes * 60)
}

/// Create duration from hours
pub fn hours(hours: i64) -> Duration {
    Duration::from_seconds(hours * 3600)
}

/// Create duration from days
pub fn days(days: i64) -> Duration {
    Duration::from_seconds(days * 86400)
}

/// Create duration from weeks
pub fn weeks(weeks: i64) -> Duration {
    Duration::from_seconds(weeks * 604800)
}

/// Create duration from milliseconds
pub fn milliseconds(milliseconds: i64) -> Duration {
    Duration::from_milliseconds(milliseconds)
}

/// Create duration from microseconds
pub fn microseconds(microseconds: i64) -> Duration {
    Duration::from_microseconds(microseconds)
}

/// Create duration from nanoseconds
pub fn nanoseconds(nanoseconds: i64) -> Duration {
    Duration::from_nanoseconds(nanoseconds)
}

// Duration arithmetic functions

/// Calculate duration between two datetimes
pub fn duration_between(start: DateTime, end: DateTime) -> Duration {
    let start_timestamp = start.to_timestamp();
    let end_timestamp = end.to_timestamp();
    let diff = end_timestamp - start_timestamp;
    Duration::from_seconds(diff)
}

/// Calculate time until a future datetime
pub fn time_until(future: DateTime) -> TimeResult<Duration> {
//     let now = crate::stdlib::time::datetime::now()?;
    Ok(duration_between(now, future))
}

/// Calculate time since a past datetime
pub fn time_since(past: DateTime) -> TimeResult<Duration> {
//     let now = crate::stdlib::time::datetime::now()?;
    Ok(duration_between(past, now))
}

// Standard library implementations

impl std::ops::Add for Duration {
    type Output = TimeResult<Duration>;
    
    fn add(self, other: Duration) -> Self::Output {
        Duration::add(&self, &other)
    }
}

impl std::ops::Sub for Duration {
    type Output = TimeResult<Duration>;
    
    fn sub(self, other: Duration) -> Self::Output {
        Duration::subtract(&self, &other)
    }
}

impl std::ops::Mul<i64> for Duration {
    type Output = TimeResult<Duration>;
    
    fn mul(self, factor: i64) -> Self::Output {
        Duration::multiply(&self, factor)
    }
}

impl std::ops::Div<i64> for Duration {
    type Output = TimeResult<Duration>;
    
    fn div(self, divisor: i64) -> Self::Output {
        Duration::divide(&self, divisor)
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.humanize())
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.seconds.cmp(&other.seconds) {
            std::cmp::Ordering::Equal => self.nanoseconds.cmp(&other.nanoseconds),
            other => other,
        }
    }
}

