// Variable management for LLVM codegen
use std::collections::HashMap;
use crate::error::CursedError;

/// Variable manager for LLVM codegen
#[derive(Debug)]
pub struct VariableManager<'ctx> {
impl<'ctx> VariableManager<'ctx> {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn declare_variable(&mut self, name: String, value: inkwell::values::BasicValueEnum<'ctx>) {
        self.variables.insert(name, value);
    pub fn get_variable(&self, name: &str) -> Option<&inkwell::values::BasicValueEnum<'ctx>> {
        self.variables.get(name)
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

impl<'ctx> Default for VariableManager<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}

/// Variable handling trait
pub trait VariableHandling<'ctx> {
    fn handle_variable_declaration(&mut self, name: &str, value: inkwell::values::BasicValueEnum<'ctx>) -> Result<(), VariableError>;
    fn handle_variable_access(&self, name: &str) -> Result<inkwell::values::BasicValueEnum<'ctx>, VariableError>;
    fn handle_variable_assignment(&mut self, name: &str, value: inkwell::values::BasicValueEnum<'ctx>) -> Result<(), VariableError>;
impl<'ctx> VariableHandling<'ctx> for VariableManager<'ctx> {
    fn handle_variable_declaration(&mut self, name: &str, value: inkwell::values::BasicValueEnum<'ctx>) -> Result<(), VariableError> {
        self.declare_variable(name.to_string(), value);
        Ok(())
    fn handle_variable_access(&self, name: &str) -> Result<inkwell::values::BasicValueEnum<'ctx>, VariableError> {
        self.get_variable(name)
            .copied()
            .ok_or_else(|| VariableError::new(format!("Variable '{}' not found", name)))
    fn handle_variable_assignment(&mut self, name: &str, value: inkwell::values::BasicValueEnum<'ctx>) -> Result<(), VariableError> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(VariableError::new(format!("Variable '{}' not declared", name)))
        }
    }
/// Variable error type
#[derive(Debug)]
pub struct VariableError {
impl VariableError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
// impl std::fmt::Display for VariableError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Variable error: {}", self.message)
//     }
// }

// impl std::error::CursedError for VariableError {}
// 