//! Common types used in the LLVM code generator

use std::collections::HashMap;
use inkwell::values::FunctionValue;

// Structure to hold information about imported functions
#[derive(Debug, Clone)]
pub struct ImportedFunctionInfo<'ctx> {
    pub mangled_name: String, 
    pub llvm_function: Option<FunctionValue<'ctx>>, 
}

// Structure to hold information about an imported package
#[derive(Debug, Clone, Default)]
pub struct ImportedPackageInfo<'ctx> {
    pub name: String, 
    pub exported_functions: HashMap<String, ImportedFunctionInfo<'ctx>>,
}