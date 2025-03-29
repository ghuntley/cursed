#![recursion_limit = "512"]

/// The CURSED programming language implementation
/// 
/// This crate provides the main API for the CURSED language,
/// including lexer, parser, compiler, and virtual machine.

pub mod ast;
pub mod code;
// pub mod compiler;  // Using stub version for now
pub mod error;
pub mod lexer;
pub mod parser;
pub mod symbol;
pub mod vm;
pub mod prelude;
// pub mod prelude_ext;  // Merged into prelude.rs
// pub mod memory;  // Using stub version for now
pub mod evaluator;
pub mod object;
pub mod repl;
pub mod helpers;

// Basic stub implementations for compiler and memory
pub mod compiler {
    pub mod symbol_table {
        pub use crate::symbol::{Symbol, SymbolScope, SymbolTable};
    }
    
    // Re-export from bytecode module
    pub type Instructions = Vec<u8>;
    
    /// Bytecode operation codes
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Opcode {
        Invalid = 0,
        Nop = 1,
        Constant = 2,
        Add = 3,
        Sub = 4,
        Mul = 5,
        Div = 6,
        True = 7,
        False = 8,
        Equal = 9,
        NotEqual = 10,
        GreaterThan = 11,
        Minus = 12,
        Bang = 13,
        JumpNotTruthy = 14,
        Jump = 15,
        Null = 16,
        SetGlobal = 17,
        GetGlobal = 18,
        Array = 19,
        Hash = 20,
        Index = 21,
        Call = 22,
        ReturnValue = 23,
        Return = 24,
        SetLocal = 25,
        GetLocal = 26,
        GetBuiltin = 27,
        Closure = 28,
        GetFree = 29,
        Pop = 30,
        GreaterThanEqual = 31,
        LessThan = 32,
        LessThanEqual = 33,
        Modulo = 34,
        Dup = 35,
        DefineType = 36,
        DefineField = 37,
        DefineInterface = 38,
        DefineMethod = 39,
    }
    
    // Basic bytecode structure
    #[derive(Debug, Clone)]
    pub struct Bytecode {
        pub instructions: Instructions,
        pub constants: Vec<crate::object::Object>,
    }
    
    // Minimal compiler implementation
    pub struct Compiler {
        instructions: Instructions,
        constants: Vec<crate::object::Object>,
    }
    
    impl Compiler {
        pub fn new() -> Self {
            Self {
                instructions: Vec::new(),
                constants: Vec::new(),
            }
        }
        
        // Compile a program
        pub fn compile(&mut self, program: &crate::ast::Program) -> Result<Bytecode, crate::error::Error> {
            self.compile_program(program)
        }
        
        // Compile a program
        pub fn compile_program(&mut self, program: &crate::ast::Program) -> Result<Bytecode, crate::error::Error> {
            for stmt in &program.statements {
                self.compile_statement(&**stmt)?;
            }
            Ok(self.bytecode())
        }
        
        // Compile a statement
        fn compile_statement(&mut self, stmt: &dyn crate::ast::Statement) -> Result<(), crate::error::Error> {
            // For type declarations
            if let Some(squad_stmt) = stmt.as_any().downcast_ref::<crate::ast::SquadStatement>() {
                return self.compile_type_declaration(squad_stmt);
            }
            
            // For interface declarations
            if let Some(collab_stmt) = stmt.as_any().downcast_ref::<crate::ast::CollabStatement>() {
                return self.compile_interface_declaration(collab_stmt);
            }
            
            // Handle other statement types
            
            // Add a Pop instruction to clean up the stack
            self.emit(Opcode::Pop, &[]);
            Ok(())
        }
        
