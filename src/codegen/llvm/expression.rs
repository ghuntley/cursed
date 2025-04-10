//! LLVM expression code generator

use crate::ast::Expression;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use inkwell::values::BasicValueEnum;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile an expression to LLVM IR.
    pub fn compile_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, Error> {
        // Delegate to appropriate expression compiler based on the type of expression
        match expr.node_type() {
            "IntegerLiteral" => self.compile_basic_expression(expr),
            "BooleanLiteral" => self.compile_basic_expression(expr),
            "StringLiteral" => self.compile_basic_expression(expr),
            "FloatLiteral" => self.compile_basic_expression(expr),
            "Identifier" => self.compile_basic_expression(expr),
            "PrefixExpression" => self.compile_basic_expression(expr),
            "InfixExpression" => self.compile_basic_expression(expr),
            "CallExpression" => self.compile_basic_expression(expr),
            _ => Err(Error::codegen(format!(
                "Unsupported expression type: {}", 
                expr.node_type()
            ))),
        }
    }
}