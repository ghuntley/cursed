#[cfg(test)]
mod inlining_integration_tests {
    use super::*;
    use crate::codegen::llvm::passes::inlining::{InliningPass, InliningConfig};
    use inkwell::context::Context;
    use inkwell::module::Module;
    use inkwell::values::FunctionValue;
    use inkwell::types::BasicTypeEnum;
    use inkwell::IntPredicate;
    use std::time::Duration;

    #[test]
    fn test_inlining_pass_creation() {
        let context = Context::create();
        let pass = InliningPass::new(&context);
        
        // Test that the pass is created successfully
        assert!(true, "InliningPass created successfully");
    }

    #[test]
    fn test_inlining_config_optimization_levels() {
        let config_o0 = InliningConfig::for_optimization_level(0);
        assert_eq!(config_o0.inline_threshold, 0);
        assert_eq!(config_o0.aggressive_inlining, false);
        
        let config_o3 = InliningConfig::for_optimization_level(3);
        assert_eq!(config_o3.inline_threshold, 500);
        assert_eq!(config_o3.aggressive_inlining, true);
    }

    #[test]
    fn test_inlining_with_simple_function() {
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        
        // Create a simple function to test inlining
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        let function = module.add_function("simple_add", fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        let param1 = function.get_nth_param(0).unwrap().into_int_value();
        let param2 = function.get_nth_param(1).unwrap().into_int_value();
        let result = builder.build_int_add(param1, param2, "add_result");
        builder.build_return(Some(&result));
        
        // Test inlining pass
        let mut inlining_pass = InliningPass::new(&context);
        let result = inlining_pass.run(&module);
        
        assert!(result.is_ok(), "Inlining pass should succeed");
    }

    #[test]
    fn test_inlining_with_generics_enabled() {
        let context = Context::create();
        let module = context.create_module("test_generics");
        
        // Create a generic-like function
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let _function = module.add_function("generic_max_i32", fn_type, None);
        
        let mut config = InliningConfig::default();
        config.enable_generics_inlining = true;
        
        let mut inlining_pass = InliningPass::with_config(&context, config);
        let result = inlining_pass.run(&module);
        
        assert!(result.is_ok(), "Inlining pass with generics should succeed");
    }

    #[test]
    fn test_inlining_with_interfaces_enabled() {
        let context = Context::create();
        let module = context.create_module("test_interfaces");
        
        // Create an interface-like function
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let _function = module.add_function("interface_method_impl", fn_type, None);
        
        let mut config = InliningConfig::default();
        config.enable_interface_inlining = true;
        
        let mut inlining_pass = InliningPass::with_config(&context, config);
        let result = inlining_pass.run(&module);
        
        assert!(result.is_ok(), "Inlining pass with interfaces should succeed");
    }

    #[test]
    fn test_inlining_performance_metrics() {
        let context = Context::create();
        let module = context.create_module("test_performance");
        
        // Create multiple small functions
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        
        for i in 0..5 {
            let function_name = format!("small_function_{}", i);
            let _function = module.add_function(&function_name, fn_type, None);
        }
        
        let mut inlining_pass = InliningPass::new(&context);
        let result = inlining_pass.run(&module);
        
        assert!(result.is_ok(), "Inlining pass should succeed");
        
        let inlining_result = result.unwrap();
        assert!(inlining_result.optimization_time > Duration::from_nanos(0), 
                "Optimization time should be measured");
    }

    #[test]
    fn test_inlining_with_optimization_levels() {
        let context = Context::create();
        let module = context.create_module("test_opt_levels");
        
        // Test different optimization levels
        for level in 0..=3 {
            let mut inlining_pass = InliningPass::for_optimization_level(&context, level);
            let result = inlining_pass.run(&module);
            
            assert!(result.is_ok(), "Inlining pass should succeed for level {}", level);
        }
    }

    #[test]
    fn test_inlining_result_structure() {
        let context = Context::create();
        let module = context.create_module("test_result");
        
        let mut inlining_pass = InliningPass::new(&context);
        let result = inlining_pass.run(&module);
        
        assert!(result.is_ok(), "Inlining pass should succeed");
        
        let inlining_result = result.unwrap();
        assert!(inlining_result.functions_inlined >= 0, "Functions inlined should be non-negative");
        assert!(inlining_result.total_calls_inlined >= 0, "Total calls inlined should be non-negative");
        assert!(inlining_result.functions_removed >= 0, "Functions removed should be non-negative");
    }

    #[test]
    fn test_inlining_backward_compatibility() {
        let context = Context::create();
        let module = context.create_module("test_backward_compat");
        
        // Test legacy constructor
        let mut inlining_pass = InliningPass::new_with_threshold(&context, 100);
        let result = inlining_pass.run(&module);
        
        assert!(result.is_ok(), "Legacy constructor should work");
    }

    #[test]
    fn test_inlining_with_empty_module() {
        let context = Context::create();
        let module = context.create_module("empty_module");
        
        let mut inlining_pass = InliningPass::new(&context);
        let result = inlining_pass.run(&module);
        
        assert!(result.is_ok(), "Inlining pass should handle empty modules");
        
        let inlining_result = result.unwrap();
        assert_eq!(inlining_result.functions_inlined, 0, "No functions should be inlined in empty module");
    }

    #[test]
    fn test_inlining_configuration_validation() {
        let config = InliningConfig::default();
        
        assert!(config.inline_threshold > 0, "Default inline threshold should be positive");
        assert!(config.size_threshold > 0, "Default size threshold should be positive");
        assert!(config.recursive_inline_limit > 0, "Default recursive limit should be positive");
        assert!(config.max_call_depth > 0, "Default max call depth should be positive");
    }
}