        // Compile a type declaration
        fn compile_type_declaration(&mut self, squad_stmt: &crate::ast::SquadStatement) -> Result<(), crate::error::Error> {
            // Add the type name to constants and get its index
            let type_name_index = self.add_constant(crate::object::Object::String(squad_stmt.name.value.clone()));
            
            // Emit instruction to load the type name onto the stack
            self.emit(Opcode::Constant, &[type_name_index]);
            
            // Emit the DefineType instruction with the number of fields
            self.emit(Opcode::DefineType, &[squad_stmt.fields.len()]);
            
            // For each field, add its name and type to constants
            for field in &squad_stmt.fields {
                // Add field name to constants
                let field_name_index = self.add_constant(crate::object::Object::String(field.name.value.clone()));
                
                // Add field type to constants
                let field_type_index = self.add_constant(crate::object::Object::String(field.type_name.value.clone()));
                
                // Load field name onto stack
                self.emit(Opcode::Constant, &[field_name_index]);
                
                // Load field type onto stack
                self.emit(Opcode::Constant, &[field_type_index]);
                
                // Define the field
                self.emit(Opcode::DefineField, &[]);
            }
            
            Ok(())
        }
        
        // Compile an interface declaration
        fn compile_interface_declaration(&mut self, collab_stmt: &crate::ast::CollabStatement) -> Result<(), crate::error::Error> {
            // Add the interface name to constants and get its index
            let interface_name_index = self.add_constant(crate::object::Object::String(collab_stmt.name.value.clone()));
            println!("DEBUG: Added interface name: {} at index {}", collab_stmt.name.value, interface_name_index);
            
            // Emit instruction to load the interface name onto the stack
            self.emit(Opcode::Constant, &[interface_name_index]);
            
            // Emit the DefineInterface instruction with the number of methods
            self.emit(Opcode::DefineInterface, &[collab_stmt.methods.len()]);
            println!("DEBUG: DefineInterface with {} methods", collab_stmt.methods.len());
            
            // For each method, add its name, parameters, and return type to constants
            for method in &collab_stmt.methods {
                // Add method name to constants
                let method_name_index = self.add_constant(crate::object::Object::String(method.name.value.clone()));
                println!("DEBUG: Added method name: {} at index {}", method.name.value, method_name_index);
                
                // Load method name onto stack
                self.emit(Opcode::Constant, &[method_name_index]);
                
                // Add parameter count to constants
                let param_count_index = self.add_constant(crate::object::Object::Integer(method.parameters.len() as i64));
                println!("DEBUG: Added param count: {} at index {}", method.parameters.len(), param_count_index);
                
                // Load parameter count onto stack
                self.emit(Opcode::Constant, &[param_count_index]);
                
                // For each parameter, add its name and type to constants
                for param in &method.parameters {
                    // Add parameter name to constants
                    let param_name_index = self.add_constant(crate::object::Object::String(param.name.value.clone()));
                    println!("DEBUG: Added param name: {} at index {}", param.name.value, param_name_index);
                    
                    // Add parameter type to constants
                    let param_type_index = self.add_constant(crate::object::Object::String(param.type_name.value.clone()));
                    println!("DEBUG: Added param type: {} at index {}", param.type_name.value, param_type_index);
                    
                    // Load parameter name onto stack
                    self.emit(Opcode::Constant, &[param_name_index]);
                    
                    // Load parameter type onto stack
                    self.emit(Opcode::Constant, &[param_type_index]);
                }
                
                // Add return type to constants (if it exists)
                let return_type_index = if let Some(return_type) = &method.return_type {
                    let idx = self.add_constant(crate::object::Object::String(return_type.value.clone()));
                    println!("DEBUG: Added return type: {} at index {}", return_type.value, idx);
                    idx
                } else {
                    let idx = self.add_constant(crate::object::Object::Null);
                    println!("DEBUG: Added null return type at index {}", idx);
                    idx
                };
                
                // Load return type onto stack
                self.emit(Opcode::Constant, &[return_type_index]);
                
                // Define the method
                self.emit(Opcode::DefineMethod, &[]);
                println!("DEBUG: Emitted DefineMethod");
            }
            
            Ok(())
        }
        
        // Helper to add a constant and get its index
        fn add_constant(&mut self, obj: crate::object::Object) -> usize {
            self.constants.push(obj);
            self.constants.len() - 1
        }
        
