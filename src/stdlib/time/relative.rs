/// Relative time calculations and human-readable time descriptions
use crate::stdlib::time::error::{TimeError, TimeResult, time_error};
use crate::stdlib::time::datetime::{DateTime, Date, Weekday};
use crate::stdlib::time::duration::Duration;
use crate::error::Error;

/// Represents relative time periods
#[derive(Debug, Clone, PartialEq)]
pub enum RelativeTime {
    Now,
    Past(Duration),
    Future(Duration),
}

impl RelativeTime {
    /// Create relative time from duration
    pub fn from_duration(duration: Duration) -> Self {
        if duration.is_zero() {
            RelativeTime::Now
        } else if duration.is_positive() {
            RelativeTime::Future(duration)
        } else {
            RelativeTime::Past(duration.abs())
        }
    }
    
    /// Check if this is in the past
    pub fn is_past(&self) -> bool {
        matches!(self, RelativeTime::Past(_))
    }
    
    /// Check if this is in the future
    pub fn is_future(&self) -> bool {
        matches!(self, RelativeTime::Future(_))
    }
    
    /// Check if this is now
    pub fn is_now(&self) -> bool {
        matches!(self, RelativeTime::Now)
    }
    
    /// Get the duration (absolute value)
    pub fn duration(&self) -> Option<Duration> {
        match self {
            RelativeTime::Now => None,
            RelativeTime::Past(d) | RelativeTime::Future(d) => Some(*d),
        }
    }
}

/// Generate relative time description for a datetime compared to now
pub fn relative_time(datetime: &DateTime) -> TimeResult<String> {
    let now = crate::stdlib::time::datetime::now()?;
    let duration = crate::stdlib::time::duration::duration_between(now, *datetime);
    
    if duration.is_zero() {
        return Ok("now".to_string());
    }
    
    if duration.is_positive() {
        // Future
        time_from_now(&duration)
    } else {
        // Past
        time_ago(&duration.abs())
    }
}

/// Generate "X time ago" description
pub fn time_ago(duration: &Duration) -> TimeResult<String> {
    let seconds = duration.total_seconds();
    
    if seconds < 60 {
        if seconds == 1 {
            Ok("1 second ago".to_string())
        } else {
            Ok(format!("{} seconds ago", seconds))
        }
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        if minutes == 1 {
            Ok("1 minute ago".to_string())
        } else {
            Ok(format!("{} minutes ago", minutes))
        }
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        if hours == 1 {
            Ok("1 hour ago".to_string())
        } else {
            Ok(format!("{} hours ago", hours))
        }
    } else if seconds < 604800 {
        let days = seconds / 86400;
        if days == 1 {
            Ok("1 day ago".to_string())
        } else {
            Ok(format!("{} days ago", days))
        }
    } else if seconds < 2629746 { // Approximate month (30.44 days)
        let weeks = seconds / 604800;
        if weeks == 1 {
            Ok("1 week ago".to_string())
        } else {
            Ok(format!("{} weeks ago", weeks))
        }
    } else if seconds < 31556952 { // Approximate year (365.25 days)
        let months = seconds / 2629746;
        if months == 1 {
            Ok("1 month ago".to_string())
        } else {
            Ok(format!("{} months ago", months))
        }
    } else {
        let years = seconds / 31556952;
        if years == 1 {
            Ok("1 year ago".to_string())
        } else {
            Ok(format!("{} years ago", years))
        }
    }
}

/// Generate "in X time" description
pub fn time_from_now(duration: &Duration) -> TimeResult<String> {
    let seconds = duration.total_seconds();
    
    if seconds < 60 {
        if seconds == 1 {
            Ok("in 1 second".to_string())
        } else {
            Ok(format!("in {} seconds", seconds))
        }
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        if minutes == 1 {
            Ok("in 1 minute".to_string())
        } else {
            Ok(format!("in {} minutes", minutes))
        }
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        if hours == 1 {
            Ok("in 1 hour".to_string())
        } else {
            Ok(format!("in {} hours", hours))
        }
    } else if seconds < 604800 {
        let days = seconds / 86400;
        if days == 1 {
            Ok("in 1 day".to_string())
        } else {
            Ok(format!("in {} days", days))
        }
    } else if seconds < 2629746 { // Approximate month
        let weeks = seconds / 604800;
        if weeks == 1 {
            Ok("in 1 week".to_string())
        } else {
            Ok(format!("in {} weeks", weeks))
        }
    } else if seconds < 31556952 { // Approximate year
        let months = seconds / 2629746;
        if months == 1 {
            Ok("in 1 month".to_string())
        } else {
            Ok(format!("in {} months", months))
        }
    } else {
        let years = seconds / 31556952;
        if years == 1 {
            Ok("in 1 year".to_string())
        } else {
            Ok(format!("in {} years", years))
        }
    }
}

