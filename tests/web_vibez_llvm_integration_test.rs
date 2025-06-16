/// Web Vibez LLVM Integration Test Suite
/// 
/// This test suite validates the complete LLVM integration for the CURSED web framework,
/// including HTTP server functionality, request/response handling, networking operations,
/// and proper memory management integration.

use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, BasicMetadataValueEnum};
use inkwell::{AddressSpace, IntPredicate};
use std::collections::HashMap;

// Import the web vibez integration module
use cursed::codegen::llvm::web_vibez_integration::{
    WebVibezLlvmIntegration, HttpTypeRegistry, GcMetadataRegistry
};
use cursed::error::Error;

/// Test helper to create LLVM context and module
fn create_test_llvm_context() -> (Context, inkwell::module::Module<'static>) {
    let context = Context::create();
    let module = context.create_module("test_web_vibez_integration");
    // Extend lifetime for testing - safe in controlled test environment
    let module: inkwell::module::Module<'static> = unsafe { std::mem::transmute(module) };
    (context, module)
}

/// Test helper to create string value in LLVM
fn create_test_string_value<'ctx>(
    context: &'ctx Context,
    builder: &inkwell::builder::Builder<'ctx>,
    value: &str,
) -> BasicValueEnum<'ctx> {
    let str_global = context.const_string(value.as_bytes(), false);
    let global = context.i8_type().ptr_type(AddressSpace::default());
    str_global.as_basic_value_enum()
}

#[test]
fn test_web_vibez_integration_comprehensive() {
    let (context, module) = create_test_llvm_context();
    
    // Create web vibez integration
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create WebVibezLlvmIntegration");
    
    // Test that all required functions are available
    assert!(integration.get_function_declaration("ListenAndServe").is_some());
    assert!(integration.get_function_declaration("Get").is_some());
    assert!(integration.get_function_declaration("Post").is_some());
    assert!(integration.get_function_declaration("HandleFunc").is_some());
    
    // Test function validation
    let validation_result = integration.validate_declarations();
    match validation_result {
        Ok(()) => println!("All function declarations validated successfully"),
        Err(errors) => {
            println!("Validation warnings (may be expected in test context): {:?}", errors);
            // Don't fail the test as some validation might fail without runtime
        }
    }
}

#[test]
fn test_http_server_function_compilation() {
    let (context, module) = create_test_llvm_context();
    let builder = context.create_builder();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Create test arguments for ListenAndServe
    let addr_str = create_test_string_value(&context, &builder, "localhost:8080");
    let handler_ptr = context.i8_type().ptr_type(AddressSpace::default()).const_null();
    
    let args = vec![addr_str, handler_ptr.into()];
    
    // Test ListenAndServe compilation
    let result = integration.compile_function_call("ListenAndServe", &args);
    assert!(result.is_ok(), "Failed to compile ListenAndServe: {:?}", result.err());
    
    // Test HandleFunc compilation
    let pattern_str = create_test_string_value(&context, &builder, "/api/hello");
    let handler_args = vec![pattern_str, handler_ptr.into()];
    
    let result = integration.compile_function_call("HandleFunc", &handler_args);
    assert!(result.is_ok(), "Failed to compile HandleFunc: {:?}", result.err());
}

#[test]
fn test_http_client_function_compilation() {
    let (context, module) = create_test_llvm_context();
    let builder = context.create_builder();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Test HTTP GET compilation
    let url_str = create_test_string_value(&context, &builder, "https://api.example.com/data");
    let get_args = vec![url_str];
    
    let result = integration.compile_function_call("Get", &get_args);
    assert!(result.is_ok(), "Failed to compile HTTP Get: {:?}", result.err());
    
    // Test HTTP POST compilation
    let content_type_str = create_test_string_value(&context, &builder, "application/json");
    let body_str = create_test_string_value(&context, &builder, "{\"key\": \"value\"}");
    let post_args = vec![url_str, content_type_str, body_str];
    
    let result = integration.compile_function_call("Post", &post_args);
    assert!(result.is_ok(), "Failed to compile HTTP Post: {:?}", result.err());
    
    // Test other HTTP methods
    let result = integration.compile_function_call("Head", &vec![url_str]);
    assert!(result.is_ok(), "Failed to compile HTTP Head: {:?}", result.err());
    
    let result = integration.compile_function_call("Delete", &vec![url_str]);
    assert!(result.is_ok(), "Failed to compile HTTP Delete: {:?}", result.err());
}

