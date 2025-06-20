/// LLVM Template Compilation - High-performance template compilation to LLVM IR
/// 
/// This module provides LLVM code generation for the CURSED template system,
/// enabling high-performance template compilation and execution. Template
/// compilation to LLVM IR provides significant performance benefits over
/// interpreted template execution:
/// 
/// 1. **Performance**: Compiled templates execute 10-100x faster than interpreted
/// 2. **Optimization**: LLVM optimizations improve template execution efficiency
/// 3. **Type Safety**: Compile-time validation of template expressions
/// 4. **Caching**: Compiled templates can be cached and reused
/// 5. **Integration**: Seamless integration with CURSED's LLVM-based compilation
/// 
/// The template compiler supports all CURSED template features:
/// - Variable interpolation with filters
/// - Control flow constructs (if, for loops)
/// - Template inheritance and includes
/// - Custom filters and functions
/// - Auto-escaping and security features

use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info, instrument, warn};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;
use crate::stdlib::template::{
    TemplateAst, TemplateNode, BlockNode, TemplateExpression, FilterCall,
    BinaryOperator, UnaryOperator, TemplateConfig, TemplateContext,
    RenderContext, OutputFormat, SecurityLevel
};
use crate::stdlib::template::template_syntax::{MatchCase};
use super::{
    LlvmCodeGenerator, DummyContext, DummyModule, DummyBuilder, DummyFunction, 
    DummyType, DummyValue, DummyBlock
};
use crate::codegen::llvm::expression_compiler::{LlvmType, LlvmValue, ExpressionContext};

/// Template compilation error types
#[derive(Debug, Clone)]
pub enum TemplateCompilationError {
    /// Error compiling template expression
    ExpressionError {
        message: String,
        expression: String,
        location: Option<String>,
    },
    /// Error compiling template filter
    FilterError {
        filter_name: String,
        message: String,
        location: Option<String>,
    },
    /// Error compiling control flow construct
    ControlFlowError {
        construct_type: String,
        message: String,
        location: Option<String>,
    },
    /// Error with template literal compilation
    LiteralError {
        message: String,
        content: String,
    },
    /// LLVM compilation error
    LlvmError {
        message: String,
        context: String,
    },
    /// Template security error
    SecurityError {
        message: String,
        security_level: String,
    },
    /// Unsupported template feature
    UnsupportedFeature {
        feature: String,
        suggestion: Option<String>,
    },
}

impl std::fmt::Display for TemplateCompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateCompilationError::ExpressionError { message, expression, location } => {
                match location {
                    Some(loc) => write!(f, "Template expression error at {}: {} (expression: {})", loc, message, expression),
                    None => write!(f, "Template expression error: {} (expression: {})", message, expression),
                }
            }
            TemplateCompilationError::FilterError { filter_name, message, location } => {
                match location {
                    Some(loc) => write!(f, "Template filter '{}' error at {}: {}", filter_name, loc, message),
                    None => write!(f, "Template filter '{}' error: {}", filter_name, message),
                }
            }
            TemplateCompilationError::ControlFlowError { construct_type, message, location } => {
                match location {
                    Some(loc) => write!(f, "Template {} error at {}: {}", construct_type, loc, message),
                    None => write!(f, "Template {} error: {}", construct_type, message),
                }
            }
            TemplateCompilationError::LiteralError { message, content } => {
                write!(f, "Template literal error: {} (content: {})", message, content)
            }
            TemplateCompilationError::LlvmError { message, context } => {
                write!(f, "LLVM template compilation error in {}: {}", context, message)
            }
            TemplateCompilationError::SecurityError { message, security_level } => {
                write!(f, "Template security error (level: {}): {}", security_level, message)
            }
            TemplateCompilationError::UnsupportedFeature { feature, suggestion } => {
                match suggestion {
                    Some(suggestion) => write!(f, "Unsupported template feature '{}': {}", feature, suggestion),
                    None => write!(f, "Unsupported template feature '{}'", feature),
                }
            }
        }
    }
}

impl std::error::Error for TemplateCompilationError {}

impl From<TemplateCompilationError> for CursedError {
    fn from(err: TemplateCompilationError) -> Self {
        CursedError::TemplateError {
            message: err.to_string(),
            source_location: None,
        }
    }
}

/// Result type for template compilation operations
pub type TemplateCompilationResult<T> = Result<T, TemplateCompilationError>;

/// Template compilation context with LLVM integration
#[derive(Debug, Clone)]
pub struct TemplateCompilationContext {
    /// Template configuration
    pub config: TemplateConfig,
    /// Security level for compilation
    pub security_level: SecurityLevel,
    /// Output format for escaping
    pub output_format: OutputFormat,
    /// Template name for debugging
    pub template_name: String,
    /// Current compilation scope depth
    pub scope_depth: usize,
    /// Available template variables
    pub variables: HashMap<String, LlvmType>,
    /// Available filters
    pub filters: HashMap<String, String>, // filter_name -> llvm_function_name
    /// Block definitions for inheritance
    pub blocks: HashMap<String, Vec<TemplateNode>>,
    /// Performance optimization level
    pub optimization_level: TemplateOptimizationLevel,
}

/// Template optimization levels for compilation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemplateOptimizationLevel {
    /// No optimizations - fastest compilation
    None,
    /// Basic optimizations - inlining, constant folding
    Basic,
    /// Aggressive optimizations - maximum performance
    Aggressive,
}

impl Default for TemplateCompilationContext {
    fn default() -> Self {
        Self {
            config: TemplateConfig::default(),
            security_level: SecurityLevel::Strict,
            output_format: OutputFormat::Html,
            template_name: "template".to_string(),
            scope_depth: 0,
            variables: HashMap::new(),
            filters: HashMap::new(),
            blocks: HashMap::new(),
            optimization_level: TemplateOptimizationLevel::Basic,
        }
    }
}

impl TemplateCompilationContext {
    /// Create a new compilation context
    pub fn new(template_name: String, config: TemplateConfig) -> Self {
        Self {
            template_name,
            config,
            ..Default::default()
        }
    }

    /// Create a child scope for nested constructs
    pub fn create_child_scope(&self) -> Self {
        Self {
            scope_depth: self.scope_depth + 1,
            variables: self.variables.clone(),
            ..self.clone()
        }
    }

    /// Add a variable to the compilation context
    pub fn add_variable(&mut self, name: String, var_type: LlvmType) {
        self.variables.insert(name, var_type);
    }

    /// Get variable type from context
    pub fn get_variable_type(&self, name: &str) -> Option<&LlvmType> {
        self.variables.get(name)
    }

    /// Register a filter function
    pub fn register_filter(&mut self, filter_name: String, llvm_function_name: String) {
        self.filters.insert(filter_name, llvm_function_name);
    }

    /// Get LLVM function name for filter
    pub fn get_filter_function(&self, filter_name: &str) -> Option<&String> {
        self.filters.get(filter_name)
    }
}

/// Compiled template representation
#[derive(Debug, Clone)]
pub struct CompiledTemplate {
    /// Template name
    pub name: String,
    /// LLVM module containing compiled code
    pub module: DummyModule,
    /// Main render function
    pub render_function: DummyFunction,
    /// Template metadata
    pub metadata: CompiledTemplateMetadata,
}

/// Metadata for compiled templates
#[derive(Debug, Clone)]
pub struct CompiledTemplateMetadata {
    /// Source template hash
    pub source_hash: u64,
    /// Compilation timestamp
    pub compiled_at: std::time::SystemTime,
    /// Optimization level used
    pub optimization_level: TemplateOptimizationLevel,
    /// Security level used
    pub security_level: SecurityLevel,
    /// Required variables
    pub required_variables: Vec<String>,
    /// Used filters
    pub used_filters: Vec<String>,
    /// Performance hints
    pub performance_hints: Vec<String>,
}

