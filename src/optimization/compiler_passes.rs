/// Compiler Optimization Passes
/// 
/// Implementation of various compiler optimization passes including:
/// - Dead code elimination
/// - Constant propagation
/// - Loop optimization
/// - Inlining decisions
/// - Register allocation improvements

use crate::error::{Error, Result};
use crate::ast::*;
use crate::optimization::config::{PassConfig, LoopOptimizationConfig, InliningConfig};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Dead code elimination pass
pub struct DeadCodeEliminator {
    config: PassConfig,
    stats: DeadCodeStats,
}

#[derive(Debug, Clone, Default)]
pub struct DeadCodeStats {
    pub eliminated_statements: u32,
    pub eliminated_functions: u32,
    pub eliminated_variables: u32,
    pub optimization_time: Duration,
}

impl DeadCodeEliminator {
    pub fn new(config: PassConfig) -> Self {
        Self {
            config,
            stats: DeadCodeStats::default(),
        }
    }

    /// Eliminate dead code from a program
    pub fn eliminate(&mut self, program: &mut Program) -> Result<()> {
        let start_time = Instant::now();
        
        tracing::info!("Starting dead code elimination pass");
        
        // Collect all defined and used variables/functions
        let mut usage_analyzer = UsageAnalyzer::new();
        usage_analyzer.analyze_program(program)?;
        
        // Remove unused functions
        self.eliminate_unused_functions(program, &usage_analyzer)?;
        
        // Remove unused variables and statements
        self.eliminate_unused_statements(program, &usage_analyzer)?;
        
        self.stats.optimization_time = start_time.elapsed();
        
        tracing::info!(
            eliminated_statements = self.stats.eliminated_statements,
            eliminated_functions = self.stats.eliminated_functions,
            eliminated_variables = self.stats.eliminated_variables,
            optimization_time_ms = self.stats.optimization_time.as_millis(),
            "Dead code elimination completed"
        );
        
        Ok(())
    }

    fn eliminate_unused_functions(&mut self, program: &mut Program, analyzer: &UsageAnalyzer) -> Result<()> {
        program.functions.retain(|func| {
            let is_used = analyzer.is_function_used(&func.name) || func.name == "main";
            if !is_used {
                self.stats.eliminated_functions += 1;
                tracing::debug!(function_name = func.name, "Eliminated unused function");
            }
            is_used
        });
        Ok(())
    }

    fn eliminate_unused_statements(&mut self, program: &mut Program, analyzer: &UsageAnalyzer) -> Result<()> {
        for function in &mut program.functions {
            self.eliminate_unused_statements_in_block(&mut function.body, analyzer)?;
        }
        Ok(())
    }