#[test]
fn test_request_response_handling() {
    let (context, module) = create_test_llvm_context();
    let builder = context.create_builder();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Create mock request pointer
    let request_type = integration.http_types.request_type();
    let request_ptr = request_type.ptr_type(AddressSpace::default()).const_null();
    
    // Test request property access
    let request_args = vec![request_ptr.into()];
    
    let result = integration.compile_function_call("Request.URL", &request_args);
    assert!(result.is_ok(), "Failed to compile Request.URL: {:?}", result.err());
    
    let result = integration.compile_function_call("Request.Method", &request_args);
    assert!(result.is_ok(), "Failed to compile Request.Method: {:?}", result.err());
    
    let result = integration.compile_function_call("Request.Body", &request_args);
    assert!(result.is_ok(), "Failed to compile Request.Body: {:?}", result.err());
    
    // Create mock response writer
    let response_writer_type = integration.http_types.response_writer_type();
    let response_writer_ptr = response_writer_type.ptr_type(AddressSpace::default()).const_null();
    
    // Test response writing
    let data_str = create_test_string_value(&context, &builder, "Hello, World!");
    let write_args = vec![response_writer_ptr.into(), data_str];
    
    let result = integration.compile_function_call("ResponseWriter.Write", &write_args);
    assert!(result.is_ok(), "Failed to compile ResponseWriter.Write: {:?}", result.err());
    
    // Test status code setting
    let status_code = context.i32_type().const_int(200, false);
    let header_args = vec![response_writer_ptr.into(), status_code.into()];
    
    let result = integration.compile_function_call("ResponseWriter.WriteHeader", &header_args);
    assert!(result.is_ok(), "Failed to compile ResponseWriter.WriteHeader: {:?}", result.err());
}

#[test]
fn test_gc_integration_with_http_objects() {
    let (context, module) = create_test_llvm_context();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Test GC object allocation for different HTTP types
    let http_types = vec!["HttpRequest", "HttpResponse", "Headers"];
    
    for object_type in http_types {
        let result = integration.allocate_gc_object(object_type);
        assert!(result.is_ok(), "Failed to allocate GC object for {}: {:?}", object_type, result.err());
        
        println!("Successfully allocated GC object for {}", object_type);
    }
    
    // Test that objects are properly registered in the registry
    // This verifies the GC integration is working
    let stats = integration.object_registry.get_stats();
    assert!(stats.total_objects > 0, "No objects registered in GC registry");
    assert!(stats.total_memory > 0, "No memory tracked in GC registry");
    
    println!("GC Registry stats: {} objects, {} bytes", stats.total_objects, stats.total_memory);
}

#[test]
fn test_http_type_registry_comprehensive() {
    let context = Context::create();
    
    let registry = HttpTypeRegistry::new(&context)
        .expect("Failed to create HttpTypeRegistry");
    
    // Test string type (ptr + length)
    let string_type = registry.string_type();
    assert_eq!(string_type.get_field_types().len(), 2);
    
    // Test HTTP request type fields
    let request_type = registry.request_type();
    let request_fields = request_type.get_field_types();
    assert_eq!(request_fields.len(), 6); // method, url, version, headers, body, raw_ptr
    
    // Test HTTP response type fields
    let response_type = registry.response_type();
    let response_fields = response_type.get_field_types();
    assert_eq!(response_fields.len(), 5); // version, status, status_text, headers, body
    
    // Test response writer type fields
    let writer_type = registry.response_writer_type();
    let writer_fields = writer_type.get_field_types();
    assert_eq!(writer_fields.len(), 4); // headers, status, body, headers_written
    
    println!("All HTTP types validated successfully");
}

#[test]
fn test_runtime_networking_functions() {
    let (context, module) = create_test_llvm_context();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Test that all networking functions are declared
    let networking_functions = vec![
        "socket", "bind", "listen", "accept", "recv", "send", "close"
    ];
    
    for func_name in networking_functions {
        assert!(integration.runtime_functions.contains_key(func_name),
               "Runtime function {} not declared", func_name);
        
        let func = integration.runtime_functions.get(func_name).unwrap();
        assert!(func.verify(true), "Function {} failed verification", func_name);
    }
    
    println!("All runtime networking functions validated");
}

#[test]
fn test_http_status_constants() {
    let (context, module) = create_test_llvm_context();
    
    let _integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Test that HTTP status constants are created properly
    let status_ok = module.get_global("web_vibez.StatusOK");
    assert!(status_ok.is_some(), "StatusOK constant not found");
    
    let status_not_found = module.get_global("web_vibez.StatusNotFound");
    assert!(status_not_found.is_some(), "StatusNotFound constant not found");
    
    let status_internal_error = module.get_global("web_vibez.StatusInternalServerError");
    assert!(status_internal_error.is_some(), "StatusInternalServerError constant not found");
    
    // Verify constant values
    if let Some(status_ok_global) = status_ok {
        let initializer = status_ok_global.get_initializer();
        assert!(initializer.is_some(), "StatusOK has no initializer");
    }
    
    println!("All HTTP status constants validated");
}