        // Helper to emit an instruction
        fn emit(&mut self, op: Opcode, operands: &[usize]) -> usize {
            let pos = self.instructions.len();
            
            // Convert opcode to u8
            let op_byte: u8 = op as u8;
            self.instructions.push(op_byte);
            
            // Add operands based on opcode
            match op {
                Opcode::Constant | Opcode::GetGlobal | Opcode::SetGlobal |
                Opcode::GetLocal | Opcode::SetLocal | Opcode::GetBuiltin |
                Opcode::Jump | Opcode::JumpNotTruthy => {
                    if !operands.is_empty() {
                        let operand = operands[0];
                        // Encode as u16 (big-endian)
                        self.instructions.push(((operand >> 8) & 0xFF) as u8);
                        self.instructions.push((operand & 0xFF) as u8);
                    }
                },
                Opcode::Array | Opcode::Hash | Opcode::DefineType | Opcode::DefineInterface => {
                    if !operands.is_empty() {
                        let operand = operands[0];
                        // Encode as u16 (big-endian)
                        self.instructions.push(((operand >> 8) & 0xFF) as u8);
                        self.instructions.push((operand & 0xFF) as u8);
                    }
                },
                _ => {
                    // No operands for other opcodes
                }
            }
            
            pos
        }
        
        pub fn bytecode(&self) -> Bytecode {
            Bytecode {
                instructions: self.instructions.clone(),
                constants: self.constants.clone(),
            }
        }
    }
    
    // Minimal compiled function
    #[derive(Debug, Clone, PartialEq)]
    pub struct CompiledFunction {
        pub instructions: Vec<u8>,
        pub num_locals: u8,
        pub num_parameters: u8,
        pub name: Option<String>,
    }
    
    // Include tests here
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ast::{Expression, ExpressionStatement, IntegerLiteral, Program, Statement};
        use crate::lexer::Lexer;
        use crate::object::Object;
        use crate::parser::Parser;
        
        #[test]
        fn test_basic_compiler() {
            // Simple test for the stub compiler
            let program = Program { statements: vec![] };
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(&program);
            assert!(result.is_ok());
        }
        
        #[test]
        fn test_compile_type_declaration() {
            // Test basic type declaration
            let input = "be_like Person squad { name tea; age normie; }";
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer).unwrap();
            let program_result = parser.parse_program();
            assert!(program_result.is_ok(), "Failed to parse program: {:?}", program_result.err());
            
            let program = program_result.unwrap();
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(&program);
            assert!(result.is_ok(), "Compilation of type declaration failed: {:?}", result.err());
            
            // Verify the bytecode contains the type name and field information
            let bytecode = result.unwrap();
            
            // Print bytecode information for debugging
            println!("Compiled bytecode:");
            println!("Constants: {:?}", bytecode.constants);
            println!("Instructions length: {}", bytecode.instructions.len());
            
            // Check that we have constants for type name and field names/types
            assert!(bytecode.constants.len() >= 5, "Not enough constants generated");
            
            // Check for type name and field names/types in the constants
            let mut found_person = false;
            let mut found_name = false;
            let mut found_tea = false;
            let mut found_age = false;
            let mut found_normie = false;
            
            for constant in &bytecode.constants {
                if let Object::String(value) = constant {
                    match value.as_str() {
                        "Person" => found_person = true,
                        "name" => found_name = true,
                        "tea" => found_tea = true,
                        "age" => found_age = true,
                        "normie" => found_normie = true,
                        _ => {}
                    }
                }
            }
            
