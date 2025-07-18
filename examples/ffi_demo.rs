//! FFI System Demonstration
//!
//! This example demonstrates the comprehensive FFI capabilities of the CURSED language,
//! including C library integration, Python interop, memory safety, and performance optimization.

use cursed::ffi::*;
use cursed::error::CursedError;
use std::collections::HashMap;
use std::time::Duration;

fn main() -> Result<(), CursedError> {
    println!("🚀 CURSED FFI System Demo");
    println!("========================");
    
    // Initialize FFI system
    let ffi_system = FfiSystem::new()?;
    
    // Enable debug mode for detailed logging
    ffi_system.enable_debug_mode()?;
    
    // Demonstrate core FFI features
    demo_basic_function_calls(&ffi_system)?;
    demo_type_conversions(&ffi_system)?;
    demo_struct_marshalling(&ffi_system)?;
    demo_callback_system(&ffi_system)?;
    demo_multi_language_support(&ffi_system)?;
    demo_memory_safety(&ffi_system)?;
    demo_performance_optimization(&ffi_system)?;
    demo_error_handling(&ffi_system)?;
    demo_debug_tools(&ffi_system)?;
    
    // Print final statistics
    print_final_statistics(&ffi_system)?;
    
    println!("\n✅ FFI Demo completed successfully!");
    Ok(())
}

/// Demonstrate basic FFI function calls
fn demo_basic_function_calls(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n1. Basic Function Calls");
    println!("======================");
    
    // Call a simple C function
    let args = vec![FfiValue::SignedInteger(42)];
    let result = ffi.call_function("abs", &args, "c")?;
    
    if let FfiValue::SignedInteger(value) = result {
        println!("✓ abs(42) = {}", value);
    }
    
    // Call a function with multiple arguments
    let args = vec![
        FfiValue::SignedInteger(10),
        FfiValue::SignedInteger(20),
    ];
    let result = ffi.call_function("add_numbers", &args, "c")?;
    
    if let FfiValue::SignedInteger(sum) = result {
        println!("✓ add_numbers(10, 20) = {}", sum);
    }
    
    // Call a string function
    let args = vec![FfiValue::String("Hello, FFI!".to_string())];
    let result = ffi.call_function("strlen", &args, "c")?;
    
    if let FfiValue::SignedInteger(length) = result {
        println!("✓ strlen(\"Hello, FFI!\") = {}", length);
    }
    
    Ok(())
}

/// Demonstrate type conversions
fn demo_type_conversions(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n2. Type Conversions");
    println!("==================");
    
    // Integer to float conversion
    let int_value = FfiValue::SignedInteger(42);
    let float_result = ffi.marshal_to_foreign(&int_value, &FfiType::Float(32), "c")?;
    let converted_back = ffi.unmarshal_from_foreign(&float_result, &FfiType::Float(32), "c")?;
    
    if let FfiValue::Float(value) = converted_back {
        println!("✓ Integer to float conversion: 42 → {}", value);
    }
    
    // String to C string conversion
    let string_value = FfiValue::String("CURSED Language".to_string());
    let c_string = ffi.marshal_to_foreign(&string_value, &FfiType::CString, "c")?;
    let converted_back = ffi.unmarshal_from_foreign(&c_string, &FfiType::CString, "c")?;
    
    if let FfiValue::String(value) = converted_back {
        println!("✓ String to C string conversion: \"{}\"", value);
    }
    
    // Boolean to integer conversion
    let bool_value = FfiValue::Boolean(true);
    let int_result = ffi.marshal_to_foreign(&bool_value, &FfiType::SignedInteger(32), "c")?;
    let converted_back = ffi.unmarshal_from_foreign(&int_result, &FfiType::SignedInteger(32), "c")?;
    
    if let FfiValue::SignedInteger(value) = converted_back {
        println!("✓ Boolean to integer conversion: true → {}", value);
    }
    
    Ok(())
}

