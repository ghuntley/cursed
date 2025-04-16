//! Switch statement compilation for LLVM code generation

use inkwell::values::{BasicValueEnum, IntValue};
use crate::ast::control_flow::{SwitchStatement, SwitchCase};
use crate::ast::expressions::StringLiteral;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use super::statement::StatementCompilation;

/// Trait for switch statement compilation
pub trait SwitchStatementCompilation<'ctx> {
    /// Compile a switch statement
    fn compile_switch_statement(&mut self, stmt: &SwitchStatement) -> Result<(), Error>;
    
    /// Compile a string-based switch statement
    fn compile_string_switch(&mut self, value: BasicValueEnum<'ctx>, cases: &[SwitchCase], default_case: Option<&SwitchCase>) -> Result<(), Error>;
    
    /// Compile an integer-based switch statement
    fn compile_int_switch(&mut self, value: IntValue<'ctx>, cases: &[SwitchCase], default_case: Option<&SwitchCase>) -> Result<(), Error>;
}

// Implementation will be added in a subsequent edit