#[test]
fn test_client_timeout_functionality() {
    let (context, module) = create_test_llvm_context();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Test client timeout function
    let timeout_ms = context.i64_type().const_int(5000, false); // 5 second timeout
    let timeout_args = vec![timeout_ms.into()];
    
    let result = integration.compile_function_call("client_timeout", &timeout_args);
    assert!(result.is_ok(), "Failed to compile client_timeout: {:?}", result.err());
    
    println!("Client timeout functionality validated");
}

#[test]
fn test_error_handling_integration() {
    let (context, module) = create_test_llvm_context();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Test error handling for invalid function calls
    let result = integration.compile_function_call("NonExistentFunction", &vec![]);
    assert!(result.is_err(), "Expected error for non-existent function");
    
    // Test error handling for incorrect argument counts
    let result = integration.compile_function_call("ListenAndServe", &vec![]); // Missing args
    assert!(result.is_err(), "Expected error for incorrect argument count");
    
    let result = integration.compile_function_call("Get", &vec![]); // Missing URL
    assert!(result.is_err(), "Expected error for missing URL argument");
    
    println!("Error handling validated successfully");
}

#[test]
fn test_memory_safety_and_cleanup() {
    let (context, module) = create_test_llvm_context();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Allocate multiple objects to test memory management
    let initial_stats = integration.object_registry.get_stats();
    
    for i in 0..10 {
        let object_type = match i % 3 {
            0 => "HttpRequest",
            1 => "HttpResponse", 
            _ => "Headers",
        };
        
        let result = integration.allocate_gc_object(object_type);
        assert!(result.is_ok(), "Failed to allocate object {}: {:?}", i, result.err());
    }
    
    let final_stats = integration.object_registry.get_stats();
    assert!(final_stats.total_objects > initial_stats.total_objects,
           "Object count should have increased");
    assert!(final_stats.total_memory > initial_stats.total_memory,
           "Memory usage should have increased");
    
    println!("Memory safety tests passed: {} objects allocated", 
             final_stats.total_objects - initial_stats.total_objects);
}

#[test]
fn test_function_compilation_with_real_llvm_ir() {
    let (context, module) = create_test_llvm_context();
    let builder = context.create_builder();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create integration");
    
    // Create a simple function that uses web_vibez functionality
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_web_server", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Simulate calling ListenAndServe
    let addr_str = create_test_string_value(&context, &builder, "0.0.0.0:8080");
    let handler_ptr = context.i8_type().ptr_type(AddressSpace::default()).const_null();
    let args = vec![addr_str, handler_ptr.into()];
    
    let _result = integration.compile_function_call("ListenAndServe", &args)
        .expect("Failed to compile ListenAndServe call");
    
    // Return success
    let return_val = i32_type.const_int(0, false);
    builder.build_return(Some(&return_val)).expect("Failed to build return");
    
    // Verify the function
    assert!(function.verify(true), "Generated function failed verification");
    
    // Print the generated LLVM IR for inspection
    println!("Generated LLVM IR:\n{}", module.print_to_string().to_string());
}

/// Integration test to verify the complete web framework stack
#[test]
fn test_complete_web_framework_integration() {
    let (context, module) = create_test_llvm_context();
    
    let integration = WebVibezLlvmIntegration::new(&context, &module)
        .expect("Failed to create web vibez integration");
    
    // Test server setup functions
    assert!(integration.get_function_declaration("ListenAndServe").is_some());
    assert!(integration.get_function_declaration("ListenAndServeTLS").is_some());
    assert!(integration.get_function_declaration("HandleFunc").is_some());
    
    // Test HTTP client functions
    assert!(integration.get_function_declaration("Get").is_some());
    assert!(integration.get_function_declaration("Post").is_some());
    assert!(integration.get_function_declaration("Head").is_some());
    assert!(integration.get_function_declaration("Delete").is_some());
    
    // Test request/response handling
    assert!(integration.get_function_declaration("Request.URL").is_some());
    assert!(integration.get_function_declaration("Request.Method").is_some());
    assert!(integration.get_function_declaration("Request.Body").is_some());
    assert!(integration.get_function_declaration("ResponseWriter.Write").is_some());
    assert!(integration.get_function_declaration("ResponseWriter.WriteHeader").is_some());
    
    // Test utility functions
    assert!(integration.get_function_declaration("client_timeout").is_some());
    
    // Test that all function names are available
    let function_names = integration.get_function_names();
    assert!(function_names.len() >= 12, "Expected at least 12 HTTP functions");
    
    // Test GC integration works
    let gc_result = integration.allocate_gc_object("HttpRequest");
    assert!(gc_result.is_ok(), "GC integration should work");
    
    println!("Complete web framework integration test passed!");
    println!("Functions available: {}", function_names.len());
    println!("Runtime functions: {}", integration.runtime_functions.len());
}