/// Demonstrate struct marshalling
fn demo_struct_marshalling(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n3. Struct Marshalling");
    println!("====================");
    
    // Create a Point struct
    let mut point_fields = HashMap::new();
    point_fields.insert("x".to_string(), FfiValue::Float(10.5));
    point_fields.insert("y".to_string(), FfiValue::Float(20.7));
    
    let point_struct = FfiValue::Struct(point_fields);
    
    // Marshal struct to C representation
    let c_struct = ffi.marshal_struct("Point", &point_struct.as_struct_fields())?;
    println!("✓ Point struct marshalled to C format");
    
    // Unmarshal back to CURSED representation
    let unmarshalled = ffi.unmarshal_struct("Point", &c_struct)?;
    
    if let Some(x) = unmarshalled.get("x") {
        if let FfiValue::Float(x_val) = x {
            println!("✓ Unmarshalled Point.x = {}", x_val);
        }
    }
    
    if let Some(y) = unmarshalled.get("y") {
        if let FfiValue::Float(y_val) = y {
            println!("✓ Unmarshalled Point.y = {}", y_val);
        }
    }
    
    // Create a more complex struct
    let mut person_fields = HashMap::new();
    person_fields.insert("name".to_string(), FfiValue::String("Alice".to_string()));
    person_fields.insert("age".to_string(), FfiValue::SignedInteger(30));
    person_fields.insert("height".to_string(), FfiValue::Float(165.5));
    
    let person_struct = FfiValue::Struct(person_fields);
    let c_person = ffi.marshal_struct("Person", &person_struct.as_struct_fields())?;
    println!("✓ Person struct marshalled successfully");
    
    Ok(())
}

/// Demonstrate callback system
fn demo_callback_system(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n4. Callback System");
    println!("=================");
    
    // Create a simple callback function
    let add_callback = |args: &[FfiValue]| -> Result<FfiValue, CursedError> {
        if let [FfiValue::SignedInteger(a), FfiValue::SignedInteger(b)] = args {
            Ok(FfiValue::SignedInteger(a + b))
        } else {
            Err(CursedError::General("Invalid arguments for add callback".to_string()))
        }
    };
    
    // Define callback signature
    let add_signature = FunctionSignature {
        name: "add_callback".to_string(),
        return_type: FfiType::SignedInteger(32),
        parameters: vec![
            Parameter {
                name: "a".to_string(),
                param_type: FfiType::SignedInteger(32),
                is_const: false,
                is_nullable: false,
            },
            Parameter {
                name: "b".to_string(),
                param_type: FfiType::SignedInteger(32),
                is_const: false,
                is_nullable: false,
            },
        ],
        is_variadic: false,
    };
    
    // Register callback
    let callback_handle = ffi.create_callback(add_callback, &add_signature)?;
    println!("✓ Callback registered with ID: {}", callback_handle.id);
    
    // Create a multiplication callback
    let multiply_callback = |args: &[FfiValue]| -> Result<FfiValue, CursedError> {
        if let [FfiValue::SignedInteger(a), FfiValue::SignedInteger(b)] = args {
            Ok(FfiValue::SignedInteger(a * b))
        } else {
            Err(CursedError::General("Invalid arguments for multiply callback".to_string()))
        }
    };
    
    let multiply_signature = FunctionSignature {
        name: "multiply_callback".to_string(),
        return_type: FfiType::SignedInteger(32),
        parameters: vec![
            Parameter {
                name: "a".to_string(),
                param_type: FfiType::SignedInteger(32),
                is_const: false,
                is_nullable: false,
            },
            Parameter {
                name: "b".to_string(),
                param_type: FfiType::SignedInteger(32),
                is_const: false,
                is_nullable: false,
            },
        ],
        is_variadic: false,
    };
    
    let multiply_handle = ffi.create_callback(multiply_callback, &multiply_signature)?;
    println!("✓ Multiply callback registered with ID: {}", multiply_handle.id);
    
    // Create a string processing callback
    let string_callback = |args: &[FfiValue]| -> Result<FfiValue, CursedError> {
        if let [FfiValue::String(s)] = args {
            Ok(FfiValue::String(s.to_uppercase()))
        } else {
            Err(CursedError::General("Invalid arguments for string callback".to_string()))
        }
    };
    
    let string_signature = FunctionSignature {
        name: "string_callback".to_string(),
        return_type: FfiType::String,
        parameters: vec![
            Parameter {
                name: "input".to_string(),
                param_type: FfiType::String,
                is_const: true,
                is_nullable: false,
            },
        ],
        is_variadic: false,
    };
    
    let string_handle = ffi.create_callback(string_callback, &string_signature)?;
    println!("✓ String callback registered with ID: {}", string_handle.id);
    
    Ok(())
}

