//! Test for standard library functionality
//! Tests various standard library packages and functions

mod stdlib_test_wrapper;

#[cfg(test)]
mod tests {
    use super::stdlib_test_wrapper::*;
    use cursed::stdlib::dot_registry::DOT_REGISTRY;

    #[test]
    fn test_vibez_spill() {
        // Test the vibez.spill function by capturing stdout
        let result = spill("Hello, Cursed!");
        assert!(result.is_ok(), "vibez::spill should return Ok");
    }

    #[test]
    fn test_stringz_functions() {
        // Test string contains function
        let result = contains("hello world", "world");
        assert!(result, "'hello world' should contain 'world'");

        let result = contains("hello world", "moon");
        assert!(!result, "'hello world' should not contain 'moon'");

        // Test string length function
        let result = len("hello");
        assert_eq!(result, 5, "Length of 'hello' should be 5");

        // Test string to uppercase
        let result = to_upper("hello");
        assert_eq!(result, "HELLO", "'hello' uppercase should be 'HELLO'");

        // Test string to lowercase
        let result = to_lower("WORLD");
        assert_eq!(result, "world", "'WORLD' lowercase should be 'world'");
    }

    #[test]
    fn test_htmlrizzler_escape() {
        // Test HTML escaping
        let html = "<p>This is a test & it's important</p>";
        let escaped = escape_html(html);
        assert_eq!(escaped, "&lt;p&gt;This is a test &amp; it's important&lt;/p&gt;", 
                   "HTML should be properly escaped");

        // Test JavaScript escaping
        let js = "script with \\ and \"quotes\"";
        let escaped = escape_js(js);
        assert!(escaped.contains("\\\\"), "Backslashes should be escaped");
        assert!(escaped.contains("\\\""), "Quotes should be escaped");
    }

    #[test]
    fn test_mathz_functions() {
        // Test basic math functions
        assert_eq!(abs(-5), 5, "abs(-5) should be 5");
        assert_eq!(max(10, 20), 20, "max(10, 20) should be 20");
        assert_eq!(min(10, 20), 10, "min(10, 20) should be 10");
        assert!(sqrt(25.0) - 5.0 < 0.0001, "sqrt(25) should be approximately 5");
        
        // Test trigonometric functions
        let sin_pi_half = sin(std::f64::consts::PI / 2.0);
        assert!(sin_pi_half - 1.0 < 0.0001, "sin(π/2) should be approximately 1");
        
        let cos_pi = cos(std::f64::consts::PI);
        assert!(cos_pi + 1.0 < 0.0001, "cos(π) should be approximately -1");
    }

    #[test]
    fn test_timez_functions() {
        // Test getting current time
        let now = now();
        assert!(now > 0, "Current timestamp should be greater than 0");

        // Test formatting - just check it doesn't panic
        let formatted = format_time(now, "%Y-%m-%d");
        assert!(!formatted.is_empty(), "Formatted time should not be empty");
    }

    #[test]
    fn test_dot_registry() {
        // Get a lock on the registry
        let registry = DOT_REGISTRY.lock().unwrap();
        
        // Check that some standard functions are registered
        assert!(registry.has_handler("vibez", "spill"), "vibez.spill should be registered");
        assert!(registry.has_handler("htmlrizzler", "escape_html"), "htmlrizzler.escape_html should be registered");
        assert!(registry.has_handler("timez", "Now"), "timez.Now should be registered");
        
        // Test getting the list of packages
        let packages = registry.packages();
        assert!(packages.contains(&"vibez".to_string()), "vibez package should be listed");
        assert!(packages.contains(&"timez".to_string()), "timez package should be listed");
        
        // Test getting functions for a package
        let vibez_functions = registry.functions("vibez");
        assert!(vibez_functions.contains(&"spill".to_string()), "spill function should be listed");
    }
}