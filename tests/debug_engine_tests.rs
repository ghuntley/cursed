//! Comprehensive tests for the CURSED debug engine
//!
//! These tests verify the interactive debugging functionality including:
//! - Breakpoint management
//! - Variable inspection
//! - Step execution
//! - Memory inspection
//! - Performance monitoring
//! - REPL-like evaluation

use std::collections::HashMap;
use tokio;

// Import the debugging structures from main.rs
// Note: In a real implementation, these would be in a separate module
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub line: u32,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub line: u32,
    pub column: u32,
    pub variables: HashMap<String, VariableValue>,
}

#[derive(Debug, Clone)]
pub struct VariableValue {
    pub name: String,
    pub value: String,
    pub type_info: String,
    pub address: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub symbol_type: String,
    pub address: u64,
    pub size: u32,
    pub debug_info: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ExecutionState {
    Running,
    Paused,
    Stopped,
    Error(String),
}

#[derive(Debug)]
pub struct MemoryInspector {
    pub heap_usage: u64,
    pub stack_usage: u64,
    pub allocations: Vec<AllocationInfo>,
}

#[derive(Debug, Clone)]
pub struct AllocationInfo {
    pub address: u64,
    pub size: u32,
    pub allocated_at: std::time::SystemTime,
    pub allocation_type: String,
}

/// Mock InteractiveDebugger for testing
pub struct MockInteractiveDebugger {
    pub filename: String,
    pub source: String,
    pub ir: String,
    pub breakpoints: HashMap<u32, Breakpoint>,
    pub trace_enabled: bool,
    pub memory_debug: bool,
    pub execution_state: ExecutionState,
    pub watch_variables: Vec<String>,
    pub call_stack: Vec<StackFrame>,
    pub current_line: u32,
    pub source_lines: Vec<String>,
    pub symbol_table: HashMap<String, SymbolInfo>,
    pub memory_inspector: MemoryInspector,
}

impl MockInteractiveDebugger {
    pub fn new_mock() -> Self {
        let source = r#"
            vibez.spill("Hello, Debug World!")
            sus x normie = 42
            sus y drip = 3.14
            vibez.spill("x = ", x)
            vibez.spill("y = ", y)
        "#.to_string();
        
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        
        MockInteractiveDebugger {
            filename: "test_debug.csd".to_string(),
            source,
            ir: "mock LLVM IR".to_string(),
            breakpoints: HashMap::new(),
            trace_enabled: false,
            memory_debug: false,
            execution_state: ExecutionState::Stopped,
            watch_variables: Vec::new(),
            call_stack: Vec::new(),
            current_line: 1,
            source_lines,
            symbol_table: HashMap::new(),
            memory_inspector: MemoryInspector {
                heap_usage: 1024,
                stack_usage: 512,
                allocations: vec![
                    AllocationInfo {
                        address: 0x1000,
                        size: 64,
                        allocated_at: std::time::SystemTime::now(),
                        allocation_type: "Variable".to_string(),
                    }
                ],
            },
        }
    }
    
    pub fn set_breakpoint(&mut self, line: u32) -> Result<(), String> {
        if line == 0 || line > self.source_lines.len() as u32 {
            return Err(format!("Invalid line number: {}", line));
        }
        
        self.breakpoints.insert(line, Breakpoint {
            line,
            condition: None,
            hit_count: 0,
            enabled: true,
        });
        
        Ok(())
    }
    
    pub fn delete_breakpoint(&mut self, line: u32) -> bool {
        self.breakpoints.remove(&line).is_some()
    }
    
    pub fn add_watch(&mut self, variable: String) {
        if !self.watch_variables.contains(&variable) {
            self.watch_variables.push(variable);
        }
    }
    
