/// Date/time formatting and parsing functionality
use crate::stdlib::time::error::{TimeError, TimeResult, parse_error, format_error};
use crate::stdlib::time::datetime::{DateTime, Date, Time};
use crate::stdlib::time::duration::Duration;

/// Standard date/time format patterns
#[derive(Debug, Clone, PartialEq)]
pub enum DateTimeFormat {
    /// ISO 8601 format: 2023-12-25T15:30:45
    Iso8601,
    /// RFC 3339 format: 2023-12-25T15:30:45Z
    Rfc3339,
    /// RFC 2822 format: Mon, 25 Dec 2023 15:30:45 +0000
    Rfc2822,
    /// US format: 12/25/2023 3:30:45 PM
    Us,
    /// European format: 25/12/2023 15:30:45
    European,
    /// Unix timestamp: 1703519445
    UnixTimestamp,
    /// Custom format string
    Custom(String),
}

impl DateTimeFormat {
    /// Get the format string for this format
    pub fn format_string(&self) -> &str {
        match self {
            DateTimeFormat::Iso8601 => "%Y-%m-%dT%H:%M:%S",
            DateTimeFormat::Rfc3339 => "%Y-%m-%dT%H:%M:%SZ",
            DateTimeFormat::Rfc2822 => "%a, %d %b %Y %H:%M:%S +0000",
            DateTimeFormat::Us => "%m/%d/%Y %I:%M:%S %p",
            DateTimeFormat::European => "%d/%m/%Y %H:%M:%S",
            DateTimeFormat::UnixTimestamp => "%s",
            DateTimeFormat::Custom(format) => format,
        }
    }
}

/// Format a DateTime to string
pub fn format_datetime(datetime: &DateTime, format: &DateTimeFormat) -> TimeResult<String> {
    match format {
        DateTimeFormat::Iso8601 => format_iso8601(datetime),
        DateTimeFormat::Rfc3339 => format_rfc3339(datetime),
        DateTimeFormat::Rfc2822 => format_rfc2822(datetime),
        DateTimeFormat::Us => format_us(datetime),
        DateTimeFormat::European => format_european(datetime),
        DateTimeFormat::UnixTimestamp => Ok(datetime.to_timestamp().to_string()),
        DateTimeFormat::Custom(format_str) => format_custom(datetime, format_str),
    }
}

/// Parse a DateTime from string
pub fn parse_datetime(input: &str, format: &DateTimeFormat) -> TimeResult<DateTime> {
    match format {
        DateTimeFormat::Iso8601 => parse_iso8601(input),
        DateTimeFormat::Rfc3339 => parse_rfc3339(input),
        DateTimeFormat::Rfc2822 => parse_rfc2822(input),
        DateTimeFormat::Us => parse_us(input),
        DateTimeFormat::European => parse_european(input),
        DateTimeFormat::UnixTimestamp => parse_unix_timestamp(input),
        DateTimeFormat::Custom(format_str) => parse_custom(input, format_str),
    }
}

/// Format a Date to string
pub fn format_date(date: &Date, format: &str) -> TimeResult<String> {
    format_with_pattern(&format!("{:04}-{:02}-{:02}", date.year, date.month, date.day), format)
}

/// Parse a Date from string
pub fn parse_date(input: &str, format: &str) -> TimeResult<Date> {
    // Simple implementation for common formats
    if format == "%Y-%m-%d" || format == "YYYY-MM-DD" {
        parse_iso_date(input)
    } else if format == "%m/%d/%Y" || format == "MM/DD/YYYY" {
        parse_us_date(input)
    } else if format == "%d/%m/%Y" || format == "DD/MM/YYYY" {
        parse_european_date(input)
    } else {
        Err(format_error(format, "Unsupported date format"))
    }
}

/// Format a Time to string
pub fn format_time(time: &Time, format: &str) -> TimeResult<String> {
    match format {
        "%H:%M:%S" | "HH:MM:SS" => Ok(format!("{:02}:{:02}:{:02}", time.hour, time.minute, time.second)),
        "%I:%M:%S %p" | "hh:MM:SS AM/PM" => {
            let (hour_12, am_pm) = if time.hour == 0 {
                (12, "AM")
            } else if time.hour < 12 {
                (time.hour, "AM")
            } else if time.hour == 12 {
                (12, "PM")
            } else {
                (time.hour - 12, "PM")
            };
            Ok(format!("{:02}:{:02}:{:02} {}", hour_12, time.minute, time.second, am_pm))
        }
        "%H:%M" | "HH:MM" => Ok(format!("{:02}:{:02}", time.hour, time.minute)),
        _ => Err(format_error(format, "Unsupported time format")),
    }
}