            assert!(found_person, "Type name 'Person' not found in constants");
            assert!(found_name, "Field name 'name' not found in constants");
            assert!(found_tea, "Field type 'tea' not found in constants");
            assert!(found_age, "Field name 'age' not found in constants");
            assert!(found_normie, "Field type 'normie' not found in constants");
        }
        
        #[test]
        fn test_compile_and_run_type_declaration() {
            // Test compiling and running type declarations through the VM
            let input = "be_like Person squad { name tea; age normie; }";
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer).unwrap();
            let program_result = parser.parse_program();
            assert!(program_result.is_ok(), "Failed to parse program: {:?}", program_result.err());
            
            let program = program_result.unwrap();
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(&program);
            assert!(result.is_ok(), "Compilation of type declaration failed: {:?}", result.err());
            
            let bytecode = result.unwrap();
            
            // Print bytecode information for debugging
            println!("Compiled bytecode:");
            println!("Constants: {:?}", bytecode.constants);
            println!("Instructions length: {}", bytecode.instructions.len());
            
            // Create a VM and run the bytecode
            let mut vm = crate::vm::VM::new();
            
            // Run the bytecode
            let result = vm.run_with_bytecode(bytecode);
            match &result {
                Ok(obj) => println!("Execution succeeded: {:?}", obj),
                Err(e) => println!("Execution failed: {:?}", e)
            }
            assert!(result.is_ok(), "VM execution failed: {:?}", result.err());
            
            // Assert that we got a Struct object back for type declarations
            let result_obj = result.unwrap();
            match result_obj.as_ref() {
                Object::Struct { name, fields } => {
                    assert_eq!(name, "Person", "Expected type name 'Person'");
                    assert_eq!(fields.len(), 2, "Expected 2 fields");
                    
                    // Verify field names and types
                    assert_eq!(fields[0].0, "name", "Expected first field name to be 'name'");
                    assert_eq!(fields[0].1, "tea", "Expected first field type to be 'tea'");
                    
                    assert_eq!(fields[1].0, "age", "Expected second field name to be 'age'");
                    assert_eq!(fields[1].1, "normie", "Expected second field type to be 'normie'");
                },
                _ => panic!("Expected struct, got {:?}", result_obj),
            }
        }
        
        #[test]
        fn test_compile_interface_declaration() {
            // Test basic interface declaration
            let input = "be_like Greeter collab { greet(name string) string; farewell(name string); }";
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer).unwrap();
            let program_result = parser.parse_program();
            assert!(program_result.is_ok(), "Failed to parse program: {:?}", program_result.err());
            
            let program = program_result.unwrap();
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(&program);
            assert!(result.is_ok(), "Compilation of interface declaration failed: {:?}", result.err());
            
            // Verify the bytecode contains the interface name and method information
            let bytecode = result.unwrap();
            
            // Print bytecode information for debugging
            println!("Compiled bytecode:");
            println!("Constants: {:?}", bytecode.constants);
            println!("Instructions length: {}", bytecode.instructions.len());
            
            // Check that we have constants for interface name and method details
            assert!(bytecode.constants.len() >= 6, "Not enough constants generated");
            
            // Check for interface name and method names/parameter types in the constants
            let mut found_greeter = false;
            let mut found_greet = false;
            let mut found_farewell = false;
            let mut found_name = false;
            let mut found_string = false;
            
            for constant in &bytecode.constants {
                if let Object::String(value) = constant {
                    match value.as_str() {
                        "Greeter" => found_greeter = true,
                        "greet" => found_greet = true,
                        "farewell" => found_farewell = true,
                        "name" => found_name = true,
                        "string" => found_string = true,
                        _ => {}
                    }
                }
            }
            
            assert!(found_greeter, "Interface name 'Greeter' not found in constants");
            assert!(found_greet, "Method name 'greet' not found in constants");
            assert!(found_farewell, "Method name 'farewell' not found in constants");
            assert!(found_name, "Parameter name 'name' not found in constants");
            assert!(found_string, "Parameter type 'string' not found in constants");
            
            // Test interface declaration with return type
            let input = "be_like Writer collab { write(data string) number; close() bool; }";
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer).unwrap();
            let program_result = parser.parse_program();
            assert!(program_result.is_ok(), "Failed to parse program: {:?}", program_result.err());
            
            let program = program_result.unwrap();
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(&program);
            assert!(result.is_ok(), "Compilation of interface with return types failed: {:?}", result.err());
        }
        
        #[test]
        fn test_compile_and_run_interface_declaration() {
            // Test compiling and running interface declarations through the VM
            let input = "be_like Greeter collab { greet(name string) string; farewell(name string); }";
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer).unwrap();
            let program_result = parser.parse_program();
            assert!(program_result.is_ok(), "Failed to parse program: {:?}", program_result.err());
            
            let program = program_result.unwrap();
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(&program);
            assert!(result.is_ok(), "Compilation of interface declaration failed: {:?}", result.err());
            
            let bytecode = result.unwrap();
            
            // Print bytecode information for debugging
            println!("Compiled interface bytecode:");
            println!("Constants: {:?}", bytecode.constants);
            println!("Instructions length: {}", bytecode.instructions.len());
            
            // Create a VM and run the bytecode
            let mut vm = crate::vm::VM::new();
            
            // Run the bytecode
            let result = vm.run_with_bytecode(bytecode);
            match &result {
                Ok(obj) => println!("Execution succeeded: {:?}", obj),
                Err(e) => println!("Execution failed: {:?}", e)
            }
            assert!(result.is_ok(), "VM execution failed: {:?}", result.err());
            
            // The result should be an interface definition
            let result_obj = result.unwrap();
            
            // Verify the result is an interface with the correct methods
            match &*result_obj {
                Object::Interface { name, methods } => {
                    assert_eq!(name, "Greeter", "Expected interface name 'Greeter'");
                    assert_eq!(methods.len(), 2, "Expected 2 methods");
                    
                    // Verify first method (greet)
                    let greet_method = &methods[0];
                    assert_eq!(greet_method.0, "greet", "Expected first method name to be 'greet'");
                    assert_eq!(greet_method.1.len(), 1, "Expected 'greet' to have 1 parameter");
                    assert_eq!(greet_method.1[0].0, "name", "Expected parameter name to be 'name'");
                    assert_eq!(greet_method.1[0].1, "string", "Expected parameter type to be 'string'");
                    assert_eq!(greet_method.2.as_ref().unwrap(), "string", "Expected return type to be 'string'");
                    
                    // Verify second method (farewell)
                    let farewell_method = &methods[1];
                    assert_eq!(farewell_method.0, "farewell", "Expected second method name to be 'farewell'");
                    assert_eq!(farewell_method.1.len(), 1, "Expected 'farewell' to have 1 parameter");
                    assert_eq!(farewell_method.1[0].0, "name", "Expected parameter name to be 'name'");
                    assert_eq!(farewell_method.1[0].1, "string", "Expected parameter type to be 'string'");
                    assert!(farewell_method.2.is_none(), "Expected no return type for 'farewell'");
                },
                _ => panic!("Expected interface, got {:?}", result_obj),
            }
            
            // Test with multiple parameters and different return types
            let input = "be_like Calculator collab { add(a number, b number) number; multiply(a number, b number) number; }";
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer).unwrap();
            let program_result = parser.parse_program();
            assert!(program_result.is_ok(), "Failed to parse program: {:?}", program_result.err());
            
            let program = program_result.unwrap();
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(&program);
            assert!(result.is_ok(), "Compilation of multi-parameter interface failed: {:?}", result.err());
            
            let bytecode = result.unwrap();
            
            // Create a VM and run the bytecode
            let mut vm = crate::vm::VM::new();
            
            // Run the bytecode
            let result = vm.run_with_bytecode(bytecode);
            assert!(result.is_ok(), "VM execution of multi-parameter interface failed: {:?}", result.err());
            
            // Verify the result is an interface with the correct methods
            let result_obj = result.unwrap();
            match &*result_obj {
                Object::Interface { name, methods } => {
                    assert_eq!(name, "Calculator", "Expected interface name 'Calculator'");
                    assert_eq!(methods.len(), 2, "Expected 2 methods");
                    
                    // Check both methods have 2 parameters each
                    for method in methods {
                        assert_eq!(method.1.len(), 2, "Expected method to have 2 parameters");
                    }
                },
                _ => panic!("Expected interface, got {:?}", result_obj),
            }
        }
    }
}

