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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_flag_constants() {
        assert_eq!(Ldate, 1);
        assert_eq!(Ltime, 2);
        assert_eq!(Lmicroseconds, 4);
        assert_eq!(Llongfile, 8);
        assert_eq!(Lshortfile, 16);
        assert_eq!(LUTC, 32);
        assert_eq!(Lmsgprefix, 64);
        assert_eq!(LstdFlags, Ldate | Ltime);
    }
    
    #[test]
    fn test_has_flag() {
        let flags = Ldate | Ltime;
        assert!(has_flag(flags, Ldate));
        assert!(has_flag(flags, Ltime));
        assert!(!has_flag(flags, Lmicroseconds));
    }
    
    #[test]
    fn test_set_flag() {
        let flags = Ldate;
        let new_flags = set_flag(flags, Ltime);
        assert_eq!(new_flags, Ldate | Ltime);
    }
    
    #[test]
    fn test_clear_flag() {
        let flags = Ldate | Ltime;
        let new_flags = clear_flag(flags, Ldate);
        assert_eq!(new_flags, Ltime);
    }
    
    #[test]
    fn test_toggle_flag() {
        let flags = Ldate;
        let toggled = toggle_flag(flags, Ltime);
        assert_eq!(toggled, Ldate | Ltime);
        
        let toggled_again = toggle_flag(toggled, Ldate);
        assert_eq!(toggled_again, Ltime);
    }
    
    #[test]
    fn test_describe_flags() {
        let flags = Ldate | Ltime;
        let description = describe_flags(flags);
        assert!(description.contains("Ldate"));
        assert!(description.contains("Ltime"));
    }
    
    #[test]
    fn test_validate_flags() {
        // Valid combinations
        assert!(validate_flags(LstdFlags).is_ok());
        assert!(validate_flags(Ldate | Ltime | Lmicroseconds).is_ok());
        
        // Invalid combinations
        assert!(validate_flags(Llongfile | Lshortfile).is_err());
        assert!(validate_flags(Lmicroseconds).is_err()); // Missing Ltime
    }
    
    #[test]
    fn test_presets() {
        assert_eq!(presets::MINIMAL, 0);
        assert_eq!(presets::BASIC, LstdFlags);
        assert!(has_flag(presets::DETAILED, Lshortfile));
        assert!(has_flag(presets::FULL, Lmicroseconds));
        assert!(has_flag(presets::DEBUG, Llongfile));
        assert!(has_flag(presets::PRODUCTION, LUTC));
    }
}
