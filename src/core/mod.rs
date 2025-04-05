pub mod symbol_table;
pub mod type_checker;
pub mod generic_instantiation;
pub mod channel;
pub mod goroutine;
pub mod thread_safe_goroutine;

/// Represents a compiled function in CURSED
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    pub name: String,
    pub bytecode: Vec<u8>,
    pub ir_representation: String,
    pub num_locals: usize,
    pub num_parameters: usize,
    pub free_variables: Vec<String>,
    pub is_variadic: bool,
    // Add more fields as needed for the bytecode compiler
}

impl CompiledFunction {
    /// Create a new compiled function
    pub fn new(name: String, bytecode: Vec<u8>) -> Self {
        CompiledFunction {
            name,
            bytecode,
            ir_representation: String::new(),
            num_locals: 0,
            num_parameters: 0,
            free_variables: Vec::new(),
            is_variadic: false,
        }
    }
    
    /// Create a new compiled function with full details
    pub fn with_details(
        name: String, 
        bytecode: Vec<u8>,
        ir_representation: String,
        num_locals: usize,
        num_parameters: usize,
        free_variables: Vec<String>,
        is_variadic: bool
    ) -> Self {
        CompiledFunction {
            name,
            bytecode,
            ir_representation,
            num_locals,
            num_parameters,
            free_variables,
            is_variadic,
        }
    }
}