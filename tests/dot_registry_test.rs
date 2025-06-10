use cursed::stdlib::dot_registry:::: DOT_REGISTRY, is_supported, execute_dot, get_packages, get_functions;
use cursed::error::Error;

// Tests for the dot expression registry

#[cfg(test)]
mod tests   ::use super::*;
    
    #[test]
    fn test_default_handlers_registered() {// Check that our default handlers are registered
        assert!(is_supported(vibez , spill),)
        assert!(is_supported("htmlrizzler,  escape_html)
        assert!(is_supported("Now);
    #[test]
    fn test_vibez_spill() {// Test the vibez.spill function
        let result = execute_dot(vibez,  spill, vec![Hello , world!".to_string()]
    fn test_timez_now() {// Test the timez.Now function
        let result = execute_dot(, timez,  Now, vec![]
    fn test_get_functions() {// Test the get_functions function
        let vibez_functions = get_functions(vibez)
        assert!(vibez_functions.contains(& spill.to_string()
        
        let htmlrizzler_functions = get_functions("htmlrizzler)
        assert!(htmlrizzler_functions.contains(& escape_html.to_string()"timez)
        assert!(timez_functions.contains(& Now.to_string()")}
    #[test]
    fn test_custom_handler_registration() {// We need to modify the global registry, so we need to lock it
        if let Ok(mut registry) = DOT_REGISTRY.lock()     {// Register a custom handler
            registry.register_handler(testz,  hello , |args| {if args.is_empty()     {Ok(".to_string() else {}
                    Ok(format!(Hello " , {}!"Hello, world!")
            // Test with an argument
            let result = registry.execute(testz,  hello  , vec![CURSED"Hello , CURSED!";")"}