use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local, Utc, TimeZone};
use crate::error::CursedError;
use super::flags::*;

/// Format a log entry according to the specified flags
pub fn format_log_entry(
    prefix: &str,
    flags: i32,
    call_depth: usize,
    message: &str,
) -> Result<(), Error> {
    let mut parts = Vec::new();
    
    // Handle prefix placement
    let prefix_at_start = !has_flag(flags, Lmsgprefix);
    if prefix_at_start && !prefix.is_empty() {
        parts.push(prefix.to_string());
    }
    
    // Add date if requested
    if has_flag(flags, Ldate) {
        let date_str = format_date(flags)?;
        parts.push(date_str);
    }
    
    // Add time if requested
    if has_flag(flags, Ltime) || has_flag(flags, Lmicroseconds) {
        let time_str = format_time(flags)?;
        parts.push(time_str);
    }
    
    // Add file information if requested
    if has_flag(flags, Llongfile) || has_flag(flags, Lshortfile) {
        let file_str = format_file_info(flags, call_depth)?;
        parts.push(file_str);
    }
    
    // Handle message with prefix placement
    let final_message = if has_flag(flags, Lmsgprefix) && !prefix.is_empty() {
        format!("{}{}", prefix, message)
    } else {
        message.to_string()
    };
    
    // Combine all parts
    let mut result = if parts.is_empty() {
        final_message
    } else {
        format!("{} {}", parts.join(" "), final_message)
    };
    
    // Ensure the message ends with a newline
    if !result.ends_with('\n') {
        result.push('\n');
    }
    
    Ok(result)
}

/// Format date according to flags
fn format_date(flags: i32) -> Result<(), Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| CursedError::Runtime(format!("Failed to get system time: {}", e)))?;
    
    let timestamp = now.as_secs() as i64;
    
    if has_flag(flags, LUTC) {
        let datetime = Utc.timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| CursedError::Runtime("Invalid timestamp".to_string()))?;
        Ok(datetime.format("%Y/%m/%d").to_string())
    } else {
        let datetime = Local.timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| CursedError::Runtime("Invalid timestamp".to_string()))?;
        Ok(datetime.format("%Y/%m/%d").to_string())
    }
}

/// Format time according to flags
fn format_time(flags: i32) -> Result<(), Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| CursedError::Runtime(format!("Failed to get system time: {}", e)))?;
    
    let timestamp = now.as_secs() as i64;
    let microseconds = now.subsec_micros();
    
    if has_flag(flags, LUTC) {
        let datetime = Utc.timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| CursedError::Runtime("Invalid timestamp".to_string()))?;
        
        if has_flag(flags, Lmicroseconds) {
            Ok(format!("{}.{:06}", datetime.format("%H:%M:%S"), microseconds))
        } else {
            Ok(datetime.format("%H:%M:%S").to_string())
        }
    } else {
        let datetime = Local.timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| CursedError::Runtime("Invalid timestamp".to_string()))?;
        
        if has_flag(flags, Lmicroseconds) {
            Ok(format!("{}.{:06}", datetime.format("%H:%M:%S"), microseconds))
        } else {
            Ok(datetime.format("%H:%M:%S").to_string())
        }
    }
}

/// Format file information according to flags
fn format_file_info(flags: i32, call_depth: usize) -> Result<(), Error> {
    // For now, we'll provide a basic implementation
    // In a full implementation, this would use stack inspection
    // to get the actual caller information
    
    let file_info = get_caller_info(call_depth)?;
    
    if has_flag(flags, Llongfile) {
        Ok(format!("{}:{}", file_info.file, file_info.line))
    } else if has_flag(flags, Lshortfile) {
        let filename = std::path::Path::new(&file_info.file)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&file_info.file);
        Ok(format!("{}:{}", filename, file_info.line))
    } else {
        Ok(String::new())
    }
}

/// Caller information structure
#[derive(Debug)]
struct CallerInfo {
    file: String,
    line: u32,
    function: String,
}

/// Get caller information from the stack
/// This is a simplified implementation - a full implementation would use
/// backtrace crates or other stack inspection mechanisms
fn get_caller_info(_call_depth: usize) -> Result<(), Error> {
    // For now, return placeholder information
    // In a production implementation, this would use:
    // - backtrace crate for stack inspection
    // - debug information to get file/line numbers
    // - proper call depth calculation
    
    Ok(CallerInfo {
        file: "main.csd".to_string(),
        line: 1,
        function: "main".to_string(),
    })
}

