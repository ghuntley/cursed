//! Core runtime components for CURSED

pub mod thread_safe_goroutine;
pub mod type_checker;
pub mod symbol_table;
pub mod channel;

/// Compiled function representation
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    /// Name of the function
    pub name: String,
    /// Bytecode instructions
    pub bytecode: Vec<u8>,
    /// IR representation of the function
    pub ir_representation: String,
    /// Number of local variables
    pub num_locals: usize,
    /// Number of parameters
    pub num_parameters: usize,
    /// Names of free variables captured from outer scopes
    pub free_variables: Vec<String>,
    /// Whether this function accepts variadic arguments
    pub is_variadic: bool,
}