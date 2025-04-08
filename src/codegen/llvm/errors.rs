//! Error types for LLVM code generation

/// Errors that can occur during LLVM code generation
#[derive(Debug, Clone)]
pub enum LlvmCodegenError {
    /// A variable was referenced before it was defined
    UndefinedVariable(String),
    
    /// A function was called before it was defined
    UndefinedFunction(String),
    
    /// A type was referenced that does not exist
    UndefinedType(String),
    
    /// An imported package was not found
    ImportError(String),
    
    /// A return statement was encountered outside a function
    ReturnOutsideFunction,
    
    /// A break statement was encountered outside a loop
    BreakOutsideLoop,
    
    /// An error occurred during LLVM IR validation
    InvalidLLVMIR(String),
    
    /// A generic error with a message
    GenericError(String),
}

impl LlvmCodegenError {
    /// Convert the error to a user-friendly message
    pub fn to_string(&self) -> String {
        match self {
            Self::UndefinedVariable(name) => format!("Undefined variable: {}", name),
            Self::UndefinedFunction(name) => format!("Undefined function: {}", name),
            Self::UndefinedType(name) => format!("Undefined type: {}", name),
            Self::ImportError(msg) => format!("Import error: {}", msg),
            Self::ReturnOutsideFunction => "Return statement outside function".to_string(),
            Self::BreakOutsideLoop => "Break statement outside loop".to_string(),
            Self::InvalidLLVMIR(msg) => format!("Invalid LLVM IR: {}", msg),
            Self::GenericError(msg) => msg.clone(),
        }
    }
}