/// Parse a Time from string
pub fn parse_time(input: &str, format: &str) -> TimeResult<Time> {
    match format {
        "%H:%M:%S" | "HH:MM:SS" => parse_24_hour_time(input),
        "%I:%M:%S %p" | "hh:MM:SS AM/PM" => parse_12_hour_time(input),
        "%H:%M" | "HH:MM" => parse_24_hour_time_short(input),
        _ => Err(format_error(format, "Unsupported time format")),
    }
}

/// Format a Duration to human-readable string
pub fn format_duration(duration: &Duration) -> String {
    duration.humanize()
}

/// Parse a Duration from string (e.g., "2h 30m", "1d 4h", "45s")
pub fn parse_duration(input: &str) -> TimeResult<Duration> {
    let input = input.trim();
    if input.is_empty() {
        return Err(parse_error(input, "duration", "Empty input"));
    }
    
    let mut total_seconds = 0i64;
    let mut current_num = String::new();
    
    for ch in input.chars() {
        if ch.is_ascii_digit() {
            current_num.push(ch);
        } else if ch.is_alphabetic() {
            if current_num.is_empty() {
                return Err(parse_error(input, "duration", "Missing number before unit"));
            }
            
            let num: i64 = current_num.parse()
                .map_err(|_| parse_error(input, "duration", "Invalid number"))?;
            
            let multiplier = match ch.to_ascii_lowercase() {
                's' => 1,                    // seconds
                'm' => 60,                   // minutes
                'h' => 3600,                 // hours
                'd' => 86400,                // days
                'w' => 604800,               // weeks
                _ => return Err(parse_error(input, "duration", &format!("Unknown unit: {}", ch))),
            };
            
            total_seconds += num * multiplier;
            current_num.clear();
        } else if ch.is_whitespace() {
            // Skip whitespace
            continue;
        } else {
            return Err(parse_error(input, "duration", &format!("Invalid character: {}", ch)));
        }
    }
    
    // Handle case where input ends with a number (assume seconds)
    if !current_num.is_empty() {
        let num: i64 = current_num.parse()
            .map_err(|_| parse_error(input, "duration", "Invalid number"))?;
        total_seconds += num;
    }
    
    Ok(Duration::from_seconds(total_seconds))
}

// Specific format implementations

/// Format DateTime in ISO 8601 format (2023-12-25T15:30:45)
pub fn format_iso8601(datetime: &DateTime) -> TimeResult<String> {
    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        datetime.date.year,
        datetime.date.month,
        datetime.date.day,
        datetime.time.hour,
        datetime.time.minute,
        datetime.time.second
    ))
}

/// Parse DateTime from ISO 8601 format
pub fn parse_iso8601(input: &str) -> TimeResult<DateTime> {
    let parts: Vec<&str> = input.split('T').collect();
    if parts.len() != 2 {
        return Err(parse_error(input, "ISO 8601", "Expected format: YYYY-MM-DDTHH:MM:SS"));
    }
    
    let date = parse_iso_date(parts[0])?;
    let time = parse_iso_time(parts[1])?;
    
    Ok(DateTime::new(date, time))
}

/// Format DateTime in RFC 3339 format (2023-12-25T15:30:45Z)
pub fn format_rfc3339(datetime: &DateTime) -> TimeResult<String> {
    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        datetime.date.year,
        datetime.date.month,
        datetime.date.day,
        datetime.time.hour,
        datetime.time.minute,
        datetime.time.second
    ))
}

/// Parse DateTime from RFC 3339 format
pub fn parse_rfc3339(input: &str) -> TimeResult<DateTime> {
    let input = input.trim_end_matches('Z');
    parse_iso8601(input)
}

/// Format DateTime in RFC 2822 format (Mon, 25 Dec 2023 15:30:45 +0000)
pub fn format_rfc2822(datetime: &DateTime) -> TimeResult<String> {
    let weekday = datetime.date.weekday().abbrev();
    let month = datetime.date.month_enum().abbrev();
    
    Ok(format!(
        "{}, {:02} {} {} {:02}:{:02}:{:02} +0000",
        weekday,
        datetime.date.day,
        month,
        datetime.date.year,
        datetime.time.hour,
        datetime.time.minute,
        datetime.time.second
    ))
}

/// Parse DateTime from RFC 2822 format (simplified)
pub fn parse_rfc2822(input: &str) -> TimeResult<DateTime> {
    // Simplified implementation - would need full RFC 2822 parser for production
    Err(parse_error(input, "RFC 2822", "RFC 2822 parsing not fully implemented"))
}

/// Format DateTime in US format (12/25/2023 3:30:45 PM)
pub fn format_us(datetime: &DateTime) -> TimeResult<String> {
    let time_str = format_time(&datetime.time, "%I:%M:%S %p")?;
    Ok(format!(
        "{:02}/{:02}/{} {}",
        datetime.date.month,
        datetime.date.day,
        datetime.date.year,
        time_str
    ))
}