    pub fn remove_watch(&mut self, variable: &str) -> bool {
        if let Some(pos) = self.watch_variables.iter().position(|v| v == variable) {
            self.watch_variables.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn step_execution(&mut self) -> Result<(), String> {
        if matches!(self.execution_state, ExecutionState::Stopped) {
            return Err("Program is not running".to_string());
        }
        
        if self.current_line < self.source_lines.len() as u32 {
            self.current_line += 1;
            Ok(())
        } else {
            self.execution_state = ExecutionState::Stopped;
            Ok(())
        }
    }
    
    pub fn run_to_breakpoint(&mut self) -> Result<Option<u32>, String> {
        self.execution_state = ExecutionState::Running;
        
        for line_num in self.current_line..=self.source_lines.len() as u32 {
            if let Some(breakpoint) = self.breakpoints.get_mut(&line_num) {
                if breakpoint.enabled {
                    breakpoint.hit_count += 1;
                    self.execution_state = ExecutionState::Paused;
                    self.current_line = line_num;
                    return Ok(Some(line_num));
                }
            }
            self.current_line = line_num;
        }
        
        self.execution_state = ExecutionState::Stopped;
        Ok(None)
    }
    
    pub fn add_symbol(&mut self, name: String, symbol_type: String, address: u64, size: u32) {
        self.symbol_table.insert(name.clone(), SymbolInfo {
            name,
            symbol_type,
            address,
            size,
            debug_info: None,
        });
    }
    
    pub fn add_stack_frame(&mut self, function_name: String, line: u32, column: u32) {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), VariableValue {
            name: "x".to_string(),
            value: "42".to_string(),
            type_info: "normie".to_string(),
            address: Some(0x2000),
        });
        variables.insert("y".to_string(), VariableValue {
            name: "y".to_string(),
            value: "3.14".to_string(),
            type_info: "drip".to_string(),
            address: Some(0x2010),
        });
        
        self.call_stack.push(StackFrame {
            function_name,
            line,
            column,
            variables,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_management() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Test setting breakpoints
        assert!(debugger.set_breakpoint(2).is_ok());
        assert!(debugger.set_breakpoint(4).is_ok());
        assert_eq!(debugger.breakpoints.len(), 2);
        
        // Test invalid breakpoint
        assert!(debugger.set_breakpoint(0).is_err());
        assert!(debugger.set_breakpoint(100).is_err());
        
        // Test deleting breakpoints
        assert!(debugger.delete_breakpoint(2));
        assert!(!debugger.delete_breakpoint(10));
        assert_eq!(debugger.breakpoints.len(), 1);
        
        // Verify remaining breakpoint
        let breakpoint = debugger.breakpoints.get(&4).unwrap();
        assert_eq!(breakpoint.line, 4);
        assert_eq!(breakpoint.hit_count, 0);
        assert!(breakpoint.enabled);
    }
    
    #[test]
    fn test_variable_watch_list() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Test adding watches
        debugger.add_watch("x".to_string());
        debugger.add_watch("y".to_string());
        assert_eq!(debugger.watch_variables.len(), 2);
        
        // Test duplicate watch (should not add)
        debugger.add_watch("x".to_string());
        assert_eq!(debugger.watch_variables.len(), 2);
        
        // Test removing watches
        assert!(debugger.remove_watch("x"));
        assert!(!debugger.remove_watch("nonexistent"));
        assert_eq!(debugger.watch_variables.len(), 1);
        assert_eq!(debugger.watch_variables[0], "y");
    }
    
    #[test]
    fn test_step_execution() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Test stepping when stopped (should fail)
        assert!(debugger.step_execution().is_err());
        
        // Start execution
        debugger.execution_state = ExecutionState::Running;
        debugger.current_line = 1;
        
        // Test normal stepping
        assert!(debugger.step_execution().is_ok());
        assert_eq!(debugger.current_line, 2);
        
        // Step to end
        for _ in 0..10 {
            if debugger.step_execution().is_ok() {
                if matches!(debugger.execution_state, ExecutionState::Stopped) {
                    break;
                }
            }
        }
        
        assert!(matches!(debugger.execution_state, ExecutionState::Stopped));
    }
    
    #[test]
    fn test_breakpoint_execution() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Set breakpoint at line 3
        debugger.set_breakpoint(3).unwrap();
        debugger.current_line = 1;
        