pub mod memory {
    pub mod gc {
        use std::collections::HashSet;
        
        // Basic traceable trait
        pub trait Traceable {
            fn trace(&self, visitor: &mut dyn Visitor);
            fn size(&self) -> usize;
        }
        
        // Simple trace trait as alias
        pub trait Trace: Traceable {}
        
        // Simple visitor trait
        pub trait Visitor {
            fn visit(&mut self, obj: &dyn Traceable);
            fn visit_ptr(&mut self, ptr: usize, tag: crate::memory::tagged::Tag);
        }
        
        // Simple GC
        pub struct GarbageCollector {
            marked: HashSet<usize>,
        }
        
        impl GarbageCollector {
            pub fn new() -> Self {
                Self {
                    marked: HashSet::new(),
                }
            }
        }
        
        // GC reference
        pub struct Gc<T: Traceable + 'static> {
            inner: T,
        }
        
        impl<T: Traceable + 'static> Gc<T> {
            pub fn new(value: T) -> Self {
                Self {
                    inner: value,
                }
            }
        }
    }
    
    pub mod tagged {
        use std::ptr::NonNull;
        
        // Simple tag enum
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Tag {
            Int,
            Float,
            String,
            Boolean,
            Array,
            Map,
            Function,
            Null,
            Object,
        }
        
        // Tagged pointer
        pub struct TaggedPtr<T> {
            ptr: NonNull<T>,
            tag: Tag,
        }
        
        // Type-erased tagged pointer
        pub struct TaggedDynPtr {
            ptr: NonNull<u8>,
            tag: Tag,
        }
        
        // NonNull extension
        pub trait NonNullExt<T> {
            fn with_tag(self, tag: Tag) -> TaggedPtr<T>;
        }
        
        impl<T> NonNullExt<T> for NonNull<T> {
            fn with_tag(self, tag: Tag) -> TaggedPtr<T> {
                TaggedPtr { ptr: self, tag }
            }
        }
    }
    
    // Allocator modules
    pub mod allocator {}
    pub mod block {}
    pub mod bump {}
    
    // Re-exports
    pub use gc::{Traceable, Visitor, GarbageCollector, Gc};
    pub use tagged::{TaggedPtr, Tag, NonNullExt};
    
    // Minimal memory manager
    pub struct MemoryManager {}
    
    impl MemoryManager {
        pub fn new() -> Self {
            Self {}
        }
    }
    
    // Allocated object
    pub struct Allocated<T> {
        pub inner: T,
    }
}