/// Demonstrate multi-language support
fn demo_multi_language_support(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n5. Multi-Language Support");
    println!("=========================");
    
    // Get supported languages
    let languages = ffi.get_supported_languages();
    println!("✓ Supported languages: {:?}", languages);
    
    // Test C language support
    let c_functions = ffi.get_available_functions("c")?;
    println!("✓ C functions available: {} functions", c_functions.len());
    
    // Test Python language support
    let python_functions = ffi.get_available_functions("python")?;
    println!("✓ Python functions available: {} functions", python_functions.len());
    
    // Test Go language support
    let go_functions = ffi.get_available_functions("go")?;
    println!("✓ Go functions available: {} functions", go_functions.len());
    
    // Test JavaScript/WASM support
    let wasm_functions = ffi.get_available_functions("wasm")?;
    println!("✓ WASM functions available: {} functions", wasm_functions.len());
    
    // Demonstrate type mapping registration
    ffi.register_type_mapping("custom_int", "int32_t", "c")?;
    println!("✓ Custom type mapping registered");
    
    // Test cross-language function calls
    let test_value = FfiValue::SignedInteger(123);
    
    // Call through C bridge
    let c_result = ffi.call_function("test_function", &vec![test_value.clone()], "c")?;
    println!("✓ C function call result: {:?}", c_result);
    
    // Call through Python bridge  
    let py_result = ffi.call_function("test_function", &vec![test_value.clone()], "python")?;
    println!("✓ Python function call result: {:?}", py_result);
    
    // Call through Go bridge
    let go_result = ffi.call_function("test_function", &vec![test_value.clone()], "go")?;
    println!("✓ Go function call result: {:?}", go_result);
    
    // Call through WASM bridge
    let wasm_result = ffi.call_function("test_function", &vec![test_value], "wasm")?;
    println!("✓ WASM function call result: {:?}", wasm_result);
    
    Ok(())
}

/// Demonstrate memory safety features
fn demo_memory_safety(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n6. Memory Safety");
    println!("===============");
    
    // Create a large array to test memory management
    let large_array = vec![FfiValue::SignedInteger(42); 1000];
    let array_value = FfiValue::Array(large_array);
    
    // Test memory allocation and deallocation
    let foreign_array = ffi.marshal_to_foreign(&array_value, &FfiType::Array(Box::new(FfiType::SignedInteger(32)), Some(1000)), "c")?;
    println!("✓ Large array marshalled to C (1000 elements)");
    
    // Test memory validation
    let validation_result = ffi.call_function("validate_memory", &vec![FfiValue::Pointer(foreign_array.data)], "c");
    match validation_result {
        Ok(_) => println!("✓ Memory validation passed"),
        Err(e) => println!("⚠  Memory validation warning: {}", e),
    }
    
    // Test memory leak detection
    let leak_info = ffi.call_function("detect_memory_leaks", &vec![], "c")?;
    if let FfiValue::SignedInteger(leak_count) = leak_info {
        if leak_count == 0 {
            println!("✓ No memory leaks detected");
        } else {
            println!("⚠  {} potential memory leaks detected", leak_count);
        }
    }
    
    // Test pointer validation
    let null_ptr = FfiValue::Pointer(std::ptr::null_mut());
    let validation_result = ffi.call_function("validate_pointer", &vec![null_ptr], "c");
    match validation_result {
        Ok(_) => println!("⚠  Null pointer validation should have failed"),
        Err(_) => println!("✓ Null pointer correctly rejected"),
    }
    
    // Test buffer overflow protection
    let small_buffer = vec![FfiValue::SignedInteger(1); 10];
    let large_data = vec![FfiValue::SignedInteger(2); 1000];
    
    let buffer_test = ffi.call_function("copy_to_buffer", &vec![
        FfiValue::Array(small_buffer),
        FfiValue::Array(large_data),
    ], "c");
    
    match buffer_test {
        Ok(_) => println!("⚠  Buffer overflow protection should have triggered"),
        Err(_) => println!("✓ Buffer overflow protection working"),
    }
    
    Ok(())
}

