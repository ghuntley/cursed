use crate::error::Error;
/// Core date and time structures and functionality
use std::time::{SystemTime, UNIX_EPOCH};
use crate::stdlib::time::error::{TimeError, TimeResult, invalid_date_error, invalid_time_error, system_time_error};
use crate::stdlib::time::duration::Duration;

/// Represents a day of the week
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

impl Weekday {
    /// Get the weekday name
    pub fn name(&self) -> &'static str {
        match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }
    }
    
    /// Get the abbreviated weekday name
    pub fn abbrev(&self) -> &'static str {
        match self {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        }
    }
    
    /// Create weekday from number (1=Monday, 7=Sunday)
    pub fn from_number(n: u32) -> TimeResult<Self> {
        match n {
            1 => Ok(Weekday::Monday),
            2 => Ok(Weekday::Tuesday),
            3 => Ok(Weekday::Wednesday),
            4 => Ok(Weekday::Thursday),
            5 => Ok(Weekday::Friday),
            6 => Ok(Weekday::Saturday),
            7 => Ok(Weekday::Sunday),
            _ => Err(TimeError::General {
                message: format!("Invalid weekday number: {}", n),
            }),
        }
    }
    
    /// Get the next weekday
    pub fn next(&self) -> Self {
        match self {
            Weekday::Monday => Weekday::Tuesday,
            Weekday::Tuesday => Weekday::Wednesday,
            Weekday::Wednesday => Weekday::Thursday,
            Weekday::Thursday => Weekday::Friday,
            Weekday::Friday => Weekday::Saturday,
            Weekday::Saturday => Weekday::Sunday,
            Weekday::Sunday => Weekday::Monday,
        }
    }
    
    /// Get the previous weekday
    pub fn previous(&self) -> Self {
        match self {
            Weekday::Monday => Weekday::Sunday,
            Weekday::Tuesday => Weekday::Monday,
            Weekday::Wednesday => Weekday::Tuesday,
            Weekday::Thursday => Weekday::Wednesday,
            Weekday::Friday => Weekday::Thursday,
            Weekday::Saturday => Weekday::Friday,
            Weekday::Sunday => Weekday::Saturday,
        }
    }
}

/// Represents a month of the year
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    /// Get the month name
    pub fn name(&self) -> &'static str {
        match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }
    }
    
    /// Get the abbreviated month name
    pub fn abbrev(&self) -> &'static str {
        match self {
            Month::January => "Jan",
            Month::February => "Feb",
            Month::March => "Mar",
            Month::April => "Apr",
            Month::May => "May",
            Month::June => "Jun",
            Month::July => "Jul",
            Month::August => "Aug",
            Month::September => "Sep",
            Month::October => "Oct",
            Month::November => "Nov",
            Month::December => "Dec",
        }
    }
    
    /// Create month from number (1=January, 12=December)
    pub fn from_number(n: u32) -> TimeResult<Self> {
        match n {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(TimeError::General {
                message: format!("Invalid month number: {}", n),
            }),
        }
    }
    
    /// Get the next month
    pub fn next(&self) -> Self {
        match self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }
    
    /// Get the previous month
    pub fn previous(&self) -> Self {
        match self {
            Month::January => Month::December,
            Month::February => Month::January,
            Month::March => Month::February,
            Month::April => Month::March,
            Month::May => Month::April,
            Month::June => Month::May,
            Month::July => Month::June,
            Month::August => Month::July,
            Month::September => Month::August,
            Month::October => Month::September,
            Month::November => Month::October,
            Month::December => Month::November,
        }
    }
}