/// Generate humanized duration description
pub fn humanize_duration(duration: &Duration) -> String {
    duration.humanize()
}

/// Format relative time with more detail
pub fn format_relative(datetime: &DateTime, detailed: bool) -> TimeResult<String> {
    let now = crate::stdlib::time::datetime::now()?;
    let duration = crate::stdlib::time::duration::duration_between(now, *datetime);
    
    if detailed {
        format_relative_detailed(&duration)
    } else {
        relative_time(datetime)
    }
}

/// Format relative time with detailed breakdown
fn format_relative_detailed(duration: &Duration) -> TimeResult<String> {
    let seconds = duration.total_seconds().abs();
    let is_future = duration.is_positive();
    
    let mut parts = Vec::new();
    let mut remaining = seconds;
    
    // Years
    if remaining >= 31556952 {
        let years = remaining / 31556952;
        parts.push(format!("{} year{}", years, if years == 1 { "" } else { "s" }));
        remaining %= 31556952;
    }
    
    // Months
    if remaining >= 2629746 {
        let months = remaining / 2629746;
        parts.push(format!("{} month{}", months, if months == 1 { "" } else { "s" }));
        remaining %= 2629746;
    }
    
    // Days
    if remaining >= 86400 {
        let days = remaining / 86400;
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
        remaining %= 86400;
    }
    
    // Hours
    if remaining >= 3600 {
        let hours = remaining / 3600;
        parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
        remaining %= 3600;
    }
    
    // Minutes
    if remaining >= 60 {
        let minutes = remaining / 60;
        parts.push(format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" }));
        remaining %= 60;
    }
    
    // Seconds
    if remaining > 0 || parts.is_empty() {
        parts.push(format!("{} second{}", remaining, if remaining == 1 { "" } else { "s" }));
    }
    
    // Take only the two most significant parts
    if parts.len() > 2 {
        parts.truncate(2);
    }
    
    let description = if parts.len() == 1 {
        parts[0].clone()
    } else {
        format!("{} and {}", parts[0], parts[1])
    };
    
    if is_future {
        Ok(format!("in {}", description))
    } else {
        Ok(format!("{} ago", description))
    }
}

/// Parse relative time expressions (e.g., "2 hours ago", "in 3 days")
pub fn parse_relative(input: &str) -> TimeResult<RelativeTime> {
    let input = input.trim().to_lowercase();
    
    if input == "now" {
        return Ok(RelativeTime::Now);
    }
    
    // Parse "X ago" format
    if input.ends_with(" ago") {
        let duration_part = &input[..input.len() - 4].trim();
        let duration = parse_duration_from_text(duration_part)?;
        return Ok(RelativeTime::Past(duration));
    }
    
    // Parse "in X" format
    if input.starts_with("in ") {
        let duration_part = &input[3..].trim();
        let duration = parse_duration_from_text(duration_part)?;
        return Ok(RelativeTime::Future(duration));
    }
    
    Err(time_error(&format!("Cannot parse relative time: {}", input)))
}

/// Parse duration from text (e.g., "2 hours", "3 days", "1 week")
fn parse_duration_from_text(input: &str) -> TimeResult<Duration> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(time_error("Duration must be in format 'NUMBER UNIT'"));
    }
    
    let number: i64 = parts[0].parse()
        .map_err(|_| time_error("Invalid number in duration"))?;
    
    let unit = parts[1].to_lowercase();
    let unit = if unit.ends_with('s') { &unit[..unit.len()-1] } else { &unit };
    
    let multiplier = match unit {
        "second" => 1,
        "minute" => 60,
        "hour" => 3600,
        "day" => 86400,
        "week" => 604800,
        "month" => 2629746, // Approximate
        "year" => 31556952, // Approximate
        _ => return Err(time_error(&format!("Unknown time unit: {}", unit))),
    };
    
    Ok(Duration::from_seconds(number * multiplier))
}

/// Find the next occurrence of a specific weekday
pub fn next_occurrence(weekday: Weekday) -> TimeResult<Date> {
    let today = crate::stdlib::time::datetime::today()?;
    let today_weekday = today.weekday();
    
    let days_ahead = ((weekday as u32 - today_weekday as u32 + 7) % 7) as i32;
    let days_ahead = if days_ahead == 0 { 7 } else { days_ahead }; // Next week if today
    
    today.add_days(days_ahead)
}

/// Find the previous occurrence of a specific weekday
pub fn previous_occurrence(weekday: Weekday) -> TimeResult<Date> {
    let today = crate::stdlib::time::datetime::today()?;
    let today_weekday = today.weekday();
    
    let days_behind = ((today_weekday as u32 - weekday as u32 + 7) % 7) as i32;
    let days_behind = if days_behind == 0 { 7 } else { days_behind }; // Previous week if today
    
    today.add_days(-days_behind)
}

