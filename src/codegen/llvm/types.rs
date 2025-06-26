//! LLVM type system for CURSED

use crate::error_types::Result;

/// LLVM Context for managing compilation state
#[derive(Debug)]
pub struct LlvmContext {
    pub target_triple: String,
    pub modules: Vec<LlvmModule>,
}

impl LlvmContext {
    pub fn new() -> Self {
        Self {
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            modules: Vec::new(),
        }
    }

    pub fn create_module(&mut self, name: String) -> &mut LlvmModule {
        let module = LlvmModule::new(name);
        self.modules.push(module);
        self.modules.last_mut().unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum LlvmType {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Pointer(Box<LlvmType>),
    Array(Box<LlvmType>, usize),
    Struct(Vec<LlvmType>),
    Function(Vec<LlvmType>, Box<LlvmType>),
    Void,
}

#[derive(Debug, Clone)]
pub struct LlvmValue {
    pub value_type: LlvmType,
    pub name: String,
}

impl LlvmValue {
    pub fn new(value_type: LlvmType, name: String) -> Self {
        Self { value_type, name }
    }
}

#[derive(Debug, Clone)]
pub struct LlvmFunction {
    pub name: String,
    pub return_type: LlvmType,
    pub parameters: Vec<LlvmType>,
    pub body: Vec<String>, // Simplified body representation
}

impl LlvmFunction {
    pub fn new(name: String, return_type: LlvmType, parameters: Vec<LlvmType>) -> Self {
        Self {
            name,
            return_type,
            parameters,
            body: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LlvmModule {
    pub name: String,
    pub functions: Vec<LlvmFunction>,
    pub globals: Vec<LlvmValue>,
}

impl LlvmModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            globals: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: LlvmFunction) {
        self.functions.push(function);
    }

    pub fn add_global(&mut self, global: LlvmValue) {
        self.globals.push(global);
    }
}