/// Utility function to get current timestamp as a formatted string
pub fn get_timestamp(use_utc: bool, include_microseconds: bool) -> Result<(), Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| CursedError::Runtime(format!("Failed to get system time: {}", e)))?;
    
    let timestamp = now.as_secs() as i64;
    let microseconds = now.subsec_micros();
    
    if use_utc {
        let datetime = Utc.timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| CursedError::Runtime("Invalid timestamp".to_string()))?;
        
        if include_microseconds {
            Ok(format!("{}.{:06}", datetime.format("%Y/%m/%d %H:%M:%S"), microseconds))
        } else {
            Ok(datetime.format("%Y/%m/%d %H:%M:%S").to_string())
        }
    } else {
        let datetime = Local.timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| CursedError::Runtime("Invalid timestamp".to_string()))?;
        
        if include_microseconds {
            Ok(format!("{}.{:06}", datetime.format("%Y/%m/%d %H:%M:%S"), microseconds))
        } else {
            Ok(datetime.format("%Y/%m/%d %H:%M:%S").to_string())
        }
    }
}

/// Format a message with timestamp and optional prefix
pub fn format_message(message: &str, prefix: Option<&str>, timestamp: bool) -> String {
    let mut parts = Vec::new();
    
    if timestamp {
        if let Ok(ts) = get_timestamp(false, false) {
            parts.push(ts);
        }
    }
    
    if let Some(p) = prefix {
        if !p.is_empty() {
            parts.push(p.to_string());
        }
    }
    
    parts.push(message.to_string());
    
    let mut result = parts.join(" ");
    if !result.ends_with('\n') {
        result.push('\n');
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_log_entry_basic() {
        let result = format_log_entry("", 0, 0, "test message").unwrap();
        assert_eq!(result, "test message\n");
    }
    
    #[test]
    fn test_format_log_entry_with_prefix() {
        let result = format_log_entry("INFO: ", 0, 0, "test message").unwrap();
        assert_eq!(result, "INFO: test message\n");
    }
    
    #[test]
    fn test_format_log_entry_with_prefix_after() {
        let result = format_log_entry("INFO: ", Lmsgprefix, 0, "test message").unwrap();
        assert_eq!(result, "INFO: test message\n");
    }
    
    #[test]
    fn test_format_log_entry_with_date() {
        let result = format_log_entry("", Ldate, 0, "test message").unwrap();
        // Should contain a date in YYYY/MM/DD format
        assert!(result.contains("/"));
        assert!(result.ends_with("test message\n"));
    }
    
    #[test]
    fn test_format_log_entry_with_time() {
        let result = format_log_entry("", Ltime, 0, "test message").unwrap();
        // Should contain time in HH:MM:SS format
        assert!(result.contains(":"));
        assert!(result.ends_with("test message\n"));
    }
    
    #[test]
    fn test_format_log_entry_with_microseconds() {
        let result = format_log_entry("", Ltime | Lmicroseconds, 0, "test message").unwrap();
        // Should contain microseconds (dot followed by 6 digits)
        assert!(result.contains("."));
        assert!(result.ends_with("test message\n"));
    }
    
    #[test]
    fn test_format_log_entry_with_shortfile() {
        let result = format_log_entry("", Lshortfile, 0, "test message").unwrap();
        // Should contain filename and line number
        assert!(result.contains("main.csd:1"));
        assert!(result.ends_with("test message\n"));
    }
    
    #[test]
    fn test_format_log_entry_standard_flags() {
        let result = format_log_entry("", LstdFlags, 0, "test message").unwrap();
        // Should contain both date and time
        assert!(result.contains("/"));
        assert!(result.contains(":"));
        assert!(result.ends_with("test message\n"));
    }
    
    #[test]
    fn test_get_timestamp() {
        let ts_local = get_timestamp(false, false).unwrap();
        let ts_utc = get_timestamp(true, false).unwrap();
        let ts_micro = get_timestamp(false, true).unwrap();
        
        // All should contain date and time
        assert!(ts_local.contains("/"));
        assert!(ts_local.contains(":"));
        
        assert!(ts_utc.contains("/"));
        assert!(ts_utc.contains(":"));
        
        assert!(ts_micro.contains("/"));
        assert!(ts_micro.contains(":"));
        assert!(ts_micro.contains(".")); // Microseconds
    }
    
    #[test]
    fn test_format_message() {
        let msg = format_message("test", None, false);
        assert_eq!(msg, "test\n");
        
        let msg_with_prefix = format_message("test", Some("INFO: "), false);
        assert_eq!(msg_with_prefix, "INFO:  test\n");
        
        let msg_with_timestamp = format_message("test", None, true);
        assert!(msg_with_timestamp.contains("/"));
        assert!(msg_with_timestamp.ends_with("test\n"));
    }
    
    #[test]
    fn test_newline_handling() {
        // Message without newline should get one added
        let result = format_log_entry("", 0, 0, "test").unwrap();
        assert!(result.ends_with('\n'));
        
        // Message with newline should not get another one
        let result2 = format_log_entry("", 0, 0, "test\n").unwrap();
        assert_eq!(result2.matches('\n').count(), 1);
    }
}