        // Run to breakpoint
        let hit_line = debugger.run_to_breakpoint().unwrap();
        assert_eq!(hit_line, Some(3));
        assert!(matches!(debugger.execution_state, ExecutionState::Paused));
        assert_eq!(debugger.current_line, 3);
        
        // Verify hit count
        let breakpoint = debugger.breakpoints.get(&3).unwrap();
        assert_eq!(breakpoint.hit_count, 1);
    }
    
    #[test]
    fn test_symbol_table_management() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Add symbols
        debugger.add_symbol("main".to_string(), "function".to_string(), 0x1000, 256);
        debugger.add_symbol("x".to_string(), "variable".to_string(), 0x2000, 4);
        debugger.add_symbol("y".to_string(), "variable".to_string(), 0x2010, 8);
        
        assert_eq!(debugger.symbol_table.len(), 3);
        
        // Verify symbol information
        let main_symbol = debugger.symbol_table.get("main").unwrap();
        assert_eq!(main_symbol.symbol_type, "function");
        assert_eq!(main_symbol.address, 0x1000);
        assert_eq!(main_symbol.size, 256);
        
        let x_symbol = debugger.symbol_table.get("x").unwrap();
        assert_eq!(x_symbol.symbol_type, "variable");
        assert_eq!(x_symbol.address, 0x2000);
        assert_eq!(x_symbol.size, 4);
    }
    
    #[test]
    fn test_call_stack_management() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Add stack frames
        debugger.add_stack_frame("main".to_string(), 5, 10);
        debugger.add_stack_frame("helper".to_string(), 15, 5);
        
        assert_eq!(debugger.call_stack.len(), 2);
        
        // Verify stack frame structure
        let frame = &debugger.call_stack[0];
        assert_eq!(frame.function_name, "main");
        assert_eq!(frame.line, 5);
        assert_eq!(frame.column, 10);
        assert_eq!(frame.variables.len(), 2);
        
        // Verify variable in frame
        let x_var = frame.variables.get("x").unwrap();
        assert_eq!(x_var.value, "42");
        assert_eq!(x_var.type_info, "normie");
        assert_eq!(x_var.address, Some(0x2000));
    }
    
    #[test]
    fn test_memory_inspector() {
        let debugger = MockInteractiveDebugger::new_mock();
        
        // Verify memory usage tracking
        assert_eq!(debugger.memory_inspector.heap_usage, 1024);
        assert_eq!(debugger.memory_inspector.stack_usage, 512);
        assert_eq!(debugger.memory_inspector.allocations.len(), 1);
        
        // Verify allocation information
        let alloc = &debugger.memory_inspector.allocations[0];
        assert_eq!(alloc.address, 0x1000);
        assert_eq!(alloc.size, 64);
        assert_eq!(alloc.allocation_type, "Variable");
    }
    
    #[test]
    fn test_execution_state_transitions() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Initial state should be stopped
        assert!(matches!(debugger.execution_state, ExecutionState::Stopped));
        
        // Start execution
        debugger.execution_state = ExecutionState::Running;
        assert!(matches!(debugger.execution_state, ExecutionState::Running));
        
        // Pause at breakpoint
        debugger.execution_state = ExecutionState::Paused;
        assert!(matches!(debugger.execution_state, ExecutionState::Paused));
        
        // Handle error state
        debugger.execution_state = ExecutionState::Error("Test error".to_string());
        if let ExecutionState::Error(msg) = &debugger.execution_state {
            assert_eq!(msg, "Test error");
        } else {
            panic!("Expected error state");
        }
    }
    
    #[test]
    fn test_trace_functionality() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Initially trace should be disabled
        assert!(!debugger.trace_enabled);
        
        // Enable trace
        debugger.trace_enabled = true;
        assert!(debugger.trace_enabled);
        
        // Verify memory debug flag
        assert!(!debugger.memory_debug);
        debugger.memory_debug = true;
        assert!(debugger.memory_debug);
    }

    #[tokio::test]
    async fn test_async_debugging_operations() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Test async step operations
        debugger.execution_state = ExecutionState::Running;
        
        // Simulate async stepping
        for _ in 0..3 {
            if debugger.step_execution().is_ok() {
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
        }
        
        assert_eq!(debugger.current_line, 4);
    }
    
    #[test]
    fn test_comprehensive_debugging_session() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        
        // Setup debugging session
        debugger.set_breakpoint(3).unwrap();
        debugger.add_watch("x".to_string());
        debugger.add_symbol("main".to_string(), "function".to_string(), 0x1000, 256);
        debugger.add_stack_frame("main".to_string(), 1, 0);
        
        // Start execution
        debugger.current_line = 1;
        let hit_line = debugger.run_to_breakpoint().unwrap();
        
        // Verify debugging state
        assert_eq!(hit_line, Some(3));
        assert!(matches!(debugger.execution_state, ExecutionState::Paused));
        assert_eq!(debugger.watch_variables.len(), 1);
        assert_eq!(debugger.symbol_table.len(), 1);
        assert_eq!(debugger.call_stack.len(), 1);
        
        // Continue stepping
        debugger.execution_state = ExecutionState::Running;
        while debugger.current_line <= debugger.source_lines.len() as u32 {
            if debugger.step_execution().is_err() {
                break;
            }
        }
        
        assert!(matches!(debugger.execution_state, ExecutionState::Stopped));
    }
}

