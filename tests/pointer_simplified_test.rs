//! Simplified test for pointer operations using the test helpers
//!
//! This test demonstrates the use of the standardized LLVM code generator
//! with a focus on pointer operations.

mod llvm_test_helpers;
use llvm_test_helpers::*;
use cursed::error::Error;

#[test]
fn test_pointer_simple() -> Result<(), Error> {
    // Test code that exercises pointer operations
    let input = r#"vibe pointer_simple_test

slay test_pointer_simple() normie {
    sus x = 42                 // Integer variable
    sus ptr = @x               // Take address, ptr is a pointer to x
    sus value = @ptr           // Dereference pointer, value is 42
    
    @ptr = 100                 // Store 100 through the pointer (modifies x)
    
    lowkey value == 42 {       // Original value was 42
        puts(1)
    }
    
    lowkey @ptr == 100 {       // New value is 100
        puts(1)
    }
    
    lowkey x == 100 {          // x was modified through the pointer
        puts(1)
    } fax {
        puts(0)
    }
    
    yolo value                 // Return the original value (42)
}
"#;

    // Run the test and verify the result
    let result = run_code_test::<i32>(input, "test_pointer_simple")?;
    
    // Should return 42 (the original value)
    assert_eq!(result, 42, "Pointer test returned incorrect value: {}", result);
    
    Ok(())
}

#[test]
fn test_pointer_null() -> Result<(), Error> {
    // Test code that exercises null pointer handling
    let input = r#"vibe pointer_null_test

slay test_pointer_null() normie {
    sus null_ptr = @normie(0)       // Create a null pointer (special syntax in our language)
    sus default_value = 0           // Initialize to catch the result
    
    lowkey null_ptr == @normie(0) { // Null check
        puts(1)                     // Should print 1
        default_value = 42          // Set a value to verify we entered this block
    }
    
    // This is a potentially unsafe operation
    // Our implementation should detect the null pointer and avoid crashing
    // Instead, it should return a default value of 0
    sus result = @null_ptr
    
    // We should reach this point without crashing
    puts(99)
    
    // Return a combination of our checks
    yolo default_value + result    // Should be 42 + 0 = 42
}
"#;

    // Run the test and verify the result
    let result = run_code_test::<i32>(input, "test_pointer_null")?;
    
    // Should return 42 + 0 = 42
    assert_eq!(result, 42, "Null pointer test returned incorrect value: {}", result);
    
    Ok(())
}

#[test]
fn test_pointer_struct() -> Result<(), Error> {
    // Test code that exercises struct field access through pointers
    let input = r#"vibe pointer_struct_test

be_like Point squad {
    x normie
    y normie
}

slay test_pointer_struct() normie {
    sus p = Point{x: 10, y: 20}     // Create a struct
    sus ptr = @p                    // Take address, ptr is a pointer to the struct
    
    // Access fields through pointer
    sus x_val = ptr.x
    sus y_val = ptr.y
    
    lowkey x_val == 10 {            // Original x was 10
        puts(1)
    }
    
    // Modify fields through pointer
    ptr.x = 30
    ptr.y = 40
    
    // Verify modifications affected the original struct
    lowkey p.x == 30 {
        puts(1)
    }
    
    lowkey p.y == 40 {
        puts(1)
    }
    
    // Return sum of original values
    yolo x_val + y_val              // Should be 10 + 20 = 30
}
"#;

    // Run the test and verify the result
    let result = run_code_test::<i32>(input, "test_pointer_struct")?;
    
    // Should return 10 + 20 = 30
    assert_eq!(result, 30, "Struct pointer test returned incorrect value: {}", result);
    
    Ok(())
} 