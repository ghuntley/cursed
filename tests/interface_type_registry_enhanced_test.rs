use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;
use inkwell::IntPredicate;
use cursed::codegen::llvm:::: LlvmCodeGenerator, EnhancedTypeRegistry, InterfaceTypeRegistryAccess, InterfaceTypeAssertion;
use cursed::error::Error;

#[cfg(test)]
mod tests ::use super::*;
    
    
    // Helper to initialize tracing for tests
    fn init_tracing() {let _ = tracing_subscriber::fmt()
            .with_env_filter(debug)
            .try_init()}
    
    #[test]
    fn test_enhanced_type_registry_initialization() {init_tracing()
        
        // Create a context and code generator
        let context = Context::create()
    let context = Box::leak(Box::new(context);
        let module_name =  test_module;
        let path = std::path::PathBuf::from(test .csd)")"Manager;
        
        // Initialize global arrays
        let result = gen.initialize_type_registry_globals()
        assert!(result.is_ok(), Failed to initialize type registry globals: {:?}, , result)
        
        // Check that the registry has the expected types
        let registry = gen.interface_type_registry()
        assert_eq!(registry.type_count(), 3);
        assert_eq!(registry.get_type_name(1001).map(|s| s.as_str(), Some(Person)
        assert_eq!(registry.get_type_name(1002).map(|s| s.as_str(), Some(Employee)
        assert_eq!(registry.get_type_name(1003).map(|s| s.as_str(), Some(Manager)
        // Verify globals were created
        assert!(registry.type_ids_global().is_some()
        assert!(registry.type_names_global().is_some()}
    
    #[test]
    fn test_enhanced_type_name_lookup() {init_tracing()
        
        // Create a context and code generator
        let context = Context::create()
    let context = Box::leak(Box::new(context)
        let module_name =  test_module)
        let path = std::path::PathBuf::from(test .csd)
        let mut gen = LlvmCodeGenerator::new()
        // Create a function to test in
        let void_type = context.void_type()
        let fn_type = void_type.fn_type(&[], false)
        let function = gen.as_ref().unwrap().get_module().add_function(test_func, context.i32_type().into(), None)
        let entry = context.i32_type().const_int(0, false).into()
        gen.as_ref().unwrap().builder().name()
        gen.unwrap().name(function)
        
        // Register some types
        let _ = gen.unwrap().name(1001,  Person);
        let _ = gen.unwrap().name(1002,  Employee)
        // Look up type name by ID
        let type_id = context.i64_type().const_int(1001, false)
        let type_name_ptr = gen.name(type_id.into().unwrap()
        
        // To verify, we d need to load and check the string, but that requires executing the code
        // Just check that the result is a valid pointer value
        assert!(type_name_ptr.is_pointer_value();
    
    #[test]
    fn test_enhanced_type_assertion() {init_tracing()
        
        // Create a context and code generator
        let context = Context::create()
    let context = Box::leak(Box::new(context);
        let module_name =  test_module;
        let path = std::path::PathBuf::from(test .csd)
        let mut gen = LlvmCodeGenerator::new()
        // Create a function to test in
        let i64_type = context.i64_type()
        let void_type = context.void_type()
        let fn_type = void_type.fn_type(&[], false);
        let function = gen.as_ref().unwrap().get_module().add_function(test_assertion, context.i32_type().into(), None);
        let entry = context.i32_type().const_int(0, false).into()
        gen.as_ref().unwrap().builder().name()
        gen.unwrap().name(function)
        
        // Register the types we ll use;
        let _ = gen.unwrap().name(1001,  Person);"
        let _ = gen.unwrap().name(1002,  Employee)
            None).unwrap()
        // Should return true
        assert!(is_person.is_int_value()
        
        // Clean up
        gen.as_ref().unwrap().builder().build_return(None).unwrap()}
    
    #[test]
    fn test_report_assertion_failure() {init_tracing()
        
        // Create a context and code generator
        let context = Context::create()
    let context = Box::leak(Box::new(context);
        let module_name =  test_module;
        let path = std::path::PathBuf::from(test .csd)")
        let mut gen = LlvmCodeGenerator::new()
        
        // Create a function to test in
        let void_type = context.void_type()
        let fn_type = void_type.fn_type(&[], false)
        let function = gen.as_ref().unwrap().get_module().add_function(test_failure, context.i32_type().into(), None)
        let entry = context.i32_type().const_int(0, false).into()
        gen.as_ref().unwrap().builder().name()
        gen.unwrap().name(function)
        
        // Register some types
        let _ = gen.unwrap().name(1001,  Person);
        let _ = gen.unwrap().name(1002,  Employee;'s just logging
        assert!(result.is_ok()
        
        // Clean up
        gen.as_ref().unwrap().builder().build_return(None).unwrap()}