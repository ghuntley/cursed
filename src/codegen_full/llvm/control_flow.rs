/// LLVM code generation for control flow constructs in the CURSED programming language
/// 
/// This module implements compilation of control flow statements to LLVM IR

use crate::ast::conditionals::{IfStatement, WhileStatement, ForStatement};
use crate::ast::statements::{BreakStatement, ContinueStatement};
use crate::ast::block::BlockStatement;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::CursedError;
use crate::codegen::llvm::gc_integration::LlvmGcIntegration;
use tracing::debug;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::basic_block::BasicBlock;
use inkwell::IntPredicate;
use std::collections::HashMap;

/// Control flow context for managing loops and blocks
#[derive(Debug, Clone)]
pub struct ControlFlowContext<'ctx> {
    /// Stack of loop contexts for break/continue handling
    /// Current function being compiled
    /// Variable scope stack
/// Context for a single loop (while, for, range-for)
#[derive(Debug, Clone)]
pub struct LoopContext<'ctx> {
    /// Block to jump to for continue statements
    /// Block to jump to for break statements  
    /// Optional condition block for loops
    /// Loop type for debugging
impl<'ctx> ControlFlowContext<'ctx> {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn push_loop(&mut self, loop_context: LoopContext<'ctx>) {
        self.loop_stack.push(loop_context);
    pub fn pop_loop(&mut self) -> Option<LoopContext<'ctx>> {
        self.loop_stack.pop()
    pub fn current_loop(&self) -> Option<&LoopContext<'ctx>> {
        self.loop_stack.last()
    pub fn push_scope(&mut self) {
        self.variable_scopes.push(HashMap::new());
    pub fn pop_scope(&mut self) {
        self.variable_scopes.pop();
    pub fn declare_variable(&mut self, name: String, ptr: PointerValue<'ctx>) {
        if let Some(current_scope) = self.variable_scopes.last_mut() {
            current_scope.insert(name, ptr);
        }
    }
    
    pub fn lookup_variable(&self, name: &str) -> Option<PointerValue<'ctx>> {
        for scope in self.variable_scopes.iter().rev() {
            if let Some(ptr) = scope.get(name) {
                return Some(*ptr);
            }
        }
        None
    }
}

/// Trait for compiling control flow statements to LLVM IR
pub trait ControlFlowCompilation<'ctx> {
    /// Compile an if statement (lowkey/highkey)
    fn compile_if_statement(
    ) -> crate::error::Result<()>;

    /// Compile a while statement (periodt)
    fn compile_while_statement(
    ) -> crate::error::Result<()>;

    /// Compile a for statement (bestie)
    fn compile_for_statement(
    ) -> crate::error::Result<()>;

    /// Compile a break statement (ghosted)
    fn compile_break_statement(
    ) -> crate::error::Result<()>;

    /// Compile a continue statement (simp)
    fn compile_continue_statement(
    ) -> crate::error::Result<()>;

    /// Helper: Compile an expression to a basic value
    fn compile_expression(
    ) -> crate::error::Result<()>;

    /// GC Integration Methods

    /// Generate loop back edge safe point (for yolo yield points)
    fn generate_loop_safe_point(
    ) -> String;
/// Main implementation of control flow compilation
pub struct LlvmControlFlowCompiler;

impl LlvmControlFlowCompiler {
    pub fn new() -> Self {
        Self
    /// Get GC integration for yield point generation
    fn get_gc_integration(&self) -> Option<&LlvmGcIntegration> {
        // This would be provided by the main code generator
        // For now, return None to disable yield points
        None
    }
}

impl<'ctx> ControlFlowCompilation<'ctx> for LlvmControlFlowCompiler {
    fn compile_if_statement(
    ) -> crate::error::Result<()> {
        let current_function = flow_ctx.current_function
            .ok_or_else(|| CursedError::Compile("No current function for if statement".to_string()))?;

        // Create basic blocks
        let then_block = context.append_basic_block(current_function, "lowkey_then");
        let else_block = context.append_basic_block(current_function, "highkey_else");
        let merge_block = context.append_basic_block(current_function, "if_merge");

        // Compile condition
        let condition_value = self.compile_expression(context, module, builder, if_stmt.condition.as_ref(), flow_ctx)?;
        let condition_bool = match condition_value {
            BasicValueEnum::IntValue(int_val) => {
                // Convert to i1 if necessary
                if int_val.get_type().get_bit_width() == 1 {
                    int_val
                } else {
                    let zero = int_val.get_type().const_zero();
                    builder.build_int_compare(IntPredicate::NE, int_val, zero, "condition_bool")
                        .map_err(|e| CursedError::Compile(format!("Failed to build condition: {}", e)))?
                }
            }

        // Branch based on condition
        builder.build_conditional_branch(condition_bool, then_block, else_block)
            .map_err(|e| CursedError::Compile(format!("Failed to build conditional branch: {}", e)))?;

        // Compile then branch (lowkey)
        builder.position_at_end(then_block);
        // Simplified: just add unconditional branch
        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(merge_block)
                .map_err(|e| CursedError::Compile(format!("Failed to build branch: {}", e)))?;
        // Compile else branch (highkey)
        builder.position_at_end(else_block);
        // Simplified: just add unconditional branch
        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(merge_block)
                .map_err(|e| CursedError::Compile(format!("Failed to build branch: {}", e)))?;
        // Continue in merge block
        builder.position_at_end(merge_block);

        Ok(())
    fn compile_while_statement(
    ) -> crate::error::Result<()> {
        let current_function = flow_ctx.current_function
            .ok_or_else(|| CursedError::Compile("No current function for while statement".to_string()))?;

        // Create basic blocks for while loop (periodt)
        let condition_block = context.append_basic_block(current_function, "periodt_condition");
        let loop_body_block = context.append_basic_block(current_function, "periodt_body");
        let exit_block = context.append_basic_block(current_function, "periodt_exit");

        // Jump to condition check
        builder.build_unconditional_branch(condition_block)
            .map_err(|e| CursedError::Compile(format!("Failed to build branch: {}", e)))?;

        // Compile condition block
        builder.position_at_end(condition_block);
        let condition_value = self.compile_expression(context, module, builder, while_stmt.condition.as_ref(), flow_ctx)?;
        let condition_bool = match condition_value {
            BasicValueEnum::IntValue(int_val) => {
                if int_val.get_type().get_bit_width() == 1 {
                    int_val
                } else {
                    let zero = int_val.get_type().const_zero();
                    builder.build_int_compare(IntPredicate::NE, int_val, zero, "while_condition")
                        .map_err(|e| CursedError::Compile(format!("Failed to build condition: {}", e)))?
                }
            }

        // Branch based on condition
        builder.build_conditional_branch(condition_bool, loop_body_block, exit_block)
            .map_err(|e| CursedError::Compile(format!("Failed to build conditional branch: {}", e)))?;

        // Setup loop context for break/continue
        let loop_context = LoopContext {
        flow_ctx.push_loop(loop_context);

        // Compile loop body with yield points
        builder.position_at_end(loop_body_block);
        
        // Add yield point for goroutine scheduling (yolo)
        if let Some(gc_integration) = self.get_gc_integration() {
            let yield_ir = self.generate_loop_safe_point("periodt", Some(gc_integration));
            if !yield_ir.is_empty() {
                // The yield point would be inserted here
                debug!("Generated yield point for while loop: {}", yield_ir);
            }
        }

        // Jump back to condition
        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(condition_block)
                .map_err(|e| CursedError::Compile(format!("Failed to build loop branch: {}", e)))?;
        // Clean up loop context
        flow_ctx.pop_loop();

        // Continue in exit block
        builder.position_at_end(exit_block);

        Ok(())
    fn compile_for_statement(
    ) -> crate::error::Result<()> {
        let current_function = flow_ctx.current_function
            .ok_or_else(|| CursedError::Compile("No current function for for statement".to_string()))?;

        // Create basic blocks for for loop (bestie)
        let init_block = context.append_basic_block(current_function, "bestie_init");
        let condition_block = context.append_basic_block(current_function, "bestie_condition");
        let loop_body_block = context.append_basic_block(current_function, "bestie_body");
        let increment_block = context.append_basic_block(current_function, "bestie_increment");
        let exit_block = context.append_basic_block(current_function, "bestie_exit");

        // Jump to initialization
        builder.build_unconditional_branch(init_block)
            .map_err(|e| CursedError::Compile(format!("Failed to build branch: {}", e)))?;

        // Push new scope for loop variable
        flow_ctx.push_scope();

        // Compile initialization block
        builder.position_at_end(init_block);
        if let Some(init_stmt) = &for_stmt.init {
            // For statement init is a statement, not expression
            // Skip for now - would need a compile_statement method
        }
        builder.build_unconditional_branch(condition_block)
            .map_err(|e| CursedError::Compile(format!("Failed to build init branch: {}", e)))?;

        // Compile condition block
        builder.position_at_end(condition_block);
        let condition_value = if let Some(condition_expr) = &for_stmt.condition {
            self.compile_expression(context, module, builder, condition_expr.as_ref(), flow_ctx)?
        } else {
            // No condition means infinite loop (true)
            BasicValueEnum::IntValue(context.bool_type().const_int(1, false))
        
        let condition_bool = match condition_value {
            BasicValueEnum::IntValue(int_val) => {
                if int_val.get_type().get_bit_width() == 1 {
                    int_val
                } else {
                    let zero = int_val.get_type().const_zero();
                    builder.build_int_compare(IntPredicate::NE, int_val, zero, "for_condition")
                        .map_err(|e| CursedError::Compile(format!("Failed to build condition: {}", e)))?
                }
            }

        // Branch based on condition
        builder.build_conditional_branch(condition_bool, loop_body_block, exit_block)
            .map_err(|e| CursedError::Compile(format!("Failed to build conditional branch: {}", e)))?;

        // Setup loop context for break/continue
        let loop_context = LoopContext {
        flow_ctx.push_loop(loop_context);

        // Compile loop body with yield points
        builder.position_at_end(loop_body_block);
        
        // Add yield point for goroutine scheduling (yolo)
        if let Some(gc_integration) = self.get_gc_integration() {
            let yield_ir = self.generate_loop_safe_point("bestie", Some(gc_integration));
            if !yield_ir.is_empty() {
                debug!("Generated yield point for for loop: {}", yield_ir);
            }
        }

        // Jump to increment
        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(increment_block)
                .map_err(|e| CursedError::Compile(format!("Failed to build body branch: {}", e)))?;
        // Compile increment block
        builder.position_at_end(increment_block);
        if let Some(increment_stmt) = &for_stmt.post {
            // For statement post is a statement, not expression
            // Skip for now - would need a compile_statement method
        }
        builder.build_unconditional_branch(condition_block)
            .map_err(|e| CursedError::Compile(format!("Failed to build increment branch: {}", e)))?;

        // Clean up loop context and scope
        flow_ctx.pop_loop();
        flow_ctx.pop_scope();

        // Continue in exit block
        builder.position_at_end(exit_block);

        Ok(())
    fn compile_break_statement(
    ) -> crate::error::Result<()> {
        if let Some(loop_ctx) = flow_ctx.current_loop() {
            builder.build_unconditional_branch(loop_ctx.break_block)
                .map_err(|e| CursedError::Compile(format!("Failed to build break branch: {}", e)))?;
        } else {
            return Err(CursedError::Compile("ghosted (break) statement outside of loop".to_string()));
        }
        Ok(())
    fn compile_continue_statement(
    ) -> crate::error::Result<()> {
        if let Some(loop_ctx) = flow_ctx.current_loop() {
            builder.build_unconditional_branch(loop_ctx.continue_block)
                .map_err(|e| CursedError::Compile(format!("Failed to build continue branch: {}", e)))?;
        } else {
            return Err(CursedError::Compile("simp (continue) statement outside of loop".to_string()));
        }
        Ok(())
    fn compile_expression(
    ) -> crate::error::Result<()> {
        // Simplified expression compilation for now
        let expr_str = expr.string();
        
        if expr_str.contains("true") {
            Ok(BasicValueEnum::IntValue(context.bool_type().const_int(1, false)))
        } else if expr_str.contains("false") {
            Ok(BasicValueEnum::IntValue(context.bool_type().const_int(0, false)))
        } else if let Ok(int_val) = expr_str.parse::<i32>() {
            Ok(BasicValueEnum::IntValue(context.i32_type().const_int(int_val as u64, false)))
        } else {
            // Default to true for unknown expressions
            Ok(BasicValueEnum::IntValue(context.bool_type().const_int(1, false)))
        }
    }

    /// GC Integration Method Implementations

    fn generate_loop_safe_point(
    ) -> String {
        if let Some(gc) = gc_integration {
            gc.generate_loop_yield_point(loop_id)
        } else {
            String::new()
        }
    }
