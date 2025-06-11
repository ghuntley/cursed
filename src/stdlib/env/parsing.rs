/// Environment variable parsing and type conversion for CURSED standard library

use std::path::PathBuf;
use std::str::FromStr;

use super::core::{get_env, get_path_separator};
use super::error::{EnvError, EnvResult, not_found_error, invalid_value_error};

/// Parse an environment variable to a specific type
/// 
/// Attempts to parse the environment variable value to type `T`.
/// Returns an error if the variable doesn't exist or cannot be parsed.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env;
/// 
/// let port: u16 = parse_env("PORT")?;
/// let debug: bool = parse_env("DEBUG")?;
/// ```
pub fn parse_env<T>(key: &str) -> EnvResult<T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    
    value.parse::<T>().map_err(|err| {
        invalid_value_error(
            key,
            &value,
            std::any::type_name::<T>(),
            &err.to_string()
        )
    })
}

/// Parse an environment variable with a default value
/// 
/// Attempts to parse the environment variable to type `T`.
/// If the variable doesn't exist or cannot be parsed, returns the default value.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env_with_default;
/// 
/// let port: u16 = parse_env_with_default("PORT", 8080)?;
/// let timeout: u64 = parse_env_with_default("TIMEOUT", 30)?;
/// ```
pub fn parse_env_with_default<T>(key: &str, default: T) -> EnvResult<T>
where
    T: FromStr + Clone,
    T::Err: std::fmt::Display,
{
    match parse_env(key) {
        Ok(value) => Ok(value),
        Err(EnvError::NotFound { .. }) => Ok(default),
        Err(err) => Err(err),
    }
}

/// Parse a PATH-like environment variable
/// 
/// Splits the environment variable by the platform's path separator
/// and returns a vector of PathBuf objects.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::get_path_env;
/// 
/// if let Some(paths) = get_path_env("PATH") {
///     for path in paths {
///         println!("PATH entry: {:?}", path);
///     }
/// }
/// ```
pub fn get_path_env(key: &str) -> Option<Vec<PathBuf>> {
    let value = get_env(key)?;
    let separator = get_path_separator();
    
    Some(
        value
            .split(separator)
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .collect()
    )
}

/// Parse an environment variable as an integer
/// 
/// Convenience function for parsing integer environment variables.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::get_int_env;
/// 
/// let max_connections = get_int_env("MAX_CONNECTIONS")?;
/// ```
pub fn get_int_env(key: &str) -> EnvResult<i64> {
    parse_env(key)
}

/// Parse an environment variable as a float
/// 
/// Convenience function for parsing floating-point environment variables.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::get_float_env;
/// 
/// let cpu_threshold = get_float_env("CPU_THRESHOLD")?;
/// ```
pub fn get_float_env(key: &str) -> EnvResult<f64> {
    parse_env(key)
}

/// Parse an environment variable as a boolean
/// 
/// Supports various boolean representations:
/// - "true", "TRUE", "True", "1", "yes", "YES", "Yes", "on", "ON", "On" -> true
/// - "false", "FALSE", "False", "0", "no", "NO", "No", "off", "OFF", "Off" -> false
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::get_bool_env;
/// 
/// let debug_mode = get_bool_env("DEBUG")?;
/// let enable_logging = get_bool_env("ENABLE_LOGGING")?;
/// ```
pub fn get_bool_env(key: &str) -> EnvResult<bool> {
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => Err(invalid_value_error(
            key,
            &value,
            "boolean",
            "Expected true/false, 1/0, yes/no, or on/off"
        )),
    }
}

/// Parse an environment variable as a numeric value with bounds checking
/// 
/// Parses the environment variable as a number and validates it's within the specified range.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::get_numeric_env;
/// 
/// let port: u16 = get_numeric_env("PORT", 1, 65535)?;
/// let percentage: f64 = get_numeric_env("CPU_LIMIT", 0.0, 100.0)?;
/// ```
pub fn get_numeric_env<T>(key: &str, min: T, max: T) -> EnvResult<T>
where
    T: FromStr + PartialOrd + Copy + std::fmt::Display,
    T::Err: std::fmt::Display,
{
    let value: T = parse_env(key)?;
    
    if value < min || value > max {
        return Err(invalid_value_error(
            key,
            &format!("{}", value),
            &format!("number between {} and {}", min, max),
            "Value is outside the valid range"
        ));
    }
    
    Ok(value)
}

/// Parse an environment variable as a comma-separated list
/// 
/// Splits the environment variable by commas and trims whitespace.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env_list;
/// 
/// // ALLOWED_HOSTS=localhost,127.0.0.1,example.com
/// let hosts = parse_env_list("ALLOWED_HOSTS")?;
/// ```
pub fn parse_env_list(key: &str) -> EnvResult<Vec<String>> {
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    
    Ok(
        value
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    )
}

