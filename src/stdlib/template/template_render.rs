/// Template Renderer - Executes template AST and generates output
use std::collections::HashMap;
use std::sync::Arc;
use std::io::Write;
use tracing::{debug, error, info, instrument, warn};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;
use super::template_core::{TemplateConfig, TemplateContext, TemplateLoader};
use super::template_syntax::{
    TemplateAst, TemplateNode, BlockNode, TemplateExpression, FilterCall, BinaryOperator, UnaryOperator
};
use super::template_filters::FilterRegistry;

/// Template renderer that executes AST nodes
pub struct TemplateRenderer {
    /// Filter registry for template functions
    filters: Arc<FilterRegistry>,
    /// Template loader for includes
    loader: Arc<dyn TemplateLoader>,
    /// Template configuration
    config: TemplateConfig,
    /// Current nesting depth (for recursion protection)
    nesting_depth: usize,
}

impl TemplateRenderer {
    /// Create a new template renderer
    pub fn new(
        filters: Arc<FilterRegistry>,
        loader: Arc<dyn TemplateLoader>,
        config: &TemplateConfig,
    ) -> Self {
        Self {
            filters,
            loader,
            config: config.clone(),
            nesting_depth: 0,
        }
    }

    /// Render a template AST with the given context
    #[instrument(skip(self, ast, context))]
    pub fn render(&self, ast: &TemplateAst, context: TemplateContext) -> Result<String, CursedError> {
        info!(nodes = ast.nodes.len(), "Starting template rendering");
        
        let mut output = String::new();
        self.render_nodes(&ast.nodes, &context, &mut output)?;
        
        info!(output_length = output.len(), "Template rendering completed");
        Ok(output)
    }

    /// Render template nodes to a writer
    #[instrument(skip(self, nodes, context, output))]
    fn render_nodes(
        &self,
        nodes: &[TemplateNode],
        context: &TemplateContext,
        output: &mut String,
    ) -> Result<(), CursedError> {
        for node in nodes {
            self.render_node(node, context, output)?;
        }
        Ok(())
    }

    /// Render a single template node
    #[instrument(skip(self, node, context, output))]
    fn render_node(
        &self,
        node: &TemplateNode,
        context: &TemplateContext,
        output: &mut String,
    ) -> Result<(), CursedError> {
        match node {
            TemplateNode::Text(text) => {
                output.push_str(text);
            }
            TemplateNode::Variable { name, filters } => {
                self.render_variable(name, filters, context, output)?;
            }
            TemplateNode::Block(block) => {
                self.render_block(block, context, output)?;
            }
            TemplateNode::Comment(_) => {
                // Comments are not rendered
            }
            TemplateNode::Include { template_name, context: include_context } => {
                self.render_include(template_name, include_context, context, output)?;
            }
            TemplateNode::Layout { name, blocks } => {
                self.render_layout(name, blocks, context, output)?;
            }
            TemplateNode::BlockDef { name: _, content } => {
                // Block definitions are rendered in place for now
                self.render_nodes(content, context, output)?;
            }
        }
        Ok(())
    }

    /// Render a variable with optional filters
    #[instrument(skip(self, filters, context, output))]
    fn render_variable(
        &self,
        name: &str,
        filters: &[FilterCall],
        context: &TemplateContext,
        output: &mut String,
    ) -> Result<(), CursedError> {
        // Get the variable value
        let mut value = context.get(name)
            .cloned()
            .unwrap_or_else(|| {
                if self.config.strict_mode {
                    warn!(variable = name, "Undefined variable in strict mode");
                }
                CursedObject::Nil
            });

        // Apply filters in sequence
        for filter in filters {
            value = self.apply_filter(&filter.name, &filter.args, value, context)?;
        }

        // Convert to string and apply escaping if needed
        let string_value = self.object_to_string(&value)?;
        let final_value = if self.config.auto_escape {
            self.escape_html(&string_value)
        } else {
            string_value
        };

        output.push_str(&final_value);
        Ok(())
    }