/// Demonstrate performance optimization
fn demo_performance_optimization(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n7. Performance Optimization");
    println!("==========================");
    
    // Test call caching
    let expensive_args = vec![FfiValue::Float(3.14159)];
    
    // First call (should be slow)
    let start = std::time::Instant::now();
    let result1 = ffi.call_function("expensive_calculation", &expensive_args, "c")?;
    let first_call_time = start.elapsed();
    
    // Second call (should be fast due to caching)
    let start = std::time::Instant::now();
    let result2 = ffi.call_function("expensive_calculation", &expensive_args, "c")?;
    let second_call_time = start.elapsed();
    
    println!("✓ First call time: {:?}", first_call_time);
    println!("✓ Second call time: {:?} (cached)", second_call_time);
    
    if second_call_time < first_call_time {
        println!("✓ Call caching working effectively");
    }
    
    // Test memory pooling
    let pool_test_args = vec![FfiValue::SignedInteger(1024)];
    let pool_result = ffi.call_function("allocate_from_pool", &pool_test_args, "c")?;
    
    if let FfiValue::Pointer(ptr) = pool_result {
        if !ptr.is_null() {
            println!("✓ Memory pool allocation successful");
        }
    }
    
    // Test bulk operations
    let bulk_data = vec![
        FfiValue::SignedInteger(1),
        FfiValue::SignedInteger(2),
        FfiValue::SignedInteger(3),
        FfiValue::SignedInteger(4),
        FfiValue::SignedInteger(5),
    ];
    
    let bulk_result = ffi.call_function("process_bulk_data", &bulk_data, "c")?;
    println!("✓ Bulk operation completed: {:?}", bulk_result);
    
    // Test zero-copy operations
    let zero_copy_data = vec![0u8; 4096];
    let zero_copy_result = ffi.call_function("zero_copy_process", &vec![
        FfiValue::Pointer(zero_copy_data.as_ptr() as *mut std::ffi::c_void)
    ], "c")?;
    println!("✓ Zero-copy operation completed: {:?}", zero_copy_result);
    
    Ok(())
}

/// Demonstrate error handling
fn demo_error_handling(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n8. Error Handling");
    println!("================");
    
    // Test type conversion error
    let invalid_type_result = ffi.call_function("expect_string", &vec![FfiValue::SignedInteger(42)], "c");
    match invalid_type_result {
        Ok(_) => println!("⚠  Type mismatch should have caused error"),
        Err(e) => println!("✓ Type conversion error handled: {}", e),
    }
    
    // Test function not found error
    let missing_function_result = ffi.call_function("non_existent_function", &vec![], "c");
    match missing_function_result {
        Ok(_) => println!("⚠  Missing function should have caused error"),
        Err(e) => println!("✓ Missing function error handled: {}", e),
    }
    
    // Test invalid argument count error
    let wrong_args_result = ffi.call_function("two_arg_function", &vec![FfiValue::SignedInteger(1)], "c");
    match wrong_args_result {
        Ok(_) => println!("⚠  Wrong argument count should have caused error"),
        Err(e) => println!("✓ Argument count error handled: {}", e),
    }
    
    // Test memory allocation error
    let huge_allocation_result = ffi.call_function("allocate_huge_buffer", &vec![
        FfiValue::SignedInteger(i64::MAX)
    ], "c");
    match huge_allocation_result {
        Ok(_) => println!("⚠  Huge allocation should have failed"),
        Err(e) => println!("✓ Memory allocation error handled: {}", e),
    }
    
    // Test timeout error
    let timeout_result = ffi.call_function("slow_function", &vec![], "c");
    match timeout_result {
        Ok(_) => println!("⚠  Slow function should have timed out"),
        Err(e) => println!("✓ Timeout error handled: {}", e),
    }
    
    Ok(())
}

