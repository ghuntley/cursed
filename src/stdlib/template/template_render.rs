
use crate::error::CursedError;
/// Template Renderer - Executes template AST and generates output
use std::collections::HashMap;
use std::sync::Arc;
use std::io::Write;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn, span, Level};

use crate::object::Object as CursedObject;
use super::template_core::{TemplateConfig, TemplateContext, TemplateLoader};
use super::template_syntax::{
    TemplateAst, TemplateNode, BlockNode, TemplateExpression, FilterCall, BinaryOperator, UnaryOperator
// };

use super::template_filters::FilterRegistry;
use super::template_security::{TemplateSecurityValidator, SecurityContext};

/// Rendering context that extends TemplateContext with rendering-specific state
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Base template context
    /// Current template path for error reporting
    /// Rendering start time for performance tracking
    /// Security settings
    /// Output format for escaping
    /// Block content for template inheritance
    /// Parent template for inheritance chain
    /// Security context for enhanced security features
impl RenderContext {
    pub fn new(template_context: TemplateContext) -> Self {
        Self {
        }
    }

    pub fn new() -> Self {
        Self::new(TemplateContext::new())
    pub fn with_template(mut self, template_name: String) -> Self {
        self.current_template = Some(template_name);
        self
    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.security_level = level;
        self
    pub fn with_output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = format;
        self
    pub fn get(&self, key: &str) -> Option<CursedObject> {
        self.template_context.get(key)
    pub fn set(&mut self, key: String, value: CursedObject) -> crate::error::Result<()> {
        self.template_context.set(key, value)
    pub fn update(&mut self, key: String, value: CursedObject) -> crate::error::Result<()> {
        self.template_context.update(key, value)
    /// Set variable in context
    pub fn set_variable(&mut self, key: String, value: CursedObject) {
        let _ = self.template_context.set(key, value);
    /// Add a block definition for inheritance
    pub fn add_block(&mut self, name: String, content: Vec<TemplateNode>) {
        self.blocks.insert(name, content);
    /// Get block content by name
    pub fn get_block(&self, name: &str) -> Option<&Vec<TemplateNode>> {
        self.blocks.get(name)
    /// Set parent template for inheritance
    pub fn set_parent_template(&mut self, template_name: String) {
        self.parent_template = Some(template_name);
    /// Set security context for enhanced security features
    pub fn with_security_context(mut self, security_context: SecurityContext) -> Self {
        self.security_context = Some(security_context);
        self
    }
}

/// Security level for template rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityLevel {
    /// Strict security with auto-escaping and validation
    /// Moderate security with optional escaping
    /// Relaxed security for trusted templates
/// Output format for proper escaping
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
/// Comprehensive rendering result with metadata
#[derive(Debug, Clone)]
pub struct RenderResult {
    /// Rendered output
    /// Rendering duration
    /// Number of nodes processed
    /// Number of variables resolved
    /// Number of filters applied
    /// Memory usage estimate in bytes
    /// Security issues detected
    /// Performance warnings
impl RenderResult {
    pub fn new(output: String) -> Self {
        Self {
        }
    }

    pub fn with_metrics(
    ) -> Self {
        self.render_time = render_time;
        self.nodes_processed = nodes_processed;
        self.variables_resolved = variables_resolved;
        self.filters_applied = filters_applied;
        self
    pub fn add_security_warning(&mut self, warning: String) {
        self.security_warnings.push(warning);
    pub fn add_performance_warning(&mut self, warning: String) {
        self.performance_warnings.push(warning);
    }
}

/// Template renderer that executes AST nodes
pub struct TemplateRenderer {
    /// Filter registry for template functions
    /// Template loader for includes
    /// Template configuration
    /// Current nesting depth (for recursion protection)
    /// Performance metrics
impl TemplateRenderer {
    /// Create a new template renderer
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    /// Render a template AST with the given context
    #[instrument(skip(self, ast, context))]
    pub fn render(&self, ast: &TemplateAst, context: TemplateContext) -> crate::error::Result<()> {
        let render_context = RenderContext::new(context);
        self.render_with_context(ast, render_context)
    /// Render with comprehensive context and return detailed results
    #[instrument(skip(self, ast, render_context))]
    pub fn render_with_context(&self, ast: &TemplateAst, render_context: RenderContext) -> crate::error::Result<()> {
        let start_time = Instant::now();
        info!(nodes = ast.nodes.len(), "Starting template rendering");
        
        let mut renderer = TemplateRenderer {

        let mut output = String::new();
        renderer.render_nodes_with_context(&ast.nodes, &render_context, &mut output)?;
        
        let render_time = start_time.elapsed();
        info!(
            "Template rendering completed"
        );
        
        Ok(output)
    /// Render with full result metadata
    #[instrument(skip(self, ast, render_context))]
    pub fn render_with_result(&self, ast: &TemplateAst, render_context: RenderContext) -> crate::error::Result<()> {
        let start_time = Instant::now();
        info!(nodes = ast.nodes.len(), "Starting comprehensive template rendering");
        
        let mut renderer = TemplateRenderer {

        let mut output = String::new();
        renderer.render_nodes_with_context(&ast.nodes, &render_context, &mut output)?;
        
        let render_time = start_time.elapsed();
        let mut result = RenderResult::new(output)
            .with_metrics(
            );

        // Add performance warnings
        if render_time > Duration::from_millis(1000) {
            result.add_performance_warning(format!(
                renderer.nodes_processed
            ));
        if renderer.nodes_processed > 10000 {
            result.add_performance_warning(format!(
                renderer.nodes_processed
            ));
        info!(
            "Comprehensive template rendering completed"
        );
        
        Ok(result)
    /// Render template nodes to a writer
    #[instrument(skip(self, nodes, context, output))]
    fn render_nodes(
    ) -> crate::error::Result<()> {
        let render_context = RenderContext::new(context.clone());
        self.render_nodes_with_context(nodes, &render_context, output)
    /// Render template nodes with enhanced context
    #[instrument(skip(self, nodes, render_context, output))]
    fn render_nodes_with_context(
    ) -> crate::error::Result<()> {
        for node in nodes {
            self.render_node_with_context(node, render_context, output)?;
            self.nodes_processed += 1;
        }
        Ok(())
    /// Render a single template node
    #[instrument(skip(self, node, context, output))]
    fn render_node(
    ) -> crate::error::Result<()> {
        let render_context = RenderContext::new(context.clone());
        self.render_node_with_context(node, &render_context, output)
    /// Render a single template node with enhanced context
    #[instrument(skip(self, node, render_context, output))]
    fn render_node_with_context(
    ) -> crate::error::Result<()> {
        match node {
            TemplateNode::Text(text) => {
                output.push_str(text);
            }
            TemplateNode::Variable { expression, filters, .. } => {
                self.render_variable_expression_with_context(expression, filters, render_context, output)?;
            }
            TemplateNode::Block { block, .. } => {
                self.render_block_with_context(block, render_context, output)?;
            }
            TemplateNode::Comment { .. } => {
                // Comments are not rendered
            }
            TemplateNode::Include { template_name, context: include_context, .. } => {
                self.render_include_with_context(template_name, include_context, render_context, output)?;
            }
            TemplateNode::Extends { name, .. } => {
                // Handle extends - load base template
                self.render_extends_with_context(name, render_context, output)?;
            }
            TemplateNode::BlockDef { name, content, .. } => {
                // For now, render block content directly 
                // Block inheritance will be handled by explicit extends processing
                self.render_nodes_with_context(content, render_context, output)?;
            }
            TemplateNode::Raw { content, .. } => {
                // Raw content is not processed
                output.push_str(content);
            }
            TemplateNode::Set { name, value, .. } => {
                // Set variable in context
                self.render_set_variable(name, value, render_context)?;
            }
            TemplateNode::LowkeyIf { condition, then_branch, else_branch, .. } => {
                // CURSED-style conditional
                self.render_lowkey_if(condition, then_branch, else_branch, render_context, output)?;
            }
            TemplateNode::StanLoop { variable, iterator, body, .. } => {
                // CURSED-style loop  
                self.render_stan_loop(variable, iterator, body, render_context, output)?;
            }
        }
        Ok(())
    /// Render a variable expression with optional filters
    #[instrument(skip(self, filters, render_context, output))]
    fn render_variable_expression_with_context(
    ) -> crate::error::Result<()> {
        // Track variable resolution
        self.variables_resolved += 1;

        // Evaluate the expression
        let mut value = self.evaluate_expression_with_context(expression, render_context)?;

        // Apply filters in sequence
        for filter in filters {
            value = self.apply_filter_with_context(&filter.name, &filter.args, value, render_context)?;
            self.filters_applied += 1;
        // Convert to string and apply contextual escaping
        let string_value = self.object_to_string_cursed(&value)?;
        let final_value = self.apply_security_escaping(&string_value, render_context)?;

        output.push_str(&final_value);
        Ok(())
    /// Render a variable with enhanced context and security
    #[instrument(skip(self, filters, render_context, output))]
    fn render_variable_with_context(
    ) -> crate::error::Result<()> {
        // Track variable resolution
        self.variables_resolved += 1;

        // Get the variable value with CURSED-style Gen Z slang support
        let mut value = render_context.template_context.get(name)
            .unwrap_or_else(|| {
                if self.config.strict_mode {
                    warn!(variable = name, "Undefined variable in strict mode - that's not it, chief");
                }
                CursedObject::Nil
            });

        // Apply filters in sequence
        for filter in filters {
            value = self.apply_filter_with_context(&filter.name, &filter.args, value, render_context)?;
            self.filters_applied += 1;
        // Convert to string and apply contextual escaping
        let string_value = self.object_to_string_cursed(&value)?;
        let final_value = self.apply_security_escaping(&string_value, render_context)?;

        output.push_str(&final_value);
        Ok(())
    /// Render a block statement
    #[instrument(skip(self, block, context, output))]
    fn render_block(
    ) -> crate::error::Result<()> {
        let render_context = RenderContext::new(context.clone());
        self.render_block_with_context(block, &render_context, output)
    /// Render a block statement with enhanced context
    #[instrument(skip(self, block, render_context, output))]
    fn render_block_with_context(
    ) -> crate::error::Result<()> {
        match block {
            BlockNode::If { condition, then_branch, elsif_branches, else_branch } => {
                let condition_value = self.evaluate_expression_with_context(condition, render_context)?;
                if self.is_truthy_cursed(&condition_value) {
                    self.render_nodes_with_context(then_branch, render_context, output)?;
                } else {
                    // Check elsif branches
                    let mut branch_taken = false;
                    for (elsif_condition, elsif_body) in elsif_branches {
                        let elsif_value = self.evaluate_expression_with_context(elsif_condition, render_context)?;
                        if self.is_truthy_cursed(&elsif_value) {
                            self.render_nodes_with_context(elsif_body, render_context, output)?;
                            branch_taken = true;
                            break;
                        }
                    }
                    // If no elsif branch was taken, try else
                    if !branch_taken {
                        if let Some(else_nodes) = else_branch {
                            self.render_nodes_with_context(else_nodes, render_context, output)?;
                        }
                    }
                }
            }
            BlockNode::For { variable, iterator, body, else_body } => {
                let iterable = self.evaluate_expression_with_context(iterator, render_context)?;
                let items = self.make_iterable(&iterable)?;
                
                if items.is_empty() {
                    if let Some(else_nodes) = else_body {
                        self.render_nodes_with_context(else_nodes, render_context, output)?;
                    }
                } else {
                    for (index, item) in items.iter().enumerate() {
                        // Create a new scoped context for the loop iteration
                        let loop_scope = render_context.template_context
                            .create_loop_scope(variable.clone(), item.clone(), index)
                            .map_err(|e| CursedError::TemplateError {
                            })?;
                        
                        let mut loop_render_context = render_context.clone();
                        loop_render_context.template_context = loop_scope;
                        
                        debug!(
                            "For loop iteration with proper scoping - periodt"
                        );
                        self.render_nodes_with_context(body, &loop_render_context, output)?;
                    }
                }
            }
            BlockNode::While { condition, body } => {
                // Simple while loop implementation (with safety limit)
                let mut iterations = 0;
                const MAX_ITERATIONS: usize = 10000; // Safety limit
                
                while iterations < MAX_ITERATIONS {
                    let condition_value = self.evaluate_expression_with_context(condition, render_context)?;
                    if !self.is_truthy_cursed(&condition_value) {
                        break;
                    }
                    self.render_nodes_with_context(body, render_context, output)?;
                    iterations += 1;
                if iterations >= MAX_ITERATIONS {
                    warn!("While loop hit maximum iteration limit - sus behavior detected");
                }
            }
            BlockNode::When { condition, body } => {
                let condition_value = self.evaluate_expression_with_context(condition, render_context)?;
                if self.is_truthy_cursed(&condition_value) {
                    self.render_nodes_with_context(body, render_context, output)?;
                }
            }
            BlockNode::Each { iterator, body } => {
                let iterable = self.evaluate_expression_with_context(iterator, render_context)?;
                let items = self.make_iterable(&iterable)?;
                
                for (index, item) in items.iter().enumerate() {
                    let mut loop_context = render_context.clone();
                    debug!(
                        "Each loop iteration - no cap"
                    );
                    self.render_nodes_with_context(body, &loop_context, output)?;
                }
            }
            BlockNode::Loop { count, body } => {
                let count_value = self.evaluate_expression_with_context(count, render_context)?;
                let loop_count = match count_value {
                    _ => return Err(CursedError::TemplateError {
                
                for i in 0..loop_count {
                    let mut loop_context = render_context.clone();
                    debug!(
                        "Loop iteration - fr fr"
                    );
                    self.render_nodes_with_context(body, &loop_context, output)?;
                }
            }
            BlockNode::RangeFor { variable, start, end, step, body } => {
                let start_val = self.evaluate_expression_with_context(start, render_context)?;
                let end_val = self.evaluate_expression_with_context(end, render_context)?;
                let step_val = if let Some(step_expr) = step {
                    self.evaluate_expression_with_context(step_expr, render_context)?
                } else {
                    CursedObject::Integer(1)
                
                let start_num = self.extract_number(&start_val)?;
                let end_num = self.extract_number(&end_val)?;
                let step_num = self.extract_number(&step_val)?;
                
                let mut current = start_num;
                while (step_num > 0.0 && current <= end_num) || (step_num < 0.0 && current >= end_num) {
                    let mut loop_context = render_context.clone();
                    debug!(
                        "Range for iteration - periodt"
                    );
                    self.render_nodes_with_context(body, &loop_context, output)?;
                    current += step_num;
                }
            }
            BlockNode::Match { value, cases, default_case } => {
                let expr_value = self.evaluate_expression_with_context(value, render_context)?;
                let mut matched = false;
                
                for case in cases {
                    let pattern_value = self.evaluate_expression_with_context(&case.pattern, render_context)?;
                    if self.objects_vibe(&expr_value, &pattern_value) {
                        self.render_nodes_with_context(&case.body, render_context, output)?;
                        matched = true;
                        break;
                    }
                }
                
                // If no case matched, try default case
                if !matched {
                    if let Some(default_nodes) = default_case {
                        self.render_nodes_with_context(default_nodes, render_context, output)?;
                    }
                }
            }
            BlockNode::With { context, body } => {
                // Enhanced context block - render with current context for now
                debug!(context = ?context, "With block - that's the vibe");
                self.render_nodes_with_context(body, render_context, output)?;
            }
        }
        Ok(())
    /// Render CURSED-style set variable
    #[instrument(skip(self, value, render_context))]
    fn render_set_variable(
    ) -> crate::error::Result<()> {
        let evaluated_value = self.evaluate_expression_with_context(value, render_context)?;
        
        // Actually update the template context
        render_context.template_context.update(name.to_string(), evaluated_value)
            .map_err(|e| CursedError::TemplateError {
            })?;
        
        debug!(variable = name, "Set template variable - fr fr");
        Ok(())
    /// Render CURSED-style lowkey conditional
    #[instrument(skip(self, then_branch, else_branch, render_context, output))]
    fn render_lowkey_if(
    ) -> crate::error::Result<()> {
        let condition_value = self.evaluate_expression_with_context(condition, render_context)?;
        if self.is_truthy_cursed(&condition_value) {
            self.render_nodes_with_context(then_branch, render_context, output)?;
        } else if let Some(else_nodes) = else_branch {
            self.render_nodes_with_context(else_nodes, render_context, output)?;
        }
        Ok(())
    /// Render CURSED-style stan loop
    #[instrument(skip(self, body, render_context, output))]
    fn render_stan_loop(
    ) -> crate::error::Result<()> {
        let iterable = self.evaluate_expression_with_context(iterator, render_context)?;
        let items = self.make_iterable(&iterable)?;
        
        for (index, item) in items.iter().enumerate() {
            // Create a new scoped context for the loop iteration
            let loop_scope = render_context.template_context
                .create_loop_scope(variable.to_string(), item.clone(), index)
                .map_err(|e| CursedError::TemplateError {
                })?;
            
            let mut loop_render_context = render_context.clone();
            loop_render_context.template_context = loop_scope;
            
            debug!(
                "Stan loop iteration with proper scoping - that's the tea"
            );
            self.render_nodes_with_context(body, &loop_render_context, output)?;
        }
        Ok(())
    /// Render template extends
    #[instrument(skip(self, render_context, output))]
    fn render_extends_with_context(
    ) -> crate::error::Result<()> {
        // Load the base template
        let template_source = self.loader.load(name)?;
        debug!(template = name, "Loading base template - lowkey extends vibes");
        
        // Parse the base template
        use super::template_syntax::{TemplateLexer, TemplateParser};
        
        let mut lexer = TemplateLexer::new(&template_source);
        let tokens = lexer.tokenize().map_err(|e| {
            CursedError::TemplateError {
            }
        })?;
        
        let mut parser = TemplateParser::new(tokens);
        let base_ast = parser.parse().map_err(|e| {
            CursedError::TemplateError {
            }
        })?;
        
        // Create a mutable copy of the render context for inheritance
        let mut inheritance_context = render_context.clone();
        inheritance_context.set_parent_template(name.to_string());
        
        // Render the base template with block inheritance
        self.render_ast_with_inheritance(&base_ast, &inheritance_context, output)?;
        
        Ok(())
    /// Render AST with block inheritance support
    fn render_ast_with_inheritance(
    ) -> crate::error::Result<()> {
        for node in &ast.nodes {
            match node {
                TemplateNode::BlockDef { name, content, .. } => {
                    // Check if child template has overridden this block
                    if let Some(child_content) = render_context.get_block(name) {
                        debug!("Rendering overridden block: {}", name);
                        self.render_nodes_with_context(child_content, render_context, output)?;
                    } else {
                        debug!("Rendering default block: {}", name);
                        self.render_nodes_with_context(content, render_context, output)?;
                    }
                }
                _ => {
                    // Render other nodes normally
                    self.render_node_with_context(node, render_context, output)?;
                }
            }
        }
        Ok(())
    /// Render a template include
    #[instrument(skip(self, include_context, context, output))]
    fn render_include(
    ) -> crate::error::Result<()> {
        let render_context = RenderContext::new(context.clone());
        self.render_include_with_context(template_name, include_context, &render_context, output)
    /// Render a template include with enhanced context
    #[instrument(skip(self, include_context, render_context, output))]
    fn render_include_with_context(
    ) -> crate::error::Result<()> {
        // Prevent infinite recursion
        if self.nesting_depth >= self.config.max_nesting_depth {
            return Err(CursedError::TemplateError {
            });
        // Load and parse the included template
        let template_source = self.loader.load(template_name)?;
        let lexer = super::template_syntax::TemplateLexer::new(&template_source, &self.config.delimiters);
        let mut lexer_mut = lexer;
        let tokens = lexer_mut.tokenize()?;
        let mut parser = super::template_syntax::TemplateParser::new(tokens);
        let ast = parser.parse()?;

        // Create include context with proper merging
        let mut include_vars = HashMap::new();
        if let Some(ctx_vars) = include_context {
            for (key, expr) in ctx_vars {
                let value = self.evaluate_expression_with_context(expr, render_context)?;
                include_vars.insert(key.clone(), value);
                debug!(key = key, "Setting include context variable");
            }
        }
        
        let include_ctx = render_context.template_context.create_include_context(include_vars)
            .map_err(|e| CursedError::TemplateError {
            })?;

        // Render with increased nesting depth
        let mut nested_renderer = TemplateRenderer {

        let include_render_context = RenderContext::new(include_ctx);
        nested_renderer.render_nodes_with_context(&ast.nodes, &include_render_context, output)?;
        Ok(())
    /// Render a layout
    #[instrument(skip(self, blocks, context, output))]
    fn render_layout(
    ) -> crate::error::Result<()> {
        let render_context = RenderContext::new(context.clone());
        self.render_layout_with_context(name, blocks, &render_context, output)
    /// Render a layout with enhanced context
    #[instrument(skip(self, blocks, render_context, output))]
    fn render_layout_with_context(
    ) -> crate::error::Result<()> {
        // Load the layout template
        let layout_source = self.loader.load(name)?;
        let lexer = super::template_syntax::TemplateLexer::new(&layout_source, &self.config.delimiters);
        let mut lexer_mut = lexer;
        let tokens = lexer_mut.tokenize()?;
        let mut parser = super::template_syntax::TemplateParser::new(tokens);
        let ast = parser.parse()?;

        // Create layout context with blocks
        let mut layout_context = render_context.template_context.clone();
        for (block_name, block_nodes) in blocks {
            let mut block_output = String::new();
            self.render_nodes_with_context(block_nodes, render_context, &mut block_output)?;
            // In a real implementation, we'd update the context
            debug!(block_name = block_name, "Rendering layout block");
        let layout_render_context = RenderContext::new(layout_context);
        self.render_nodes_with_context(&ast.nodes, &layout_render_context, output)?;
        Ok(())
    /// Evaluate a template expression
    #[instrument(skip(self, expr, context))]
    fn evaluate_expression(
    ) -> crate::error::Result<()> {
        let render_context = RenderContext::new(context.clone());
        self.evaluate_expression_with_context(expr, &render_context)
    /// Evaluate a template expression with enhanced context
    #[instrument(skip(self, expr, render_context))]
    fn evaluate_expression_with_context(
    ) -> crate::error::Result<()> {
        match expr {
            TemplateExpression::Variable(name) => {
                // Support CURSED-style variable names
                let normalized_name = self.normalize_cursed_variable_name(name);
                Ok(render_context.template_context.get(&normalized_name)
                    .unwrap_or(CursedObject::Nil))
            }
            TemplateExpression::String(value) => {
                Ok(CursedObject::String(value.clone()))
            }
            TemplateExpression::Number(value) => {
                Ok(CursedObject::Float(*value))
            }
            TemplateExpression::Boolean(value) => {
                Ok(CursedObject::Boolean(*value))
            }
            TemplateExpression::Null => {
                Ok(CursedObject::Nil)
            }
            TemplateExpression::Array(items) => {
                let mut array_values = Vec::new();
                for item in items {
                    array_values.push(self.evaluate_expression_with_context(item, render_context)?);
                }
                Ok(CursedObject::Array(array_values))
            }
            TemplateExpression::Object(map) => {
                let mut object_map = HashMap::new();
                for (key, value_expr) in map {
                    let value = self.evaluate_expression_with_context(value_expr, render_context)?;
                    object_map.insert(key.clone(), value);
                }
                Ok(CursedObject::Map(object_map))
            }
            TemplateExpression::FunctionCall { name, args } => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate_expression_with_context(arg, render_context)?);
                }
                self.apply_filter_with_context(name, &[], CursedObject::Nil, render_context)
            }
            TemplateExpression::MethodCall { object, method, args } => {
                let obj_value = self.evaluate_expression_with_context(object, render_context)?;
                let mut arg_values = vec![obj_value];
                for arg in args {
                    arg_values.push(self.evaluate_expression_with_context(arg, render_context)?);
                }
                self.apply_filter_with_context(method, &[], CursedObject::Nil, render_context)
            }
            TemplateExpression::PropertyAccess { object, property } => {
                let obj_value = self.evaluate_expression_with_context(object, render_context)?;
                self.get_property_cursed(&obj_value, property)
            }
            TemplateExpression::IndexAccess { object, index } => {
                let obj_value = self.evaluate_expression_with_context(object, render_context)?;
                let index_value = self.evaluate_expression_with_context(index, render_context)?;
                self.get_index_cursed(&obj_value, &index_value)
            }
            TemplateExpression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression_with_context(left, render_context)?;
                let right_val = self.evaluate_expression_with_context(right, render_context)?;
                self.apply_binary_op_cursed(&left_val, operator, &right_val)
            }
            TemplateExpression::UnaryOp { operator, operand } => {
                let operand_val = self.evaluate_expression_with_context(operand, render_context)?;
                self.apply_unary_op_cursed(operator, &operand_val)
            }
            TemplateExpression::Conditional { condition, then_expr, else_expr } => {
                let condition_val = self.evaluate_expression_with_context(condition, render_context)?;
                if self.is_truthy_cursed(&condition_val) {
                    self.evaluate_expression_with_context(then_expr, render_context)
                } else {
                    self.evaluate_expression_with_context(else_expr, render_context)
                }
            }
            TemplateExpression::Sus(expr) => {
                let expr_val = self.evaluate_expression_with_context(expr, render_context)?;
                Ok(CursedObject::Boolean(self.is_truthy_cursed(&expr_val)))
            }
            TemplateExpression::Cap(expr) => {
                let expr_val = self.evaluate_expression_with_context(expr, render_context)?;
                Ok(CursedObject::Boolean(!self.is_truthy_cursed(&expr_val)))
            }
            TemplateExpression::Facts(expr) => {
                let expr_val = self.evaluate_expression_with_context(expr, render_context)?;
                Ok(CursedObject::String(self.get_cursed_type_name(&expr_val)))
            }
        }
    /// Normalize CURSED-style variable names
    fn normalize_cursed_variable_name(&self, name: &str) -> String {
        match name {
        }
    }

    /// Apply a filter to a value
    #[instrument(skip(self, args, value, context))]
    fn apply_filter(
    ) -> crate::error::Result<()> {
        let render_context = RenderContext::new(context.clone());
        self.apply_filter_with_context(filter_name, args, value, &render_context)
    /// Apply a filter with enhanced context
    #[instrument(skip(self, args, value, render_context))]
    fn apply_filter_with_context(
    ) -> crate::error::Result<()> {
        let mut filter_args = vec![value];
        for arg_expr in args {
            filter_args.push(self.evaluate_expression_with_context(arg_expr, render_context)?);
        // Support CURSED-style Gen Z slang filter names
        let normalized_filter_name = self.normalize_cursed_filter_name(filter_name);
        
        match self.filters.apply(&normalized_filter_name, &filter_args) {
            Err(e) => {
                error!(filter = filter_name, "Filter application failed - not it, bestie");
                Err(e)
            }
        }
    /// Normalize CURSED-style filter names to standard names
    fn normalize_cursed_filter_name(&self, filter_name: &str) -> String {
        match filter_name {
        }
    }

    /// Apply binary operation
    fn apply_binary_op(
    ) -> crate::error::Result<()> {
        match op {
            BinaryOperator::And => Ok(CursedObject::Boolean(
                self.is_truthy(left) && self.is_truthy(right)
            BinaryOperator::Or => Ok(CursedObject::Boolean(
                self.is_truthy(left) || self.is_truthy(right)
            // CURSED-style operators
        }
    }

    /// Apply unary operation
    fn apply_unary_op(
    ) -> crate::error::Result<()> {
        match op {
            UnaryOperator::Minus => match operand {
                _ => Err(CursedError::TemplateError {
            UnaryOperator::Plus => match operand {
                _ => Err(CursedError::TemplateError {
            // CURSED-style operators
        }
    }

    /// Helper methods for arithmetic operations
    fn add_objects(&self, left: &CursedObject, right: &CursedObject) -> crate::error::Result<()> {
        match (left, right) {
            _ => Err(CursedError::TemplateError {
        }
    }

    fn sub_objects(&self, left: &CursedObject, right: &CursedObject) -> crate::error::Result<()> {
        match (left, right) {
            _ => Err(CursedError::TemplateError {
        }
    }

    fn mul_objects(&self, left: &CursedObject, right: &CursedObject) -> crate::error::Result<()> {
        match (left, right) {
            _ => Err(CursedError::TemplateError {
        }
    }

    fn div_objects(&self, left: &CursedObject, right: &CursedObject) -> crate::error::Result<()> {
        match (left, right) {
            (CursedObject::Integer(a), CursedObject::Integer(b)) => {
                if *b == 0 {
                    return Err(CursedError::TemplateError {
                    });
                }
                Ok(CursedObject::Float(*a as f64 / *b as f64))
            }
            (CursedObject::Float(a), CursedObject::Float(b)) => {
                if *b == 0.0 {
                    return Err(CursedError::TemplateError {
                    });
                }
                Ok(CursedObject::Float(a / b))
            }
            (CursedObject::Integer(a), CursedObject::Float(b)) => {
                if *b == 0.0 {
                    return Err(CursedError::TemplateError {
                    });
                }
                Ok(CursedObject::Float(*a as f64 / b))
            }
            (CursedObject::Float(a), CursedObject::Integer(b)) => {
                if *b == 0 {
                    return Err(CursedError::TemplateError {
                    });
                }
                Ok(CursedObject::Float(a / *b as f64))
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    fn mod_objects(&self, left: &CursedObject, right: &CursedObject) -> crate::error::Result<()> {
        match (left, right) {
            (CursedObject::Integer(a), CursedObject::Integer(b)) => {
                if *b == 0 {
                    return Err(CursedError::TemplateError {
                    });
                }
                Ok(CursedObject::Integer(a % b))
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    fn compare_objects<F>(&self, left: &CursedObject, right: &CursedObject, op: F) -> crate::error::Result<()>
    where
    {
        let left_num = self.extract_number(left)?;
        let right_num = self.extract_number(right)?;
        Ok(CursedObject::Boolean(op(left_num, right_num)))
    fn objects_equal(&self, left: &CursedObject, right: &CursedObject) -> bool {
        match (left, right) {
        }
    }

    /// Extract numeric value from object
    fn extract_number(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Get property from object
    fn get_property(&self, obj: &CursedObject, property: &str) -> crate::error::Result<()> {
        match obj {
            CursedObject::Map(map) => {
                Ok(map.get(property).cloned().unwrap_or(CursedObject::Nil))
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Get property from object with CURSED-style slang support
    fn get_property_cursed(&self, obj: &CursedObject, property: &str) -> crate::error::Result<()> {
        // Normalize property name for CURSED-style access
        let normalized_property = match property {

        match obj {
            CursedObject::Map(map) => {
                Ok(map.get(normalized_property).cloned().unwrap_or(CursedObject::Nil))
            }
            CursedObject::Array(arr) if normalized_property == "length" => {
                Ok(CursedObject::Integer(arr.len() as i64))
            }
            CursedObject::String(s) if normalized_property == "length" => {
                Ok(CursedObject::Integer(s.len() as i64))
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Get index from object with CURSED-style support
    fn get_index_cursed(&self, obj: &CursedObject, index: &CursedObject) -> crate::error::Result<()> {
        match (obj, index) {
            (CursedObject::Array(arr), CursedObject::Integer(i)) => {
                let idx = *i as usize;
                Ok(arr.get(idx).cloned().unwrap_or(CursedObject::Nil))
            }
            (CursedObject::Map(map), CursedObject::String(key)) => {
                Ok(map.get(key).cloned().unwrap_or(CursedObject::Nil))
            }
            (CursedObject::String(s), CursedObject::Integer(i)) => {
                let idx = *i as usize;
                Ok(s.chars().nth(idx)
                    .map(|c| CursedObject::Char(c))
                    .unwrap_or(CursedObject::Nil))
            }
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Apply binary operation with CURSED-style semantics
    fn apply_binary_op_cursed(
    ) -> crate::error::Result<()> {
        // Support CURSED-style Gen Z truthiness and operators
        match op {
            BinaryOperator::And => Ok(CursedObject::Boolean(
                self.is_truthy_cursed(left) && self.is_truthy_cursed(right)
            BinaryOperator::Or => Ok(CursedObject::Boolean(
                self.is_truthy_cursed(left) || self.is_truthy_cursed(right)
            BinaryOperator::Vibe => Ok(CursedObject::Boolean(
                self.objects_vibe(left, right) // Loose equality
            BinaryOperator::NoVibe => Ok(CursedObject::Boolean(
                !self.objects_vibe(left, right) // Loose inequality
            BinaryOperator::Slay => Ok(CursedObject::Boolean(
                self.object_contains(left, right) // Contains/in
            BinaryOperator::NoSlay => Ok(CursedObject::Boolean(
                !self.object_contains(left, right) // Not contains
            _ => self.apply_binary_op(left, op, right), // Fall back to standard implementation
        }
    }

    /// Apply unary operation with CURSED-style semantics
    fn apply_unary_op_cursed(
    ) -> crate::error::Result<()> {
        match op {
            UnaryOperator::Sus => Ok(CursedObject::Boolean(self.is_truthy_cursed(operand))), // Truthiness check
            UnaryOperator::Cap => Ok(CursedObject::Boolean(!self.is_truthy_cursed(operand))), // Falsy check  
            UnaryOperator::Facts => Ok(CursedObject::String(self.get_cursed_type_name(operand))), // Type check
            _ => self.apply_unary_op(op, operand), // Fall back to standard implementation
        }
    }

    /// CURSED-style loose equality (vibe check)
    fn objects_vibe(&self, left: &CursedObject, right: &CursedObject) -> bool {
        match (left, right) {
            // Exact equality first
            // CURSED-style loose equality
            (CursedObject::String(a), CursedObject::String(b)) => {
                a.to_lowercase() == b.to_lowercase() // Case-insensitive
            }
            (CursedObject::Boolean(a), CursedObject::String(b)) => {
                (*a && (b == "slay" || b == "true" || b == "fr")) ||
                (!*a && (b == "sus" || b == "false" || b == "cap"))
            }
            (CursedObject::String(a), CursedObject::Boolean(b)) => {
                (*b && (a == "slay" || a == "true" || a == "fr")) ||
                (!*b && (a == "sus" || a == "false" || a == "cap"))
            }
        }
    }

    /// CURSED-style contains check (slay operation)
    fn object_contains(&self, container: &CursedObject, item: &CursedObject) -> bool {
        match container {
            CursedObject::String(s) => {
                if let CursedObject::String(search) = item {
                    s.contains(search)
                } else {
                    false
                }
            }
            CursedObject::Map(map) => {
                if let CursedObject::String(key) = item {
                    map.contains_key(key)
                } else {
                    map.values().any(|value| self.objects_vibe(value, item))
                }
            }
        }
    }

    /// Get CURSED-style type name
    fn get_cursed_type_name(&self, obj: &CursedObject) -> String {
        match obj {
            CursedObject::String(_) => "vibes".to_string(), // String type
            CursedObject::Integer(_) => "digits".to_string(), // Integer type
            CursedObject::Float(_) => "decimals".to_string(), // Float type
            CursedObject::Boolean(_) => "truth".to_string(), // Boolean type
            CursedObject::Array(_) => "squad".to_string(), // Array type
            CursedObject::Map(_) => "collab".to_string(), // Object type
            CursedObject::Char(_) => "letter".to_string(), // Character type
            CursedObject::Nil => "periodt".to_string(), // Nil type
        }
    }

    /// Check if object is truthy with CURSED-style Gen Z slang semantics
    fn is_truthy_cursed(&self, obj: &CursedObject) -> bool {
        match obj {
            CursedObject::String(s) => match s.as_str() {
            _ => self.is_truthy(obj), // Fall back to standard implementation
        }
    }

    /// Convert object to iterable list
    fn make_iterable(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            _ => Err(CursedError::TemplateError {
        }
    }

    /// Check if object is truthy
    fn is_truthy(&self, obj: &CursedObject) -> bool {
        match obj {
            CursedObject::Char(_) => true, // Characters are always truthy
        }
    }

    /// Convert object to string
    fn object_to_string(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
        }
    }

    /// Apply security escaping based on output format and security level
    fn apply_security_escaping(&self, s: &str, render_context: &RenderContext) -> crate::error::Result<()> {
        match render_context.security_level {
            SecurityLevel::Strict | SecurityLevel::Moderate => {
                match render_context.output_format {
                }
            }
        }
    }

    /// Escape HTML characters
    fn escape_html(&self, s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&#x27;")
    /// Escape XML characters
    fn escape_xml(&self, s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&apos;")
    /// Escape JSON characters
    fn escape_json(&self, s: &str) -> String {
        s.replace('\\', "\\\\")
         .replace('"', "\\\"")
         .replace('\n', "\\n")
         .replace('\r', "\\r")
         .replace('\t', "\\t")
    /// Escape CSS characters
    fn escape_css(&self, s: &str) -> String {
        s.chars()
            .map(|c| match c {
            })
            .collect()
    /// Escape JavaScript characters
    fn escape_javascript(&self, s: &str) -> String {
        s.replace('\\', "\\\\")
         .replace('"', "\\\"")
         .replace('\'', "\\'")
         .replace('\n', "\\n")
         .replace('\r', "\\r")
         .replace('\t', "\\t")
         .replace('<', "\\u003C")
         .replace('>', "\\u003E")
    /// Convert object to string with CURSED-style Gen Z slang
    fn object_to_string_cursed(&self, obj: &CursedObject) -> crate::error::Result<()> {
        match obj {
            CursedObject::Boolean(true) => Ok("slay".to_string()), // CURSED-style true
            CursedObject::Boolean(false) => Ok("sus".to_string()), // CURSED-style false
            CursedObject::Nil => Ok("periodt".to_string()), // CURSED-style nil/empty
            CursedObject::Array(arr) => {
                let items: Vec<String> = arr.iter()
                    .map(|item| self.object_to_string_cursed(item))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(format!("[{}]", items.join(", ")))
            }
            CursedObject::Map(map) => {
                let items: crate::error::Result<()> = map.iter()
                    .map(|(k, v)| Ok(format!("{}: {}", k, self.object_to_string_cursed(v)?)))
                    .collect();
                Ok(format!("{{{}}}", items?.join(", ")))
            }
        }
    }
}

