/// Integration tests for process execution LLVM compilation
/// 
/// Tests the end-to-end compilation of exec_slay and exec_vibez keywords
/// from CURSED source code to executable LLVM IR with proper runtime integration.

use cursed::codegen::llvm::{ProcessExecutionCompiler, LlvmCodeGeneratorReal};
use cursed::stdlib::process::core::ProcessManager;
use cursed::ast::expressions::{Expression, FunctionCall, Literal, LiteralValue};
use cursed::ast::identifiers::Identifier;
use cursed::error::Error;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use cursed::codegen::llvm::{
        initialize_process_execution_runtime, compile_exec_slay_command, compile_exec_vibez_command,
        process_runtime_integration, set_runtime_process_manager, get_runtime_process_manager
    };
    use cursed::stdlib::process::llvm_integration::ProcessLlvmIntegration;
    use tracing_test::traced_test;
    use inkwell::context::Context;
    use inkwell::module::Module;
    use inkwell::builder::Builder;
    use inkwell::values::BasicValueEnum;
    use std::sync::Arc;

    fn create_test_llvm_generator() -> LlvmCodeGeneratorReal<'static> {
        let context = Box::leak(Box::new(Context::create()));
        let module = context.create_module("test_process_execution");
        let builder = context.create_builder();
        
        LlvmCodeGeneratorReal {
            context,
            module,
            builder,
            runtime: Arc::new(cursed::runtime::Runtime::new().unwrap()),
            debug_generator: cursed::codegen::llvm::LlvmDebugCodeGenerator::new(
                cursed::debug::DebugConfig::default()
            ),
            module_name: Some("test_process_execution".to_string()),
            web_vibez_integration: None,
            expression_compiler: cursed::codegen::llvm::LlvmExpressionCompiler::new(),
            type_context: cursed::codegen::llvm::TypeCompilationContext::new("test_process_execution".to_string()),
            gc_integration: None,
            package_context: None,
            optimization_manager: None,
            optimization_enabled: false,
            temp_counter: std::cell::RefCell::new(0),
            block_counter: std::cell::RefCell::new(0),
            current_function: std::cell::RefCell::new(None),
            result_type_registry: HashMap::new(),
            option_type_registry: HashMap::new(),
            template_compiler: None,
        }
    }

    fn setup_process_manager() -> ProcessManager {
        ProcessManager::new()
    }

    #[traced_test]
    #[test]
    fn test_process_execution_runtime_initialization() {
        let mut generator = create_test_llvm_generator();
        
        // Test runtime initialization
        let result = initialize_process_execution_runtime(&mut generator);
        assert!(result.is_ok(), "Failed to initialize process execution runtime: {:?}", result.err());
        
        // Verify that runtime functions are declared in the module
        assert!(generator.module().get_function("cursed_exec_slay").is_some());
        assert!(generator.module().get_function("cursed_exec_vibez").is_some());
        assert!(generator.module().get_function("cursed_process_spawn").is_some());
        assert!(generator.module().get_function("cursed_process_wait").is_some());
        assert!(generator.module().get_function("cursed_process_signal").is_some());
        assert!(generator.module().get_function("cursed_process_terminate").is_some());
        assert!(generator.module().get_function("cursed_process_pipeline").is_some());
        assert!(generator.module().get_function("cursed_background_task").is_some());
        assert!(generator.module().get_function("cursed_io_redirection").is_some());
    }

    #[traced_test]
    #[test]
    fn test_exec_slay_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Test compiling exec_slay command
        let command = "echo";
        let args = vec!["Hello".to_string(), "World".to_string()];
        
        let result = generator.compile_exec_slay(command, &args, None);
        assert!(result.is_ok(), "Failed to compile exec_slay: {:?}", result.err());
        
        let _llvm_value = result.unwrap();
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_exec_slay"));
        assert!(module_ir.to_string().contains("echo"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_exec_vibez_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Test compiling exec_vibez command
        let command = "ls";
        let args = vec!["-la".to_string()];
        
        let result = generator.compile_exec_vibez(command, &args, None);
        assert!(result.is_ok(), "Failed to compile exec_vibez: {:?}", result.err());
        
        let _llvm_value = result.unwrap();
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_exec_vibez"));
        assert!(module_ir.to_string().contains("ls"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_process_spawn_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Test compiling process spawn with environment variables
        let command = "env";
        let args = vec![];
        let mut env = HashMap::new();
        env.insert("TEST_VAR".to_string(), "test_value".to_string());
        
        let result = generator.compile_process_spawn(command, &args, Some(&env));
        assert!(result.is_ok(), "Failed to compile process spawn: {:?}", result.err());
        
        let _pid_value = result.unwrap();
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_process_spawn"));
        assert!(module_ir.to_string().contains("env"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_process_wait_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Create a mock PID expression
        let pid_literal = Literal {
            value: LiteralValue::Integer(1234),
        };
        let pid_expr = Expression::Literal(pid_literal);
        
        let result = generator.compile_process_wait(&pid_expr);
        assert!(result.is_ok(), "Failed to compile process wait: {:?}", result.err());
        
        let _exit_code = result.unwrap();
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_process_wait"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_process_signal_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Create a mock PID expression
        let pid_literal = Literal {
            value: LiteralValue::Integer(1234),
        };
        let pid_expr = Expression::Literal(pid_literal);
        
        // Test sending SIGTERM signal
        let signal = 15; // SIGTERM
        let result = generator.compile_process_signal(&pid_expr, signal);
        assert!(result.is_ok(), "Failed to compile process signal: {:?}", result.err());
        
        let _signal_result = result.unwrap();
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_process_signal"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_process_terminate_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Create a mock PID expression
        let pid_literal = Literal {
            value: LiteralValue::Integer(1234),
        };
        let pid_expr = Expression::Literal(pid_literal);
        
        // Test graceful termination
        let result = generator.compile_process_terminate(&pid_expr, false);
        assert!(result.is_ok(), "Failed to compile process terminate: {:?}", result.err());
        
        let _terminate_result = result.unwrap();
        
        // Test forced termination
        let result_force = generator.compile_process_terminate(&pid_expr, true);
        assert!(result_force.is_ok(), "Failed to compile process terminate (force): {:?}", result_force.err());
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_process_terminate"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_process_pipeline_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Test compiling process pipeline
        let commands = vec![
            ("cat", &vec!["file.txt".to_string()][..]),
            ("grep", &vec!["pattern".to_string()][..]),
            ("wc", &vec!["-l".to_string()][..]),
        ];
        
        let result = generator.compile_process_pipeline(&commands);
        assert!(result.is_ok(), "Failed to compile process pipeline: {:?}", result.err());
        
        let _pipeline_handle = result.unwrap();
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_process_pipeline"));
        assert!(module_ir.to_string().contains("cat"));
        assert!(module_ir.to_string().contains("grep"));
        assert!(module_ir.to_string().contains("wc"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_io_redirection_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Test compiling I/O redirection
        let stdin_file = Some("/dev/null");
        let stdout_file = Some("/tmp/output.txt");
        let stderr_file = Some("/tmp/error.txt");
        
        let result = generator.compile_io_redirection(stdin_file, stdout_file, stderr_file);
        assert!(result.is_ok(), "Failed to compile I/O redirection: {:?}", result.err());
        
        let _io_config = result.unwrap();
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_io_redirection"));
        assert!(module_ir.to_string().contains("/dev/null"));
        assert!(module_ir.to_string().contains("/tmp/output.txt"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_background_task_compilation() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Initialize runtime functions
        assert!(initialize_process_execution_runtime(&mut generator).is_ok());
        
        // Create a mock command expression
        let command_literal = Literal {
            value: LiteralValue::String("sleep 10".to_string()),
        };
        let command_expr = Expression::Literal(command_literal);
        
        let result = generator.compile_background_task(&command_expr);
        assert!(result.is_ok(), "Failed to compile background task: {:?}", result.err());
        
        let _task_handle = result.unwrap();
        
        // Verify the LLVM module contains the expected IR
        let module_ir = generator.module().print_to_string();
        assert!(module_ir.to_string().contains("cursed_background_task"));
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_runtime_manager_lifecycle() {
        // Test setting and getting runtime process manager
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Test initial state
        assert!(get_runtime_process_manager().is_none());
        
        // Set process manager
        set_runtime_process_manager(manager_ptr);
        
        // Verify it's set correctly
        let retrieved = get_runtime_process_manager();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), manager_ptr);
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
        assert!(get_runtime_process_manager().is_none());
    }

    #[traced_test]
    #[test]
    fn test_helper_functions() {
        let mut generator = create_test_llvm_generator();
        let manager = setup_process_manager();
        let manager_ptr = &manager as *const ProcessManager as *mut ProcessManager;
        
        // Set up runtime process manager
        set_runtime_process_manager(manager_ptr);
        
        // Test compile_exec_slay_command helper
        let command = "echo";
        let args = vec!["test".to_string()];
        let result = compile_exec_slay_command(&mut generator, command, &args, None);
        assert!(result.is_ok(), "Helper function compile_exec_slay_command failed: {:?}", result.err());
        
        // Test compile_exec_vibez_command helper
        let result = compile_exec_vibez_command(&mut generator, command, &args, None);
        assert!(result.is_ok(), "Helper function compile_exec_vibez_command failed: {:?}", result.err());
        
        // Clean up
        set_runtime_process_manager(std::ptr::null_mut());
    }

    #[traced_test]
    #[test]
    fn test_error_handling() {
        let mut generator = create_test_llvm_generator();
        
        // Test compilation without runtime manager (should fail)
        let command = "echo";
        let args = vec!["test".to_string()];
        
        let result = generator.compile_exec_slay(command, &args, None);
        assert!(result.is_err(), "Should fail without runtime manager");
        
        if let Err(error) = result {
            match error {
                Error::Runtime(msg) => {
                    assert!(msg.contains("No runtime process manager available"));
                }
                _ => panic!("Expected Runtime error, got: {:?}", error),
            }
        }
    }

    #[traced_test]
    #[test]
    fn test_string_array_creation() {
        let generator = create_test_llvm_generator();
        
        // Test empty string array
        let empty_strings: Vec<String> = vec![];
        let result = generator.create_string_array(&empty_strings);
        assert!(result.is_ok(), "Failed to create empty string array: {:?}", result.err());
        
        // Test string array with multiple elements
        let strings = vec!["first".to_string(), "second".to_string(), "third".to_string()];
        let result = generator.create_string_array(&strings);
        assert!(result.is_ok(), "Failed to create string array: {:?}", result.err());
    }

    #[traced_test]
    #[test]
    fn test_global_string_creation() {
        let generator = create_test_llvm_generator();
        
        // Test creating global string constants
        let test_string = "Hello, CURSED!";
        let string_ptr = generator.create_global_string(test_string);
        
        // Verify that the string pointer is valid (non-null)
        assert!(!string_ptr.is_null());
        
        // Test empty string
        let empty_string = "";
        let empty_ptr = generator.create_global_string(empty_string);
        assert!(!empty_ptr.is_null());
        
        // Test string with special characters
        let special_string = "Test\nwith\tspecial\rcharacters";
        let special_ptr = generator.create_global_string(special_string);
        assert!(!special_ptr.is_null());
    }
}
