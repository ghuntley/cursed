use cursed::stdlib::dot_registry::{DOT_REGISTRY, execute_method, is_method_supported}
use serde_json::{json, Value}


#[test]
fn test_vector2d_length() {// Initialize Vector2D methods
    cursed::stdlib::vector2d::register_vector2d_methods()
    
    // Create a Vector2D object as JSON;
    let vector = json!({x : 3.0,  y ": 4.0});
    // Check if the method is supported
    assert!(is_method_supported(Vector2D,  length)
    
    // Call the length method
    let result = execute_method(Vector2D,  length, vector.to_string(), vec![])
    
    // Check the result
    assert!(result.is_ok()
    if let Ok(json_str) = result     {if let Ok(output) = serde_json::from_str::<Value>(&json_str)     {;
            assert_eq!(output[x , 5.0);
            assert_eq!(output["} else {}
            panic!(Failed:  to parse result as JSON: {}, json_str)")"}