/// Represents a date (year, month, day)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    /// Create a new date
    pub fn new(year: i32, month: u32, day: u32) -> TimeResult<Self> {
        if month < 1 || month > 12 {
            return Err(invalid_date_error(year, month, day, "Month must be between 1 and 12"));
        }
        
        if day < 1 || day > days_in_month(year, month) {
            return Err(invalid_date_error(year, month, day, "Invalid day for the given month"));
        }
        
        Ok(Date { year, month, day })
    }
    
    /// Get the weekday for this date
    pub fn weekday(&self) -> Weekday {
        // Zeller's congruence algorithm
        let mut year = self.year;
        let mut month = self.month as i32;
        
        if month < 3 {
            month += 12;
            year -= 1;
        }
        
        let k = year % 100;
        let j = year / 100;
        
        let h = (self.day as i32 + (13 * (month + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        
        // Convert to our weekday system (1=Monday, 7=Sunday)
        let weekday_num = if h == 0 { 7 } else { ((h + 5) % 7) + 1 };
        
        Weekday::from_number(weekday_num as u32).unwrap()
    }
    
    /// Get the month enum
    pub fn month_enum(&self) -> Month {
        Month::from_number(self.month).unwrap()
    }
    
    /// Add days to this date
    pub fn add_days(&self, days: i32) -> TimeResult<Self> {
        let mut result = *self;
        
        if days > 0 {
            for _ in 0..days {
                result = result.next_day()?;
            }
        } else if days < 0 {
            for _ in 0..(-days) {
                result = result.previous_day()?;
            }
        }
        
        Ok(result)
    }
    
    /// Get the next day
    pub fn next_day(&self) -> TimeResult<Self> {
        let max_day = days_in_month(self.year, self.month);
        
        if self.day < max_day {
            Ok(Date {
                year: self.year,
                month: self.month,
                day: self.day + 1,
            })
        } else if self.month < 12 {
            Ok(Date {
                year: self.year,
                month: self.month + 1,
                day: 1,
            })
        } else {
            Ok(Date {
                year: self.year + 1,
                month: 1,
                day: 1,
            })
        }
    }
    
    /// Get the previous day
    pub fn previous_day(&self) -> TimeResult<Self> {
        if self.day > 1 {
            Ok(Date {
                year: self.year,
                month: self.month,
                day: self.day - 1,
            })
        } else if self.month > 1 {
            let prev_month = self.month - 1;
            Ok(Date {
                year: self.year,
                month: prev_month,
                day: days_in_month(self.year, prev_month),
            })
        } else {
            Ok(Date {
                year: self.year - 1,
                month: 12,
                day: 31,
            })
        }
    }
    
    /// Get day of year (1-366)
    pub fn day_of_year(&self) -> u32 {
        day_of_year(self.year, self.month, self.day)
    }
    
    /// Get week of year (1-53)
    pub fn week_of_year(&self) -> u32 {
        week_of_year(self.year, self.month, self.day)
    }
}

/// Represents a time (hour, minute, second, nanosecond)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub nanosecond: u32,
}

impl Time {
    /// Create a new time
    pub fn new(hour: u32, minute: u32, second: u32, nanosecond: u32) -> TimeResult<Self> {
        if hour >= 24 {
            return Err(invalid_time_error(hour, minute, second, "Hour must be less than 24"));
        }
        if minute >= 60 {
            return Err(invalid_time_error(hour, minute, second, "Minute must be less than 60"));
        }
        if second >= 60 {
            return Err(invalid_time_error(hour, minute, second, "Second must be less than 60"));
        }
        if nanosecond >= 1_000_000_000 {
            return Err(invalid_time_error(hour, minute, second, "Nanosecond must be less than 1,000,000,000"));
        }
        
        Ok(Time { hour, minute, second, nanosecond })
    }
    
    /// Create time from total seconds since midnight
    pub fn from_seconds(seconds: u32) -> TimeResult<Self> {
        if seconds >= 86400 {
            return Err(TimeError::General {
                message: "Seconds must be less than 86400".to_string(),
            });
        }
        
        let hour = seconds / 3600;
        let minute = (seconds % 3600) / 60;
        let second = seconds % 60;
        
        Ok(Time { hour, minute, second, nanosecond: 0 })
    }
    
    /// Convert to total seconds since midnight
    pub fn to_seconds(&self) -> u32 {
        self.hour * 3600 + self.minute * 60 + self.second
    }
    
    /// Convert to total nanoseconds since midnight
    pub fn to_nanoseconds(&self) -> u64 {
        (self.to_seconds() as u64) * 1_000_000_000 + (self.nanosecond as u64)
    }
}

/// Represents a complete date and time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    /// Create a new datetime
    pub fn new(date: Date, time: Time) -> Self {
        DateTime { date, time }
    }
    
    /// Create datetime from components
    pub fn from_components(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, nanosecond: u32) -> TimeResult<Self> {
        let date = Date::new(year, month, day)?;
        let time = Time::new(hour, minute, second, nanosecond)?;
        Ok(DateTime { date, time })
    }
    
    /// Create datetime from Unix timestamp
    pub fn from_timestamp(timestamp: i64) -> TimeResult<Self> {
        from_timestamp(timestamp)
    }
    
    /// Convert to Unix timestamp
    pub fn to_timestamp(&self) -> i64 {
        // Simple implementation - days since Unix epoch * seconds per day + time
        let days_since_epoch = days_since_unix_epoch(self.date.year, self.date.month, self.date.day);
        days_since_epoch * 86400 + self.time.to_seconds() as i64
    }
    
    /// Add duration to this datetime
    pub fn add_duration(&self, duration: Duration) -> TimeResult<Self> {
        let total_seconds = self.to_timestamp() + duration.total_seconds();
        Self::from_timestamp(total_seconds)
    }
    
    /// Subtract duration from this datetime
    pub fn subtract_duration(&self, duration: Duration) -> TimeResult<Self> {
        let total_seconds = self.to_timestamp() - duration.total_seconds();
        Self::from_timestamp(total_seconds)
    }
}

