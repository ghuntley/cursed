use crate::error::CursedError;
/// Time and date handling library for CURSED
/// 
/// This module provides comprehensive time, date, and duration functionality
/// including parsing, formatting, arithmetic, timezone support, and benchmarking.

pub mod datetime;
pub mod duration;
pub mod formatting;
pub mod timezone;
pub mod relative;
pub mod sleep;
pub mod benchmarking;
pub mod error;

// Re-export core types and functions
pub use error::{TimeError, TimeResult, time_error, parse_error, invalid_date_error, timezone_error};

// Core date/time structures
pub use datetime::{
    days_in_month, is_leap_year, day_of_year, week_of_year
// };

// Duration and time arithmetic
pub use duration::{
    duration_between, time_until, time_since
// };

// Formatting and parsing
pub use formatting::{
    format_rfc3339, parse_rfc3339, format_custom, parse_custom
// };

// Timezone support
pub use timezone::{
    convert_timezone, get_timezone_offset, list_timezones
// };

// Relative time calculations
pub use relative::{
    next_occurrence, previous_occurrence
// };

// Sleep and timing utilities
pub use sleep::{
    sleep_until, timeout, delay, Timer, Stopwatch
// };

// Benchmarking and performance measurement
pub use benchmarking::{
    PerformanceCounter
// };

pub use sleep::system_time_nanos;

// Common constants
pub const SECONDS_PER_MINUTE: i64 = 60;
pub const SECONDS_PER_HOUR: i64 = 3600;
pub const SECONDS_PER_DAY: i64 = 86400;
pub const SECONDS_PER_WEEK: i64 = 604800;
pub const MILLISECONDS_PER_SECOND: i64 = 1000;
pub const MICROSECONDS_PER_SECOND: i64 = 1_000_000;
pub const NANOSECONDS_PER_SECOND: i64 = 1_000_000_000;

// Unix epoch constants
pub const UNIX_EPOCH_YEAR: i32 = 1970;
pub const UNIX_EPOCH_MONTH: u32 = 1;
pub const UNIX_EPOCH_DAY: u32 = 1;

// Calendar constants
pub const DAYS_PER_WEEK: u32 = 7;
pub const MONTHS_PER_YEAR: u32 = 12;
pub const HOURS_PER_DAY: u32 = 24;
pub const MINUTES_PER_HOUR: u32 = 60;

// Leap year constants
pub const DAYS_PER_REGULAR_YEAR: u32 = 365;
pub const DAYS_PER_LEAP_YEAR: u32 = 366;
pub const LEAP_YEAR_CYCLE: u32 = 4;
pub const CENTURY_CYCLE: u32 = 100;
pub const QUAD_CENTURY_CYCLE: u32 = 400;
