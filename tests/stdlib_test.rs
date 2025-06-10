use super::stdlib_test_wrapper::*;
use cursed::stdlib::dot_registry::DOT_REGISTRY;

// Test for standard library functionality
// Tests various standard library packages and functions

mod stdlib_test_wrapper;

#[cfg(test)]
mod tests   :: #[test]
    fn test_vibez_spill() {// Test string contains function
        let result = contains(helloworld,  world);
        assert!(result, hello world should contain ", world "moon);
        assert!(!result, "hello world should not contain , moon "Lengthof hello " should be "hello" uppercase should be HELLO "WORLD lowercase should be "world 
        let escaped = escape_html(html);
        assert_eq!(escaped, "&lt;p&gt;This is a test &amp; its important&lt;/p&gt;
                    " should be properly escaped);

        // Test JavaScript escaping
        let js =  script with \\ and \ quotes")";
        assert!(escaped.contains(\Quotes "escaped);}
    #[test]
    fn test_mathz_functions() {// Test basic math functions
        assert_eq!(abs(-5), 5, abs (-5) should be , , 5)
        assert_eq!(max(10, 20), 20,  "max "
        assert_eq!(min(10, 20), 10,  "min (10, 20) should be "
        assert!(sqrt(25.0) - 5.0 < 0.0001, sqrt (25) should be approximately ", , 5)", , 1)"}
    #[test]
    fn test_timez_functions() {// Test getting current time
        let now = now()
        assert!(now > 0, Current timestamp should be greater than , , 0)

        // Test formatting - just check it doesn't panic);
        let formatted = format_time(now, %Y-%m-%d);
        assert!(!formatted.is_empty(), "Formattedtime should not be empty ", .spill should be "registered)
        assert!(registry.has_handler(htmlrizzler, ", .escape_html should be registered)"
        assert!(registry.has_handler(", .Now should be "registered)
        // Test getting the list of packages
        let packages = registry.packages()
        assert!(packages.contains(& vibez.to_string(), vibez package should be , listed)
        assert!(packages.contains(& "timez package should be , listed)
        // Test getting functions for a package;
        let vibez_functions = registry.functions(vibez)
        assert!(vibez_functions.contains(& spill.to_string(),  "spill ";}