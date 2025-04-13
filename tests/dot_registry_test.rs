//! Tests for the dot expression registry

#[cfg(test)]
mod tests {
    use cursed::stdlib::dot_registry::{DOT_REGISTRY, is_supported, execute_dot, get_packages, get_functions};
    use cursed::error::Error;
    
    #[test]
    fn test_default_handlers_registered() {
        // Check that our default handlers are registered
        assert!(is_supported("vibez", "spill"));
        assert!(is_supported("htmlrizzler", "escape_html"));
        assert!(is_supported("timez", "Now"));
    }
    
    #[test]
    fn test_vibez_spill() {
        // Test the vibez.spill function
        let result = execute_dot("vibez", "spill", vec!["Hello, world!".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }
    
    #[test]
    fn test_htmlrizzler_escape_html() {
        // Test the htmlrizzler.escape_html function
        let input = "<script>alert('XSS');</script>";
        let result = execute_dot("htmlrizzler", "escape_html", vec![input.to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "&lt;script&gt;alert(&#39;XSS&#39;);&lt;/script&gt;");
    }
    
    #[test]
    fn test_timez_now() {
        // Test the timez.Now function
        let result = execute_dot("timez", "Now", vec![]);
        assert!(result.is_ok());
        // We can't assert on the exact value since it'll be different each time,
        // but we can check that it's a number followed by 's'
        let time_str = result.unwrap();
        assert!(time_str.ends_with('s'));
        let time_value = time_str.trim_end_matches('s');
        assert!(time_value.parse::<u64>().is_ok());
    }
    
    #[test]
    fn test_get_packages() {
        // Test the get_packages function
        let packages = get_packages();
        assert!(packages.contains(&"vibez".to_string()));
        assert!(packages.contains(&"htmlrizzler".to_string()));
        assert!(packages.contains(&"timez".to_string()));
    }
    
    #[test]
    fn test_get_functions() {
        // Test the get_functions function
        let vibez_functions = get_functions("vibez");
        assert!(vibez_functions.contains(&"spill".to_string()));
        
        let htmlrizzler_functions = get_functions("htmlrizzler");
        assert!(htmlrizzler_functions.contains(&"escape_html".to_string()));
        
        let timez_functions = get_functions("timez");
        assert!(timez_functions.contains(&"Now".to_string()));
    }
    
    #[test]
    fn test_custom_handler_registration() {
        // We need to modify the global registry, so we need to lock it
        if let Ok(mut registry) = DOT_REGISTRY.lock() {
            // Register a custom handler
            registry.register_handler("testz", "hello", |args| {
                if args.is_empty() {
                    Ok("Hello, world!".to_string())
                } else {
                    Ok(format!("Hello, {}!", args[0]))
                }
            });
            
            // Check that our custom handler is registered
            assert!(registry.has_handler("testz", "hello"));
            
            // Test with no arguments
            let result = registry.execute("testz", "hello", vec![]);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Hello, world!");
            
            // Test with an argument
            let result = registry.execute("testz", "hello", vec!["CURSED".to_string()]);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Hello, CURSED!");
        } else {
            panic!("Failed to lock dot registry");
        }
    }
}