/// Logging format flags that control the output format
/// These flags can be combined using bitwise OR operations

/// Ldate - Include date: 2025/04/13
pub const Ldate: i32 = 1 << 0;

/// Ltime - Include time: 15:04:05
pub const Ltime: i32 = 1 << 1;

/// Lmicroseconds - Include microseconds: 15:04:05.123456
/// Note: This flag also includes Ltime
pub const Lmicroseconds: i32 = 1 << 2;

/// Llongfile - Include full file path and line: /path/to/file.csd:23
pub const Llongfile: i32 = 1 << 3;

/// Lshortfile - Include just filename and line: file.csd:23
pub const Lshortfile: i32 = 1 << 4;

/// LUTC - Use UTC time instead of local time
pub const LUTC: i32 = 1 << 5;

/// Lmsgprefix - Move prefix to before the message instead of line start
pub const Lmsgprefix: i32 = 1 << 6;

/// LstdFlags - Default flags: Ldate | Ltime
pub const LstdFlags: i32 = Ldate | Ltime;

/// Helper functions for flag manipulation

/// Check if a specific flag is set
pub fn has_flag(flags: i32, flag: i32) -> bool {
    (flags & flag) != 0
}

/// Set a specific flag
pub fn set_flag(flags: i32, flag: i32) -> i32 {
    flags | flag
}

/// Clear a specific flag
pub fn clear_flag(flags: i32, flag: i32) -> i32 {
    flags & !flag
}

/// Toggle a specific flag
pub fn toggle_flag(flags: i32, flag: i32) -> i32 {
    flags ^ flag
}

/// Get a human-readable description of the flags
pub fn describe_flags(flags: i32) -> String {
    let mut parts = Vec::new();
    
    if has_flag(flags, Ldate) {
        parts.push("Ldate");
    }
    if has_flag(flags, Ltime) {
        parts.push("Ltime");  
    }
    if has_flag(flags, Lmicroseconds) {
        parts.push("Lmicroseconds");
    }
    if has_flag(flags, Llongfile) {
        parts.push("Llongfile");
    }
    if has_flag(flags, Lshortfile) {
        parts.push("Lshortfile");
    }
    if has_flag(flags, LUTC) {
        parts.push("LUTC");
    }
    if has_flag(flags, Lmsgprefix) {
        parts.push("Lmsgprefix");
    }
    
    if parts.is_empty() {
        "No flags".to_string()
    } else {
        parts.join(" | ")
    }
}

/// Validate that flags don't have conflicting combinations
pub fn validate_flags(flags: i32) -> Result<(), String> {
    // Check for mutually exclusive flags
    if has_flag(flags, Llongfile) && has_flag(flags, Lshortfile) {
        return Err("Cannot use both Llongfile and Lshortfile flags".to_string());
    }
    
    // Lmicroseconds implies Ltime
    if has_flag(flags, Lmicroseconds) && !has_flag(flags, Ltime) {
        return Err("Lmicroseconds flag requires Ltime flag".to_string());
    }
    
    Ok(())
}

/// Common flag combinations
pub mod presets {
    use super::*;
    
    /// Minimal logging - just the message
    pub const MINIMAL: i32 = 0;
    
    /// Basic logging with timestamp
    pub const BASIC: i32 = LstdFlags;
    
    /// Detailed logging with file information
    pub const DETAILED: i32 = LstdFlags | Lshortfile;
    
    /// Full logging with all information
    pub const FULL: i32 = LstdFlags | Lmicroseconds | Lshortfile;
    
    /// Debug logging with full file paths
    pub const DEBUG: i32 = LstdFlags | Lmicroseconds | Llongfile;
    
    /// Production logging with UTC time
    pub const PRODUCTION: i32 = LstdFlags | LUTC | Lshortfile;
}