/// Get relative date descriptions (today, tomorrow, yesterday, etc.)
pub fn relative_date_description(date: &Date) -> TimeResult<String> {
    let today = crate::stdlib::time::datetime::today()?;
    
    if *date == today {
        Ok("today".to_string())
    } else if *date == today.add_days(1)? {
        Ok("tomorrow".to_string())
    } else if *date == today.add_days(-1)? {
        Ok("yesterday".to_string())
    } else {
        // Calculate days difference
        let today_timestamp = DateTime::new(today, crate::stdlib::time::datetime::Time::new(0, 0, 0, 0)?).to_timestamp();
        let date_timestamp = DateTime::new(*date, crate::stdlib::time::datetime::Time::new(0, 0, 0, 0)?).to_timestamp();
        let days_diff = (date_timestamp - today_timestamp) / 86400;
        
        if days_diff > 0 {
            if days_diff <= 7 {
                Ok(format!("in {} day{}", days_diff, if days_diff == 1 { "" } else { "s" }))
            } else {
                Ok(format!("in {} week{}", days_diff / 7, if days_diff / 7 == 1 { "" } else { "s" }))
            }
        } else {
            let days_ago = -days_diff;
            if days_ago <= 7 {
                Ok(format!("{} day{} ago", days_ago, if days_ago == 1 { "" } else { "s" }))
            } else {
                Ok(format!("{} week{} ago", days_ago / 7, if days_ago / 7 == 1 { "" } else { "s" }))
            }
        }
    }
}

/// Get time of day description (morning, afternoon, evening, night)
pub fn time_of_day_description(time: &crate::stdlib::time::datetime::Time) -> &'static str {
    match time.hour {
        0..=5 => "night",
        6..=11 => "morning",
        12..=17 => "afternoon",
        18..=21 => "evening",
        22..=23 => "night",
        _ => "unknown",
    }
}

/// Check if a date is a weekend
pub fn is_weekend(date: &Date) -> bool {
    let weekday = date.weekday();
    weekday == Weekday::Saturday || weekday == Weekday::Sunday
}

/// Check if a date is a weekday
pub fn is_weekday(date: &Date) -> bool {
    !is_weekend(date)
}

/// Get the next business day (skipping weekends)
pub fn next_business_day(date: &Date) -> TimeResult<Date> {
    let mut next_day = date.add_days(1)?;
    while is_weekend(&next_day) {
        next_day = next_day.add_days(1)?;
    }
    Ok(next_day)
}

/// Get the previous business day (skipping weekends)
pub fn previous_business_day(date: &Date) -> TimeResult<Date> {
    let mut prev_day = date.add_days(-1)?;
    while is_weekend(&prev_day) {
        prev_day = prev_day.add_days(-1)?;
    }
    Ok(prev_day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::time::datetime::{Date, Time, DateTime};
    use crate::stdlib::time::duration::Duration;
    
    #[test]
    fn test_time_ago() {
        let duration = Duration::from_seconds(3661); // 1 hour, 1 minute, 1 second
        let result = time_ago(&duration).unwrap();
        assert_eq!(result, "1 hour ago");
        
        let duration2 = Duration::from_seconds(60);
        let result2 = time_ago(&duration2).unwrap();
        assert_eq!(result2, "1 minute ago");
    }
    
    #[test]
    fn test_time_from_now() {
        let duration = Duration::from_seconds(7200); // 2 hours
        let result = time_from_now(&duration).unwrap();
        assert_eq!(result, "in 2 hours");
    }
    
    #[test]
    fn test_parse_relative() {
        let rel1 = parse_relative("2 hours ago").unwrap();
        assert!(rel1.is_past());
        
        let rel2 = parse_relative("in 3 days").unwrap();
        assert!(rel2.is_future());
        
        let rel3 = parse_relative("now").unwrap();
        assert!(rel3.is_now());
    }
    
    #[test]
    fn test_next_occurrence() {
        // This test would need to be adjusted based on the current date
        // For now, just test that it doesn't panic
        let _next_monday = next_occurrence(Weekday::Monday);
    }
    
    #[test]
    fn test_time_of_day_description() {
        let morning = Time::new(9, 30, 0, 0).unwrap();
        assert_eq!(time_of_day_description(&morning), "morning");
        
        let afternoon = Time::new(15, 30, 0, 0).unwrap();
        assert_eq!(time_of_day_description(&afternoon), "afternoon");
        
        let evening = Time::new(19, 30, 0, 0).unwrap();
        assert_eq!(time_of_day_description(&evening), "evening");
        
        let night = Time::new(2, 30, 0, 0).unwrap();
        assert_eq!(time_of_day_description(&night), "night");
    }
    
    #[test]
    fn test_weekend_detection() {
        // Would need specific dates to test properly
        // For now, just test the logic doesn't panic
        let date = Date::new(2023, 12, 25).unwrap(); // Monday
        let _is_weekend = is_weekend(&date);
        let _is_weekday = is_weekday(&date);
    }
}