    /// Render a block statement
    #[instrument(skip(self, block, context, output))]
    fn render_block(
        &self,
        block: &BlockNode,
        context: &TemplateContext,
        output: &mut String,
    ) -> Result<(), CursedError> {
        match block {
            BlockNode::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate_expression(condition, context)?;
                if self.is_truthy(&condition_value) {
                    self.render_nodes(then_branch, context, output)?;
                } else if let Some(else_nodes) = else_branch {
                    self.render_nodes(else_nodes, context, output)?;
                }
            }
            BlockNode::For { variable, iterator, body, else_body } => {
                let iterable = self.evaluate_expression(iterator, context)?;
                let items = self.make_iterable(&iterable)?;
                
                if items.is_empty() {
                    if let Some(else_nodes) = else_body {
                        self.render_nodes(else_nodes, context, output)?;
                    }
                } else {
                    for (index, item) in items.iter().enumerate() {
                        let mut loop_context = TemplateContext::with_parent(context.clone());
                        loop_context.set(variable.clone(), item.clone());
                        loop_context.set("@index".to_string(), CursedObject::Integer(index as i64));
                        loop_context.set("@first".to_string(), CursedObject::Boolean(index == 0));
                        loop_context.set("@last".to_string(), CursedObject::Boolean(index == items.len() - 1));
                        loop_context.set("@even".to_string(), CursedObject::Boolean(index % 2 == 0));
                        loop_context.set("@odd".to_string(), CursedObject::Boolean(index % 2 == 1));
                        
                        self.render_nodes(body, &loop_context, output)?;
                    }
                }
            }
            BlockNode::When { condition, body } => {
                let condition_value = self.evaluate_expression(condition, context)?;
                if self.is_truthy(&condition_value) {
                    self.render_nodes(body, context, output)?;
                }
            }
            BlockNode::Each { iterator, body } => {
                let iterable = self.evaluate_expression(iterator, context)?;
                let items = self.make_iterable(&iterable)?;
                
                for (index, item) in items.iter().enumerate() {
                    let mut loop_context = TemplateContext::with_parent(context.clone());
                    loop_context.set("@item".to_string(), item.clone());
                    loop_context.set("@index".to_string(), CursedObject::Integer(index as i64));
                    loop_context.set("@first".to_string(), CursedObject::Boolean(index == 0));
                    loop_context.set("@last".to_string(), CursedObject::Boolean(index == items.len() - 1));
                    
                    self.render_nodes(body, &loop_context, output)?;
                }
            }
            BlockNode::Loop { count, body } => {
                let count_value = self.evaluate_expression(count, context)?;
                let loop_count = match count_value {
                    CursedObject::Integer(n) => n as usize,
                    _ => return Err(CursedError::TemplateError {
                        message: "Loop count must be an integer".to_string(),
                        source_location: None,
                    }),
                };
                
                for i in 0..loop_count {
                    let mut loop_context = TemplateContext::with_parent(context.clone());
                    loop_context.set("@index".to_string(), CursedObject::Integer(i as i64));
                    loop_context.set("@first".to_string(), CursedObject::Boolean(i == 0));
                    loop_context.set("@last".to_string(), CursedObject::Boolean(i == loop_count - 1));
                    
                    self.render_nodes(body, &loop_context, output)?;
                }
            }
            BlockNode::RangeFor { variable, start, end, step, body } => {
                let start_val = self.evaluate_expression(start, context)?;
                let end_val = self.evaluate_expression(end, context)?;
                let step_val = if let Some(step_expr) = step {
                    self.evaluate_expression(step_expr, context)?
                } else {
                    CursedObject::Integer(1)
                };
                
                let start_num = self.extract_number(&start_val)?;
                let end_num = self.extract_number(&end_val)?;
                let step_num = self.extract_number(&step_val)?;
                
                let mut current = start_num;
                while (step_num > 0.0 && current <= end_num) || (step_num < 0.0 && current >= end_num) {
                    let mut loop_context = TemplateContext::with_parent(context.clone());
                    loop_context.set(variable.clone(), CursedObject::Float(current));
                    
                    self.render_nodes(body, &loop_context, output)?;
                    current += step_num;
                }
            }
        }
        Ok(())
    }

    /// Render a template include
    #[instrument(skip(self, include_context, context, output))]
    fn render_include(
        &self,
        template_name: &str,
        include_context: &Option<HashMap<String, TemplateExpression>>,
        context: &TemplateContext,
        output: &mut String,
    ) -> Result<(), CursedError> {
        // Prevent infinite recursion
        if self.nesting_depth >= self.config.max_nesting_depth {
            return Err(CursedError::TemplateError {
                message: format!("Maximum nesting depth {} exceeded", self.config.max_nesting_depth),
                source_location: None,
            });
        }

        // Load and parse the included template
        let template_source = self.loader.load(template_name)?;
        let lexer = super::template_syntax::TemplateLexer::new(&template_source, &self.config.delimiters);
        let mut lexer_mut = lexer;
        let tokens = lexer_mut.tokenize()?;
        let mut parser = super::template_syntax::TemplateParser::new(tokens);
        let ast = parser.parse()?;

        // Create include context
        let mut include_ctx = context.clone();
        if let Some(ctx_vars) = include_context {
            for (key, expr) in ctx_vars {
                let value = self.evaluate_expression(expr, context)?;
                include_ctx.set(key.clone(), value);
            }
        }

        // Render with increased nesting depth
        let mut nested_renderer = TemplateRenderer {
            filters: Arc::clone(&self.filters),
            loader: Arc::clone(&self.loader),
            config: self.config.clone(),
            nesting_depth: self.nesting_depth + 1,
        };

        nested_renderer.render_nodes(&ast.nodes, &include_ctx, output)?;
        Ok(())
    }

    /// Render a layout
    #[instrument(skip(self, blocks, context, output))]
    fn render_layout(
        &self,
        name: &str,
        blocks: &HashMap<String, Vec<TemplateNode>>,
        context: &TemplateContext,
        output: &mut String,
    ) -> Result<(), CursedError> {
        // Load the layout template
        let layout_source = self.loader.load(name)?;
        let lexer = super::template_syntax::TemplateLexer::new(&layout_source, &self.config.delimiters);
        let mut lexer_mut = lexer;
        let tokens = lexer_mut.tokenize()?;
        let mut parser = super::template_syntax::TemplateParser::new(tokens);
        let ast = parser.parse()?;

        // Create layout context with blocks
        let mut layout_context = context.clone();
        for (block_name, block_nodes) in blocks {
            let mut block_output = String::new();
            self.render_nodes(block_nodes, context, &mut block_output)?;
            layout_context.set(format!("@block_{}", block_name), CursedObject::String(block_output));
        }

        self.render_nodes(&ast.nodes, &layout_context, output)?;
        Ok(())
    }

    /// Evaluate a template expression
    #[instrument(skip(self, expr, context))]
    fn evaluate_expression(
        &self,
        expr: &TemplateExpression,
        context: &TemplateContext,
    ) -> Result<CursedObject, CursedError> {
        match expr {
            TemplateExpression::Variable(name) => {
                Ok(context.get(name).cloned().unwrap_or(CursedObject::Nil))
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
            TemplateExpression::FunctionCall { name, args } => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate_expression(arg, context)?);
                }
                self.apply_filter(name, &[], CursedObject::Nil, context)
            }
            TemplateExpression::PropertyAccess { object, property } => {
                let obj_value = self.evaluate_expression(object, context)?;
                self.get_property(&obj_value, property)
            }
            TemplateExpression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression(left, context)?;
                let right_val = self.evaluate_expression(right, context)?;
                self.apply_binary_op(&left_val, operator, &right_val)
            }
            TemplateExpression::UnaryOp { operator, operand } => {
                let operand_val = self.evaluate_expression(operand, context)?;
                self.apply_unary_op(operator, &operand_val)
            }
        }
    }

    /// Apply a filter to a value
    #[instrument(skip(self, args, value, context))]
    fn apply_filter(
        &self,
        filter_name: &str,
        args: &[TemplateExpression],
        value: CursedObject,
        context: &TemplateContext,
    ) -> Result<CursedObject, CursedError> {
        let mut filter_args = vec![value];
        for arg_expr in args {
            filter_args.push(self.evaluate_expression(arg_expr, context)?);
        }

        self.filters.apply(filter_name, &filter_args)
    }

    /// Apply binary operation
    fn apply_binary_op(
        &self,
        left: &CursedObject,
        op: &BinaryOperator,
        right: &CursedObject,
    ) -> Result<CursedObject, CursedError> {
        match op {
            BinaryOperator::Add => self.add_objects(left, right),
            BinaryOperator::Sub => self.sub_objects(left, right),
            BinaryOperator::Mul => self.mul_objects(left, right),
            BinaryOperator::Div => self.div_objects(left, right),
            BinaryOperator::Mod => self.mod_objects(left, right),
            BinaryOperator::Eq => Ok(CursedObject::Boolean(self.objects_equal(left, right))),
            BinaryOperator::Ne => Ok(CursedObject::Boolean(!self.objects_equal(left, right))),
            BinaryOperator::Lt => self.compare_objects(left, right, |a, b| a < b),
            BinaryOperator::Le => self.compare_objects(left, right, |a, b| a <= b),
            BinaryOperator::Gt => self.compare_objects(left, right, |a, b| a > b),
            BinaryOperator::Ge => self.compare_objects(left, right, |a, b| a >= b),
            BinaryOperator::And => Ok(CursedObject::Boolean(
                self.is_truthy(left) && self.is_truthy(right)
            )),
            BinaryOperator::Or => Ok(CursedObject::Boolean(
                self.is_truthy(left) || self.is_truthy(right)
            )),
        }
    }

    /// Apply unary operation
    fn apply_unary_op(
        &self,
        op: &UnaryOperator,
        operand: &CursedObject,
    ) -> Result<CursedObject, CursedError> {
        match op {
            UnaryOperator::Not => Ok(CursedObject::Boolean(!self.is_truthy(operand))),
            UnaryOperator::Minus => match operand {
                CursedObject::Integer(n) => Ok(CursedObject::Integer(-n)),
                CursedObject::Float(n) => Ok(CursedObject::Float(-n)),
                _ => Err(CursedError::TemplateError {
                    message: "Cannot negate non-numeric value".to_string(),
                    source_location: None,
                }),
            },
        }
    }

    /// Helper methods for arithmetic operations
    fn add_objects(&self, left: &CursedObject, right: &CursedObject) -> Result<CursedObject, CursedError> {
        match (left, right) {
            (CursedObject::Integer(a), CursedObject::Integer(b)) => Ok(CursedObject::Integer(a + b)),
            (CursedObject::Float(a), CursedObject::Float(b)) => Ok(CursedObject::Float(a + b)),
            (CursedObject::Integer(a), CursedObject::Float(b)) => Ok(CursedObject::Float(*a as f64 + b)),
            (CursedObject::Float(a), CursedObject::Integer(b)) => Ok(CursedObject::Float(a + *b as f64)),
            (CursedObject::String(a), CursedObject::String(b)) => Ok(CursedObject::String(format!("{}{}", a, b))),
            _ => Err(CursedError::TemplateError {
                message: "Cannot add incompatible types".to_string(),
                source_location: None,
            }),
        }
    }

    fn sub_objects(&self, left: &CursedObject, right: &CursedObject) -> Result<CursedObject, CursedError> {
        match (left, right) {
            (CursedObject::Integer(a), CursedObject::Integer(b)) => Ok(CursedObject::Integer(a - b)),
            (CursedObject::Float(a), CursedObject::Float(b)) => Ok(CursedObject::Float(a - b)),
            (CursedObject::Integer(a), CursedObject::Float(b)) => Ok(CursedObject::Float(*a as f64 - b)),
            (CursedObject::Float(a), CursedObject::Integer(b)) => Ok(CursedObject::Float(a - *b as f64)),
            _ => Err(CursedError::TemplateError {
                message: "Cannot subtract incompatible types".to_string(),
                source_location: None,
            }),
        }
    }

    fn mul_objects(&self, left: &CursedObject, right: &CursedObject) -> Result<CursedObject, CursedError> {
        match (left, right) {
            (CursedObject::Integer(a), CursedObject::Integer(b)) => Ok(CursedObject::Integer(a * b)),
            (CursedObject::Float(a), CursedObject::Float(b)) => Ok(CursedObject::Float(a * b)),
            (CursedObject::Integer(a), CursedObject::Float(b)) => Ok(CursedObject::Float(*a as f64 * b)),
            (CursedObject::Float(a), CursedObject::Integer(b)) => Ok(CursedObject::Float(a * *b as f64)),
            _ => Err(CursedError::TemplateError {
                message: "Cannot multiply incompatible types".to_string(),
                source_location: None,
            }),
        }
    }

    fn div_objects(&self, left: &CursedObject, right: &CursedObject) -> Result<CursedObject, CursedError> {
        match (left, right) {
            (CursedObject::Integer(a), CursedObject::Integer(b)) => {
                if *b == 0 {
                    return Err(CursedError::TemplateError {
                        message: "Division by zero".to_string(),
                        source_location: None,
                    });
                }
                Ok(CursedObject::Float(*a as f64 / *b as f64))
            }
            (CursedObject::Float(a), CursedObject::Float(b)) => {
                if *b == 0.0 {
                    return Err(CursedError::TemplateError {
                        message: "Division by zero".to_string(),
                        source_location: None,
                    });
                }
                Ok(CursedObject::Float(a / b))
            }
            (CursedObject::Integer(a), CursedObject::Float(b)) => {
                if *b == 0.0 {
                    return Err(CursedError::TemplateError {
                        message: "Division by zero".to_string(),
                        source_location: None,
                    });
                }
                Ok(CursedObject::Float(*a as f64 / b))
            }
            (CursedObject::Float(a), CursedObject::Integer(b)) => {
                if *b == 0 {
                    return Err(CursedError::TemplateError {
                        message: "Division by zero".to_string(),
                        source_location: None,
                    });
                }
                Ok(CursedObject::Float(a / *b as f64))
            }
            _ => Err(CursedError::TemplateError {
                message: "Cannot divide incompatible types".to_string(),
                source_location: None,
            }),
        }
    }

    fn mod_objects(&self, left: &CursedObject, right: &CursedObject) -> Result<CursedObject, CursedError> {
        match (left, right) {
            (CursedObject::Integer(a), CursedObject::Integer(b)) => {
                if *b == 0 {
                    return Err(CursedError::TemplateError {
                        message: "Modulo by zero".to_string(),
                        source_location: None,
                    });
                }
                Ok(CursedObject::Integer(a % b))
            }
            _ => Err(CursedError::TemplateError {
                message: "Modulo operation only supported for integers".to_string(),
                source_location: None,
            }),
        }
    }

    fn compare_objects<F>(&self, left: &CursedObject, right: &CursedObject, op: F) -> Result<CursedObject, CursedError>
    where
        F: Fn(f64, f64) -> bool,
    {
        let left_num = self.extract_number(left)?;
        let right_num = self.extract_number(right)?;
        Ok(CursedObject::Boolean(op(left_num, right_num)))
    }

    fn objects_equal(&self, left: &CursedObject, right: &CursedObject) -> bool {
        match (left, right) {
            (CursedObject::String(a), CursedObject::String(b)) => a == b,
            (CursedObject::Integer(a), CursedObject::Integer(b)) => a == b,
            (CursedObject::Float(a), CursedObject::Float(b)) => (a - b).abs() < f64::EPSILON,
            (CursedObject::Boolean(a), CursedObject::Boolean(b)) => a == b,
            (CursedObject::Nil, CursedObject::Nil) => true,
            _ => false,
        }
    }

    /// Extract numeric value from object
    fn extract_number(&self, obj: &CursedObject) -> Result<f64, CursedError> {
        match obj {
            CursedObject::Integer(n) => Ok(*n as f64),
            CursedObject::Float(n) => Ok(*n),
            _ => Err(CursedError::TemplateError {
                message: "Expected numeric value".to_string(),
                source_location: None,
            }),
        }
    }

    /// Get property from object
    fn get_property(&self, obj: &CursedObject, property: &str) -> Result<CursedObject, CursedError> {
        match obj {
            CursedObject::Map(map) => {
                Ok(map.get(property).cloned().unwrap_or(CursedObject::Nil))
            }
            _ => Err(CursedError::TemplateError {
                message: format!("Cannot access property '{}' on non-object", property),
                source_location: None,
            }),
        }
    }

    /// Convert object to iterable list
    fn make_iterable(&self, obj: &CursedObject) -> Result<Vec<CursedObject>, CursedError> {
        match obj {
            CursedObject::Array(arr) => Ok(arr.clone()),
            CursedObject::String(s) => Ok(s.chars().map(|c| CursedObject::String(c.to_string())).collect()),
            _ => Err(CursedError::TemplateError {
                message: "Value is not iterable".to_string(),
                source_location: None,
            }),
        }
    }

    /// Check if object is truthy
    fn is_truthy(&self, obj: &CursedObject) -> bool {
        match obj {
            CursedObject::Boolean(b) => *b,
            CursedObject::Nil => false,
            CursedObject::Integer(n) => *n != 0,
            CursedObject::Float(n) => *n != 0.0,
            CursedObject::String(s) => !s.is_empty(),
            CursedObject::Array(arr) => !arr.is_empty(),
            CursedObject::Map(map) => !map.is_empty(),
            _ => true,
        }
    }

    /// Convert object to string
    fn object_to_string(&self, obj: &CursedObject) -> Result<String, CursedError> {
        match obj {
            CursedObject::String(s) => Ok(s.clone()),
            CursedObject::Integer(n) => Ok(n.to_string()),
            CursedObject::Float(n) => Ok(n.to_string()),
            CursedObject::Boolean(b) => Ok(b.to_string()),
            CursedObject::Nil => Ok("".to_string()),
            _ => Ok(format!("{:?}", obj)),
        }
    }

    /// Escape HTML characters
    fn escape_html(&self, s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&#x27;")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::CursedObject;
    use std::collections::HashMap;

    #[test]
    fn test_render_text_node() {
        let filters = Arc::new(FilterRegistry::new());
        let loader = Arc::new(crate::stdlib::template::template_core::FileSystemLoader::new("templates"));
        let config = crate::stdlib::template::template_core::TemplateConfig::default();
        
        let renderer = TemplateRenderer::new(filters, loader, &config);
        let mut output = String::new();
        let context = TemplateContext::new();
        
        let node = TemplateNode::Text("Hello World".to_string());
        renderer.render_node(&node, &context, &mut output).unwrap();
        
        assert_eq!(output, "Hello World");
    }

    #[test]
    fn test_render_variable_node() {
        let filters = Arc::new(FilterRegistry::new());
        let loader = Arc::new(crate::stdlib::template::template_core::FileSystemLoader::new("templates"));
        let config = crate::stdlib::template::template_core::TemplateConfig::default();
        
        let renderer = TemplateRenderer::new(filters, loader, &config);
        let mut output = String::new();
        let mut context = TemplateContext::new();
        context.set("name", CursedObject::String("Alice".to_string()));
        
        let node = TemplateNode::Variable {
            name: "name".to_string(),
            filters: vec![],
        };
        renderer.render_node(&node, &context, &mut output).unwrap();
        
        assert_eq!(output, "Alice");
    }

    #[test]
    fn test_binary_operations() {
        let filters = Arc::new(FilterRegistry::new());
        let loader = Arc::new(crate::stdlib::template::template_core::FileSystemLoader::new("templates"));
        let config = crate::stdlib::template::template_core::TemplateConfig::default();
        
        let renderer = TemplateRenderer::new(filters, loader, &config);
        
        let left = CursedObject::Integer(5);
        let right = CursedObject::Integer(3);
        
        let result = renderer.apply_binary_op(&left, &BinaryOperator::Add, &right).unwrap();
        assert_eq!(result, CursedObject::Integer(8));
        
        let result = renderer.apply_binary_op(&left, &BinaryOperator::Sub, &right).unwrap();
        assert_eq!(result, CursedObject::Integer(2));
    }

    #[test]
    fn test_is_truthy() {
        let filters = Arc::new(FilterRegistry::new());
        let loader = Arc::new(crate::stdlib::template::template_core::FileSystemLoader::new("templates"));
        let config = crate::stdlib::template::template_core::TemplateConfig::default();
        
        let renderer = TemplateRenderer::new(filters, loader, &config);
        
        assert!(renderer.is_truthy(&CursedObject::Boolean(true)));
        assert!(!renderer.is_truthy(&CursedObject::Boolean(false)));
        assert!(!renderer.is_truthy(&CursedObject::Nil));
        assert!(renderer.is_truthy(&CursedObject::Integer(1)));
        assert!(!renderer.is_truthy(&CursedObject::Integer(0)));
        assert!(renderer.is_truthy(&CursedObject::String("hello".to_string())));
        assert!(!renderer.is_truthy(&CursedObject::String("".to_string())));
    }
}