/// Parse DateTime from US format
pub fn parse_us(input: &str) -> TimeResult<DateTime> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(parse_error(input, "US format", "Expected format: MM/DD/YYYY HH:MM:SS AM/PM"));
    }
    
    let date = parse_us_date(parts[0])?;
    let time_str = parts[1..].join(" ");
    let time = parse_12_hour_time(&time_str)?;
    
    Ok(DateTime::new(date, time))
}

/// Format DateTime in European format (25/12/2023 15:30:45)
pub fn format_european(datetime: &DateTime) -> TimeResult<String> {
    let time_str = format_time(&datetime.time, "%H:%M:%S")?;
    Ok(format!(
        "{:02}/{:02}/{} {}",
        datetime.date.day,
        datetime.date.month,
        datetime.date.year,
        time_str
    ))
}

/// Parse DateTime from European format
pub fn parse_european(input: &str) -> TimeResult<DateTime> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(parse_error(input, "European format", "Expected format: DD/MM/YYYY HH:MM:SS"));
    }
    
    let date = parse_european_date(parts[0])?;
    let time = parse_24_hour_time(parts[1])?;
    
    Ok(DateTime::new(date, time))
}

/// Format DateTime with custom format string
pub fn format_custom(datetime: &DateTime, format: &str) -> TimeResult<String> {
    // Simple implementation of common format specifiers
    let mut result = format.to_string();
    
    // Replace format specifiers
    result = result.replace("%Y", &format!("{:04}", datetime.date.year));
    result = result.replace("%m", &format!("{:02}", datetime.date.month));
    result = result.replace("%d", &format!("{:02}", datetime.date.day));
    result = result.replace("%H", &format!("{:02}", datetime.time.hour));
    result = result.replace("%M", &format!("{:02}", datetime.time.minute));
    result = result.replace("%S", &format!("{:02}", datetime.time.second));
    
    // Weekday and month names
    result = result.replace("%a", datetime.date.weekday().abbrev());
    result = result.replace("%A", datetime.date.weekday().name());
    result = result.replace("%b", datetime.date.month_enum().abbrev());
    result = result.replace("%B", datetime.date.month_enum().name());
    
    Ok(result)
}

/// Parse DateTime with custom format string
pub fn parse_custom(input: &str, format: &str) -> TimeResult<DateTime> {
    // Simplified implementation - would need full format parser for production
    if format.contains("%Y-%m-%dT%H:%M:%S") {
        parse_iso8601(input)
    } else {
        Err(parse_error(input, format, "Custom format parsing not fully implemented"))
    }
}

// Helper parsing functions

fn parse_iso_date(input: &str) -> TimeResult<Date> {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 3 {
        return Err(parse_error(input, "YYYY-MM-DD", "Invalid date format"));
    }
    
    let year: i32 = parts[0].parse()
        .map_err(|_| parse_error(input, "YYYY-MM-DD", "Invalid year"))?;
    let month: u32 = parts[1].parse()
        .map_err(|_| parse_error(input, "YYYY-MM-DD", "Invalid month"))?;
    let day: u32 = parts[2].parse()
        .map_err(|_| parse_error(input, "YYYY-MM-DD", "Invalid day"))?;
    
    Date::new(year, month, day)
}

fn parse_us_date(input: &str) -> TimeResult<Date> {
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() != 3 {
        return Err(parse_error(input, "MM/DD/YYYY", "Invalid date format"));
    }
    
    let month: u32 = parts[0].parse()
        .map_err(|_| parse_error(input, "MM/DD/YYYY", "Invalid month"))?;
    let day: u32 = parts[1].parse()
        .map_err(|_| parse_error(input, "MM/DD/YYYY", "Invalid day"))?;
    let year: i32 = parts[2].parse()
        .map_err(|_| parse_error(input, "MM/DD/YYYY", "Invalid year"))?;
    
    Date::new(year, month, day)
}

fn parse_european_date(input: &str) -> TimeResult<Date> {
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() != 3 {
        return Err(parse_error(input, "DD/MM/YYYY", "Invalid date format"));
    }
    
    let day: u32 = parts[0].parse()
        .map_err(|_| parse_error(input, "DD/MM/YYYY", "Invalid day"))?;
    let month: u32 = parts[1].parse()
        .map_err(|_| parse_error(input, "DD/MM/YYYY", "Invalid month"))?;
    let year: i32 = parts[2].parse()
        .map_err(|_| parse_error(input, "DD/MM/YYYY", "Invalid year"))?;
    
    Date::new(year, month, day)
}