/// Version of the CURSED language
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Authors of the CURSED language
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
/// Description of the CURSED language
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// Convenience re-exports at the crate level
pub use prelude::*;
pub use error::{Error, ErrorReporter, SourceLocation};
pub use ast::{Node, Statement, Expression, Program};
pub use object::Object;
pub use lexer::{Lexer, Token};

// Re-export VM and related items
pub use vm::VM;
pub use vm::Frame;
pub use vm::Closure;
pub use vm::constants::*;

// Re-export parsers
pub use parser::Parser;

// Re-export compiler
pub use compiler::Compiler;
pub use compiler::Bytecode;
pub use compiler::Instructions;
pub use compiler::Opcode;
pub use compiler::CompiledFunction;
pub use compiler::symbol_table::SymbolTable;
pub use compiler::symbol_table::Symbol;
pub use compiler::symbol_table::SymbolScope;

// Re-export repl
pub use repl::start_repl;

// Memory management
pub use memory::MemoryManager;
pub use memory::Allocated;

// Memory internals
pub use memory::tagged::{TaggedPtr, Tag, NonNullExt};
pub use memory::gc::{Traceable, Visitor, GarbageCollector, Gc};

/// Main entry point for the REPL
pub fn run_repl() -> Result<(), Error> {
    repl::start_repl()
}