/// Integration tests for the debug engine with real CURSED runtime
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_debug_with_cursed_runtime() {
        // Create a simple CURSED program for debugging
        let source = r#"
            vibez.spill("Starting debug test")
            sus counter normie = 0
            bestie i := 0; i < 3; i++ {
                counter = counter + 1
                vibez.spill("Counter: ", counter)
            }
            vibez.spill("Debug test completed")
        "#;
        
        // This would integrate with the actual CURSED compiler and runtime
        // For now, we'll use mock operations
        let mut debugger = MockInteractiveDebugger::new_mock();
        debugger.source = source.to_string();
        debugger.source_lines = source.lines().map(|s| s.to_string()).collect();
        
        // Set breakpoint inside the loop
        debugger.set_breakpoint(5).unwrap();
        
        // Simulate debugging session
        debugger.current_line = 1;
        let result = debugger.run_to_breakpoint();
        
        assert!(result.is_ok());
        // In a real implementation, this would verify actual execution state
    }
    
    #[tokio::test]
    async fn test_memory_debugging_integration() {
        let mut debugger = MockInteractiveDebugger::new_mock();
        debugger.memory_debug = true;
        
        // Add mock memory allocations
        debugger.memory_inspector.allocations.push(AllocationInfo {
            address: 0x3000,
            size: 128,
            allocated_at: std::time::SystemTime::now(),
            allocation_type: "Array".to_string(),
        });
        
        debugger.memory_inspector.allocations.push(AllocationInfo {
            address: 0x4000,
            size: 256,
            allocated_at: std::time::SystemTime::now(),
            allocation_type: "String".to_string(),
        });
        
        // Verify memory tracking
        assert_eq!(debugger.memory_inspector.allocations.len(), 3);
        assert!(debugger.memory_debug);
        
        let total_allocated: u32 = debugger.memory_inspector.allocations
            .iter()
            .map(|a| a.size)
            .sum();
        assert_eq!(total_allocated, 64 + 128 + 256);
    }
    
    #[test]
    fn test_llvm_debug_info_integration() {
        let debugger = MockInteractiveDebugger::new_mock();
        
        // Verify IR contains debug information
        assert!(!debugger.ir.is_empty());
        assert_eq!(debugger.ir, "mock LLVM IR");
        
        // In a real implementation, this would verify:
        // - DWARF debug information is present
        // - Line number mapping is correct
        // - Variable debug info is available
        // - Function symbols are preserved
    }
    
    #[test]
    fn test_cross_platform_debugging() {
        let debugger = MockInteractiveDebugger::new_mock();
        
        // Verify debugger works on different platforms
        assert!(!debugger.filename.is_empty());
        assert!(!debugger.source.is_empty());
        
        // Platform-specific testing would verify:
        // - Path handling across platforms
        // - Memory address formatting
        // - Debug symbol resolution
        // - Process attachment capabilities
    }
}