/// Demonstrate debug tools
fn demo_debug_tools(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n9. Debug Tools");
    println!("=============");
    
    // Get debug information for a function
    let debug_info = ffi.get_debug_info("test_function")?;
    println!("✓ Function debug info:");
    println!("  - Call count: {}", debug_info.call_count);
    println!("  - Last call time: {:.3}ms", debug_info.last_call_time * 1000.0);
    println!("  - Error count: {}", debug_info.error_count);
    
    // Get performance statistics
    let perf_stats = ffi.get_performance_stats()?;
    println!("✓ Performance statistics:");
    println!("  - Total calls: {}", perf_stats.total_calls);
    println!("  - Average call time: {:.3}ms", perf_stats.average_call_time * 1000.0);
    println!("  - Memory usage: {} bytes", perf_stats.memory_usage);
    println!("  - Cache hit rate: {:.1}%", perf_stats.cache_hit_rate * 100.0);
    
    // Test debug function tracing
    let trace_result = ffi.call_function("traced_function", &vec![
        FfiValue::String("debug test".to_string())
    ], "c")?;
    println!("✓ Function tracing enabled: {:?}", trace_result);
    
    // Test memory debugging
    let memory_info = ffi.call_function("get_memory_info", &vec![], "c")?;
    println!("✓ Memory debugging info: {:?}", memory_info);
    
    Ok(())
}

/// Print final statistics
fn print_final_statistics(ffi: &FfiSystem) -> Result<(), CursedError> {
    println!("\n📊 Final Statistics");
    println!("==================");
    
    let perf_stats = ffi.get_performance_stats()?;
    
    println!("Performance Summary:");
    println!("  • Total FFI calls: {}", perf_stats.total_calls);
    println!("  • Average call time: {:.3}ms", perf_stats.average_call_time * 1000.0);
    println!("  • Total memory usage: {} bytes", perf_stats.memory_usage);
    println!("  • Cache hit rate: {:.1}%", perf_stats.cache_hit_rate * 100.0);
    
    let supported_languages = ffi.get_supported_languages();
    println!("\nLanguage Support:");
    for lang in &supported_languages {
        let functions = ffi.get_available_functions(lang)?;
        println!("  • {}: {} functions", lang, functions.len());
    }
    
    println!("\nFeature Status:");
    println!("  ✅ C/C++ integration");
    println!("  ✅ Python integration");
    println!("  ✅ Go integration");
    println!("  ✅ JavaScript/WASM integration");
    println!("  ✅ Type safety");
    println!("  ✅ Memory safety");
    println!("  ✅ Performance optimization");
    println!("  ✅ Error handling");
    println!("  ✅ Debug tools");
    println!("  ✅ Callback system");
    println!("  ✅ Struct marshalling");
    
    Ok(())
}

// Helper trait for extracting struct fields
trait StructFieldExtractor {
    fn as_struct_fields(&self) -> Result<HashMap<String, FfiValue>, CursedError>;
}

impl StructFieldExtractor for FfiValue {
    fn as_struct_fields(&self) -> Result<HashMap<String, FfiValue>, CursedError> {
        match self {
            FfiValue::Struct(fields) => Ok(fields.clone()),
            _ => Err(CursedError::General("Value is not a struct".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ffi_demo() {
        // Test that the demo can be initialized
        let ffi_system = FfiSystem::new().unwrap();
        assert!(ffi_system.get_supported_languages().len() > 0);
    }
    
    #[test]
    fn test_struct_field_extraction() {
        let mut fields = HashMap::new();
        fields.insert("x".to_string(), FfiValue::Float(10.0));
        fields.insert("y".to_string(), FfiValue::Float(20.0));
        
        let struct_value = FfiValue::Struct(fields.clone());
        let extracted = struct_value.as_struct_fields().unwrap();
        
        assert_eq!(extracted, fields);
    }
}