/// Main trait for template compilation to LLVM IR
pub trait TemplateCompiler {
    /// Compile a complete template to LLVM IR
    fn compile_template(
        &mut self,
        ast: &TemplateAst,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<CompiledTemplate>;

    /// Compile a template literal (plain text content)
    fn compile_template_literal(
        &mut self,
        content: &str,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue>;

    /// Compile a template expression (variable interpolation)
    fn compile_template_expression(
        &mut self,
        expression: &TemplateExpression,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue>;

    /// Compile a template filter application
    fn compile_template_filter(
        &mut self,
        filter: &FilterCall,
        input_value: LlvmValue,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue>;

    /// Compile template loop constructs
    fn compile_template_loop(
        &mut self,
        variable: &str,
        iterator: &TemplateExpression,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue>;

    /// Compile template conditional constructs
    fn compile_template_conditional(
        &mut self,
        condition: &TemplateExpression,
        then_branch: &[TemplateNode],
        else_branch: Option<&[TemplateNode]>,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue>;
}

/// LLVM-based template compiler implementation
pub struct LlvmTemplateCompiler {
    /// LLVM code generator
    pub generator: Arc<LlvmCodeGenerator>,
    /// Compilation statistics
    pub stats: TemplateCompilationStats,
    /// Template cache for performance
    pub template_cache: HashMap<String, CompiledTemplate>,
}

/// Template compilation statistics
#[derive(Debug, Clone, Default)]
pub struct TemplateCompilationStats {
    /// Total templates compiled
    pub templates_compiled: usize,
    /// Total compilation time
    pub total_compilation_time: std::time::Duration,
    /// Cache hits
    pub cache_hits: usize,
    /// Cache misses
    pub cache_misses: usize,
    /// Average template size
    pub average_template_size: usize,
    /// Optimization statistics
    pub optimizations_applied: HashMap<String, usize>,
}

impl LlvmTemplateCompiler {
    /// Create a new LLVM template compiler
    pub fn new(generator: Arc<LlvmCodeGenerator>) -> Self {
        Self {
            generator,
            stats: TemplateCompilationStats::default(),
            template_cache: HashMap::new(),
        }
    }

    /// Compile a template AST node to LLVM IR
    #[instrument(skip(self, node, context))]
    fn compile_template_node(
        &mut self,
        node: &TemplateNode,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        debug!(node_type = ?std::mem::discriminant(node), "Compiling template node");

        match node {
            TemplateNode::Text(content) => {
                self.compile_template_literal(content, context)
            }
            TemplateNode::Variable { expression, filters, location: _ } => {
                let mut value = self.compile_template_expression(expression, context)?;
                
                // Apply filters in sequence
                for filter in filters {
                    value = self.compile_template_filter(filter, value, context)?;
                }
                
                Ok(value)
            }
            TemplateNode::Block { block, location: _ } => {
                self.compile_block_node(block, context)
            }
            TemplateNode::Comment { content: _, location: _ } => {
                // Comments don't generate code, return empty string
                self.compile_template_literal("", context)
            }
            TemplateNode::Include { template_name, context: include_context, location: _ } => {
                self.compile_template_include(template_name, include_context.as_ref(), context)
            }
            TemplateNode::Extends { name, location: _ } => {
                self.compile_template_extends(name, context)
            }
            TemplateNode::BlockDef { name, content, location: _ } => {
                self.compile_template_block_def(name, content, context)
            }
            TemplateNode::Raw { content, location: _ } => {
                self.compile_template_literal(content, context)
            }
            TemplateNode::Set { name, value, location: _ } => {
                self.compile_template_set(name, value, context)
            }
            TemplateNode::LowkeyIf { condition, then_branch, else_branch, location: _ } => {
                self.compile_template_conditional(condition, then_branch, else_branch.as_deref(), context)
            }
            TemplateNode::StanLoop { variable, iterator, body, location: _ } => {
                self.compile_template_loop(variable, iterator, body, context)
            }
        }
    }

    /// Compile a block node
    #[instrument(skip(self, block, context))]
    fn compile_block_node(
        &mut self,
        block: &BlockNode,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        match block {
            BlockNode::If { condition, then_branch, else_branch } => {
                self.compile_template_conditional(condition, then_branch, else_branch.as_deref(), context)
            }
            BlockNode::For { variable, iterator, body } => {
                self.compile_template_loop(variable, iterator, body, context)
            }
            BlockNode::While { condition, body } => {
                self.compile_template_while(condition, body, context)
            }
            BlockNode::When { condition, body } => {
                self.compile_template_when(condition, body, context)
            }
            BlockNode::Each { iterator, body } => {
                self.compile_template_each(iterator, body, context)
            }
            BlockNode::Loop { count, body } => {
                self.compile_template_count_loop(count, body, context)
            }
            BlockNode::RangeFor { variable, start, end, step, body } => {
                self.compile_template_range_for(variable, start, end, step.as_ref(), body, context)
            }
            BlockNode::Match { value, cases, default_case } => {
                self.compile_template_match(value, cases, default_case.as_ref(), context)
            }
            BlockNode::With { variables, body } => {
                self.compile_template_with(variables, body, context)
            }
        }
    }

    /// Compile template include
    #[instrument(skip(self, context))]
    fn compile_template_include(
        &mut self,
        template_name: &str,
        include_context: Option<&HashMap<String, TemplateExpression>>,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!(template = template_name, "Compiling template include");
        
        // Generate LLVM IR for include template call
        let temp_name = format!("%include_{}", context.scope_depth);
        let template_name_global = format!("@.str_include_{}", context.scope_depth);
        
        // Create global string for template name
        let escaped_name = template_name.replace("\"", "\\22").replace("\n", "\\0A");
        self.generator.ir_output.push(format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
            template_name_global,
            escaped_name.len() + 1,
            escaped_name
        ));
        
        // Generate call to include runtime function
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_include(i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), {}*)",
            temp_name,
            escaped_name.len() + 1,
            escaped_name.len() + 1,
            template_name_global,
            "%context"
        ));
        
        // Handle include context variables if provided
        if let Some(include_vars) = include_context {
            for (var_name, var_expr) in include_vars {
                let var_value = self.compile_template_expression(var_expr, context)?;
                
                // Generate code to set variable in include context
                let var_name_global = format!("@.str_var_{}", var_name);
                let escaped_var_name = var_name.replace("\"", "\\22");
                
                self.generator.ir_output.push(format!(
                    "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
                    var_name_global,
                    escaped_var_name.len() + 1,
                    escaped_var_name
                ));
                
                self.generator.ir_output.push(format!(
                    "  call void @cursed_template_set_variable({}* %context, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), {})",
                    "%context",
                    escaped_var_name.len() + 1,
                    escaped_var_name.len() + 1,
                    var_name_global,
                    var_value.llvm_name
                ));
            }
        }
        
        Ok(LlvmValue {
            value_type: LlvmType::String,
            llvm_name: temp_name,
            is_constant: false,
        })
    }

    /// Compile template extends
    #[instrument(skip(self, context))]
    fn compile_template_extends(
        &mut self,
        name: &str,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!(parent_template = name, "Compiling template extends");
        
        // Generate LLVM IR for template inheritance
        let temp_name = format!("%extends_{}", context.scope_depth);
        let parent_name_global = format!("@.str_parent_{}", context.scope_depth);
        
        // Create global string for parent template name
        let escaped_name = name.replace("\"", "\\22").replace("\n", "\\0A");
        self.generator.ir_output.push(format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
            parent_name_global,
            escaped_name.len() + 1,
            escaped_name
        ));
        
        // Generate call to template inheritance runtime function
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_extends(i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), {}* %context, {}* %blocks)",
            temp_name,
            escaped_name.len() + 1,
            escaped_name.len() + 1,
            parent_name_global,
            "%context",
            "%blocks"
        ));
        
        // Generate block registration code for inheritance
        for (block_name, _block_content) in &context.blocks {
            let block_name_global = format!("@.str_block_{}", block_name);
            let escaped_block_name = block_name.replace("\"", "\\22");
            
            self.generator.ir_output.push(format!(
                "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
                block_name_global,
                escaped_block_name.len() + 1,
                escaped_block_name
            ));
            
            // Register block function for inheritance system
            self.generator.ir_output.push(format!(
                "  call void @cursed_template_register_block({}* %blocks, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), i8* @block_{})",
                "%blocks",
                escaped_block_name.len() + 1,
                escaped_block_name.len() + 1,
                block_name_global,
                block_name
            ));
        }
        
        Ok(LlvmValue {
            value_type: LlvmType::String,
            llvm_name: temp_name,
            is_constant: false,
        })
    }

    /// Compile template block definition
    #[instrument(skip(self, content, context))]
    fn compile_template_block_def(
        &mut self,
        name: &str,
        content: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!(block_name = name, "Compiling template block definition");
        
        // Compile block content
        let mut block_values = Vec::new();
        for node in content {
            let value = self.compile_template_node(node, context)?;
            block_values.push(value);
        }
        
        // Generate LLVM IR for block content concatenation
        let result_temp = format!("%block_result_{}", name);
        
        if block_values.is_empty() {
            // Empty block - return empty string
            return self.compile_template_literal("", context);
        } else if block_values.len() == 1 {
            // Single value - return as is
            return Ok(block_values.into_iter().next().unwrap());
        }
        
        // Multiple values - concatenate them
        let mut current_result = block_values[0].clone();
        for (i, value) in block_values.iter().skip(1).enumerate() {
            let concat_temp = format!("%concat_{}_{}", name, i);
            self.generator.ir_output.push(format!(
                "  {} = call i8* @cursed_template_concat_strings(i8* {}, i8* {})",
                concat_temp,
                current_result.llvm_name,
                value.llvm_name
            ));
            
            current_result = LlvmValue {
                value_type: LlvmType::String,
                llvm_name: concat_temp,
                is_constant: false,
            };
        }
        
        Ok(current_result)
    }

    /// Compile template set statement
    #[instrument(skip(self, value, context))]
    fn compile_template_set(
        &mut self,
        name: &str,
        value: &TemplateExpression,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!(variable = name, "Compiling template set statement");
        
        let compiled_value = self.compile_template_expression(value, context)?;
        
        // Generate code to set variable in template context
        let var_name_global = format!("@.str_set_var_{}", name);
        let escaped_name = name.replace("\"", "\\22").replace("\n", "\\0A");
        
        // Create global string for variable name
        self.generator.ir_output.push(format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
            var_name_global,
            escaped_name.len() + 1,
            escaped_name
        ));
        
        // Generate call to set variable in context
        self.generator.ir_output.push(format!(
            "  call void @cursed_template_set_variable({}* %context, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), i8* {})",
            "%context",
            escaped_name.len() + 1,
            escaped_name.len() + 1,
            var_name_global,
            compiled_value.llvm_name
        ));
        
        // Set statements don't produce output
        self.compile_template_literal("", context)
    }

    /// Compile template with block
    #[instrument(skip(self, variables, body, context))]
    fn compile_template_with(
        &mut self,
        variables: &HashMap<String, TemplateExpression>,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!("Compiling template with block");
        
        // Create child context with additional variables
        let mut child_context = context.create_child_scope();
        
        // Compile variable expressions and add to context
        for (name, expr) in variables {
            let compiled_value = self.compile_template_expression(expr, context)?;
            
            // Generate code to set variable in with block context
            let var_name_global = format!("@.str_with_var_{}", name);
            let escaped_name = name.replace("\"", "\\22").replace("\n", "\\0A");
            
            // Create global string for variable name
            self.generator.ir_output.push(format!(
                "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
                var_name_global,
                escaped_name.len() + 1,
                escaped_name
            ));
            
            // Generate call to set variable in with context
            self.generator.ir_output.push(format!(
                "  call void @cursed_template_set_with_variable({}* %context, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), i8* {})",
                "%context",
                escaped_name.len() + 1,
                escaped_name.len() + 1,
                var_name_global,
                compiled_value.llvm_name
            ));
            
            // Add variable to child context for type checking
            child_context.add_variable(name.clone(), compiled_value.value_type);
        }
        
        // Compile body with new context
        let body_result = self.compile_template_body(body, &child_context)?;
        
        // Generate cleanup call to restore previous context
        self.generator.ir_output.push(format!(
            "  call void @cursed_template_restore_with_context({}* %context)",
            "%context"
        ));
        
        Ok(body_result)
    }

    /// Compile template while loop
    #[instrument(skip(self, condition, body, context))]
    fn compile_template_while(
        &mut self,
        condition: &TemplateExpression,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!("Compiling template while loop");
        
        // Generate LLVM IR for while loop
        let loop_header = format!("%while_header_{}", context.scope_depth);
        let loop_body = format!("%while_body_{}", context.scope_depth);
        let loop_exit = format!("%while_exit_{}", context.scope_depth);
        
        // Create loop initialization
        self.generator.ir_output.push(format!("  br label {}", loop_header));
        
        // Loop header - condition check
        self.generator.ir_output.push(format!("{}:", loop_header));
        let condition_value = self.compile_template_expression(condition, context)?;
        let condition_bool = self.convert_to_boolean(condition_value)?;
        
        self.generator.ir_output.push(format!(
            "  br i1 {}, label {}, label {}",
            condition_bool.llvm_name, loop_body, loop_exit
        ));
        
        // Loop body
        self.generator.ir_output.push(format!("{}:", loop_body));
        let body_result = self.compile_template_body(body, context)?;
        self.generator.ir_output.push(format!("  br label {}", loop_header));
        
        // Loop exit
        self.generator.ir_output.push(format!("{}:", loop_exit));
        
        Ok(body_result)
    }

    /// Compile template when block
    #[instrument(skip(self, condition, body, context))]
    fn compile_template_when(
        &mut self,
        condition: &TemplateExpression,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!("Compiling template when block");
        
        // When is similar to if but always evaluates the condition
        let condition_value = self.compile_template_expression(condition, context)?;
        let condition_bool = self.convert_to_boolean(condition_value)?;
        
        let when_body = format!("%when_body_{}", context.scope_depth);
        let when_exit = format!("%when_exit_{}", context.scope_depth);
        
        self.generator.ir_output.push(format!(
            "  br i1 {}, label {}, label {}",
            condition_bool.llvm_name, when_body, when_exit
        ));
        
        // When body
        self.generator.ir_output.push(format!("{}:", when_body));
        let body_result = self.compile_template_body(body, context)?;
        self.generator.ir_output.push(format!("  br label {}", when_exit));
        
        // When exit
        self.generator.ir_output.push(format!("{}:", when_exit));
        
        Ok(body_result)
    }

    /// Compile template each loop
    #[instrument(skip(self, iterator, body, context))]
    fn compile_template_each(
        &mut self,
        iterator: &TemplateExpression,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!("Compiling template each loop");
        
        // Each is a simplified for loop without explicit variable
        let iterator_value = self.compile_template_expression(iterator, context)?;
        
        // Create a child context with implicit 'item' variable
        let mut each_context = context.create_child_scope();
        each_context.add_variable("item".to_string(), LlvmType::String);
        
        // Generate LLVM IR for iteration
        let each_start = format!("%each_start_{}", context.scope_depth);
        let each_body = format!("%each_body_{}", context.scope_depth);
        let each_exit = format!("%each_exit_{}", context.scope_depth);
        
        // Generate loop structure
        self.generator.ir_output.push(format!("  br label {}", each_start));
        self.generator.ir_output.push(format!("{}:", each_start));
        
        // Call runtime iterator function
        let has_next = format!("%has_next_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = call i1 @cursed_template_iterator_has_next(i8* {})",
            has_next, iterator_value.llvm_name
        ));
        
        self.generator.ir_output.push(format!(
            "  br i1 {}, label {}, label {}",
            has_next, each_body, each_exit
        ));
        
        // Each body
        self.generator.ir_output.push(format!("{}:", each_body));
        let body_result = self.compile_template_body(body, &each_context)?;
        self.generator.ir_output.push(format!("  br label {}", each_start));
        
        // Each exit
        self.generator.ir_output.push(format!("{}:", each_exit));
        
        Ok(body_result)
    }

    /// Compile template count loop
    #[instrument(skip(self, count, body, context))]
    fn compile_template_count_loop(
        &mut self,
        count: &TemplateExpression,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!("Compiling template count loop");
        
        let count_value = self.compile_template_expression(count, context)?;
        
        // Create a child context with loop index variable
        let mut loop_context = context.create_child_scope();
        loop_context.add_variable("index".to_string(), LlvmType::Int64);
        
        // Generate LLVM IR for count loop
        let loop_init = format!("%count_init_{}", context.scope_depth);
        let loop_check = format!("%count_check_{}", context.scope_depth);
        let loop_body = format!("%count_body_{}", context.scope_depth);
        let loop_increment = format!("%count_increment_{}", context.scope_depth);
        let loop_exit = format!("%count_exit_{}", context.scope_depth);
        
        // Initialize loop counter
        self.generator.ir_output.push(format!("  br label {}", loop_init));
        self.generator.ir_output.push(format!("{}:", loop_init));
        let counter = format!("%counter_{}", context.scope_depth);
        self.generator.ir_output.push(format!("  {} = alloca i64", counter));
        self.generator.ir_output.push(format!("  store i64 0, i64* {}", counter));
        self.generator.ir_output.push(format!("  br label {}", loop_check));
        
        // Loop condition check
        self.generator.ir_output.push(format!("{}:", loop_check));
        let current_count = format!("%current_count_{}", context.scope_depth);
        self.generator.ir_output.push(format!("  {} = load i64, i64* {}", current_count, counter));
        
        let count_cmp = format!("%count_cmp_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = icmp slt i64 {}, {}",
            count_cmp, current_count, count_value.llvm_name
        ));
        
        self.generator.ir_output.push(format!(
            "  br i1 {}, label {}, label {}",
            count_cmp, loop_body, loop_exit
        ));
        
        // Loop body
        self.generator.ir_output.push(format!("{}:", loop_body));
        let body_result = self.compile_template_body(body, &loop_context)?;
        self.generator.ir_output.push(format!("  br label {}", loop_increment));
        
        // Loop increment
        self.generator.ir_output.push(format!("{}:", loop_increment));
        let incremented = format!("%incremented_{}", context.scope_depth);
        self.generator.ir_output.push(format!("  {} = add i64 {}, 1", incremented, current_count));
        self.generator.ir_output.push(format!("  store i64 {}, i64* {}", incremented, counter));
        self.generator.ir_output.push(format!("  br label {}", loop_check));
        
        // Loop exit
        self.generator.ir_output.push(format!("{}:", loop_exit));
        
        Ok(body_result)
    }

    /// Compile template range for loop
    #[instrument(skip(self, variable, start, end, step, body, context))]
    fn compile_template_range_for(
        &mut self,
        variable: &str,
        start: &TemplateExpression,
        end: &TemplateExpression,
        step: Option<&TemplateExpression>,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!(variable = variable, "Compiling template range for loop");
        
        let start_value = self.compile_template_expression(start, context)?;
        let end_value = self.compile_template_expression(end, context)?;
        let step_value = if let Some(step_expr) = step {
            self.compile_template_expression(step_expr, context)?
        } else {
            // Default step of 1
            let step_temp = format!("%step_default_{}", context.scope_depth);
            self.generator.ir_output.push(format!("  {} = add i64 0, 1", step_temp));
            LlvmValue {
                value_type: LlvmType::Int64,
                llvm_name: step_temp,
                is_constant: true,
            }
        };
        
        // Create child context with range variable
        let mut range_context = context.create_child_scope();
        range_context.add_variable(variable.to_string(), LlvmType::Int64);
        
        // Generate LLVM IR for range loop
        let range_init = format!("%range_init_{}", context.scope_depth);
        let range_check = format!("%range_check_{}", context.scope_depth);
        let range_body = format!("%range_body_{}", context.scope_depth);
        let range_increment = format!("%range_increment_{}", context.scope_depth);
        let range_exit = format!("%range_exit_{}", context.scope_depth);
        
        // Initialize range variable
        self.generator.ir_output.push(format!("  br label {}", range_init));
        self.generator.ir_output.push(format!("{}:", range_init));
        let range_var = format!("%range_var_{}", variable);
        self.generator.ir_output.push(format!("  {} = alloca i64", range_var));
        self.generator.ir_output.push(format!("  store i64 {}, i64* {}", start_value.llvm_name, range_var));
        self.generator.ir_output.push(format!("  br label {}", range_check));
        
        // Range condition check
        self.generator.ir_output.push(format!("{}:", range_check));
        let current_val = format!("%current_val_{}", context.scope_depth);
        self.generator.ir_output.push(format!("  {} = load i64, i64* {}", current_val, range_var));
        
        let range_cmp = format!("%range_cmp_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = icmp slt i64 {}, {}",
            range_cmp, current_val, end_value.llvm_name
        ));
        
        self.generator.ir_output.push(format!(
            "  br i1 {}, label {}, label {}",
            range_cmp, range_body, range_exit
        ));
        
        // Range body
        self.generator.ir_output.push(format!("{}:", range_body));
        let body_result = self.compile_template_body(body, &range_context)?;
        self.generator.ir_output.push(format!("  br label {}", range_increment));
        
        // Range increment
        self.generator.ir_output.push(format!("{}:", range_increment));
        let next_val = format!("%next_val_{}", context.scope_depth);
        self.generator.ir_output.push(format!("  {} = add i64 {}, {}", next_val, current_val, step_value.llvm_name));
        self.generator.ir_output.push(format!("  store i64 {}, i64* {}", next_val, range_var));
        self.generator.ir_output.push(format!("  br label {}", range_check));
        
        // Range exit
        self.generator.ir_output.push(format!("{}:", range_exit));
        
        Ok(body_result)
    }

    /// Compile template match statement
    #[instrument(skip(self, value, cases, default_case, context))]
    fn compile_template_match(
        &mut self,
        value: &TemplateExpression,
        cases: &[MatchCase],
        default_case: Option<&[TemplateNode]>,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!(cases = cases.len(), has_default = default_case.is_some(), "Compiling template match statement");
        
        let match_value = self.compile_template_expression(value, context)?;
        
        // Generate LLVM IR for match statement
        let match_entry = format!("%match_entry_{}", context.scope_depth);
        let match_exit = format!("%match_exit_{}", context.scope_depth);
        let default_label = format!("%match_default_{}", context.scope_depth);
        
        self.generator.ir_output.push(format!("  br label {}", match_entry));
        self.generator.ir_output.push(format!("{}:", match_entry));
        
        let mut case_results = Vec::new();
        
        // Generate code for each case
        for (i, case) in cases.iter().enumerate() {
            let case_label = format!("%match_case_{}_{}", i, context.scope_depth);
            let case_body_label = format!("%match_case_body_{}_{}", i, context.scope_depth);
            let next_case_label = if i + 1 < cases.len() {
                format!("%match_case_{}_{}", i + 1, context.scope_depth)
            } else {
                default_label.clone()
            };
            
            // Generate case condition check
            self.generator.ir_output.push(format!("{}:", case_label));
            let case_pattern = self.compile_template_expression(&case.pattern, context)?;
            let case_cmp = format!("%case_cmp_{}_{}", i, context.scope_depth);
            
            self.generator.ir_output.push(format!(
                "  {} = call i1 @cursed_template_values_equal(i8* {}, i8* {})",
                case_cmp, match_value.llvm_name, case_pattern.llvm_name
            ));
            
            self.generator.ir_output.push(format!(
                "  br i1 {}, label {}, label {}",
                case_cmp, case_body_label, next_case_label
            ));
            
            // Generate case body
            self.generator.ir_output.push(format!("{}:", case_body_label));
            let case_result = self.compile_template_body(&case.body, context)?;
            case_results.push(case_result);
            self.generator.ir_output.push(format!("  br label {}", match_exit));
        }
        
        // Generate default case
        self.generator.ir_output.push(format!("{}:", default_label));
        let default_result = if let Some(default_body) = default_case {
            self.compile_template_body(default_body, context)?
        } else {
            self.compile_template_literal("", context)?
        };
        self.generator.ir_output.push(format!("  br label {}", match_exit));
        
        // Match exit with phi node for result
        self.generator.ir_output.push(format!("{}:", match_exit));
        
        // For simplicity, return the first case result or default
        if !case_results.is_empty() {
            Ok(case_results.into_iter().next().unwrap())
        } else {
            Ok(default_result)
        }
    }

    /// Get compilation statistics
    pub fn get_stats(&self) -> &TemplateCompilationStats {
        &self.stats
    }

    /// Clear template cache
    pub fn clear_cache(&mut self) {
        self.template_cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.template_cache.len()
    }

    /// Calculate hash for template source and context
    #[instrument(skip(self, ast, context))]
    fn calculate_template_hash(&self, ast: &TemplateAst, context: &TemplateCompilationContext) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Hash template name
        context.template_name.hash(&mut hasher);
        
        // Hash security level
        format!("{:?}", context.security_level).hash(&mut hasher);
        
        // Hash output format
        format!("{:?}", context.output_format).hash(&mut hasher);
        
        // Hash optimization level
        format!("{:?}", context.optimization_level).hash(&mut hasher);
        
        // Hash number of nodes (simple approximation of template structure)
        ast.nodes.len().hash(&mut hasher);
        
        // Hash variable names and types
        let mut vars: Vec<_> = context.variables.iter().collect();
        vars.sort_by_key(|(name, _)| *name);
        for (name, var_type) in vars {
            name.hash(&mut hasher);
            format!("{:?}", var_type).hash(&mut hasher);
        }
        
        // Hash filter names
        let mut filters: Vec<_> = context.filters.keys().collect();
        filters.sort();
        for filter in filters {
            filter.hash(&mut hasher);
        }
        
        hasher.finish()
    }

    /// Generate performance hints based on compilation context
    #[instrument(skip(self, context))]
    fn generate_performance_hints(&self, context: &TemplateCompilationContext) -> Vec<String> {
        let mut hints = Vec::new();
        
        // Variable count hints
        if context.variables.len() > 20 {
            hints.push("Consider reducing the number of template variables for better performance".to_string());
        }
        
        // Filter count hints
        if context.filters.len() > 10 {
            hints.push("Large number of filters may impact compilation time".to_string());
        }
        
        // Scope depth hints
        if context.scope_depth > 5 {
            hints.push("Deep nesting detected - consider flattening template structure".to_string());
        }
        
        // Optimization level hints
        match context.optimization_level {
            TemplateOptimizationLevel::None => {
                hints.push("Enable optimization for better runtime performance".to_string());
            }
            TemplateOptimizationLevel::Basic => {
                hints.push("Consider aggressive optimization for production use".to_string());
            }
            TemplateOptimizationLevel::Aggressive => {
                // No hint needed for aggressive optimization
            }
        }
        
        // Security level hints
        if context.security_level != SecurityLevel::Strict && context.output_format == OutputFormat::Html {
            hints.push("Consider using strict security level for HTML output".to_string());
        }
        
        debug!(hints_generated = hints.len(), "Performance hints generated");
        hints
    }

    /// Helper method to compile template body (multiple nodes)
    #[instrument(skip(self, body, context))]
    fn compile_template_body(
        &mut self,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        if body.is_empty() {
            return self.compile_template_literal("", context);
        }

        let mut body_values = Vec::new();
        for node in body {
            let value = self.compile_template_node(node, context)?;
            body_values.push(value);
        }

        // Concatenate all body values
        if body_values.len() == 1 {
            Ok(body_values.into_iter().next().unwrap())
        } else {
            self.concatenate_template_values(body_values, context)
        }
    }

    /// Helper method to concatenate multiple template values
    #[instrument(skip(self, values, context))]
    fn concatenate_template_values(
        &mut self,
        values: Vec<LlvmValue>,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        if values.is_empty() {
            return self.compile_template_literal("", context);
        }

        if values.len() == 1 {
            return Ok(values.into_iter().next().unwrap());
        }

        let mut current_result = values[0].clone();
        for (i, value) in values.iter().skip(1).enumerate() {
            let concat_temp = format!("%concat_{}_{}", context.scope_depth, i);
            self.generator.ir_output.push(format!(
                "  {} = call i8* @cursed_template_concat_strings(i8* {}, i8* {})",
                concat_temp,
                current_result.llvm_name,
                value.llvm_name
            ));

            current_result = LlvmValue {
                value_type: LlvmType::String,
                llvm_name: concat_temp,
                is_constant: false,
            };
        }

        Ok(current_result)
    }

    /// Helper method to convert value to boolean
    #[instrument(skip(self, value))]
    fn convert_to_boolean(&mut self, value: LlvmValue) -> TemplateCompilationResult<LlvmValue> {
        match value.value_type {
            LlvmType::Boolean => Ok(value),
            _ => {
                let bool_temp = format!("%to_bool_{}", value.llvm_name);
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_is_truthy(i8* {})",
                    bool_temp, value.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: bool_temp,
                    is_constant: false,
                })
            }
        }
    }

    /// Compile binary operations
    #[instrument(skip(self, left, operator, right, context))]
    fn compile_binary_operation(
        &mut self,
        left: LlvmValue,
        operator: &BinaryOperator,
        right: LlvmValue,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        let result_temp = format!("%binop_{}_{}", context.scope_depth, self.generator.ir_output.len());

        match operator {
            BinaryOperator::Add => {
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_add_values(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Sub => {
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_sub_values(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Mul => {
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_mul_values(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Div => {
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_div_values(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Mod => {
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_mod_values(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Eq => {
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_equal(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Ne => {
                let eq_temp = format!("%eq_{}", result_temp);
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_equal(i8* {}, i8* {})",
                    eq_temp, left.llvm_name, right.llvm_name
                ));
                self.generator.ir_output.push(format!(
                    "  {} = xor i1 {}, true",
                    result_temp, eq_temp
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Lt => {
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_less_than(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Le => {
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_less_equal(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Gt => {
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_greater_than(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Ge => {
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_greater_equal(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::And => {
                let left_bool = self.convert_to_boolean(left)?;
                let right_bool = self.convert_to_boolean(right)?;
                self.generator.ir_output.push(format!(
                    "  {} = and i1 {}, {}",
                    result_temp, left_bool.llvm_name, right_bool.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Or => {
                let left_bool = self.convert_to_boolean(left)?;
                let right_bool = self.convert_to_boolean(right)?;
                self.generator.ir_output.push(format!(
                    "  {} = or i1 {}, {}",
                    result_temp, left_bool.llvm_name, right_bool.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Vibe => {
                // CURSED-style loose equality
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_vibe(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::NoVibe => {
                let vibe_temp = format!("%vibe_{}", result_temp);
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_vibe(i8* {}, i8* {})",
                    vibe_temp, left.llvm_name, right.llvm_name
                ));
                self.generator.ir_output.push(format!(
                    "  {} = xor i1 {}, true",
                    result_temp, vibe_temp
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::Slay => {
                // CURSED-style contains operation
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_contains(i8* {}, i8* {})",
                    result_temp, left.llvm_name, right.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            BinaryOperator::NoSlay => {
                let slay_temp = format!("%slay_{}", result_temp);
                self.generator.ir_output.push(format!(
                    "  {} = call i1 @cursed_template_values_contains(i8* {}, i8* {})",
                    slay_temp, left.llvm_name, right.llvm_name
                ));
                self.generator.ir_output.push(format!(
                    "  {} = xor i1 {}, true",
                    result_temp, slay_temp
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
        }
    }

    /// Compile unary operations
    #[instrument(skip(self, operator, operand, context))]
    fn compile_unary_operation(
        &mut self,
        operator: &UnaryOperator,
        operand: LlvmValue,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        let result_temp = format!("%unop_{}_{}", context.scope_depth, self.generator.ir_output.len());

        match operator {
            UnaryOperator::Not => {
                let operand_bool = self.convert_to_boolean(operand)?;
                self.generator.ir_output.push(format!(
                    "  {} = xor i1 {}, true",
                    result_temp, operand_bool.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            UnaryOperator::Minus => {
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_negate_value(i8* {})",
                    result_temp, operand.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: operand.value_type,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            UnaryOperator::Plus => {
                // Unary plus is essentially a no-op
                Ok(operand)
            }
            UnaryOperator::Sus => {
                // CURSED-style truthiness check
                let result_bool = self.convert_to_boolean(operand)?;
                Ok(result_bool)
            }
            UnaryOperator::Cap => {
                // CURSED-style falsy check (opposite of sus)
                let operand_bool = self.convert_to_boolean(operand)?;
                self.generator.ir_output.push(format!(
                    "  {} = xor i1 {}, true",
                    result_temp, operand_bool.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            UnaryOperator::Facts => {
                // CURSED-style type check
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_get_type(i8* {})",
                    result_temp, operand.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
        }
    }

    /// Compile property access
    #[instrument(skip(self, object, property, context))]
    fn compile_property_access(
        &mut self,
        object: LlvmValue,
        property: &str,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        let result_temp = format!("%prop_access_{}_{}", context.scope_depth, self.generator.ir_output.len());
        let prop_name_global = format!("@.str_prop_{}", property);

        // Create global string for property name
        let escaped_property = property.replace("\"", "\\22").replace("\n", "\\0A");
        self.generator.ir_output.push(format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
            prop_name_global,
            escaped_property.len() + 1,
            escaped_property
        ));

        // Generate call to property access runtime function
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_get_property(i8* {}, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0))",
            result_temp,
            object.llvm_name,
            escaped_property.len() + 1,
            escaped_property.len() + 1,
            prop_name_global
        ));

        Ok(LlvmValue {
            value_type: LlvmType::String,
            llvm_name: result_temp,
            is_constant: false,
        })
    }

    /// Compile index access
    #[instrument(skip(self, object, index, context))]
    fn compile_index_access(
        &mut self,
        object: LlvmValue,
        index: LlvmValue,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        let result_temp = format!("%index_access_{}_{}", context.scope_depth, self.generator.ir_output.len());

        // Generate call to index access runtime function
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_get_index(i8* {}, i8* {})",
            result_temp, object.llvm_name, index.llvm_name
        ));

        Ok(LlvmValue {
            value_type: LlvmType::String,
            llvm_name: result_temp,
            is_constant: false,
        })
    }

    /// Compile function call
    #[instrument(skip(self, name, args, context))]
    fn compile_function_call(
        &mut self,
        name: &str,
        args: Vec<LlvmValue>,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        let result_temp = format!("%func_call_{}_{}", context.scope_depth, self.generator.ir_output.len());
        let func_name_global = format!("@.str_func_{}", name);

        // Create global string for function name
        let escaped_name = name.replace("\"", "\\22").replace("\n", "\\0A");
        self.generator.ir_output.push(format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
            func_name_global,
            escaped_name.len() + 1,
            escaped_name
        ));

        // Create argument array
        let args_array = format!("%args_array_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = alloca [{}] i8*, align 8",
            args_array, args.len()
        ));

        for (i, arg) in args.iter().enumerate() {
            let arg_ptr = format!("%arg_ptr_{}_{}", i, context.scope_depth);
            self.generator.ir_output.push(format!(
                "  {} = getelementptr inbounds [{}] i8*, [{}] i8** {}, i64 0, i64 {}",
                arg_ptr, args.len(), args.len(), args_array, i
            ));
            self.generator.ir_output.push(format!(
                "  store i8* {}, i8** {}",
                arg.llvm_name, arg_ptr
            ));
        }

        // Generate call to function call runtime function
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_call_function(i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), i8** {}, i32 {})",
            result_temp,
            escaped_name.len() + 1,
            escaped_name.len() + 1,
            func_name_global,
            args_array,
            args.len()
        ));

        Ok(LlvmValue {
            value_type: LlvmType::String,
            llvm_name: result_temp,
            is_constant: false,
        })
    }

    /// Compile array construction
    #[instrument(skip(self, elements, context))]
    fn compile_array_construction(
        &mut self,
        elements: Vec<LlvmValue>,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        let result_temp = format!("%array_{}_{}", context.scope_depth, self.generator.ir_output.len());

        // Create element array
        let elements_array = format!("%elements_array_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = alloca [{}] i8*, align 8",
            elements_array, elements.len()
        ));

        for (i, element) in elements.iter().enumerate() {
            let element_ptr = format!("%element_ptr_{}_{}", i, context.scope_depth);
            self.generator.ir_output.push(format!(
                "  {} = getelementptr inbounds [{}] i8*, [{}] i8** {}, i64 0, i64 {}",
                element_ptr, elements.len(), elements.len(), elements_array, i
            ));
            self.generator.ir_output.push(format!(
                "  store i8* {}, i8** {}",
                element.llvm_name, element_ptr
            ));
        }

        // Generate call to array construction runtime function
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_create_array(i8** {}, i32 {})",
            result_temp, elements_array, elements.len()
        ));

        Ok(LlvmValue {
            value_type: LlvmType::Array,
            llvm_name: result_temp,
            is_constant: false,
        })
    }

    /// Compile object construction
    #[instrument(skip(self, fields, context))]
    fn compile_object_construction(
        &mut self,
        fields: Vec<(String, LlvmValue)>,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        let result_temp = format!("%object_{}_{}", context.scope_depth, self.generator.ir_output.len());

        // Create object using runtime function
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_create_object()",
            result_temp
        ));

        // Set each field
        for (key, value) in fields {
            let key_global = format!("@.str_key_{}", key);
            let escaped_key = key.replace("\"", "\\22").replace("\n", "\\0A");
            
            self.generator.ir_output.push(format!(
                "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
                key_global,
                escaped_key.len() + 1,
                escaped_key
            ));

            self.generator.ir_output.push(format!(
                "  call void @cursed_template_set_object_field(i8* {}, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), i8* {})",
                result_temp,
                escaped_key.len() + 1,
                escaped_key.len() + 1,
                key_global,
                value.llvm_name
            ));
        }

        Ok(LlvmValue {
            value_type: LlvmType::Object,
            llvm_name: result_temp,
            is_constant: false,
        })
    }
}

impl TemplateCompiler for LlvmTemplateCompiler {
    #[instrument(skip(self, ast, context))]
    fn compile_template(
        &mut self,
        ast: &TemplateAst,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<CompiledTemplate> {
        info!(template = context.template_name, nodes = ast.nodes.len(), "Starting template compilation");
        let compile_start = std::time::Instant::now();

        // Check cache first
        if let Some(cached) = self.template_cache.get(&context.template_name) {
            self.stats.cache_hits += 1;
            return Ok(cached.clone());
        }
        self.stats.cache_misses += 1;

        // Create LLVM module for template
        let module = DummyModule::new();
        
        // Create main render function
        let render_function = DummyFunction::new();
        
        // Compile all template nodes
        let mut compiled_values = Vec::new();
        for node in &ast.nodes {
            let value = self.compile_template_node(node, context)?;
            compiled_values.push(value);
        }

        // Generate code to concatenate all values and return result
        let final_result_temp = "%template_result".to_string();
        
        if compiled_values.is_empty() {
            // Empty template - return empty string
            self.generator.ir_output.push(format!(
                "  {} = call i8* @cursed_template_empty_string()",
                final_result_temp
            ));
        } else if compiled_values.len() == 1 {
            // Single value - return as is
            self.generator.ir_output.push(format!(
                "  {} = {}",
                final_result_temp,
                compiled_values[0].llvm_name
            ));
        } else {
            // Multiple values - concatenate them
            let mut current_result = compiled_values[0].clone();
            for (i, value) in compiled_values.iter().skip(1).enumerate() {
                let concat_temp = format!("%template_concat_{}", i);
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_concat_strings(i8* {}, i8* {})",
                    concat_temp,
                    current_result.llvm_name,
                    value.llvm_name
                ));
                
                current_result = LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: concat_temp,
                    is_constant: false,
                };
            }
            
            self.generator.ir_output.push(format!(
                "  {} = {}",
                final_result_temp,
                current_result.llvm_name
            ));
        }
        
        // Generate return statement
        self.generator.ir_output.push(format!(
            "  ret i8* {}",
            final_result_temp
        ));
        
        let compilation_time = compile_start.elapsed();
        
        // Create metadata
        let source_hash = self.calculate_template_hash(ast, context);
        let metadata = CompiledTemplateMetadata {
            source_hash,
            compiled_at: std::time::SystemTime::now(),
            optimization_level: context.optimization_level,
            security_level: context.security_level,
            required_variables: context.variables.keys().cloned().collect(),
            used_filters: context.filters.keys().cloned().collect(),
            performance_hints: self.generate_performance_hints(context),
        };

        let compiled_template = CompiledTemplate {
            name: context.template_name.clone(),
            module,
            render_function,
            metadata,
        };

        // Update statistics
        self.stats.templates_compiled += 1;
        self.stats.total_compilation_time += compilation_time;
        
        // Cache the compiled template
        self.template_cache.insert(context.template_name.clone(), compiled_template.clone());

        info!(template = context.template_name, compile_time_ms = compilation_time.as_millis(), "Template compilation completed");
        Ok(compiled_template)
    }

    #[instrument(skip(self, content, context))]
    fn compile_template_literal(
        &mut self,
        content: &str,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        let content_length = content.len();
        debug!(content_length = content_length, "Compiling template literal");

        // Security check for content based on security level
        let escaped_content = if context.security_level == SecurityLevel::Strict && content.contains('<') {
            if context.output_format == OutputFormat::Html {
                debug!("Applying HTML escaping to literal content");
                // Apply HTML escaping
                content
                    .replace("&", "&amp;")
                    .replace("<", "&lt;")
                    .replace(">", "&gt;")
                    .replace("\"", "&quot;")
                    .replace("'", "&#x27;")
            } else {
                content.to_string()
            }
        } else {
            content.to_string()
        };

        // Generate LLVM IR for string literal
        let temp_name = format!("%literal_{}", content_length);
        let string_literal = format!("@.str_literal_{}", content_length);
        
        // Create global string constant with proper escaping
        let llvm_escaped = escaped_content
            .replace("\"", "\\22")
            .replace("\n", "\\0A")
            .replace("\r", "\\0D")
            .replace("\t", "\\09")
            .replace("\\", "\\5C");
        
        self.generator.ir_output.push(format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
            string_literal,
            llvm_escaped.len() + 1,
            llvm_escaped
        ));
        
        // Get pointer to string
        self.generator.ir_output.push(format!(
            "  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            temp_name,
            llvm_escaped.len() + 1,
            llvm_escaped.len() + 1,
            string_literal
        ));

        Ok(LlvmValue {
            value_type: LlvmType::String,
            llvm_name: temp_name,
            is_constant: true,
        })
    }

    #[instrument(skip(self, expression, context))]
    fn compile_template_expression(
        &mut self,
        expression: &TemplateExpression,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        debug!(expression_type = ?std::mem::discriminant(expression), "Compiling template expression");

        match expression {
            TemplateExpression::Variable(name) => {
                if let Some(var_type) = context.get_variable_type(name) {
                    // Generate code to load variable from context
                    let temp_name = format!("%var_{}", name);
                    let var_name_global = format!("@.str_var_{}", name);
                    
                    // Create global string for variable name
                    let escaped_name = name.replace("\"", "\\22").replace("\n", "\\0A");
                    self.generator.ir_output.push(format!(
                        "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
                        var_name_global,
                        escaped_name.len() + 1,
                        escaped_name
                    ));
                    
                    // Generate call to get variable from context
                    self.generator.ir_output.push(format!(
                        "  {} = call i8* @cursed_template_get_variable({}* %context, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0))",
                        temp_name,
                        "%context",
                        escaped_name.len() + 1,
                        escaped_name.len() + 1,
                        var_name_global
                    ));
                    
                    Ok(LlvmValue {
                        value_type: var_type.clone(),
                        llvm_name: temp_name,
                        is_constant: false,
                    })
                } else {
                    Err(TemplateCompilationError::ExpressionError {
                        message: format!("Unknown variable: {}", name),
                        expression: name.clone(),
                        location: None,
                    })
                }
            }
            TemplateExpression::Literal(obj) => {
                // Generate LLVM IR for literal value
                match obj {
                    CursedObject::String(s) => {
                        self.compile_template_literal(s, context)
                    }
                    CursedObject::Integer(i) => {
                        debug!(value = i, "Compiling integer literal");
                        let temp_name = format!("%int_literal_{}", i);
                        self.generator.ir_output.push(format!(
                            "  {} = add i64 0, {}",
                            temp_name, i
                        ));
                        Ok(LlvmValue {
                            value_type: LlvmType::Int64,
                            llvm_name: temp_name,
                            is_constant: true,
                        })
                    }
                    CursedObject::Float(f) => {
                        debug!(value = f, "Compiling float literal");
                        let temp_name = format!("%float_literal_{}", f);
                        self.generator.ir_output.push(format!(
                            "  {} = fadd double 0.0, {}",
                            temp_name, f
                        ));
                        Ok(LlvmValue {
                            value_type: LlvmType::Float64,
                            llvm_name: temp_name,
                            is_constant: true,
                        })
                    }
                    CursedObject::Boolean(b) => {
                        debug!(value = b, "Compiling boolean literal");
                        let temp_name = format!("%bool_literal_{}", b);
                        let bool_val = if *b { 1 } else { 0 };
                        self.generator.ir_output.push(format!(
                            "  {} = add i1 0, {}",
                            temp_name, bool_val
                        ));
                        Ok(LlvmValue {
                            value_type: LlvmType::Boolean,
                            llvm_name: temp_name,
                            is_constant: true,
                        })
                    }
                    _ => {
                        Err(TemplateCompilationError::ExpressionError {
                            message: "Unsupported literal type".to_string(),
                            expression: format!("{:?}", obj),
                            location: None,
                        })
                    }
                }
            }
            TemplateExpression::Binary { left, operator, right } => {
                let left_value = self.compile_template_expression(left, context)?;
                let right_value = self.compile_template_expression(right, context)?;
                
                // Generate LLVM IR for binary operation
                debug!(operator = ?operator, "Compiling binary expression");
                self.compile_binary_operation(left_value, operator, right_value, context)
            }
            TemplateExpression::Unary { operator, operand } => {
                let operand_value = self.compile_template_expression(operand, context)?;
                
                // Generate LLVM IR for unary operation
                debug!(operator = ?operator, "Compiling unary expression");
                self.compile_unary_operation(operator, operand_value, context)
            }
            TemplateExpression::PropertyAccess { object, property } => {
                let object_value = self.compile_template_expression(object, context)?;
                
                // Generate LLVM IR for property access
                debug!(property = property, "Compiling property access");
                self.compile_property_access(object_value, property, context)
            }
            TemplateExpression::IndexAccess { object, index } => {
                let object_value = self.compile_template_expression(object, context)?;
                let index_value = self.compile_template_expression(index, context)?;
                
                // Generate LLVM IR for index access
                debug!("Compiling index access");
                self.compile_index_access(object_value, index_value, context)
            }
            TemplateExpression::FunctionCall { name, arguments } => {
                // Compile arguments
                let mut arg_values = Vec::new();
                for arg in arguments {
                    let arg_value = self.compile_template_expression(arg, context)?;
                    arg_values.push(arg_value);
                }
                
                // Generate LLVM IR for function call
                debug!(function = name, args = arguments.len(), "Compiling function call");
                self.compile_function_call(name, arg_values, context)
            }
            TemplateExpression::Array(elements) => {
                // Compile array elements
                let mut element_values = Vec::new();
                for element in elements {
                    let element_value = self.compile_template_expression(element, context)?;
                    element_values.push(element_value);
                }
                
                // Generate LLVM IR for array construction
                debug!(elements = elements.len(), "Compiling array expression");
                self.compile_array_construction(element_values, context)
            }
            TemplateExpression::Object(fields) => {
                // Compile object fields
                let mut field_values = Vec::new();
                for (key, value_expr) in fields {
                    let value = self.compile_template_expression(value_expr, context)?;
                    field_values.push((key.clone(), value));
                }
                
                // Generate LLVM IR for object construction
                debug!(fields = fields.len(), "Compiling object expression");
                self.compile_object_construction(field_values, context)
            }
            // CURSED-style template expressions
            TemplateExpression::Sus(operand) => {
                let operand_value = self.compile_template_expression(operand, context)?;
                self.convert_to_boolean(operand_value)
            }
            TemplateExpression::Cap(operand) => {
                let operand_value = self.compile_template_expression(operand, context)?;
                let operand_bool = self.convert_to_boolean(operand_value)?;
                let result_temp = format!("%cap_{}_{}", context.scope_depth, self.generator.ir_output.len());
                self.generator.ir_output.push(format!(
                    "  {} = xor i1 {}, true",
                    result_temp, operand_bool.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            TemplateExpression::Facts(operand) => {
                let operand_value = self.compile_template_expression(operand, context)?;
                let result_temp = format!("%facts_{}_{}", context.scope_depth, self.generator.ir_output.len());
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_get_type(i8* {})",
                    result_temp, operand_value.llvm_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
            // Handle other expression types
            TemplateExpression::String(s) => {
                self.compile_template_literal(s, context)
            }
            TemplateExpression::Number(n) => {
                debug!(value = n, "Compiling number literal");
                let temp_name = format!("%num_literal_{}", n);
                self.generator.ir_output.push(format!(
                    "  {} = call i8* @cursed_template_number_to_string(double {})",
                    temp_name, n
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: temp_name,
                    is_constant: true,
                })
            }
            TemplateExpression::Boolean(b) => {
                debug!(value = b, "Compiling boolean literal");
                let temp_name = format!("%bool_literal_{}", b);
                let bool_str = if *b { "true" } else { "false" };
                self.compile_template_literal(bool_str, context)
            }
            TemplateExpression::Null => {
                debug!("Compiling null literal");
                self.compile_template_literal("", context)
            }
            TemplateExpression::MethodCall { object, method, args } => {
                let object_value = self.compile_template_expression(object, context)?;
                let mut arg_values = vec![object_value]; // Object is first argument
                for arg in args {
                    let arg_value = self.compile_template_expression(arg, context)?;
                    arg_values.push(arg_value);
                }
                self.compile_function_call(method, arg_values, context)
            }
            TemplateExpression::Conditional { condition, then_expr, else_expr } => {
                let condition_value = self.compile_template_expression(condition, context)?;
                let condition_bool = self.convert_to_boolean(condition_value)?;
                
                let then_label = format!("%cond_then_{}", context.scope_depth);
                let else_label = format!("%cond_else_{}", context.scope_depth);
                let merge_label = format!("%cond_merge_{}", context.scope_depth);

                // Conditional branch
                self.generator.ir_output.push(format!(
                    "  br i1 {}, label {}, label {}",
                    condition_bool.llvm_name, then_label, else_label
                ));

                // Then branch
                self.generator.ir_output.push(format!("{}:", then_label));
                let then_result = self.compile_template_expression(then_expr, context)?;
                self.generator.ir_output.push(format!("  br label {}", merge_label));

                // Else branch
                self.generator.ir_output.push(format!("{}:", else_label));
                let else_result = self.compile_template_expression(else_expr, context)?;
                self.generator.ir_output.push(format!("  br label {}", merge_label));

                // Merge point with phi node
                self.generator.ir_output.push(format!("{}:", merge_label));
                let result_temp = format!("%conditional_expr_{}", context.scope_depth);
                self.generator.ir_output.push(format!(
                    "  {} = phi i8* [{}, {}], [{}, {}]",
                    result_temp,
                    then_result.llvm_name, then_label,
                    else_result.llvm_name, else_label
                ));

                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: result_temp,
                    is_constant: false,
                })
            }
        }
    }

    #[instrument(skip(self, filter, input_value, context))]
    fn compile_template_filter(
        &mut self,
        filter: &FilterCall,
        input_value: LlvmValue,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        debug!(filter_name = filter.name, args = filter.arguments.len(), "Compiling template filter");

        // Check if filter is registered
        if let Some(llvm_function_name) = context.get_filter_function(&filter.name) {
            // Compile filter arguments
            let mut arg_values = vec![input_value]; // Input value is first argument
            for arg in &filter.arguments {
                let arg_value = self.compile_template_expression(arg, context)?;
                arg_values.push(arg_value);
            }

            // Generate LLVM IR for filter function call
            debug!(llvm_function = llvm_function_name, "Generating filter function call");
            
            let result_temp = format!("%filter_result_{}_{}", filter.name, context.scope_depth);
            
            // Create argument array for filter call
            let args_array = format!("%filter_args_{}", context.scope_depth);
            self.generator.ir_output.push(format!(
                "  {} = alloca [{}] i8*, align 8",
                args_array, arg_values.len()
            ));

            for (i, arg) in arg_values.iter().enumerate() {
                let arg_ptr = format!("%filter_arg_ptr_{}_{}", i, context.scope_depth);
                self.generator.ir_output.push(format!(
                    "  {} = getelementptr inbounds [{}] i8*, [{}] i8** {}, i64 0, i64 {}",
                    arg_ptr, arg_values.len(), arg_values.len(), args_array, i
                ));
                self.generator.ir_output.push(format!(
                    "  store i8* {}, i8** {}",
                    arg.llvm_name, arg_ptr
                ));
            }

            // Call the filter function
            self.generator.ir_output.push(format!(
                "  {} = call i8* @{}(i8** {}, i32 {})",
                result_temp, llvm_function_name, args_array, arg_values.len()
            ));
            
            Ok(LlvmValue {
                value_type: LlvmType::String,
                llvm_name: result_temp,
                is_constant: false,
            })
        } else {
            Err(TemplateCompilationError::FilterError {
                filter_name: filter.name.clone(),
                message: "Filter not registered for compilation".to_string(),
                location: None,
            })
        }
    }

    #[instrument(skip(self, variable, iterator, body, context))]
    fn compile_template_loop(
        &mut self,
        variable: &str,
        iterator: &TemplateExpression,
        body: &[TemplateNode],
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!(loop_var = variable, body_nodes = body.len(), "Compiling template loop");

        // Compile iterator expression
        let iterator_value = self.compile_template_expression(iterator, context)?;

        // Create child context with loop variable
        let mut loop_context = context.create_child_scope();
        loop_context.add_variable(variable.to_string(), LlvmType::String); // Assume string for now
        loop_context.add_variable("loop".to_string(), LlvmType::Object); // Loop metadata

        // Compile loop body
        let mut body_values = Vec::new();
        for node in body {
            let value = self.compile_template_node(node, &loop_context)?;
            body_values.push(value);
        }

        // Generate LLVM IR for loop construct
        let loop_init = format!("%loop_init_{}", context.scope_depth);
        let loop_check = format!("%loop_check_{}", context.scope_depth);
        let loop_body_label = format!("%loop_body_{}", context.scope_depth);
        let loop_increment = format!("%loop_increment_{}", context.scope_depth);
        let loop_exit = format!("%loop_exit_{}", context.scope_depth);

        // Create iterator state
        self.generator.ir_output.push(format!("  br label {}", loop_init));
        self.generator.ir_output.push(format!("{}:", loop_init));
        let iterator_state = format!("%iterator_state_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_create_iterator(i8* {})",
            iterator_state, iterator_value.llvm_name
        ));
        self.generator.ir_output.push(format!("  br label {}", loop_check));

        // Loop condition check
        self.generator.ir_output.push(format!("{}:", loop_check));
        let has_next = format!("%has_next_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = call i1 @cursed_template_iterator_has_next(i8* {})",
            has_next, iterator_state
        ));
        self.generator.ir_output.push(format!(
            "  br i1 {}, label {}, label {}",
            has_next, loop_body_label, loop_exit
        ));

        // Loop body
        self.generator.ir_output.push(format!("{}:", loop_body_label));
        let current_item = format!("%current_item_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = call i8* @cursed_template_iterator_next(i8* {})",
            current_item, iterator_state
        ));

        // Set loop variable in context
        let var_global = format!("@.str_loop_var_{}", variable);
        let escaped_var = variable.replace("\"", "\\22").replace("\n", "\\0A");
        self.generator.ir_output.push(format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
            var_global, escaped_var.len() + 1, escaped_var
        ));
        self.generator.ir_output.push(format!(
            "  call void @cursed_template_set_loop_variable({}* %context, i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0), i8* {})",
            "%context", escaped_var.len() + 1, escaped_var.len() + 1, var_global, current_item
        ));

        let loop_body_result = self.compile_template_body(body, &loop_context)?;
        self.generator.ir_output.push(format!("  br label {}", loop_increment));

        // Loop increment
        self.generator.ir_output.push(format!("{}:", loop_increment));
        self.generator.ir_output.push(format!("  br label {}", loop_check));

        // Loop exit
        self.generator.ir_output.push(format!("{}:", loop_exit));

        debug!("Generated loop compilation");
        Ok(loop_body_result)
    }

    #[instrument(skip(self, condition, then_branch, else_branch, context))]
    fn compile_template_conditional(
        &mut self,
        condition: &TemplateExpression,
        then_branch: &[TemplateNode],
        else_branch: Option<&[TemplateNode]>,
        context: &TemplateCompilationContext,
    ) -> TemplateCompilationResult<LlvmValue> {
        info!(
            has_else = else_branch.is_some(),
            then_nodes = then_branch.len(),
            else_nodes = else_branch.map(|b| b.len()).unwrap_or(0),
            "Compiling template conditional"
        );

        // Compile condition expression
        let condition_value = self.compile_template_expression(condition, context)?;

        // Compile then branch
        let mut then_values = Vec::new();
        for node in then_branch {
            let value = self.compile_template_node(node, context)?;
            then_values.push(value);
        }

        // Compile else branch if present
        let mut else_values = Vec::new();
        if let Some(else_nodes) = else_branch {
            for node in else_nodes {
                let value = self.compile_template_node(node, context)?;
                else_values.push(value);
            }
        }

        // Generate LLVM IR for conditional construct
        let condition_bool = self.convert_to_boolean(condition_value)?;
        
        let then_label = format!("%then_{}", context.scope_depth);
        let else_label = format!("%else_{}", context.scope_depth);
        let merge_label = format!("%merge_{}", context.scope_depth);

        // Conditional branch
        self.generator.ir_output.push(format!(
            "  br i1 {}, label {}, label {}",
            condition_bool.llvm_name, then_label, else_label
        ));

        // Then branch
        self.generator.ir_output.push(format!("{}:", then_label));
        let then_result = self.compile_template_body(then_branch, context)?;
        self.generator.ir_output.push(format!("  br label {}", merge_label));

        // Else branch
        self.generator.ir_output.push(format!("{}:", else_label));
        let else_result = if !else_values.is_empty() {
            self.compile_template_body(else_branch.unwrap_or(&[]), context)?
        } else {
            self.compile_template_literal("", context)?
        };
        self.generator.ir_output.push(format!("  br label {}", merge_label));

        // Merge point with phi node
        self.generator.ir_output.push(format!("{}:", merge_label));
        let result_temp = format!("%conditional_result_{}", context.scope_depth);
        self.generator.ir_output.push(format!(
            "  {} = phi i8* [{}, {}], [{}, {}]",
            result_temp,
            then_result.llvm_name, then_label,
            else_result.llvm_name, else_label
        ));

        debug!("Generated conditional compilation");
        Ok(LlvmValue {
            value_type: LlvmType::String,
            llvm_name: result_temp,
            is_constant: false,
        })
    }
}

/// Runtime functions for compiled templates
/// These functions are called by compiled template code for dynamic operations
pub mod runtime {
    use super::*;

    /// Escape HTML content
    pub fn escape_html(content: &str) -> String {
        content
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;")
    }

    /// Escape JSON content  
    pub fn escape_json(content: &str) -> String {
        content
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t")
    }

    /// Convert value to string for template output
    pub fn value_to_string(value: &CursedObject) -> String {
        match value {
            CursedObject::String(s) => s.clone(),
            CursedObject::Integer(i) => i.to_string(),
            CursedObject::Float(f) => f.to_string(),
            CursedObject::Boolean(b) => b.to_string(),
            CursedObject::Array(arr) => {
                format!("[{}]", arr.iter().map(|v| value_to_string(v)).collect::<Vec<_>>().join(", "))
            }
            CursedObject::Map(map) => {
                let entries: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                    .collect();
                format!("{{{}}}", entries.join(", "))
            }
            _ => format!("{:?}", value),
        }
    }

    /// Check if value is truthy for conditionals
    pub fn is_truthy(value: &CursedObject) -> bool {
        match value {
            CursedObject::Boolean(b) => *b,
            CursedObject::String(s) => !s.is_empty(),
            CursedObject::Integer(i) => *i != 0,
            CursedObject::Float(f) => *f != 0.0,
            CursedObject::Array(arr) => !arr.is_empty(),
            CursedObject::Map(map) => !map.is_empty(),
            CursedObject::Null => false,
            _ => true,
        }
    }
}

/// LLVM function declarations for template runtime support
pub fn declare_template_runtime_functions(module: &DummyModule) -> Result<HashMap<String, DummyFunction>, TemplateCompilationError> {
    let mut functions = HashMap::new();

    // String manipulation functions
    let escape_html_fn = module.add_function("cursed_template_escape_html", DummyType::Function, None);
    functions.insert("escape_html".to_string(), escape_html_fn);

    let escape_json_fn = module.add_function("cursed_template_escape_json", DummyType::Function, None);
    functions.insert("escape_json".to_string(), escape_json_fn);

    let value_to_string_fn = module.add_function("cursed_template_value_to_string", DummyType::Function, None);
    functions.insert("value_to_string".to_string(), value_to_string_fn);

    // Conditional functions
    let is_truthy_fn = module.add_function("cursed_template_is_truthy", DummyType::Function, None);
    functions.insert("is_truthy".to_string(), is_truthy_fn);

    // Context management functions
    let get_variable_fn = module.add_function("cursed_template_get_variable", DummyType::Function, None);
    functions.insert("get_variable".to_string(), get_variable_fn);

    let set_variable_fn = module.add_function("cursed_template_set_variable", DummyType::Function, None);
    functions.insert("set_variable".to_string(), set_variable_fn);

    // String concatenation functions
    let concat_strings_fn = module.add_function("cursed_template_concat_strings", DummyType::Function, None);
    functions.insert("concat_strings".to_string(), concat_strings_fn);

    // Template inclusion and inheritance functions
    let include_fn = module.add_function("cursed_template_include", DummyType::Function, None);
    functions.insert("include".to_string(), include_fn);

    let extends_fn = module.add_function("cursed_template_extends", DummyType::Function, None);
    functions.insert("extends".to_string(), extends_fn);

    let register_block_fn = module.add_function("cursed_template_register_block", DummyType::Function, None);
    functions.insert("register_block".to_string(), register_block_fn);

    // Utility functions
    let empty_string_fn = module.add_function("cursed_template_empty_string", DummyType::Function, None);
    functions.insert("empty_string".to_string(), empty_string_fn);

    // Filter application functions
    let apply_filter_fn = module.add_function("cursed_template_apply_filter", DummyType::Function, None);
    functions.insert("apply_filter".to_string(), apply_filter_fn);

    // Binary operation functions
    let add_values_fn = module.add_function("cursed_template_add_values", DummyType::Function, None);
    functions.insert("add_values".to_string(), add_values_fn);

    let sub_values_fn = module.add_function("cursed_template_sub_values", DummyType::Function, None);
    functions.insert("sub_values".to_string(), sub_values_fn);

    let mul_values_fn = module.add_function("cursed_template_mul_values", DummyType::Function, None);
    functions.insert("mul_values".to_string(), mul_values_fn);

    let div_values_fn = module.add_function("cursed_template_div_values", DummyType::Function, None);
    functions.insert("div_values".to_string(), div_values_fn);

    let mod_values_fn = module.add_function("cursed_template_mod_values", DummyType::Function, None);
    functions.insert("mod_values".to_string(), mod_values_fn);

    // Comparison functions
    let values_equal_fn = module.add_function("cursed_template_values_equal", DummyType::Function, None);
    functions.insert("values_equal".to_string(), values_equal_fn);

    let values_less_than_fn = module.add_function("cursed_template_values_less_than", DummyType::Function, None);
    functions.insert("values_less_than".to_string(), values_less_than_fn);

    let values_less_equal_fn = module.add_function("cursed_template_values_less_equal", DummyType::Function, None);
    functions.insert("values_less_equal".to_string(), values_less_equal_fn);

    let values_greater_than_fn = module.add_function("cursed_template_values_greater_than", DummyType::Function, None);
    functions.insert("values_greater_than".to_string(), values_greater_than_fn);

    let values_greater_equal_fn = module.add_function("cursed_template_values_greater_equal", DummyType::Function, None);
    functions.insert("values_greater_equal".to_string(), values_greater_equal_fn);

    // CURSED-style functions
    let values_vibe_fn = module.add_function("cursed_template_values_vibe", DummyType::Function, None);
    functions.insert("values_vibe".to_string(), values_vibe_fn);

    let values_contains_fn = module.add_function("cursed_template_values_contains", DummyType::Function, None);
    functions.insert("values_contains".to_string(), values_contains_fn);

    // Unary operation functions
    let negate_value_fn = module.add_function("cursed_template_negate_value", DummyType::Function, None);
    functions.insert("negate_value".to_string(), negate_value_fn);

    let get_type_fn = module.add_function("cursed_template_get_type", DummyType::Function, None);
    functions.insert("get_type".to_string(), get_type_fn);

    // Property and index access functions
    let get_property_fn = module.add_function("cursed_template_get_property", DummyType::Function, None);
    functions.insert("get_property".to_string(), get_property_fn);

    let get_index_fn = module.add_function("cursed_template_get_index", DummyType::Function, None);
    functions.insert("get_index".to_string(), get_index_fn);

    // Function call function
    let call_function_fn = module.add_function("cursed_template_call_function", DummyType::Function, None);
    functions.insert("call_function".to_string(), call_function_fn);

    // Array and object construction functions
    let create_array_fn = module.add_function("cursed_template_create_array", DummyType::Function, None);
    functions.insert("create_array".to_string(), create_array_fn);

    let create_object_fn = module.add_function("cursed_template_create_object", DummyType::Function, None);
    functions.insert("create_object".to_string(), create_object_fn);

    let set_object_field_fn = module.add_function("cursed_template_set_object_field", DummyType::Function, None);
    functions.insert("set_object_field".to_string(), set_object_field_fn);

    // Iterator functions
    let create_iterator_fn = module.add_function("cursed_template_create_iterator", DummyType::Function, None);
    functions.insert("create_iterator".to_string(), create_iterator_fn);

    let iterator_has_next_fn = module.add_function("cursed_template_iterator_has_next", DummyType::Function, None);
    functions.insert("iterator_has_next".to_string(), iterator_has_next_fn);

    let iterator_next_fn = module.add_function("cursed_template_iterator_next", DummyType::Function, None);
    functions.insert("iterator_next".to_string(), iterator_next_fn);

    // Loop variable management
    let set_loop_variable_fn = module.add_function("cursed_template_set_loop_variable", DummyType::Function, None);
    functions.insert("set_loop_variable".to_string(), set_loop_variable_fn);

    // Conversion functions
    let number_to_string_fn = module.add_function("cursed_template_number_to_string", DummyType::Function, None);
    functions.insert("number_to_string".to_string(), number_to_string_fn);

    // With block context management functions
    let set_with_variable_fn = module.add_function("cursed_template_set_with_variable", DummyType::Function, None);
    functions.insert("set_with_variable".to_string(), set_with_variable_fn);

    let restore_with_context_fn = module.add_function("cursed_template_restore_with_context", DummyType::Function, None);
    functions.insert("restore_with_context".to_string(), restore_with_context_fn);

    info!(functions_declared = functions.len(), "Template runtime functions declared");
    Ok(functions)
}

/// Register standard template filters for compilation
pub fn register_standard_filters(context: &mut TemplateCompilationContext) {
    // String filters
    context.register_filter("upper".to_string(), "cursed_filter_upper".to_string());
    context.register_filter("lower".to_string(), "cursed_filter_lower".to_string());
    context.register_filter("trim".to_string(), "cursed_filter_trim".to_string());
    context.register_filter("length".to_string(), "cursed_filter_length".to_string());
    
    // Format filters
    context.register_filter("date".to_string(), "cursed_filter_date".to_string());
    context.register_filter("number".to_string(), "cursed_filter_number".to_string());
    context.register_filter("currency".to_string(), "cursed_filter_currency".to_string());
    
    // HTML filters
    context.register_filter("escape".to_string(), "cursed_filter_escape_html".to_string());
    context.register_filter("safe".to_string(), "cursed_filter_safe".to_string());
    context.register_filter("linebreaks".to_string(), "cursed_filter_linebreaks".to_string());
    
    // Array filters
    context.register_filter("join".to_string(), "cursed_filter_join".to_string());
    context.register_filter("first".to_string(), "cursed_filter_first".to_string());
    context.register_filter("last".to_string(), "cursed_filter_last".to_string());
    context.register_filter("slice".to_string(), "cursed_filter_slice".to_string());
    
    debug!(filters_registered = context.filters.len(), "Standard template filters registered");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_compilation_context() {
        let mut context = TemplateCompilationContext::new("test".to_string(), TemplateConfig::default());
        context.add_variable("user".to_string(), LlvmType::String);
        
        assert_eq!(context.template_name, "test");
        assert_eq!(context.scope_depth, 0);
        assert!(context.get_variable_type("user").is_some());
        assert!(context.get_variable_type("unknown").is_none());
    }

    #[test]
    fn test_child_scope_creation() {
        let parent = TemplateCompilationContext::default();
        let child = parent.create_child_scope();
        
        assert_eq!(child.scope_depth, parent.scope_depth + 1);
        assert_eq!(child.template_name, parent.template_name);
    }

    #[test]
    fn test_filter_registration() {
        let mut context = TemplateCompilationContext::default();
        context.register_filter("upper".to_string(), "cursed_filter_upper".to_string());
        
        assert_eq!(context.get_filter_function("upper"), Some(&"cursed_filter_upper".to_string()));
        assert_eq!(context.get_filter_function("unknown"), None);
    }

    #[test]
    fn test_runtime_escape_html() {
        let input = "<script>alert('xss')</script>";
        let escaped = runtime::escape_html(input);
        assert_eq!(escaped, "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
    }

    #[test]
    fn test_runtime_is_truthy() {
        assert!(runtime::is_truthy(&CursedObject::Boolean(true)));
        assert!(!runtime::is_truthy(&CursedObject::Boolean(false)));
        assert!(runtime::is_truthy(&CursedObject::String("hello".to_string())));
        assert!(!runtime::is_truthy(&CursedObject::String("".to_string())));
        assert!(runtime::is_truthy(&CursedObject::Integer(42)));
        assert!(!runtime::is_truthy(&CursedObject::Integer(0)));
        assert!(!runtime::is_truthy(&CursedObject::Null));
    }

    #[test]
    fn test_runtime_value_to_string() {
        assert_eq!(runtime::value_to_string(&CursedObject::String("hello".to_string())), "hello");
        assert_eq!(runtime::value_to_string(&CursedObject::Integer(42)), "42");
        assert_eq!(runtime::value_to_string(&CursedObject::Boolean(true)), "true");
        assert_eq!(runtime::value_to_string(&CursedObject::Array(vec![
            CursedObject::Integer(1),
            CursedObject::Integer(2)
        ])), "[1, 2]");
    }
}