/// Represents an instant in time for measuring elapsed time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instant {
    nanos: u64,
}

impl Instant {
    /// Create an instant representing the current time
    pub fn now() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        Instant { nanos }
    }
    
    /// Get the duration since another instant
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        let nanos_diff = self.nanos.saturating_sub(earlier.nanos);
        Duration::from_nanoseconds(nanos_diff as i64)
    }
    
    /// Get elapsed time since this instant
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

// Global functions for getting current time

/// Get the current date and time in the local timezone
pub fn now() -> TimeResult<DateTime> {
    let system_time = SystemTime::now();
    let timestamp = system_time
        .duration_since(UNIX_EPOCH)
        .map_err(|e| system_time_error(&format!("Failed to get system time: {}", e)))?
        .as_secs() as i64;
    
    from_timestamp(timestamp)
}

/// Get the current date and time in UTC
pub fn utc_now() -> TimeResult<DateTime> {
    // For now, same as now() - timezone support will be added later
    now()
}

/// Get today's date
pub fn today() -> TimeResult<Date> {
    Ok(now()?.date)
}

/// Get tomorrow's date
pub fn tomorrow() -> TimeResult<Date> {
    today()?.add_days(1)
}

/// Get yesterday's date
pub fn yesterday() -> TimeResult<Date> {
    today()?.add_days(-1)
}

/// Create datetime from Unix timestamp (seconds since 1970-01-01 00:00:00 UTC)
pub fn from_timestamp(timestamp: i64) -> TimeResult<DateTime> {
    // Convert timestamp to date/time components
    let days_since_epoch = timestamp / 86400;
    let seconds_today = (timestamp % 86400) as u32;
    
    // Convert days since epoch to date
    let (year, month, day) = days_since_epoch_to_date(days_since_epoch)?;
    let date = Date::new(year, month, day)?;
    
    // Convert seconds to time
    let time = Time::from_seconds(seconds_today)?;
    
    Ok(DateTime { date, time })
}

/// Create datetime from Unix timestamp in milliseconds
pub fn from_timestamp_millis(timestamp_millis: i64) -> TimeResult<DateTime> {
    let timestamp = timestamp_millis / 1000;
    let millis = (timestamp_millis % 1000) as u32;
    let nanos = millis * 1_000_000;
    
    let mut datetime = from_timestamp(timestamp)?;
    datetime.time.nanosecond = nanos;
    
    Ok(datetime)
}

/// Create datetime from Unix timestamp in nanoseconds
pub fn from_timestamp_nanos(timestamp_nanos: i64) -> TimeResult<DateTime> {
    let timestamp = timestamp_nanos / 1_000_000_000;
    let nanos = (timestamp_nanos % 1_000_000_000) as u32;
    
    let mut datetime = from_timestamp(timestamp)?;
    datetime.time.nanosecond = nanos;
    
    Ok(datetime)
}

// Calendar utility functions

/// Check if a year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Get the number of days in a month
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0,
    }
}

/// Get the day of year (1-366)
pub fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let mut day_of_year = day;
    for m in 1..month {
        day_of_year += days_in_month(year, m);
    }
    day_of_year
}

/// Get the week of year (1-53)
pub fn week_of_year(year: i32, month: u32, day: u32) -> u32 {
    let day_of_year = day_of_year(year, month, day);
    let jan1_date = Date::new(year, 1, 1).unwrap();
    let jan1_weekday = jan1_date.weekday() as u32;
    
    // Adjust for Monday being day 1
    let days_to_first_monday = if jan1_weekday == 1 { 0 } else { 8 - jan1_weekday };
    
    if day_of_year <= days_to_first_monday {
        // This day is in the last week of the previous year
        week_of_year(year - 1, 12, 31)
    } else {
        ((day_of_year - days_to_first_monday - 1) / 7) + 2
    }
}

// Helper functions for timestamp conversion

fn days_since_unix_epoch(year: i32, month: u32, day: u32) -> i64 {
    // Simple implementation - count days from 1970-01-01
    let mut days = 0i64;
    
    // Add days for complete years
    for y in 1970..year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }
    
    // Add days for complete months in the target year
    for m in 1..month {
        days += days_in_month(year, m) as i64;
    }
    
    // Add remaining days
    days += (day - 1) as i64;
    
    days
}

fn days_since_epoch_to_date(days: i64) -> TimeResult<(i32, u32, u32)> {
    let mut remaining_days = days;
    let mut year = 1970;
    
    // Find the year
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    
    // Find the month
    let mut month = 1u32;
    while month <= 12 {
        let days_in_month = days_in_month(year, month);
        if remaining_days < days_in_month as i64 {
            break;
        }
        remaining_days -= days_in_month as i64;
        month += 1;
    }
    
    let day = (remaining_days + 1) as u32;
    
    Ok((year, month, day))
}