fn parse_iso_time(input: &str) -> TimeResult<Time> {
    parse_24_hour_time(input)
}

fn parse_24_hour_time(input: &str) -> TimeResult<Time> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return Err(parse_error(input, "HH:MM:SS", "Invalid time format"));
    }
    
    let hour: u32 = parts[0].parse()
        .map_err(|_| parse_error(input, "HH:MM:SS", "Invalid hour"))?;
    let minute: u32 = parts[1].parse()
        .map_err(|_| parse_error(input, "HH:MM:SS", "Invalid minute"))?;
    let second: u32 = if parts.len() == 3 {
        parts[2].parse()
            .map_err(|_| parse_error(input, "HH:MM:SS", "Invalid second"))?
    } else {
        0
    };
    
    Time::new(hour, minute, second, 0)
}

fn parse_24_hour_time_short(input: &str) -> TimeResult<Time> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err(parse_error(input, "HH:MM", "Invalid time format"));
    }
    
    let hour: u32 = parts[0].parse()
        .map_err(|_| parse_error(input, "HH:MM", "Invalid hour"))?;
    let minute: u32 = parts[1].parse()
        .map_err(|_| parse_error(input, "HH:MM", "Invalid minute"))?;
    
    Time::new(hour, minute, 0, 0)
}

fn parse_12_hour_time(input: &str) -> TimeResult<Time> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(parse_error(input, "HH:MM:SS AM/PM", "Invalid 12-hour time format"));
    }
    
    let time_parts: Vec<&str> = parts[0].split(':').collect();
    if time_parts.len() != 3 {
        return Err(parse_error(input, "HH:MM:SS AM/PM", "Invalid time format"));
    }
    
    let hour_12: u32 = time_parts[0].parse()
        .map_err(|_| parse_error(input, "HH:MM:SS AM/PM", "Invalid hour"))?;
    let minute: u32 = time_parts[1].parse()
        .map_err(|_| parse_error(input, "HH:MM:SS AM/PM", "Invalid minute"))?;
    let second: u32 = time_parts[2].parse()
        .map_err(|_| parse_error(input, "HH:MM:SS AM/PM", "Invalid second"))?;
    
    let am_pm = parts[1].to_uppercase();
    if am_pm != "AM" && am_pm != "PM" {
        return Err(parse_error(input, "HH:MM:SS AM/PM", "Invalid AM/PM indicator"));
    }
    
    let hour_24 = if am_pm == "AM" {
        if hour_12 == 12 { 0 } else { hour_12 }
    } else {
        if hour_12 == 12 { 12 } else { hour_12 + 12 }
    };
    
    Time::new(hour_24, minute, second, 0)
}

fn parse_unix_timestamp(input: &str) -> TimeResult<DateTime> {
    let timestamp: i64 = input.parse()
        .map_err(|_| parse_error(input, "Unix timestamp", "Invalid timestamp"))?;
    
    crate::stdlib::time::datetime::from_timestamp(timestamp)
}

fn format_with_pattern(value: &str, _pattern: &str) -> TimeResult<String> {
    // Simplified implementation
    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::time::datetime::{Date, Time, DateTime};
    
    #[test]
    fn test_format_iso8601() {
        let date = Date::new(2023, 12, 25).unwrap();
        let time = Time::new(15, 30, 45, 0).unwrap();
        let datetime = DateTime::new(date, time);
        
        let formatted = format_iso8601(&datetime).unwrap();
        assert_eq!(formatted, "2023-12-25T15:30:45");
    }
    
    #[test]
    fn test_parse_iso8601() {
        let parsed = parse_iso8601("2023-12-25T15:30:45").unwrap();
        assert_eq!(parsed.date.year, 2023);
        assert_eq!(parsed.date.month, 12);
        assert_eq!(parsed.date.day, 25);
        assert_eq!(parsed.time.hour, 15);
        assert_eq!(parsed.time.minute, 30);
        assert_eq!(parsed.time.second, 45);
    }
    
    #[test]
    fn test_parse_duration() {
        let d1 = parse_duration("2h 30m").unwrap();
        assert_eq!(d1.total_seconds(), 9000); // 2*3600 + 30*60
        
        let d2 = parse_duration("1d 4h").unwrap();
        assert_eq!(d2.total_seconds(), 100800); // 24*3600 + 4*3600
        
        let d3 = parse_duration("45s").unwrap();
        assert_eq!(d3.total_seconds(), 45);
    }
    
    #[test]
    fn test_format_duration() {
        let d1 = Duration::from_seconds(3661);
        assert_eq!(format_duration(&d1), "1 hour, 1 minute, and 1 second");
        
        let d2 = Duration::from_seconds(60);
        assert_eq!(format_duration(&d2), "1 minute");
    }
}