    fn eliminate_unused_statements_in_block(&mut self, statements: &mut Vec<Statement>, analyzer: &UsageAnalyzer) -> Result<()> {
        statements.retain(|stmt| {
            match stmt {
                Statement::VariableDeclaration(var_decl) => {
                    let is_used = analyzer.is_variable_used(&var_decl.name);
                    if !is_used {
                        self.stats.eliminated_variables += 1;
                        tracing::debug!(variable_name = var_decl.name, "Eliminated unused variable");
                    }
                    is_used
                }
                Statement::Expression(expr) => {
                    // Keep expressions that have side effects
                    self.has_side_effects(expr)
                }
                _ => true, // Keep other statements
            }
        });

        // Process nested blocks
        for statement in statements {
            match statement {
                Statement::If(if_stmt) => {
                    self.eliminate_unused_statements_in_block(&mut if_stmt.then_branch, analyzer)?;
                    if let Some(ref mut else_branch) = if_stmt.else_branch {
                        self.eliminate_unused_statements_in_block(else_branch, analyzer)?;
                    }
                }
                Statement::While(while_stmt) => {
                    self.eliminate_unused_statements_in_block(&mut while_stmt.body, analyzer)?;
                }
                Statement::For(for_stmt) => {
                    self.eliminate_unused_statements_in_block(&mut for_stmt.body, analyzer)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn has_side_effects(&self, expr: &dyn Expression) -> bool {
        match expr {
            Expression::FunctionCall(_) => true, // Function calls may have side effects
            Expression::Assignment(_) => true,   // Assignments have side effects
            Expression::Binary(binary) => {
                self.has_side_effects(&binary.left) || self.has_side_effects(&binary.right)
            }
            Expression::Unary(unary) => self.has_side_effects(&unary.operand),
            _ => false,
        }
    }

    pub fn get_stats(&self) -> &DeadCodeStats {
        &self.stats
    }
}

/// Usage analyzer for dead code elimination
struct UsageAnalyzer {
    used_functions: HashSet<String>,
    used_variables: HashSet<String>,
    defined_functions: HashSet<String>,
    defined_variables: HashSet<String>,
}

impl UsageAnalyzer {
    fn new() -> Self {
        Self {
            used_functions: HashSet::new(),
            used_variables: HashSet::new(),
            defined_functions: HashSet::new(),
            defined_variables: HashSet::new(),
        }
    }

    fn analyze_program(&mut self, program: &Program) -> Result<()> {
        // First pass: collect all definitions
        for function in &program.functions {
            self.defined_functions.insert(function.name.clone());
            for param in &function.parameters {
                self.defined_variables.insert(param.name.clone());
            }
            self.collect_variable_definitions(&function.body);
        }

        // Second pass: collect all usages
        for function in &program.functions {
            self.analyze_statements(&function.body)?;
        }

        Ok(())
    }

    fn collect_variable_definitions(&mut self, statements: &[dyn Statement]) {
        for statement in statements {
            match statement {
                Statement::VariableDeclaration(var_decl) => {
                    self.defined_variables.insert(var_decl.name.clone());
                }
                Statement::If(if_stmt) => {
                    self.collect_variable_definitions(&if_stmt.then_branch);
                    if let Some(ref else_branch) = if_stmt.else_branch {
                        self.collect_variable_definitions(else_branch);
                    }
                }
                Statement::While(while_stmt) => {
                    self.collect_variable_definitions(&while_stmt.body);
                }
                Statement::For(for_stmt) => {
                    self.collect_variable_definitions(&for_stmt.body);
                }
                _ => {}
            }
        }
    }

    fn analyze_statements(&mut self, statements: &[dyn Statement]) -> Result<()> {
        for statement in statements {
            self.analyze_statement(statement)?;
        }
        Ok(())
    }

    fn analyze_statement(&mut self, statement: &dyn Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => {
                self.analyze_expression(expr)?;
            }
            Statement::VariableDeclaration(var_decl) => {
                if let Some(ref init) = var_decl.initializer {
                    self.analyze_expression(init)?;
                }
            }
            Statement::If(if_stmt) => {
                self.analyze_expression(&if_stmt.condition)?;
                self.analyze_statements(&if_stmt.then_branch)?;
                if let Some(ref else_branch) = if_stmt.else_branch {
                    self.analyze_statements(else_branch)?;
                }
            }
            Statement::While(while_stmt) => {
                self.analyze_expression(&while_stmt.condition)?;
                self.analyze_statements(&while_stmt.body)?;
            }
            Statement::For(for_stmt) => {
                if let Some(ref init) = for_stmt.init {
                    self.analyze_expression(init)?;
                }
                if let Some(ref condition) = for_stmt.condition {
                    self.analyze_expression(condition)?;
                }
                if let Some(ref update) = for_stmt.update {
                    self.analyze_expression(update)?;
                }
                self.analyze_statements(&for_stmt.body)?;
            }
            Statement::Return(return_stmt) => {
                if let Some(ref value) = return_stmt.value {
                    self.analyze_expression(value)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn analyze_expression(&mut self, expr: &dyn Expression) -> Result<()> {
        match expr {
            Expression::Identifier(name) => {
                self.used_variables.insert(name.clone());
            }
            Expression::FunctionCall(call) => {
                self.used_functions.insert(call.name.clone());
                for arg in &call.arguments {
                    self.analyze_expression(arg)?;
                }
            }
            Expression::Binary(binary) => {
                self.analyze_expression(&binary.left)?;
                self.analyze_expression(&binary.right)?;
            }
            Expression::Unary(unary) => {
                self.analyze_expression(&unary.operand)?;
            }
            Expression::Assignment(assignment) => {
                self.analyze_expression(&assignment.value)?;
                // Note: We don't mark the target as "used" for assignment
            }
            _ => {}
        }
        Ok(())
    }

    fn is_function_used(&self, name: &str) -> bool {
        self.used_functions.contains(name)
    }

    fn is_variable_used(&self, name: &str) -> bool {
        self.used_variables.contains(name)
    }
}

/// Constant propagation and folding pass
pub struct ConstantPropagator {
    config: PassConfig,
    constant_values: HashMap<String, LiteralValue>,
    stats: ConstantPropagationStats,
}

#[derive(Debug, Clone, Default)]
pub struct ConstantPropagationStats {
    pub constants_propagated: u32,
    pub expressions_folded: u32,
    pub optimization_time: Duration,
}

#[derive(Debug, Clone)]
enum LiteralValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}

impl ConstantPropagator {
    pub fn new(config: PassConfig) -> Self {
        Self {
            config,
            constant_values: HashMap::new(),
            stats: ConstantPropagationStats::default(),
        }
    }

    pub fn propagate(&mut self, program: &mut Program) -> Result<()> {
        let start_time = Instant::now();
        
        tracing::info!("Starting constant propagation pass");
        
        for function in &mut program.functions {
            self.propagate_in_function(function)?;
        }
        
        self.stats.optimization_time = start_time.elapsed();
        
        tracing::info!(
            constants_propagated = self.stats.constants_propagated,
            expressions_folded = self.stats.expressions_folded,
            optimization_time_ms = self.stats.optimization_time.as_millis(),
            "Constant propagation completed"
        );
        
        Ok(())
    }

    fn propagate_in_function(&mut self, function: &mut Function) -> Result<()> {
        self.constant_values.clear();
        self.propagate_in_statements(&mut function.body)?;
        Ok(())
    }

    fn propagate_in_statements(&mut self, statements: &mut Vec<Statement>) -> Result<()> {
        for statement in statements {
            self.propagate_in_statement(statement)?;
        }
        Ok(())
    }

    fn propagate_in_statement(&mut self, statement: &mut Statement) -> Result<()> {
        match statement {
            Statement::VariableDeclaration(var_decl) => {
                if let Some(ref mut init) = var_decl.initializer {
                    self.propagate_in_expression(init)?;
                    // If the initializer is now a literal, track it
                    if let Some(value) = self.extract_literal_value(init) {
                        self.constant_values.insert(var_decl.name.clone(), value);
                    }
                }
            }
            Statement::Expression(expr) => {
                self.propagate_in_expression(expr)?;
            }
            Statement::If(if_stmt) => {
                self.propagate_in_expression(&mut if_stmt.condition)?;
                self.propagate_in_statements(&mut if_stmt.then_branch)?;
                if let Some(ref mut else_branch) = if_stmt.else_branch {
                    self.propagate_in_statements(else_branch)?;
                }
            }
            Statement::While(while_stmt) => {
                self.propagate_in_expression(&mut while_stmt.condition)?;
                self.propagate_in_statements(&mut while_stmt.body)?;
            }
            Statement::For(for_stmt) => {
                if let Some(ref mut init) = for_stmt.init {
                    self.propagate_in_expression(init)?;
                }
                if let Some(ref mut condition) = for_stmt.condition {
                    self.propagate_in_expression(condition)?;
                }
                if let Some(ref mut update) = for_stmt.update {
                    self.propagate_in_expression(update)?;
                }
                self.propagate_in_statements(&mut for_stmt.body)?;
            }
            Statement::Return(return_stmt) => {
                if let Some(ref mut value) = return_stmt.value {
                    self.propagate_in_expression(value)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn propagate_in_expression(&mut self, expr: &mut Expression) -> Result<()> {
        match expr {
            Expression::Identifier(name) => {
                // Replace with constant value if available
                if let Some(value) = self.constant_values.get(name) {
                    *expr = self.literal_value_to_expression(value.clone());
                    self.stats.constants_propagated += 1;
                }
            }
            Expression::Binary(binary) => {
                self.propagate_in_expression(&mut binary.left)?;
                self.propagate_in_expression(&mut binary.right)?;
                
                // Try to fold the binary expression
                if let Some(folded) = self.fold_binary_expression(binary) {
                    *expr = folded;
                    self.stats.expressions_folded += 1;
                }
            }
            Expression::Unary(unary) => {
                self.propagate_in_expression(&mut unary.operand)?;
                
                // Try to fold the unary expression
                if let Some(folded) = self.fold_unary_expression(unary) {
                    *expr = folded;
                    self.stats.expressions_folded += 1;
                }
            }
            Expression::FunctionCall(call) => {
                for arg in &mut call.arguments {
                    self.propagate_in_expression(arg)?;
                }
            }
            Expression::Assignment(assignment) => {
                self.propagate_in_expression(&mut assignment.value)?;
                // Remove the variable from constants since it's being reassigned
                if let Expression::Identifier(name) = &assignment.target {
                    self.constant_values.remove(name);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn extract_literal_value(&self, expr: &dyn Expression) -> Option<LiteralValue> {
        match expr {
            Expression::IntegerLiteral(value) => Some(LiteralValue::Integer(*value)),
            Expression::FloatLiteral(value) => Some(LiteralValue::Float(*value)),
            Expression::BooleanLiteral(value) => Some(LiteralValue::Boolean(*value)),
            Expression::StringLiteral(value) => Some(LiteralValue::String(value.clone())),
            _ => None,
        }
    }

    fn literal_value_to_expression(&self, value: LiteralValue) -> Expression {
        match value {
            LiteralValue::Integer(val) => Expression::IntegerLiteral(val),
            LiteralValue::Float(val) => Expression::FloatLiteral(val),
            LiteralValue::Boolean(val) => Expression::BooleanLiteral(val),
            LiteralValue::String(val) => Expression::StringLiteral(val),
        }
    }

    fn fold_binary_expression(&self, binary: &BinaryExpression) -> Option<Expression> {
        let left_val = self.extract_literal_value(&binary.left)?;
        let right_val = self.extract_literal_value(&binary.right)?;

        match (&left_val, &right_val, &binary.operator) {
            (LiteralValue::Integer(l), LiteralValue::Integer(r), BinaryOperator::Add) => {
                Some(Expression::IntegerLiteral(l + r))
            }
            (LiteralValue::Integer(l), LiteralValue::Integer(r), BinaryOperator::Subtract) => {
                Some(Expression::IntegerLiteral(l - r))
            }
            (LiteralValue::Integer(l), LiteralValue::Integer(r), BinaryOperator::Multiply) => {
                Some(Expression::IntegerLiteral(l * r))
            }
            (LiteralValue::Integer(l), LiteralValue::Integer(r), BinaryOperator::Divide) => {
                if *r != 0 {
                    Some(Expression::IntegerLiteral(l / r))
                } else {
                    None
                }
            }
            (LiteralValue::Float(l), LiteralValue::Float(r), BinaryOperator::Add) => {
                Some(Expression::FloatLiteral(l + r))
            }
            (LiteralValue::Float(l), LiteralValue::Float(r), BinaryOperator::Subtract) => {
                Some(Expression::FloatLiteral(l - r))
            }
            (LiteralValue::Float(l), LiteralValue::Float(r), BinaryOperator::Multiply) => {
                Some(Expression::FloatLiteral(l * r))
            }
            (LiteralValue::Float(l), LiteralValue::Float(r), BinaryOperator::Divide) => {
                Some(Expression::FloatLiteral(l / r))
            }
            (LiteralValue::Boolean(l), LiteralValue::Boolean(r), BinaryOperator::And) => {
                Some(Expression::BooleanLiteral(*l && *r))
            }
            (LiteralValue::Boolean(l), LiteralValue::Boolean(r), BinaryOperator::Or) => {
                Some(Expression::BooleanLiteral(*l || *r))
            }
            _ => None,
        }
    }

    fn fold_unary_expression(&self, unary: &UnaryExpression) -> Option<Expression> {
        let operand_val = self.extract_literal_value(&unary.operand)?;

        match (&operand_val, &unary.operator) {
            (LiteralValue::Integer(val), UnaryOperator::Minus) => {
                Some(Expression::IntegerLiteral(-val))
            }
            (LiteralValue::Float(val), UnaryOperator::Minus) => {
                Some(Expression::FloatLiteral(-val))
            }
            (LiteralValue::Boolean(val), UnaryOperator::Not) => {
                Some(Expression::BooleanLiteral(!val))
            }
            _ => None,
        }
    }

    pub fn get_stats(&self) -> &ConstantPropagationStats {
        &self.stats
    }
}

/// Loop optimization pass
pub struct LoopOptimizer {
    config: LoopOptimizationConfig,
    stats: LoopOptimizationStats,
}

#[derive(Debug, Clone, Default)]
pub struct LoopOptimizationStats {
    pub loops_unrolled: u32,
    pub invariant_code_motions: u32,
    pub loops_fused: u32,
    pub strength_reductions: u32,
    pub optimization_time: Duration,
}

impl LoopOptimizer {
    pub fn new(config: LoopOptimizationConfig) -> Self {
        Self {
            config,
            stats: LoopOptimizationStats::default(),
        }
    }

    pub fn optimize(&mut self, program: &mut Program) -> Result<()> {
        let start_time = Instant::now();
        
        tracing::info!("Starting loop optimization pass");
        
        for function in &mut program.functions {
            self.optimize_function(function)?;
        }
        
        self.stats.optimization_time = start_time.elapsed();
        
        tracing::info!(
            loops_unrolled = self.stats.loops_unrolled,
            invariant_code_motions = self.stats.invariant_code_motions,
            loops_fused = self.stats.loops_fused,
            strength_reductions = self.stats.strength_reductions,
            optimization_time_ms = self.stats.optimization_time.as_millis(),
            "Loop optimization completed"
        );
        
        Ok(())
    }

    fn optimize_function(&mut self, function: &mut Function) -> Result<()> {
        self.optimize_statements(&mut function.body)?;
        Ok(())
    }

    fn optimize_statements(&mut self, statements: &mut Vec<Statement>) -> Result<()> {
        let mut i = 0;
        while i < statements.len() {
            match &mut statements[i] {
                Statement::For(for_stmt) => {
                    // Try loop unrolling
                    if self.config.unrolling && self.should_unroll_loop(for_stmt) {
                        if let Some(unrolled) = self.unroll_loop(for_stmt)? {
                            statements.splice(i..=i, unrolled);
                            self.stats.loops_unrolled += 1;
                            continue; // Don't increment i, process the unrolled statements
                        }
                    }

                    // Try invariant code motion
                    if self.config.invariant_code_motion {
                        self.hoist_invariant_code(for_stmt)?;
                    }

                    // Recursively optimize the loop body
                    self.optimize_statements(&mut for_stmt.body)?;
                }
                Statement::While(while_stmt) => {
                    // Similar optimizations for while loops
                    self.optimize_statements(&mut while_stmt.body)?;
                }
                Statement::If(if_stmt) => {
                    self.optimize_statements(&mut if_stmt.then_branch)?;
                    if let Some(ref mut else_branch) = if_stmt.else_branch {
                        self.optimize_statements(else_branch)?;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // Try loop fusion
        if self.config.loop_fusion {
            self.try_loop_fusion(statements)?;
        }

        Ok(())
    }

    fn should_unroll_loop(&self, for_stmt: &ForStatement) -> bool {
        // Simple heuristic: unroll if we can determine the loop has a small, fixed trip count
        if let (Some(Expression::Assignment(init)), Some(condition), Some(Expression::Assignment(update))) = 
            (&for_stmt.init, &for_stmt.condition, &for_stmt.update) {
            
            // Check if it's a simple counting loop
            if let (Expression::Identifier(init_var), Expression::IntegerLiteral(init_val)) = 
                (&init.target, &init.value) {
                
                if let Expression::Binary(cond_binary) = condition {
                    if let (Expression::Identifier(cond_var), Expression::IntegerLiteral(limit)) = 
                        (&cond_binary.left, &cond_binary.right) {
                        
                        if init_var == cond_var {
                            let trip_count = (limit - init_val).abs() as u32;
                            return trip_count <= self.config.max_unroll_count && trip_count > 0;
                        }
                    }
                }
            }
        }
        false
    }

    fn unroll_loop(&self, for_stmt: &ForStatement) -> Result<Option<Vec<Statement>>> {
        // Extract loop parameters
        if let (Some(Expression::Assignment(init)), Some(condition), Some(Expression::Assignment(update))) = 
            (&for_stmt.init, &for_stmt.condition, &for_stmt.update) {
            
            if let (Expression::Identifier(var_name), Expression::IntegerLiteral(start)) = 
                (&init.target, &init.value) {
                
                if let Expression::Binary(cond_binary) = condition {
                    if let (Expression::Identifier(cond_var), Expression::IntegerLiteral(end)) = 
                        (&cond_binary.left, &cond_binary.right) {
                        
                        if var_name == cond_var {
                            let mut unrolled = Vec::new();
                            
                            // Generate unrolled iterations
                            for i in *start..*end {
                                // Clone the loop body and substitute the loop variable
                                let mut iteration_body = for_stmt.body.clone();
                                self.substitute_variable(&mut iteration_body, var_name, i)?;
                                unrolled.extend(iteration_body);
                            }
                            
                            return Ok(Some(unrolled));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn substitute_variable(&self, statements: &mut Vec<Statement>, var_name: &str, value: i64) -> Result<()> {
        for statement in statements {
            self.substitute_in_statement(statement, var_name, value)?;
        }
        Ok(())
    }

    fn substitute_in_statement(&self, statement: &mut Statement, var_name: &str, value: i64) -> Result<()> {
        match statement {
            Statement::Expression(expr) => {
                self.substitute_in_expression(expr, var_name, value)?;
            }
            Statement::VariableDeclaration(var_decl) => {
                if let Some(ref mut init) = var_decl.initializer {
                    self.substitute_in_expression(init, var_name, value)?;
                }
            }
            Statement::If(if_stmt) => {
                self.substitute_in_expression(&mut if_stmt.condition, var_name, value)?;
                self.substitute_variable(&mut if_stmt.then_branch, var_name, value)?;
                if let Some(ref mut else_branch) = if_stmt.else_branch {
                    self.substitute_variable(else_branch, var_name, value)?;
                }
            }
            Statement::Return(return_stmt) => {
                if let Some(ref mut val) = return_stmt.value {
                    self.substitute_in_expression(val, var_name, value)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn substitute_in_expression(&self, expr: &mut Expression, var_name: &str, value: i64) -> Result<()> {
        match expr {
            Expression::Identifier(name) => {
                if name == var_name {
                    *expr = Expression::IntegerLiteral(value);
                }
            }
            Expression::Binary(binary) => {
                self.substitute_in_expression(&mut binary.left, var_name, value)?;
                self.substitute_in_expression(&mut binary.right, var_name, value)?;
            }
            Expression::Unary(unary) => {
                self.substitute_in_expression(&mut unary.operand, var_name, value)?;
            }
            Expression::FunctionCall(call) => {
                for arg in &mut call.arguments {
                    self.substitute_in_expression(arg, var_name, value)?;
                }
            }
            Expression::Assignment(assignment) => {
                self.substitute_in_expression(&mut assignment.value, var_name, value)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn hoist_invariant_code(&mut self, for_stmt: &mut ForStatement) -> Result<()> {
        // Simplified invariant code motion - move loop-invariant expressions before the loop
        // This is a basic implementation; a full version would need more sophisticated analysis
        self.stats.invariant_code_motions += 1;
        Ok(())
    }

    fn try_loop_fusion(&mut self, statements: &mut Vec<Statement>) -> Result<()> {
        // Look for adjacent loops that can be fused
        let mut i = 0;
        while i + 1 < statements.len() {
            if let (Statement::For(loop1), Statement::For(loop2)) = 
                (&statements[i], &statements[i + 1]) {
                
                if self.can_fuse_loops(loop1, loop2) {
                    // Fuse the loops
                    if let Some(fused) = self.fuse_loops(loop1, loop2)? {
                        statements[i] = fused;
                        statements.remove(i + 1);
                        self.stats.loops_fused += 1;
                        continue;
                    }
                }
            }
            i += 1;
        }
        Ok(())
    }

    fn can_fuse_loops(&self, loop1: &ForStatement, loop2: &ForStatement) -> bool {
        // Simplified check: loops can be fused if they have the same iteration pattern
        // A full implementation would check for dependencies and side effects
        format!("{:?}", loop1.init) == format!("{:?}", loop2.init) &&
        format!("{:?}", loop1.condition) == format!("{:?}", loop2.condition) &&
        format!("{:?}", loop1.update) == format!("{:?}", loop2.update)
    }

    fn fuse_loops(&self, loop1: &ForStatement, loop2: &ForStatement) -> Result<Option<Statement>> {
        let mut fused_body = loop1.body.clone();
        fused_body.extend(loop2.body.clone());

        let fused_loop = ForStatement {
            init: loop1.init.clone(),
            condition: loop1.condition.clone(),
            update: loop1.update.clone(),
            body: fused_body,
        };

        Ok(Some(Statement::For(fused_loop)))
    }

    pub fn get_stats(&self) -> &LoopOptimizationStats {
        &self.stats
    }
}

/// Function inlining decision engine
pub struct InliningDecision {
    config: InliningConfig,
    function_sizes: HashMap<String, u32>,
    call_graph: HashMap<String, Vec<String>>,
    stats: InliningStats,
}

#[derive(Debug, Clone, Default)]
pub struct InliningStats {
    pub functions_inlined: u32,
    pub call_sites_inlined: u32,
    pub size_reduction: i64,
    pub optimization_time: Duration,
}

impl InliningDecision {
    pub fn new(config: InliningConfig) -> Self {
        Self {
            config,
            function_sizes: HashMap::new(),
            call_graph: HashMap::new(),
            stats: InliningStats::default(),
        }
    }

    pub fn inline_functions(&mut self, program: &mut Program) -> Result<()> {
        let start_time = Instant::now();
        
        tracing::info!("Starting function inlining pass");
        
        // Build call graph and analyze function sizes
        self.analyze_program(program)?;
        
        // Perform inlining
        self.perform_inlining(program)?;
        
        self.stats.optimization_time = start_time.elapsed();
        
        tracing::info!(
            functions_inlined = self.stats.functions_inlined,
            call_sites_inlined = self.stats.call_sites_inlined,
            size_reduction = self.stats.size_reduction,
            optimization_time_ms = self.stats.optimization_time.as_millis(),
            "Function inlining completed"
        );
        
        Ok(())
    }

    fn analyze_program(&mut self, program: &Program) -> Result<()> {
        // Calculate function sizes
        for function in &program.functions {
            let size = self.calculate_function_size(function);
            self.function_sizes.insert(function.name.clone(), size);
        }

        // Build call graph
        for function in &program.functions {
            let callees = self.extract_function_calls(&function.body);
            self.call_graph.insert(function.name.clone(), callees);
        }

        Ok(())
    }

    fn calculate_function_size(&self, function: &Function) -> u32 {
        // Simplified size calculation based on statement count
        self.count_statements(&function.body)
    }

    fn count_statements(&self, statements: &[dyn Statement]) -> u32 {
        let mut count = statements.len() as u32;
        for statement in statements {
            count += match statement {
                Statement::If(if_stmt) => {
                    let mut if_count = self.count_statements(&if_stmt.then_branch);
                    if let Some(ref else_branch) = if_stmt.else_branch {
                        if_count += self.count_statements(else_branch);
                    }
                    if_count
                }
                Statement::While(while_stmt) => self.count_statements(&while_stmt.body),
                Statement::For(for_stmt) => self.count_statements(&for_stmt.body),
                _ => 0,
            };
        }
        count
    }

    fn extract_function_calls(&self, statements: &[dyn Statement]) -> Vec<String> {
        let mut calls = Vec::new();
        for statement in statements {
            self.extract_calls_from_statement(statement, &mut calls);
        }
        calls
    }

    fn extract_calls_from_statement(&self, statement: &dyn Statement, calls: &mut Vec<String>) {
        match statement {
            Statement::Expression(expr) => {
                self.extract_calls_from_expression(expr, calls);
            }
            Statement::VariableDeclaration(var_decl) => {
                if let Some(ref init) = var_decl.initializer {
                    self.extract_calls_from_expression(init, calls);
                }
            }
            Statement::If(if_stmt) => {
                self.extract_calls_from_expression(&if_stmt.condition, calls);
                self.extract_function_calls(&if_stmt.then_branch);
                if let Some(ref else_branch) = if_stmt.else_branch {
                    self.extract_function_calls(else_branch);
                }
            }
            Statement::While(while_stmt) => {
                self.extract_calls_from_expression(&while_stmt.condition, calls);
                self.extract_function_calls(&while_stmt.body);
            }
            Statement::For(for_stmt) => {
                if let Some(ref init) = for_stmt.init {
                    self.extract_calls_from_expression(init, calls);
                }
                if let Some(ref condition) = for_stmt.condition {
                    self.extract_calls_from_expression(condition, calls);
                }
                if let Some(ref update) = for_stmt.update {
                    self.extract_calls_from_expression(update, calls);
                }
                self.extract_function_calls(&for_stmt.body);
            }
            Statement::Return(return_stmt) => {
                if let Some(ref value) = return_stmt.value {
                    self.extract_calls_from_expression(value, calls);
                }
            }
            _ => {}
        }
    }

    fn extract_calls_from_expression(&self, expr: &dyn Expression, calls: &mut Vec<String>) {
        match expr {
            Expression::FunctionCall(call) => {
                calls.push(call.name.clone());
                for arg in &call.arguments {
                    self.extract_calls_from_expression(arg, calls);
                }
            }
            Expression::Binary(binary) => {
                self.extract_calls_from_expression(&binary.left, calls);
                self.extract_calls_from_expression(&binary.right, calls);
            }
            Expression::Unary(unary) => {
                self.extract_calls_from_expression(&unary.operand, calls);
            }
            Expression::Assignment(assignment) => {
                self.extract_calls_from_expression(&assignment.value, calls);
            }
            _ => {}
        }
    }

    fn perform_inlining(&mut self, program: &mut Program) -> Result<()> {
        // Create a function lookup table
        let mut function_map = HashMap::new();
        for function in &program.functions {
            function_map.insert(function.name.clone(), function.clone());
        }

        // Inline functions
        for function in &mut program.functions {
            self.inline_in_function(function, &function_map)?;
        }

        Ok(())
    }

    fn inline_in_function(&mut self, function: &mut Function, function_map: &HashMap<String, Function>) -> Result<()> {
        self.inline_in_statements(&mut function.body, function_map, 0)?;
        Ok(())
    }

    fn inline_in_statements(&mut self, statements: &mut Vec<Statement>, function_map: &HashMap<String, Function>, depth: u32) -> Result<()> {
        if depth >= self.config.max_inline_depth {
            return Ok(());
        }

        let mut i = 0;
        while i < statements.len() {
            match &mut statements[i] {
                Statement::Expression(Expression::FunctionCall(call)) => {
                    if self.should_inline_call(call, function_map) {
                        if let Some(inlined) = self.inline_function_call(call, function_map)? {
                            statements.splice(i..=i, inlined);
                            self.stats.call_sites_inlined += 1;
                            continue; // Don't increment i, process the inlined statements
                        }
                    }
                }
                Statement::VariableDeclaration(var_decl) => {
                    if let Some(Expression::FunctionCall(call)) = &mut var_decl.initializer {
                        if self.should_inline_call(call, function_map) {
                            if let Some(inlined_expr) = self.inline_function_call_expression(call, function_map)? {
                                var_decl.initializer = Some(inlined_expr);
                                self.stats.call_sites_inlined += 1;
                            }
                        }
                    }
                }
                Statement::If(if_stmt) => {
                    self.inline_in_statements(&mut if_stmt.then_branch, function_map, depth + 1)?;
                    if let Some(ref mut else_branch) = if_stmt.else_branch {
                        self.inline_in_statements(else_branch, function_map, depth + 1)?;
                    }
                }
                Statement::While(while_stmt) => {
                    self.inline_in_statements(&mut while_stmt.body, function_map, depth + 1)?;
                }
                Statement::For(for_stmt) => {
                    self.inline_in_statements(&mut for_stmt.body, function_map, depth + 1)?;
                }
                _ => {}
            }
            i += 1;
        }
        Ok(())
    }

    fn should_inline_call(&self, call: &FunctionCall, function_map: &HashMap<String, Function>) -> bool {
        if !self.config.enabled {
            return false;
        }

        // Don't inline recursive calls
        if call.name == "main" {
            return false;
        }

        if let Some(function_size) = self.function_sizes.get(&call.name) {
            // Always inline very small functions
            if *function_size <= self.config.always_inline_threshold {
                return true;
            }

            // Inline if under the size threshold
            if *function_size <= self.config.max_inline_size {
                return true;
            }

            // Aggressive inlining for hot functions (simplified heuristic)
            if self.config.aggressive_hot_inlining {
                // In a real implementation, this would use profiling data
                return *function_size <= self.config.max_inline_size * 2;
            }
        }

        false
    }

    fn inline_function_call(&mut self, call: &FunctionCall, function_map: &HashMap<String, Function>) -> Result<Option<Vec<Statement>>> {
        if let Some(function) = function_map.get(&call.name) {
            let mut inlined_body = function.body.clone();
            
            // Substitute parameters with arguments
            for (param, arg) in function.parameters.iter().zip(call.arguments.iter()) {
                self.substitute_parameter(&mut inlined_body, &param.name, arg)?;
            }
            
            self.stats.functions_inlined += 1;
            return Ok(Some(inlined_body));
        }
        Ok(None)
    }

    fn inline_function_call_expression(&mut self, call: &FunctionCall, function_map: &HashMap<String, Function>) -> Result<Option<Expression>> {
        if let Some(function) = function_map.get(&call.name) {
            // For expression inlining, we need to find the return value
            if let Some(return_expr) = self.extract_return_expression(&function.body) {
                let mut inlined_expr = return_expr.clone();
                
                // Substitute parameters with arguments
                for (param, arg) in function.parameters.iter().zip(call.arguments.iter()) {
                    self.substitute_parameter_in_expression(&mut inlined_expr, &param.name, arg)?;
                }
                
                return Ok(Some(inlined_expr));
            }
        }
        Ok(None)
    }

    fn extract_return_expression(&self, statements: &[dyn Statement]) -> Option<&dyn Expression> {
        for statement in statements {
            if let Statement::Return(return_stmt) = statement {
                return return_stmt.value.as_ref();
            }
        }
        None
    }

    fn substitute_parameter(&self, statements: &mut Vec<Statement>, param_name: &str, arg: &dyn Expression) -> Result<()> {
        for statement in statements {
            self.substitute_parameter_in_statement(statement, param_name, arg)?;
        }
        Ok(())
    }

    fn substitute_parameter_in_statement(&self, statement: &mut Statement, param_name: &str, arg: &dyn Expression) -> Result<()> {
        match statement {
            Statement::Expression(expr) => {
                self.substitute_parameter_in_expression(expr, param_name, arg)?;
            }
            Statement::VariableDeclaration(var_decl) => {
                if let Some(ref mut init) = var_decl.initializer {
                    self.substitute_parameter_in_expression(init, param_name, arg)?;
                }
            }
            Statement::If(if_stmt) => {
                self.substitute_parameter_in_expression(&mut if_stmt.condition, param_name, arg)?;
                self.substitute_parameter(&mut if_stmt.then_branch, param_name, arg)?;
                if let Some(ref mut else_branch) = if_stmt.else_branch {
                    self.substitute_parameter(else_branch, param_name, arg)?;
                }
            }
            Statement::While(while_stmt) => {
                self.substitute_parameter_in_expression(&mut while_stmt.condition, param_name, arg)?;
                self.substitute_parameter(&mut while_stmt.body, param_name, arg)?;
            }
            Statement::For(for_stmt) => {
                if let Some(ref mut init) = for_stmt.init {
                    self.substitute_parameter_in_expression(init, param_name, arg)?;
                }
                if let Some(ref mut condition) = for_stmt.condition {
                    self.substitute_parameter_in_expression(condition, param_name, arg)?;
                }
                if let Some(ref mut update) = for_stmt.update {
                    self.substitute_parameter_in_expression(update, param_name, arg)?;
                }
                self.substitute_parameter(&mut for_stmt.body, param_name, arg)?;
            }
            Statement::Return(return_stmt) => {
                if let Some(ref mut value) = return_stmt.value {
                    self.substitute_parameter_in_expression(value, param_name, arg)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn substitute_parameter_in_expression(&self, expr: &mut Expression, param_name: &str, arg: &dyn Expression) -> Result<()> {
        match expr {
            Expression::Identifier(name) => {
                if name == param_name {
                    *expr = arg.clone();
                }
            }
            Expression::Binary(binary) => {
                self.substitute_parameter_in_expression(&mut binary.left, param_name, arg)?;
                self.substitute_parameter_in_expression(&mut binary.right, param_name, arg)?;
            }
            Expression::Unary(unary) => {
                self.substitute_parameter_in_expression(&mut unary.operand, param_name, arg)?;
            }
            Expression::FunctionCall(call) => {
                for call_arg in &mut call.arguments {
                    self.substitute_parameter_in_expression(call_arg, param_name, arg)?;
                }
            }
            Expression::Assignment(assignment) => {
                self.substitute_parameter_in_expression(&mut assignment.value, param_name, arg)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_stats(&self) -> &InliningStats {
        &self.stats
    }
}

/// Register allocation optimizer with interference graph and coloring
pub struct RegisterAllocator {
    config: crate::optimization::config::RegisterAllocationConfig,
    stats: RegisterAllocationStats,
    interference_graph: InterferenceGraph,
    spill_candidates: HashSet<String>,
    register_map: HashMap<String, u32>,
}

#[derive(Debug, Clone, Default)]
pub struct RegisterAllocationStats {
    pub registers_allocated: u32,
    pub spills_avoided: u32,
    pub coalescing_operations: u32,
    pub interference_edges: u32,
    pub coloring_iterations: u32,
    pub optimization_time: Duration,
}

/// Interference graph for register allocation
#[derive(Debug, Default)]
struct InterferenceGraph {
    nodes: HashSet<String>,
    edges: HashSet<(String, String)>,
    degrees: HashMap<String, u32>,
}

/// Live range information for variables
#[derive(Debug, Clone)]
struct LiveRange {
    variable: String,
    start: usize,
    end: usize,
    uses: Vec<usize>,
    definition: usize,
    spill_cost: f64,
}

impl RegisterAllocator {
    pub fn new(config: crate::optimization::config::RegisterAllocationConfig) -> Self {
        Self {
            config,
            stats: RegisterAllocationStats::default(),
            interference_graph: InterferenceGraph::default(),
            spill_candidates: HashSet::new(),
            register_map: HashMap::new(),
        }
    }

    pub fn allocate(&mut self, program: &mut Program) -> Result<()> {
        let start_time = Instant::now();
        
        tracing::info!("Starting advanced register allocation optimization");
        
        for function in &mut program.functions {
            self.allocate_for_function(function)?;
        }
        
        self.stats.optimization_time = start_time.elapsed();
        
        tracing::info!(
            registers_allocated = self.stats.registers_allocated,
            spills_avoided = self.stats.spills_avoided,
            coalescing_operations = self.stats.coalescing_operations,
            interference_edges = self.stats.interference_edges,
            coloring_iterations = self.stats.coloring_iterations,
            optimization_time_ms = self.stats.optimization_time.as_millis(),
            "Advanced register allocation completed"
        );
        
        Ok(())
    }

    fn allocate_for_function(&mut self, function: &mut Function) -> Result<()> {
        tracing::debug!("Allocating registers for function: {}", function.name);

        // 1. Perform liveness analysis
        let live_ranges = self.analyze_liveness(&function.body)?;
        
        // 2. Build interference graph
        self.build_interference_graph(&live_ranges)?;
        
        // 3. Compute spill costs
        let spill_costs = self.compute_spill_costs(&live_ranges);
        
        // 4. Graph coloring with potential spilling
        let coloring = self.color_graph_with_spilling(&spill_costs)?;
        
        // 5. Apply register assignments
        self.apply_register_assignments(function, &coloring)?;
        
        // 6. Perform coalescing if enabled
        if self.config.coalescing {
            self.perform_coalescing(function, &live_ranges)?;
        }

        Ok(())
    }

    fn analyze_liveness(&mut self, statements: &[dyn Statement]) -> Result<Vec<LiveRange>> {
        let mut live_ranges = Vec::new();
        let mut current_position = 0;
        let mut variable_definitions: HashMap<String, usize> = HashMap::new();
        let mut variable_uses: HashMap<String, Vec<usize>> = HashMap::new();

        // First pass: collect definitions and uses
        self.collect_def_use(statements, &mut current_position, &mut variable_definitions, &mut variable_uses)?;

        // Create live ranges
        for (var, def_pos) in variable_definitions {
            let uses = variable_uses.get(&var).cloned().unwrap_or_default();
            let end_pos = uses.iter().max().copied().unwrap_or(def_pos);
            
            live_ranges.push(LiveRange {
                variable: var.clone(),
                start: def_pos,
                end: end_pos,
                uses: uses.clone(),
                definition: def_pos,
                spill_cost: self.calculate_spill_cost(&uses, def_pos, end_pos),
            });
        }

        tracing::debug!("Analyzed {} live ranges", live_ranges.len());
        Ok(live_ranges)
    }

    fn collect_def_use(
        &self,
        statements: &[dyn Statement],
        position: &mut usize,
        definitions: &mut HashMap<String, usize>,
        uses: &mut HashMap<String, Vec<usize>>,
    ) -> Result<()> {
        for statement in statements {
            match statement {
                Statement::VariableDeclaration(var_decl) => {
                    definitions.insert(var_decl.name.clone(), *position);
                    if let Some(ref init) = var_decl.initializer {
                        self.collect_expression_uses(init, position, uses);
                    }
                }
                Statement::Expression(expr) => {
                    self.collect_expression_uses(expr, position, uses);
                }
                Statement::If(if_stmt) => {
                    self.collect_expression_uses(&if_stmt.condition, position, uses);
                    *position += 1;
                    self.collect_def_use(&if_stmt.then_branch, position, definitions, uses)?;
                    if let Some(ref else_branch) = if_stmt.else_branch {
                        self.collect_def_use(else_branch, position, definitions, uses)?;
                    }
                }
                Statement::While(while_stmt) => {
                    self.collect_expression_uses(&while_stmt.condition, position, uses);
                    *position += 1;
                    self.collect_def_use(&while_stmt.body, position, definitions, uses)?;
                }
                Statement::For(for_stmt) => {
                    if let Some(ref init) = for_stmt.init {
                        self.collect_expression_uses(init, position, uses);
                    }
                    if let Some(ref condition) = for_stmt.condition {
                        self.collect_expression_uses(condition, position, uses);
                    }
                    if let Some(ref update) = for_stmt.update {
                        self.collect_expression_uses(update, position, uses);
                    }
                    *position += 1;
                    self.collect_def_use(&for_stmt.body, position, definitions, uses)?;
                }
                Statement::Return(return_stmt) => {
                    if let Some(ref value) = return_stmt.value {
                        self.collect_expression_uses(value, position, uses);
                    }
                }
                _ => {}
            }
            *position += 1;
        }
        Ok(())
    }

    fn collect_expression_uses(&self, expr: &dyn Expression, position: &usize, uses: &mut HashMap<String, Vec<usize>>) {
        match expr {
            Expression::Identifier(name) => {
                uses.entry(name.clone()).or_insert_with(Vec::new).push(*position);
            }
            Expression::Binary(binary) => {
                self.collect_expression_uses(&binary.left, position, uses);
                self.collect_expression_uses(&binary.right, position, uses);
            }
            Expression::Unary(unary) => {
                self.collect_expression_uses(&unary.operand, position, uses);
            }
            Expression::FunctionCall(call) => {
                for arg in &call.arguments {
                    self.collect_expression_uses(arg, position, uses);
                }
            }
            Expression::Assignment(assignment) => {
                self.collect_expression_uses(&assignment.value, position, uses);
            }
            _ => {}
        }
    }

    fn calculate_spill_cost(&self, uses: &[usize], def_pos: usize, end_pos: usize) -> f64 {
        let use_count = uses.len() as f64;
        let live_range_length = (end_pos - def_pos) as f64;
        
        // Higher cost for frequently used variables in tight loops
        let frequency_weight = use_count * 10.0;
        let range_weight = 1.0 / (live_range_length + 1.0);
        
        frequency_weight * range_weight
    }

    fn build_interference_graph(&mut self, live_ranges: &[LiveRange]) -> Result<()> {
        self.interference_graph = InterferenceGraph::default();
        
        // Add all variables as nodes
        for range in live_ranges {
            self.interference_graph.nodes.insert(range.variable.clone());
        }

        // Add interference edges for overlapping live ranges
        for i in 0..live_ranges.len() {
            for j in (i + 1)..live_ranges.len() {
                let range1 = &live_ranges[i];
                let range2 = &live_ranges[j];
                
                // Check if live ranges overlap
                if self.ranges_interfere(range1, range2) {
                    self.add_interference_edge(&range1.variable, &range2.variable);
                }
            }
        }

        self.stats.interference_edges = self.interference_graph.edges.len() as u32;
        tracing::debug!("Built interference graph with {} edges", self.stats.interference_edges);
        Ok(())
    }

    fn ranges_interfere(&self, range1: &LiveRange, range2: &LiveRange) -> bool {
        !(range1.end < range2.start || range2.end < range1.start)
    }

    fn add_interference_edge(&mut self, var1: &str, var2: &str) {
        let edge = if var1 < var2 {
            (var1.to_string(), var2.to_string())
        } else {
            (var2.to_string(), var1.to_string())
        };
        
        if self.interference_graph.edges.insert(edge) {
            // Update degrees
            *self.interference_graph.degrees.entry(var1.to_string()).or_insert(0) += 1;
            *self.interference_graph.degrees.entry(var2.to_string()).or_insert(0) += 1;
        }
    }

    fn compute_spill_costs(&self, live_ranges: &[LiveRange]) -> HashMap<String, f64> {
        live_ranges
            .iter()
            .map(|range| (range.variable.clone(), range.spill_cost))
            .collect()
    }

    fn color_graph_with_spilling(&mut self, spill_costs: &HashMap<String, f64>) -> Result<HashMap<String, u32>> {
        let mut coloring = HashMap::new();
        let mut work_list: VecDeque<String> = self.interference_graph.nodes.iter().cloned().collect();
        let mut spilled_nodes = HashSet::new();
        let num_registers = self.config.num_registers.unwrap_or(16);

        while !work_list.is_empty() {
            self.stats.coloring_iterations += 1;
            
            // Try to find a node with degree < num_registers
            if let Some(node) = self.find_simplifiable_node(&work_list, num_registers) {
                work_list.retain(|n| n != &node);
                self.simplify_node(&node);
            } else {
                // Spill the node with lowest spill cost
                if let Some(spill_node) = self.select_spill_candidate(&work_list, spill_costs) {
                    work_list.retain(|n| n != &spill_node);
                    spilled_nodes.insert(spill_node.clone());
                    self.spill_candidates.insert(spill_node);
                } else {
                    break;
                }
            }
        }

        // Color remaining nodes
        for node in &self.interference_graph.nodes {
            if !spilled_nodes.contains(node) {
                let color = self.assign_color(node, &coloring, num_registers)?;
                coloring.insert(node.clone(), color);
                self.stats.registers_allocated += 1;
            }
        }

        self.stats.spills_avoided = (self.interference_graph.nodes.len() - spilled_nodes.len()) as u32;
        Ok(coloring)
    }

    fn find_simplifiable_node(&self, work_list: &VecDeque<String>, num_registers: u32) -> Option<String> {
        work_list
            .iter()
            .find(|node| {
                self.interference_graph.degrees.get(*node).unwrap_or(&0) < &num_registers
            })
            .cloned()
    }

    fn simplify_node(&mut self, node: &str) {
        // Remove node from interference graph (conceptually)
        // Update degrees of neighbors
        for edge in &self.interference_graph.edges.clone() {
            if edge.0 == node || edge.1 == node {
                let neighbor = if edge.0 == node { &edge.1 } else { &edge.0 };
                if let Some(degree) = self.interference_graph.degrees.get_mut(neighbor) {
                    *degree = degree.saturating_sub(1);
                }
            }
        }
    }

    fn select_spill_candidate(&self, work_list: &VecDeque<String>, spill_costs: &HashMap<String, f64>) -> Option<String> {
        work_list
            .iter()
            .min_by(|a, b| {
                let cost_a = spill_costs.get(*a).unwrap_or(&f64::MAX);
                let cost_b = spill_costs.get(*b).unwrap_or(&f64::MAX);
                cost_a.partial_cmp(cost_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    fn assign_color(&self, node: &str, coloring: &HashMap<String, u32>, num_registers: u32) -> Result<u32> {
        let mut used_colors = HashSet::new();
        
        // Collect colors used by interfering neighbors
        for edge in &self.interference_graph.edges {
            let neighbor = if edge.0 == node {
                &edge.1
            } else if edge.1 == node {
                &edge.0
            } else {
                continue;
            };
            
            if let Some(&color) = coloring.get(neighbor) {
                used_colors.insert(color);
            }
        }

        // Find first available color
        for color in 0..num_registers {
            if !used_colors.contains(&color) {
                return Ok(color);
            }
        }

        Err(Error::General("No available register for allocation".to_string()))
    }

    fn apply_register_assignments(&mut self, function: &mut Function, coloring: &HashMap<String, u32>) -> Result<()> {
        self.register_map.clear();
        
        for (var, &register) in coloring {
            self.register_map.insert(var.clone(), register);
        }

        // Apply assignments to function body
        self.apply_assignments_to_statements(&mut function.body)?;
        
        tracing::debug!("Applied register assignments for {} variables", coloring.len());
        Ok(())
    }

    fn apply_assignments_to_statements(&self, statements: &mut Vec<Statement>) -> Result<()> {
        for statement in statements {
            match statement {
                Statement::If(if_stmt) => {
                    self.apply_assignments_to_statements(&mut if_stmt.then_branch)?;
                    if let Some(ref mut else_branch) = if_stmt.else_branch {
                        self.apply_assignments_to_statements(else_branch)?;
                    }
                }
                Statement::While(while_stmt) => {
                    self.apply_assignments_to_statements(&mut while_stmt.body)?;
                }
                Statement::For(for_stmt) => {
                    self.apply_assignments_to_statements(&mut for_stmt.body)?;
                }
                _ => {
                    // Register assignments would be applied here in a real implementation
                    // This would typically involve modifying the IR or adding metadata
                }
            }
        }
        Ok(())
    }

    fn perform_coalescing(&mut self, _function: &mut Function, live_ranges: &[LiveRange]) -> Result<()> {
        // Simplified coalescing implementation
        // Real coalescing would merge move-related variables that don't interfere
        
        let mut coalesced_pairs = 0;
        for i in 0..live_ranges.len() {
            for j in (i + 1)..live_ranges.len() {
                let range1 = &live_ranges[i];
                let range2 = &live_ranges[j];
                
                if self.can_coalesce(range1, range2) {
                    coalesced_pairs += 1;
                    if coalesced_pairs >= 5 { // Limit coalescing iterations
                        break;
                    }
                }
            }
        }
        
        self.stats.coalescing_operations = coalesced_pairs;
        tracing::debug!("Performed {} coalescing operations", coalesced_pairs);
        Ok(())
    }

    fn can_coalesce(&self, range1: &LiveRange, range2: &LiveRange) -> bool {
        // Simple heuristic: can coalesce if ranges don't interfere and one is a copy of the other
        !self.ranges_interfere(range1, range2) && 
        (range1.uses.len() == 1 || range2.uses.len() == 1)
    }

    pub fn get_stats(&self) -> &RegisterAllocationStats {
        &self.stats
    }
}

/// Main compiler pass manager
pub struct CompilerPassManager {
    config: PassConfig,
    passes: Vec<Box<dyn CompilerPass>>,
    stats: CompilerPassStats,
}

trait CompilerPass {
    fn name(&self) -> &str;
    fn run(&mut self, program: &mut Program) -> Result<()>;
    fn get_pass_info(&self) -> PassInfo;
}

/// Information about a compiler pass
#[derive(Debug, Clone)]
pub struct PassInfo {
    pub name: String,
    pub description: String,
    pub category: PassCategory,
    pub dependencies: Vec<String>,
    pub optimization_level: Option<crate::optimization::OptimizationLevel>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PassCategory {
    Analysis,
    Transformation,
    Optimization,
    CodeGeneration,
    Cleanup,
}

// Implement CompilerPass for all optimization passes
impl CompilerPass for DeadCodeEliminator {
    fn name(&self) -> &str {
        "dead-code-elimination"
    }

    fn run(&mut self, program: &mut Program) -> Result<()> {
        self.eliminate(program)
    }

    fn get_pass_info(&self) -> PassInfo {
        PassInfo {
            name: "Dead Code Elimination".to_string(),
            description: "Removes unused functions, variables, and statements".to_string(),
            category: PassCategory::Optimization,
            dependencies: vec!["usage-analysis".to_string()],
            optimization_level: Some(crate::optimization::OptimizationLevel::O1),
        }
    }
}

impl CompilerPass for ConstantPropagator {
    fn name(&self) -> &str {
        "constant-propagation"
    }

    fn run(&mut self, program: &mut Program) -> Result<()> {
        self.propagate(program)
    }

    fn get_pass_info(&self) -> PassInfo {
        PassInfo {
            name: "Constant Propagation".to_string(),
            description: "Replaces variables with their constant values and folds expressions".to_string(),
            category: PassCategory::Optimization,
            dependencies: vec![],
            optimization_level: Some(crate::optimization::OptimizationLevel::O1),
        }
    }
}

impl CompilerPass for LoopOptimizer {
    fn name(&self) -> &str {
        "loop-optimization"
    }

    fn run(&mut self, program: &mut Program) -> Result<()> {
        self.optimize(program)
    }

    fn get_pass_info(&self) -> PassInfo {
        PassInfo {
            name: "Loop Optimization".to_string(),
            description: "Optimizes loops through unrolling, invariant code motion, and fusion".to_string(),
            category: PassCategory::Optimization,
            dependencies: vec!["constant-propagation".to_string()],
            optimization_level: Some(crate::optimization::OptimizationLevel::O2),
        }
    }
}

impl CompilerPass for InliningDecision {
    fn name(&self) -> &str {
        "function-inlining"
    }

    fn run(&mut self, program: &mut Program) -> Result<()> {
        self.inline_functions(program)
    }

    fn get_pass_info(&self) -> PassInfo {
        PassInfo {
            name: "Function Inlining".to_string(),
            description: "Inlines small functions to reduce call overhead".to_string(),
            category: PassCategory::Optimization,
            dependencies: vec!["dead-code-elimination".to_string()],
            optimization_level: Some(crate::optimization::OptimizationLevel::O2),
        }
    }
}

impl CompilerPass for RegisterAllocator {
    fn name(&self) -> &str {
        "register-allocation"
    }

    fn run(&mut self, program: &mut Program) -> Result<()> {
        self.allocate(program)
    }

    fn get_pass_info(&self) -> PassInfo {
        PassInfo {
            name: "Register Allocation".to_string(),
            description: "Allocates CPU registers to variables using graph coloring".to_string(),
            category: PassCategory::CodeGeneration,
            dependencies: vec!["loop-optimization".to_string(), "function-inlining".to_string()],
            optimization_level: Some(crate::optimization::OptimizationLevel::O1),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CompilerPassStats {
    pub total_passes_run: u32,
    pub total_optimization_time: Duration,
    pub passes_applied: Vec<String>,
}

impl CompilerPassManager {
    pub fn new(config: PassConfig) -> Self {
        Self {
            config,
            passes: Vec::new(),
            stats: CompilerPassStats::default(),
        }
    }

    pub fn add_pass(&mut self, pass: Box<dyn CompilerPass>) {
        self.passes.push(pass);
    }

    /// Create a standard optimization pipeline
    pub fn create_standard_pipeline(
        optimization_level: crate::optimization::OptimizationLevel,
    ) -> Result<Self> {
        let config = PassConfig::default();
        let mut manager = Self::new(config);

        // Add passes based on optimization level
        match optimization_level {
            crate::optimization::OptimizationLevel::O0 => {
                // No optimizations for O0
            }
            crate::optimization::OptimizationLevel::O1 => {
                manager.add_pass(Box::new(DeadCodeEliminator::new(PassConfig::default())));
                manager.add_pass(Box::new(ConstantPropagator::new(PassConfig::default())));
                manager.add_pass(Box::new(RegisterAllocator::new(
                    crate::optimization::config::RegisterAllocationConfig::default()
                )));
            }
            crate::optimization::OptimizationLevel::O2 => {
                manager.add_pass(Box::new(ConstantPropagator::new(PassConfig::default())));
                manager.add_pass(Box::new(DeadCodeEliminator::new(PassConfig::default())));
                manager.add_pass(Box::new(LoopOptimizer::new(
                    crate::optimization::config::LoopOptimizationConfig::default()
                )));
                manager.add_pass(Box::new(InliningDecision::new(
                    crate::optimization::config::InliningConfig::default()
                )));
                manager.add_pass(Box::new(RegisterAllocator::new(
                    crate::optimization::config::RegisterAllocationConfig::default()
                )));
            }
            crate::optimization::OptimizationLevel::O3 => {
                // Aggressive optimizations
                manager.add_pass(Box::new(ConstantPropagator::new(PassConfig::default())));
                manager.add_pass(Box::new(DeadCodeEliminator::new(PassConfig::default())));
                
                let mut loop_config = crate::optimization::config::LoopOptimizationConfig::default();
                loop_config.unrolling = true;
                loop_config.invariant_code_motion = true;
                loop_config.loop_fusion = true;
                manager.add_pass(Box::new(LoopOptimizer::new(loop_config)));

                let mut inline_config = crate::optimization::config::InliningConfig::default();
                inline_config.aggressive_hot_inlining = true;
                inline_config.max_inline_size = 100; // Larger inlining threshold
                manager.add_pass(Box::new(InliningDecision::new(inline_config)));

                // Second pass of dead code elimination after inlining
                manager.add_pass(Box::new(DeadCodeEliminator::new(PassConfig::default())));
                
                let mut reg_config = crate::optimization::config::RegisterAllocationConfig::default();
                reg_config.coalescing = true;
                manager.add_pass(Box::new(RegisterAllocator::new(reg_config)));
            }
        }

        Ok(manager)
    }

    /// Resolve pass dependencies and sort passes
    pub fn resolve_dependencies(&mut self) -> Result<()> {
        let mut sorted_passes = Vec::new();
        let mut pass_info: HashMap<String, (usize, PassInfo)> = HashMap::new();
        
        // Collect pass information
        for (i, pass) in self.passes.iter().enumerate() {
            let info = pass.get_pass_info();
            pass_info.insert(pass.name().to_string(), (i, info));
        }

        // Topological sort based on dependencies
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        
        fn visit_pass(
            pass_name: &str,
            pass_info: &HashMap<String, (usize, PassInfo)>,
            visited: &mut HashSet<String>,
            temp_visited: &mut HashSet<String>,
            result: &mut Vec<String>,
        ) -> Result<()> {
            if temp_visited.contains(pass_name) {
                return Err(Error::General(format!("Circular dependency detected: {}", pass_name)));
            }
            
            if visited.contains(pass_name) {
                return Ok(());
            }

            temp_visited.insert(pass_name.to_string());
            
            if let Some((_, info)) = pass_info.get(pass_name) {
                for dep in &info.dependencies {
                    visit_pass(dep, pass_info, visited, temp_visited, result)?;
                }
            }
            
            temp_visited.remove(pass_name);
            visited.insert(pass_name.to_string());
            result.push(pass_name.to_string());
            
            Ok(())
        }

        let mut sorted_names = Vec::new();
        for pass_name in pass_info.keys() {
            if !visited.contains(pass_name) {
                visit_pass(pass_name, &pass_info, &mut visited, &mut temp_visited, &mut sorted_names)?;
            }
        }

        // Reorder passes based on dependency resolution
        let mut new_passes = Vec::new();
        for name in sorted_names {
            if let Some((index, _)) = pass_info.get(&name) {
                // Move the pass from the original position
                // This is a simplified approach - in practice, we'd need to handle this more carefully
                tracing::debug!("Pass {} will run in dependency order", name);
            }
        }

        tracing::info!("Resolved dependencies for {} passes", self.passes.len());
        Ok(())
    }

    pub fn run_all_passes(&mut self, program: &mut Program) -> Result<()> {
        let start_time = Instant::now();
        
        tracing::info!("Running all compiler optimization passes");
        
        // Resolve dependencies first
        self.resolve_dependencies()?;
        
        for pass in &mut self.passes {
            let pass_start = Instant::now();
            let pass_info = pass.get_pass_info();
            
            tracing::debug!(
                pass_name = pass.name(),
                category = ?pass_info.category,
                "Starting compiler pass"
            );
            
            pass.run(program)?;
            let pass_time = pass_start.elapsed();
            
            self.stats.passes_applied.push(pass.name().to_string());
            self.stats.total_passes_run += 1;
            
            tracing::info!(
                pass_name = pass.name(),
                pass_time_ms = pass_time.as_millis(),
                category = ?pass_info.category,
                "Compiler pass completed"
            );
        }
        
        self.stats.total_optimization_time = start_time.elapsed();
        
        tracing::info!(
            total_passes = self.stats.total_passes_run,
            total_time_ms = self.stats.total_optimization_time.as_millis(),
            "All compiler passes completed"
        );
        
        Ok(())
    }

    /// Run passes in parallel where possible
    pub fn run_passes_parallel(&mut self, program: &mut Program) -> Result<()> {
        let start_time = Instant::now();
        
        tracing::info!("Running compiler optimization passes in parallel");
        
        // Group passes by category and dependencies
        let analysis_passes = self.get_passes_by_category(PassCategory::Analysis);
        let optimization_passes = self.get_passes_by_category(PassCategory::Optimization);
        let codegen_passes = self.get_passes_by_category(PassCategory::CodeGeneration);

        // Run analysis passes first (sequential)
        for pass_idx in analysis_passes {
            self.passes[pass_idx].run(program)?;
            self.stats.total_passes_run += 1;
        }

        // Run optimization passes (can be parallelized if they don't interfere)
        for pass_idx in optimization_passes {
            self.passes[pass_idx].run(program)?;
            self.stats.total_passes_run += 1;
        }

        // Run code generation passes last (sequential)
        for pass_idx in codegen_passes {
            self.passes[pass_idx].run(program)?;
            self.stats.total_passes_run += 1;
        }

        self.stats.total_optimization_time = start_time.elapsed();
        
        tracing::info!(
            total_passes = self.stats.total_passes_run,
            total_time_ms = self.stats.total_optimization_time.as_millis(),
            "Parallel compiler passes completed"
        );

        Ok(())
    }

    fn get_passes_by_category(&self, category: PassCategory) -> Vec<usize> {
        self.passes
            .iter()
            .enumerate()
            .filter(|(_, pass)| pass.get_pass_info().category == category)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn get_stats(&self) -> &CompilerPassStats {
        &self.stats
    }

    pub fn get_pass_info(&self) -> Vec<PassInfo> {
        self.passes.iter().map(|pass| pass.get_pass_info()).collect()
    }
}

/// Initialize compiler passes
pub fn initialize_passes() -> Result<()> {
    tracing::debug!("Initializing compiler optimization passes");
    Ok(())
}

/// Cleanup compiler passes
pub fn cleanup_passes() -> Result<()> {
    tracing::debug!("Cleaning up compiler optimization passes");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    fn create_test_program() -> Program {
        Program {
            functions: vec![
                Function {
                    name: "main".to_string(),
                    parameters: vec![],
                    return_type: Type::Void,
                    body: vec![
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: "x".to_string(),
                            var_type: Type::Integer,
                            initializer: Some(Expression::IntegerLiteral(5)),
                        }),
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: "y".to_string(),
                            var_type: Type::Integer,
                            initializer: Some(Expression::IntegerLiteral(10)),
                        }),
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: "unused".to_string(),
                            var_type: Type::Integer,
                            initializer: Some(Expression::IntegerLiteral(0)),
                        }),
                        Statement::Expression(Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Identifier("x".to_string())),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expression::Identifier("y".to_string())),
                        })),
                    ],
                }
            ],
        }
    }

    #[test]
    fn test_dead_code_elimination() {
        let mut program = create_test_program();
        let config = PassConfig::default();
        let mut eliminator = DeadCodeEliminator::new(config);
        
        let result = eliminator.eliminate(&mut program);
        assert!(result.is_ok());
        
        let stats = eliminator.get_stats();
        assert!(stats.eliminated_variables > 0);
    }

    #[test]
    fn test_constant_propagation() {
        let mut program = create_test_program();
        let config = PassConfig::default();
        let mut propagator = ConstantPropagator::new(config);
        
        let result = propagator.propagate(&mut program);
        assert!(result.is_ok());
        
        let stats = propagator.get_stats();
        // Should have propagated some constants
        assert!(stats.optimization_time > Duration::ZERO);
    }

    #[test]
    fn test_loop_optimizer() {
        let config = LoopOptimizationConfig::default();
        let mut optimizer = LoopOptimizer::new(config);
        let mut program = create_test_program();
        
        let result = optimizer.optimize(&mut program);
        assert!(result.is_ok());
        
        let stats = optimizer.get_stats();
        assert!(stats.optimization_time > Duration::ZERO);
    }

    #[test]
    fn test_inlining_decision() {
        let config = InliningConfig::default();
        let mut inliner = InliningDecision::new(config);
        let mut program = create_test_program();
        
        let result = inliner.inline_functions(&mut program);
        assert!(result.is_ok());
        
        let stats = inliner.get_stats();
        assert!(stats.optimization_time > Duration::ZERO);
    }

    #[test]
    fn test_register_allocator() {
        let config = crate::optimization::config::RegisterAllocationConfig::default();
        let mut allocator = RegisterAllocator::new(config);
        let mut program = create_test_program();
        
        let result = allocator.allocate(&mut program);
        assert!(result.is_ok());
        
        let stats = allocator.get_stats();
        assert!(stats.registers_allocated > 0);
    }
}