/// Parse an environment variable as a colon-separated list (Unix-style)
/// 
/// Splits the environment variable by colons and trims whitespace.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env_colon_list;
/// 
/// // LD_LIBRARY_PATH=/usr/lib:/usr/local/lib
/// let paths = parse_env_colon_list("LD_LIBRARY_PATH")?;
/// ```
pub fn parse_env_colon_list(key: &str) -> EnvResult<Vec<String>> {
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    
    Ok(
        value
            .split(':')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    )
}

/// Parse an environment variable as a semicolon-separated list (Windows-style)
/// 
/// Splits the environment variable by semicolons and trims whitespace.
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env_semicolon_list;
/// 
/// // PATH=C:\Program Files;C:\Windows\System32
/// let paths = parse_env_semicolon_list("PATH")?;
/// ```
pub fn parse_env_semicolon_list(key: &str) -> EnvResult<Vec<String>> {
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    
    Ok(
        value
            .split(';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    )
}

/// Parse an environment variable as a platform-appropriate path list
/// 
/// Uses the platform's path separator (: on Unix, ; on Windows).
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env_path_list;
/// 
/// let library_paths = parse_env_path_list("LD_LIBRARY_PATH")?;
/// ```
pub fn parse_env_path_list(key: &str) -> EnvResult<Vec<String>> {
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    let separator = get_path_separator();
    
    Ok(
        value
            .split(separator)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    )
}

/// Parse an environment variable as a key=value configuration map
/// 
/// Supports formats like: "key1=value1,key2=value2" or "key1=value1;key2=value2"
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env_config;
/// 
/// // DATABASE_CONFIG=host=localhost,port=5432,user=admin
/// let config = parse_env_config("DATABASE_CONFIG", ",")?;
/// ```
pub fn parse_env_config(key: &str, separator: &str) -> EnvResult<std::collections::HashMap<String, String>> {
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    let mut config = std::collections::HashMap::new();
    
    for pair in value.split(separator) {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = pair.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(invalid_value_error(
                key,
                &value,
                "key=value pairs",
                &format!("Invalid pair format: '{}'", pair)
            ));
        }
        
        config.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
    }
    
    Ok(config)
}

/// Parse an environment variable as a duration in seconds
/// 
/// Supports suffixes: s (seconds), m (minutes), h (hours), d (days)
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env_duration;
/// 
/// // TIMEOUT=30s or TIMEOUT=5m or TIMEOUT=2h
/// let timeout = parse_env_duration("TIMEOUT")?;
/// ```
pub fn parse_env_duration(key: &str) -> EnvResult<std::time::Duration> {
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    let value = value.trim();
    
    if value.is_empty() {
        return Err(invalid_value_error(key, &value, "duration", "Empty value"));
    }
    
    let (number_part, suffix) = if let Some(last_char) = value.chars().last() {
        if last_char.is_alphabetic() {
            (&value[..value.len()-1], &value[value.len()-1..])
        } else {
            (value, "s") // Default to seconds
        }
    } else {
        return Err(invalid_value_error(key, &value, "duration", "Invalid format"));
    };
    
    let number: f64 = number_part.parse().map_err(|err| {
        invalid_value_error(key, &value, "duration", &format!("Invalid number: {}", err))
    })?;
    
    let seconds = match suffix.to_lowercase().as_str() {
        "s" => number,
        "m" => number * 60.0,
        "h" => number * 3600.0,
        "d" => number * 86400.0,
        _ => return Err(invalid_value_error(
            key,
            &value,
            "duration",
            "Unsupported suffix. Use s, m, h, or d"
        )),
    };
    
    if seconds < 0.0 {
        return Err(invalid_value_error(key, &value, "duration", "Duration cannot be negative"));
    }
    
    Ok(std::time::Duration::from_secs_f64(seconds))
}

/// Parse an environment variable as a memory size in bytes
/// 
/// Supports suffixes: B (bytes), KB (kilobytes), MB (megabytes), GB (gigabytes)
/// 
/// # Examples
/// ```
/// use cursed::stdlib::env::parse_env_memory_size;
/// 
/// // MAX_MEMORY=512MB or MAX_MEMORY=2GB
/// let max_memory = parse_env_memory_size("MAX_MEMORY")?;
/// ```
pub fn parse_env_memory_size(key: &str) -> EnvResult<u64> {
    let value = get_env(key).ok_or_else(|| not_found_error(key))?;
    let value = value.trim().to_uppercase();
    
    if value.is_empty() {
        return Err(invalid_value_error(key, &value, "memory size", "Empty value"));
    }
    
    let (number_part, suffix) = if value.ends_with("GB") {
        (&value[..value.len()-2], "GB")
    } else if value.ends_with("MB") {
        (&value[..value.len()-2], "MB") 
    } else if value.ends_with("KB") {
        (&value[..value.len()-2], "KB")
    } else if value.ends_with("B") {
        (&value[..value.len()-1], "B")
    } else {
        (value.as_str(), "B") // Default to bytes
    };
    
    let number: f64 = number_part.parse().map_err(|err| {
        invalid_value_error(key, &value, "memory size", &format!("Invalid number: {}", err))
    })?;
    
    let bytes = match suffix {
        "B" => number,
        "KB" => number * 1024.0,
        "MB" => number * 1024.0 * 1024.0,
        "GB" => number * 1024.0 * 1024.0 * 1024.0,
        _ => return Err(invalid_value_error(
            key,
            &value,
            "memory size",
            "Unsupported suffix. Use B, KB, MB, or GB"
        )),
    };
    
    if bytes < 0.0 {
        return Err(invalid_value_error(key, &value, "memory size", "Size cannot be negative"));
    }
    
    Ok(bytes as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::env::core::{set_env, remove_env};

    #[test]
    fn test_parse_env_int() {
        let key = "CURSED_TEST_INT";
        set_env(key, "42").unwrap();
        
        let value: i32 = parse_env(key).unwrap();
        assert_eq!(value, 42);
        
        remove_env(key).unwrap();
    }

    #[test]
    fn test_parse_env_bool() {
        let key = "CURSED_TEST_BOOL";
        
        // Test true values
        for true_val in &["true", "TRUE", "1", "yes", "on"] {
            set_env(key, true_val).unwrap();
            assert_eq!(get_bool_env(key).unwrap(), true);
        }
        
        // Test false values
        for false_val in &["false", "FALSE", "0", "no", "off"] {
            set_env(key, false_val).unwrap();
            assert_eq!(get_bool_env(key).unwrap(), false);
        }
        
        remove_env(key).unwrap();
    }

    #[test]
    fn test_parse_env_with_default() {
        let key = "CURSED_NONEXISTENT_INT";
        let default_val = 100;
        
        let value: i32 = parse_env_with_default(key, default_val).unwrap();
        assert_eq!(value, default_val);
    }

    #[test]
    fn test_parse_env_list() {
        let key = "CURSED_TEST_LIST";
        set_env(key, "item1,item2, item3 , item4").unwrap();
        
        let list = parse_env_list(key).unwrap();
        assert_eq!(list, vec!["item1", "item2", "item3", "item4"]);
        
        remove_env(key).unwrap();
    }

    #[test]
    fn test_parse_env_config() {
        let key = "CURSED_TEST_CONFIG";
        set_env(key, "host=localhost,port=5432,user=admin").unwrap();
        
        let config = parse_env_config(key, ",").unwrap();
        assert_eq!(config.get("host"), Some(&"localhost".to_string()));
        assert_eq!(config.get("port"), Some(&"5432".to_string()));
        assert_eq!(config.get("user"), Some(&"admin".to_string()));
        
        remove_env(key).unwrap();
    }

    #[test]
    fn test_parse_env_duration() {
        let key = "CURSED_TEST_DURATION";
        
        set_env(key, "30s").unwrap();
        assert_eq!(parse_env_duration(key).unwrap(), std::time::Duration::from_secs(30));
        
        set_env(key, "5m").unwrap();
        assert_eq!(parse_env_duration(key).unwrap(), std::time::Duration::from_secs(300));
        
        set_env(key, "2h").unwrap();
        assert_eq!(parse_env_duration(key).unwrap(), std::time::Duration::from_secs(7200));
        
        remove_env(key).unwrap();
    }

    #[test]
    fn test_parse_env_memory_size() {
        let key = "CURSED_TEST_MEMORY";
        
        set_env(key, "1024B").unwrap();
        assert_eq!(parse_env_memory_size(key).unwrap(), 1024);
        
        set_env(key, "1KB").unwrap();
        assert_eq!(parse_env_memory_size(key).unwrap(), 1024);
        
        set_env(key, "1MB").unwrap();
        assert_eq!(parse_env_memory_size(key).unwrap(), 1024 * 1024);
        
        remove_env(key).unwrap();
    }

    #[test]
    fn test_get_numeric_env() {
        let key = "CURSED_TEST_NUMERIC";
        set_env(key, "50").unwrap();
        
        let value: i32 = get_numeric_env(key, 1, 100).unwrap();
        assert_eq!(value, 50);
        
        // Test out of range
        set_env(key, "150").unwrap();
        assert!(get_numeric_env::<i32>(key, 1, 100).is_err());
        
        remove_env(key).unwrap();
    }
}
