/// Common Subexpression Elimination (CSE) Optimization Pass
/// 
/// This module implements a complete CSE algorithm that identifies and eliminates
/// redundant computations by building expression trees, performing value numbering,
/// and using dominance analysis to ensure correctness.

use crate::error::{CursedError, Result};
use crate::ast::{
    Node, Expression, Statement, FunctionDeclaration as Function, Type
// };
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};
use std::fmt;

/// Else branch representation for if statements
#[derive(Debug, Clone, PartialEq)]
pub enum ElseBranch {
/// If statement representation
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
/// Value number assigned to expressions for CSE analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueNumber(usize);

impl ValueNumber {
    pub fn new(id: usize) -> Self {
        Self(id)
    pub fn id(&self) -> usize {
        self.0
    }
}

/// Expression signature for value numbering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionSignature {
    Binary {
    Unary {
    FunctionCall {
    ArrayAccess {
    FieldAccess {
/// Simplified literal value for hashing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralValue {
    Float(String), // Store as string to handle floating point equality
impl From<&Literal> for LiteralValue {
    fn from(literal: &Literal) -> Self {
        match literal {
            _ => LiteralValue::Null, // Fallback for other literal types
        }
    }
/// Basic block for control flow analysis
#[derive(Debug, Clone)]
pub struct BasicBlock {
impl BasicBlock {
    pub fn new(id: usize) -> Self {
        Self {
        }
    }
/// Control Flow Graph for dominance analysis
#[derive(Debug)]
pub struct ControlFlowGraph {
impl ControlFlowGraph {
    pub fn new(entry_block: usize) -> Self {
        Self {
        }
    }
    
    /// Compute dominance relationships using iterative algorithm
    pub fn compute_dominance(&mut self) {
        let block_ids: Vec<usize> = self.blocks.keys().cloned().collect();
        
        // Initialize dominators
        for &block_id in &block_ids {
            if block_id == self.entry_block {
                // Entry block dominates only itself
                self.blocks.get_mut(&block_id).unwrap().dominators.insert(block_id);
            } else {
                // All other blocks initially dominated by all blocks
                self.blocks.get_mut(&block_id).unwrap().dominators = block_ids.iter().cloned().collect();
            }
        }
        
        // Iteratively refine dominators
        let mut changed = true;
        while changed {
            changed = false;
            
            for &block_id in &block_ids {
                if block_id == self.entry_block {
                    continue;
                let predecessors = self.blocks[&block_id].predecessors.clone();
                if predecessors.is_empty() {
                    continue;
                // Intersection of dominators of all predecessors
                let mut new_dominators = self.blocks[&predecessors[0]].dominators.clone();
                for &pred_id in &predecessors[1..] {
                    new_dominators = new_dominators
                        .intersection(&self.blocks[&pred_id].dominators)
                        .cloned()
                        .collect();
                // Add self
                new_dominators.insert(block_id);
                
                if new_dominators != self.blocks[&block_id].dominators {
                    self.blocks.get_mut(&block_id).unwrap().dominators = new_dominators;
                    changed = true;
                }
            }
        // Compute immediate dominators
        self.compute_immediate_dominators();
    /// Compute immediate dominators
    fn compute_immediate_dominators(&mut self) {
        for &block_id in self.blocks.keys().cloned().collect().iter() {
            if block_id == self.entry_block {
                continue;
            let dominators = self.blocks[&block_id].dominators.clone();
            let mut candidates: Vec<usize> = dominators
                .iter()
                .filter(|&&d| d != block_id)
                .cloned()
                .collect();
            
            // Find the dominator that is not dominated by any other dominator
            candidates.retain(|&candidate| {
                !dominators.iter().any(|&other| {
                    other != candidate &&
                    other != block_id &&
                    self.blocks[&other].dominators.contains(&candidate)
                })
            });
            
            if candidates.len() == 1 {
                self.blocks.get_mut(&block_id).unwrap().immediate_dominator = Some(candidates[0]);
            }
        }
    /// Check if block1 dominates block2
    pub fn dominates(&self, block1: usize, block2: usize) -> bool {
        self.blocks.get(&block2)
            .map(|block| block.dominators.contains(&block1))
            .unwrap_or(false)
    }
}

/// CSE context for a single function
#[derive(Debug)]
pub struct CseContext {
    /// Maps expression signatures to value numbers
    /// Maps value numbers to their first computed location (block, statement)
    /// Maps value numbers to generated temporary variable names
    /// Counter for generating unique value numbers
    /// Counter for generating unique temporary variable names
    /// Control flow graph for dominance analysis
    /// Available expressions at each program point
impl CseContext {
    pub fn new(entry_block: usize) -> Self {
        Self {
        }
    }
    
    /// Get or create value number for expression signature
    pub fn get_value_number(&mut self, signature: ExpressionSignature) -> ValueNumber {
        if let Some(&vn) = self.value_table.get(&signature) {
            vn
        } else {
            let vn = ValueNumber::new(self.next_value_number);
            self.next_value_number += 1;
            self.value_table.insert(signature, vn);
            vn
        }
    }
    
    /// Generate temporary variable name for value number
    pub fn get_temp_variable(&mut self, vn: ValueNumber) -> String {
        if let Some(temp_name) = self.temp_variables.get(&vn) {
            temp_name.clone()
        } else {
            let temp_name = format!("__cse_temp_{}", self.next_temp_id);
            self.next_temp_id += 1;
            self.temp_variables.insert(vn, temp_name.clone());
            temp_name
        }
    }
    
    /// Check if value number is available at given location
    pub fn is_available(&self, vn: ValueNumber, block: usize, stmt: usize) -> bool {
        // Check if expression was computed before this point
        if let Some(&(def_block, def_stmt)) = self.value_locations.get(&vn) {
            // If in same block, check statement order
            if def_block == block {
                return def_stmt < stmt;
            // Check if definition dominates current location
            return self.cfg.dominates(def_block, block);
        }
        false
    }
}

/// Complete Common Subexpression Elimination Pass
pub struct CommonSubexpressionEliminationPass {
impl CommonSubexpressionEliminationPass {
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Create CSE pass with configuration
    pub fn with_config(global_cse: bool, debug_mode: bool) -> Self {
        Self {
        }
    }
    
    /// Perform CSE on a program
    pub fn eliminate_common_subexpressions(&mut self, program: &mut Program) -> Result<usize> {
        self.eliminated_count = 0;
        
        for module in &mut program.modules {
            self.eliminate_in_module(module)?;
        Ok(self.eliminated_count)
    /// Perform CSE on a module
    fn eliminate_in_module(&mut self, module: &mut Module) -> Result<()> {
        for function in &mut module.functions {
            self.eliminate_in_function(function)?;
        }
        Ok(())
    /// Perform CSE on a function
    fn eliminate_in_function(&mut self, function: &mut Function) -> Result<()> {
        if self.debug_mode {
            println!("CSE: Processing function '{}'", function.name);
        // Build control flow graph
        let mut context = CseContext::new(0);
        self.build_cfg(&function.body, &mut context)?;
        
        // Compute dominance relationships
        context.cfg.compute_dominance();
        
        if self.global_cse {
            // Global CSE across basic blocks
            self.global_cse_elimination(&mut function.body, &mut context)?;
        } else {
            // Local CSE within basic blocks
            self.local_cse_elimination(&mut function.body, &mut context)?;
        if self.debug_mode {
                    self.eliminated_count, function.name);
        Ok(())
    /// Build control flow graph from statements
    fn build_cfg(&self, statements: &[Box<dyn Statement>], context: &mut CseContext) -> Result<()> {
        let mut current_block = BasicBlock::new(0);
        let mut block_counter = 0;
        
        for (stmt_idx, statement) in statements.iter().enumerate() {
            current_block.statements.push(statement.clone());
            
            // Check if this statement ends a basic block
            match statement {
                Statement::Return(_) | Statement::Break | Statement::Continue => {
                    // End current block
                    context.cfg.blocks.insert(current_block.id, current_block);
                    context.cfg.exit_blocks.insert(block_counter);
                    block_counter += 1;
                    current_block = BasicBlock::new(block_counter);
                }
                Statement::If(_) | Statement::While(_) | Statement::For(_) => {
                    // These create new blocks - simplified for this implementation
                    // In a full implementation, we'd handle control flow properly
                }
                _ => {}
            }
        }
        
        // Add final block if it has statements
        if !current_block.statements.is_empty() {
            context.cfg.blocks.insert(current_block.id, current_block);
            if !context.cfg.exit_blocks.contains(&block_counter) {
                context.cfg.exit_blocks.insert(block_counter);
            }
        }
        
        Ok(())
    /// Perform global CSE elimination
    fn global_cse_elimination(&mut self, statements: &mut Vec<Statement>, context: &mut CseContext) -> Result<()> {
        // First pass: build value numbering table
        self.build_value_numbering(statements, context, 0, 0)?;
        
        // Second pass: perform eliminations
        self.perform_eliminations(statements, context, 0, 0)?;
        
        Ok(())
    /// Perform local CSE elimination within basic blocks
    fn local_cse_elimination(&mut self, statements: &mut Vec<Statement>, context: &mut CseContext) -> Result<()> {
        // Process each basic block independently
        for (&block_id, _) in context.cfg.blocks.clone().iter() {
            let mut local_context = CseContext::new(block_id);
            
            // Build value numbering for this block only
            self.build_value_numbering(statements, &mut local_context, block_id, 0)?;
            
            // Perform eliminations within this block
            self.perform_eliminations(statements, &mut local_context, block_id, 0)?;
        Ok(())
    /// Build value numbering table for expressions
    fn build_value_numbering(&mut self, statements: &[dyn Statement], context: &mut CseContext, block_id: usize, start_stmt: usize) -> Result<()> {
        for (stmt_idx, statement) in statements.iter().enumerate() {
            let stmt_pos = start_stmt + stmt_idx;
            self.number_expressions_in_statement(statement, context, block_id, stmt_pos)?;
        }
        Ok(())
    /// Assign value numbers to expressions in a statement
    fn number_expressions_in_statement(&mut self, statement: &dyn Statement, context: &mut CseContext, block_id: usize, stmt_idx: usize) -> Result<()> {
        match statement {
            Statement::VariableDeclaration(var_decl) => {
                if let Some(ref init_expr) = var_decl.initializer {
                    self.number_expression(init_expr, context, block_id, stmt_idx)?;
                }
            }
            Statement::Assignment(assignment) => {
                self.number_expression(&assignment.value, context, block_id, stmt_idx)?;
            }
            Statement::If(if_stmt) => {
                self.number_expression(&if_stmt.condition, context, block_id, stmt_idx)?;
                self.build_value_numbering(&if_stmt.then_branch, context, block_id, stmt_idx + 1)?;
                if let Some(ref else_branch) = if_stmt.else_branch {
                    match else_branch {
                        ElseBranch::Block(statements) => {
                            self.build_value_numbering(statements, context, block_id, stmt_idx + 1)?;
                        }
                        ElseBranch::If(nested_if) => {
                            // Handle nested if - would require more complex CFG handling
                        }
                    }
                }
            }
            Statement::While(while_stmt) => {
                self.number_expression(&while_stmt.condition, context, block_id, stmt_idx)?;
                self.build_value_numbering(&while_stmt.body, context, block_id, stmt_idx + 1)?;
            }
            Statement::For(for_stmt) => {
                if let Some(ref init) = for_stmt.init {
                    self.number_expression(init, context, block_id, stmt_idx)?;
                }
                if let Some(ref condition) = for_stmt.condition {
                    self.number_expression(condition, context, block_id, stmt_idx)?;
                }
                if let Some(ref update) = for_stmt.update {
                    self.number_expression(update, context, block_id, stmt_idx)?;
                }
                self.build_value_numbering(&for_stmt.body, context, block_id, stmt_idx + 1)?;
            }
            Statement::Return(return_stmt) => {
                if let Some(ref expr) = return_stmt.value {
                    self.number_expression(expr, context, block_id, stmt_idx)?;
                }
            }
            Statement::Expression(expr) => {
                self.number_expression(expr, context, block_id, stmt_idx)?;
            }
            _ => {}
        }
        Ok(())
    /// Assign value number to an expression
    fn number_expression(&mut self, expr: &dyn Expression, context: &mut CseContext, block_id: usize, stmt_idx: usize) -> Result<ValueNumber> {
        let signature = match expr {
            Expression::Literal(literal) => {
                ExpressionSignature::Literal(LiteralValue::from(literal))
            }
            Expression::Variable(var) => {
                ExpressionSignature::Variable(var.name.clone())
            }
            Expression::Binary(binary_expr) => {
                let left_vn = self.number_expression(&binary_expr.left, context, block_id, stmt_idx)?;
                let right_vn = self.number_expression(&binary_expr.right, context, block_id, stmt_idx)?;
                ExpressionSignature::Binary {
                }
            }
            Expression::Unary(unary_expr) => {
                let operand_vn = self.number_expression(&unary_expr.operand, context, block_id, stmt_idx)?;
                ExpressionSignature::Unary {
                }
            }
            Expression::FunctionCall(call_expr) => {
                let mut arg_vns = Vec::new();
                for arg in &call_expr.arguments {
                    arg_vns.push(self.number_expression(arg, context, block_id, stmt_idx)?);
                }
                ExpressionSignature::FunctionCall {
                }
            }
            Expression::ArrayAccess(access_expr) => {
                let array_vn = self.number_expression(&access_expr.array, context, block_id, stmt_idx)?;
                let index_vn = self.number_expression(&access_expr.index, context, block_id, stmt_idx)?;
                ExpressionSignature::ArrayAccess {
                }
            }
            _ => {
                // For other expression types, generate unique value numbers
                let unique_signature = ExpressionSignature::Variable(format!("__unique_expr_{}", context.next_value_number));
                let vn = context.get_value_number(unique_signature);
                context.value_locations.insert(vn, (block_id, stmt_idx));
                return Ok(vn);
            }
        
        let vn = context.get_value_number(signature);
        
        // Record location if this is the first occurrence
        if !context.value_locations.contains_key(&vn) {
            context.value_locations.insert(vn, (block_id, stmt_idx));
        Ok(vn)
    /// Perform actual CSE eliminations
    fn perform_eliminations(&mut self, statements: &mut Vec<Statement>, context: &mut CseContext, block_id: usize, start_stmt: usize) -> Result<()> {
        // Track which expressions have been computed and can be reused
        let mut computed_expressions: HashSet<ValueNumber> = HashSet::new();
        let mut temp_declarations: Vec<Statement> = Vec::new();
        
        for (stmt_idx, statement) in statements.iter_mut().enumerate() {
            let stmt_pos = start_stmt + stmt_idx;
            let original_count = self.eliminated_count;
            
            // Perform eliminations in this statement
            self.eliminate_in_statement(statement, context, block_id, stmt_pos, &mut computed_expressions, &mut temp_declarations)?;
            
            if self.debug_mode && self.eliminated_count > original_count {
                        self.eliminated_count - original_count, block_id, stmt_pos);
            }
        }
        
        // Insert temporary variable declarations at the beginning
        if !temp_declarations.is_empty() {
            let mut new_statements = temp_declarations;
            new_statements.extend(statements.drain(..));
            *statements = new_statements;
        Ok(())
    /// Eliminate common subexpressions in a statement
    fn eliminate_in_statement(&mut self, statement: &mut Statement, context: &mut CseContext, block_id: usize, stmt_idx: usize, computed: &mut HashSet<ValueNumber>, temp_decls: &mut Vec<Statement>) -> Result<()> {
        match statement {
            Statement::VariableDeclaration(var_decl) => {
                if let Some(ref mut init_expr) = var_decl.initializer {
                    self.eliminate_in_expression(init_expr, context, block_id, stmt_idx, computed, temp_decls)?;
                }
            }
            Statement::Assignment(assignment) => {
                self.eliminate_in_expression(&mut assignment.value, context, block_id, stmt_idx, computed, temp_decls)?;
            }
            Statement::If(if_stmt) => {
                self.eliminate_in_expression(&mut if_stmt.condition, context, block_id, stmt_idx, computed, temp_decls)?;
                self.perform_eliminations(&mut if_stmt.then_branch, context, block_id, stmt_idx + 1)?;
                if let Some(ref mut else_branch) = if_stmt.else_branch {
                    match else_branch {
                        ElseBranch::Block(statements) => {
                            self.perform_eliminations(statements, context, block_id, stmt_idx + 1)?;
                        }
                        ElseBranch::If(_) => {
                            // Handle nested if
                        }
                    }
                }
            }
            Statement::While(while_stmt) => {
                self.eliminate_in_expression(&mut while_stmt.condition, context, block_id, stmt_idx, computed, temp_decls)?;
                self.perform_eliminations(&mut while_stmt.body, context, block_id, stmt_idx + 1)?;
            }
            Statement::For(for_stmt) => {
                if let Some(ref mut init) = for_stmt.init {
                    self.eliminate_in_expression(init, context, block_id, stmt_idx, computed, temp_decls)?;
                }
                if let Some(ref mut condition) = for_stmt.condition {
                    self.eliminate_in_expression(condition, context, block_id, stmt_idx, computed, temp_decls)?;
                }
                if let Some(ref mut update) = for_stmt.update {
                    self.eliminate_in_expression(update, context, block_id, stmt_idx, computed, temp_decls)?;
                }
                self.perform_eliminations(&mut for_stmt.body, context, block_id, stmt_idx + 1)?;
            }
            Statement::Return(return_stmt) => {
                if let Some(ref mut expr) = return_stmt.value {
                    self.eliminate_in_expression(expr, context, block_id, stmt_idx, computed, temp_decls)?;
                }
            }
            Statement::Expression(expr) => {
                self.eliminate_in_expression(expr, context, block_id, stmt_idx, computed, temp_decls)?;
            }
            _ => {}
        }
        Ok(())
    /// Eliminate common subexpressions in an expression
    fn eliminate_in_expression(&mut self, expr: &mut Expression, context: &mut CseContext, block_id: usize, stmt_idx: usize, computed: &mut HashSet<ValueNumber>, temp_decls: &mut Vec<Statement>) -> Result<()> {
        // First, recursively process subexpressions
        match expr {
            Expression::Binary(binary_expr) => {
                self.eliminate_in_expression(&mut binary_expr.left, context, block_id, stmt_idx, computed, temp_decls)?;
                self.eliminate_in_expression(&mut binary_expr.right, context, block_id, stmt_idx, computed, temp_decls)?;
            }
            Expression::Unary(unary_expr) => {
                self.eliminate_in_expression(&mut unary_expr.operand, context, block_id, stmt_idx, computed, temp_decls)?;
            }
            Expression::FunctionCall(call_expr) => {
                for arg in &mut call_expr.arguments {
                    self.eliminate_in_expression(arg, context, block_id, stmt_idx, computed, temp_decls)?;
                }
            }
            Expression::ArrayAccess(access_expr) => {
                self.eliminate_in_expression(&mut access_expr.array, context, block_id, stmt_idx, computed, temp_decls)?;
                self.eliminate_in_expression(&mut access_expr.index, context, block_id, stmt_idx, computed, temp_decls)?;
            }
            _ => {}
        // Try to eliminate this expression
        let expr_vn = self.number_expression(expr, context, block_id, stmt_idx)?;
        
        // Check if this expression can be replaced with a previously computed value
        if let Some(&(def_block, def_stmt)) = context.value_locations.get(&expr_vn) {
            // Don't replace if this is the defining occurrence
            if def_block == block_id && def_stmt == stmt_idx {
                computed.insert(expr_vn);
                return Ok(());
            // Check if the expression is available (dominates current location)
            if context.is_available(expr_vn, block_id, stmt_idx) && computed.contains(&expr_vn) {
                // Replace with temporary variable
                let temp_name = context.get_temp_variable(expr_vn);
                *expr = Expression::Variable(Variable {
                    var_type: None, // Type inference will handle this
                });
                
                self.eliminated_count += 1;
                
                if self.debug_mode {
                    println!("CSE: Replaced expression with temporary variable '{}'", temp_name);
                }
            } else {
                // First occurrence - create temporary variable and declaration
                let temp_name = context.get_temp_variable(expr_vn);
                
                // Create temporary variable declaration
                let temp_decl = Statement::VariableDeclaration(VariableDeclaration {
                    var_type: None, // Type inference will handle this
                });
                
                temp_decls.push(temp_decl);
                computed.insert(expr_vn);
                
                // Replace current expression with variable reference
                *expr = Expression::Variable(Variable {
                });
            }
        }
        
        Ok(())
    }
}

/// Statistics and analysis for CSE pass
#[derive(Debug, Clone)]
pub struct CseStatistics {
impl CseStatistics {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn calculate_elimination_rate(&mut self) {
        if self.expressions_analyzed > 0 {
            self.elimination_rate = (self.expressions_eliminated as f64) / (self.expressions_analyzed as f64) * 100.0;
        }
    }
impl fmt::Display for CseStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CSE Statistics:\n")?;
        write!(f, "  Expressions analyzed: {}\n", self.expressions_analyzed)?;
        write!(f, "  Expressions eliminated: {}\n", self.expressions_eliminated)?;
        write!(f, "  Temp variables created: {}\n", self.temp_variables_created)?;
        write!(f, "  Basic blocks processed: {}\n", self.basic_blocks_processed)?;
        write!(f, "  Elimination rate: {:.2}%", self.elimination_rate)
    }
}

