//! AST visitor for linting analysis

use crate::ast::*;
use crate::error::Error;
use std::collections::{HashMap, HashSet};

/// Context for analysis during AST traversal
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    pub source_lines: Vec<String>,
    pub file_name: Option<String>,
    pub declared_variables: HashMap<String, VariableInfo>,
    pub used_variables: HashSet<String>,
    pub declared_functions: HashMap<String, FunctionInfo>,
    pub called_functions: HashSet<String>,
    pub current_scope_depth: usize,
    pub in_loop: bool,
    pub in_function: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub scope_depth: usize,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub parameter_count: usize,
    pub line: usize,
}

impl AnalysisContext {
    pub fn new(source_lines: Vec<String>, file_name: Option<String>) -> Self {
        Self {
            source_lines,
            file_name,
            declared_variables: HashMap::new(),
            used_variables: HashSet::new(),
            declared_functions: HashMap::new(),
            called_functions: HashSet::new(),
            current_scope_depth: 0,
            in_loop: false,
            in_function: None,
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_scope_depth += 1;
    }

    pub fn exit_scope(&mut self) {
        if self.current_scope_depth > 0 {
            self.current_scope_depth -= 1;
        }
    }

    pub fn declare_variable(&mut self, name: String, line: usize) {
        let info = VariableInfo {
            name: name.clone(),
            scope_depth: self.current_scope_depth,
            line,
        };
        self.declared_variables.insert(name, info);
    }

    pub fn use_variable(&mut self, name: String) {
        self.used_variables.insert(name);
    }

    pub fn declare_function(&mut self, name: String, parameter_count: usize, line: usize) {
        let info = FunctionInfo {
            name: name.clone(),
            parameter_count,
            line,
        };
        self.declared_functions.insert(name, info);
    }

    pub fn call_function(&mut self, name: String) {
        self.called_functions.insert(name);
    }
}

/// Visitor for traversing AST during linting
pub struct LintVisitor {
    context: AnalysisContext,
}

impl LintVisitor {
    pub fn new(context: &AnalysisContext) -> Self {
        Self {
            context: context.clone(),
        }
    }

    pub fn context(&self) -> &AnalysisContext {
        &self.context
    }

    pub fn visit_program(&mut self, program: &Program) -> Result<(), Error> {
        for statement in &program.statements {
            self.visit_statement(statement.as_ref())?;
        }
        Ok(())
    }

    pub fn visit_statement(&mut self, statement: &dyn Statement) -> Result<(), Error> {
        // This would need to be implemented based on your actual AST structure
        // For now, this is a placeholder that demonstrates the visitor pattern
        Ok(())
    }

    pub fn visit_expression(&mut self, expression: &dyn Expression) -> Result<(), Error> {
        // This would traverse expression nodes
        Ok(())
    }
}
