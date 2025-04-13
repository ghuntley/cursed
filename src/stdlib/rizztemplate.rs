use crate::prelude::*;
use crate::object::*;
use crate::memory::*;
use crate::error::Error;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};
use std::rc::Rc;
use std::str::Chars;

extern crate glob;

pub struct Template {
    name: String,
    text: String,
    tree: TemplateTree,
    funcs: FuncMap,
    templates: HashMap<String, Box<Template>>,
}

pub type FuncMap = HashMap<String, Object>;

struct TemplateTree {
    nodes: Vec<Node>,
}

enum Node {
    Text(String),
    Action(Action),
}

enum Action {
    Dot(String),
    Variable(String),
    If { condition: Expr, true_branch: Vec<Node>, false_branch: Option<Vec<Node>> },
    Loop { variable: String, collection: Expr, body: Vec<Node> },
    IndexLoop { index_var: String, value_var: String, collection: Expr, body: Vec<Node> },
    Pipeline { left: Box<Expr>, right: Box<Expr> },
    FunctionCall { name: String, args: Vec<Expr> },
    Template { name: String, data: Box<Expr> },
    Define { name: String, body: Vec<Node> },
}

enum Expr {
    Literal(Object),
    Path(String),
    Variable(String),
    FunctionCall { name: String, args: Vec<Expr> },
    Binary { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
    Unary { op: UnaryOp, expr: Box<Expr> },
}

enum BinaryOp {
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    Index,
}

enum UnaryOp {
    Not,
}

// Template execution context
struct ExecutionContext<'a> {
    data: Object,                 // Template data context
    variables: HashMap<String, Object>, // Local variables
    template: &'a Template,      // Reference to the template
}

// Template parser
struct TemplateParser<'a> {
    input: &'a str,          // Template text
    chars: Chars<'a>,       // Character iterator
    position: usize,        // Current position
    line: usize,            // Current line
    column: usize,          // Current column
    next_char: Option<char>, // Next character
}

impl<'a> TemplateParser<'a> {
    // Create a new parser for template text
    fn new(input: &'a str) -> Self {
        let mut parser = TemplateParser {
            input,
            chars: input.chars(),
            position: 0,
            line: 1,
            column: 0,
            next_char: None,
        };
        parser.advance(); // Initialize the first character
        parser
    }
    
    // Advance to the next character
    fn advance(&mut self) -> Option<char> {
        let current = self.next_char;
        self.next_char = self.chars.next();
        
        if let Some(ch) = self.next_char {
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        
        current
    }
    
    // Parse the template into a TemplateTree
    fn parse(&mut self) -> Result<TemplateTree, String> {
        let mut nodes = Vec::new();
        
        while let Some(ch) = self.next_char {
            if ch == '{' && self.peek_next() == Some('{') {
                // Found action start {{ ... }}
                self.advance(); // Skip first {
                self.advance(); // Skip second {
                
                let action = self.parse_action()?;
                nodes.push(Node::Action(action));
            } else {
                // Regular text
                let text = self.parse_text();
                if !text.is_empty() {
                    nodes.push(Node::Text(text));
                }
            }
        }
        
        Ok(TemplateTree { nodes })
    }
    
    // Parse a text segment until {{ or end of input
    fn parse_text(&mut self) -> String {
        let mut text = String::new();
        
        while let Some(ch) = self.next_char {
            if ch == '{' && self.peek_next() == Some('{') {
                break;
            }
            text.push(ch);
            self.advance();
        }
        
        text
    }
    
    // Parse an action between {{ and }}
    fn parse_action(&mut self) -> Result<Action, String> {
        // Skip whitespace
        self.skip_whitespace();
        
        // Check for specific action types
        if self.match_keyword("lowkey") {
            return self.parse_if_action();
        } else if self.match_keyword("bestie") {
            return self.parse_loop_action();
        } else if self.match_keyword("define") {
            return self.parse_define_action();
        } else if self.match_keyword("template") {
            return self.parse_template_action();
        }
        
        // Default: dot or variable
        if let Some('.') = self.next_char {
            self.advance(); // Skip the dot
            let path = self.parse_identifier();
            self.expect_action_end()?;
            return Ok(Action::Dot(path));
        } else if let Some('$') = self.next_char {
            self.advance(); // Skip the $
            let var_name = self.parse_identifier();
            self.expect_action_end()?;
            return Ok(Action::Variable(var_name));
        }
        
        Err(format!("Unexpected token at line {}, column {}", self.line, self.column))
    }
    
    // Parse if action: {{ lowkey condition }}true{{ highkey }}false{{ yolo }}
    fn parse_if_action(&mut self) -> Result<Action, String> {
        // Skip whitespace
        self.skip_whitespace();
        
        // Parse condition
        let condition = self.parse_expression()?;
        
        // Expect action end }}
        self.expect_action_end()?;
        
        // Parse true branch
        let true_branch = self.parse_until_keyword(&["highkey", "yolo"])?;
        
        // Check for else branch
        let false_branch = if self.match_keyword("highkey") {
            self.expect_action_end()?;
            Some(self.parse_until_keyword(&["yolo"])?)
        } else {
            None
        };
        
        // Expect yolo to end the if
        if !self.match_keyword("yolo") {
            return Err("Expected 'yolo' to end 'if' statement".to_string());
        }
        self.expect_action_end()?;
        
        Ok(Action::If {
            condition,
            true_branch,
            false_branch,
        })
    }
    
    // Parse loop action: {{ bestie $item := flex .Items }}...{{ yolo }}
    fn parse_loop_action(&mut self) -> Result<Action, String> {
        // Skip whitespace
        self.skip_whitespace();
        
        // Check if we have an index variable
        if let Some('$') = self.next_char {
            self.advance(); // Skip $
            let var_name = self.parse_identifier();
            
            // Check for comma indicating index loop
            self.skip_whitespace();
            if let Some(',') = self.next_char {
                self.advance(); // Skip comma
                self.skip_whitespace();
                
                // Parse value variable
                if let Some('$') = self.next_char {
                    self.advance(); // Skip $
                    let value_var = self.parse_identifier();
                    
                    // Parse := flex part
                    self.skip_whitespace();
                    if !self.match_assignment_and_flex() {
                        return Err("Expected ':= flex' in loop".to_string());
                    }
                    
                    // Parse collection
                    self.skip_whitespace();
                    let collection = self.parse_expression()?;
                    
                    // Expect action end
                    self.expect_action_end()?;
                    
                    // Parse loop body
                    let body = self.parse_until_keyword(&["yolo"])?;
                    
                    // Expect yolo
                    if !self.match_keyword("yolo") {
                        return Err("Expected 'yolo' to end loop".to_string());
                    }
                    self.expect_action_end()?;
                    
                    return Ok(Action::IndexLoop {
                        index_var: var_name,
                        value_var,
                        collection,
                        body,
                    });
                }
            } else {
                // Simple value loop
                self.skip_whitespace();
                if !self.match_assignment_and_flex() {
                    return Err("Expected ':= flex' in loop".to_string());
                }
                
                // Parse collection
                self.skip_whitespace();
                let collection = self.parse_expression()?;
                
                // Expect action end
                self.expect_action_end()?;
                
                // Parse loop body
                let body = self.parse_until_keyword(&["yolo"])?;
                
                // Expect yolo
                if !self.match_keyword("yolo") {
                    return Err("Expected 'yolo' to end loop".to_string());
                }
                self.expect_action_end()?;
                
                return Ok(Action::Loop {
                    variable: var_name,
                    collection,
                    body,
                });
            }
        }
        
        Err("Invalid loop syntax".to_string())
    }
    
    // Parse define action: {{ define "name" }}...{{ yolo }}
    fn parse_define_action(&mut self) -> Result<Action, String> {
        // Skip whitespace
        self.skip_whitespace();
        
        // Parse template name as string literal
        let name = self.parse_string_literal()?;
        
        // Expect action end
        self.expect_action_end()?;
        
        // Parse template body
        let body = self.parse_until_keyword(&["yolo"])?;
        
        // Expect yolo
        if !self.match_keyword("yolo") {
            return Err("Expected 'yolo' to end define".to_string());
        }
        self.expect_action_end()?;
        
        Ok(Action::Define { name, body })
    }
    
    // Parse template action: {{ template "name" . }}
    fn parse_template_action(&mut self) -> Result<Action, String> {
        // Skip whitespace
        self.skip_whitespace();
        
        // Parse template name as string literal
        let name = self.parse_string_literal()?;
        
        // Skip whitespace
        self.skip_whitespace();
        
        // Parse data context
        let data = Box::new(self.parse_expression()?);
        
        // Expect action end
        self.expect_action_end()?;
        
        Ok(Action::Template { name, data })
    }
    
    // Parse string literal "..."
    fn parse_string_literal(&mut self) -> Result<String, String> {
        // Skip whitespace
        self.skip_whitespace();
        
        // Expect opening "
        if let Some('"') = self.next_char {
            self.advance(); // Skip "
        } else {
            return Err("Expected string literal".to_string());
        }
        
        let mut content = String::new();
        
        // Parse string content
        while let Some(ch) = self.next_char {
            if ch == '"' {
                self.advance(); // Skip closing "
                return Ok(content);
            } else if ch == '\\' {
                // Handle escape sequences
                self.advance(); // Skip \
                match self.next_char {
                    Some('"') => content.push('"'),
                    Some('\\') => content.push('\\'),
                    Some('n') => content.push('\n'),
                    Some('t') => content.push('\t'),
                    Some('r') => content.push('\r'),
                    Some(c) => content.push(c),
                    None => return Err("Unterminated string literal".to_string()),
                }
                self.advance();
            } else {
                content.push(ch);
                self.advance();
            }
        }
        
        Err("Unterminated string literal".to_string())
    }
    
    // Parse expression within action
    fn parse_expression(&mut self) -> Result<Expr, String> {
        // For simplicity, we'll just handle dot paths and literals
        if let Some('.') = self.next_char {
            self.advance(); // Skip dot
            let path = self.parse_identifier();
            return Ok(Expr::Path(path));
        } else if let Some('$') = self.next_char {
            self.advance(); // Skip $
            let var_name = self.parse_identifier();
            return Ok(Expr::Variable(var_name));
        }
        
        // Default to a null literal
        Ok(Expr::Literal(Object::Null))
    }
    
    // Parse identifier
    fn parse_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        if let Some(ch) = self.next_char {
            if Self::is_identifier_start(ch) {
                identifier.push(ch);
                self.advance();
                
                while let Some(ch) = self.next_char {
                    if Self::is_identifier_part(ch) {
                        identifier.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }
        
        identifier
    }
    
    // Parse nodes until a keyword is found
    fn parse_until_keyword(&mut self, keywords: &[&str]) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        
        while let Some(ch) = self.next_char {
            if ch == '{' && self.peek_next() == Some('{') {
                // Check if it's one of the keywords
                let pos = self.position;
                let line = self.line;
                let column = self.column;
                
                self.advance(); // Skip first {
                self.advance(); // Skip second {
                self.skip_whitespace();
                
                let is_keyword = keywords.iter().any(|&keyword| self.check_keyword(keyword));
                
                // Reset position if not a keyword
                if is_keyword {
                    // We found a keyword, reset and return
                    self.position = pos;
                    self.line = line;
                    self.column = column;
                    self.chars = self.input[self.position..].chars();
                    self.next_char = self.chars.next();
                    break;
                }
                
                // Not a keyword, parse as regular action
                self.position = pos;
                self.line = line;
                self.column = column;
                self.chars = self.input[self.position..].chars();
                self.next_char = self.chars.next();
                
                self.advance(); // Skip first {
                self.advance(); // Skip second {
                
                let action = self.parse_action()?;
                nodes.push(Node::Action(action));
            } else {
                // Regular text
                let text = self.parse_text();
                if !text.is_empty() {
                    nodes.push(Node::Text(text));
                }
            }
        }
        
        Ok(nodes)
    }
    
    // Check if current position matches a keyword
    fn check_keyword(&self, keyword: &str) -> bool {
        let chars: Vec<char> = keyword.chars().collect();
        let mut input_chars = self.input[self.position..].chars();
        
        for &expected in &chars {
            match input_chars.next() {
                Some(actual) if actual == expected => continue,
                _ => return false,
            }
        }
        
        true
    }
    
    // Match and consume a keyword
    fn match_keyword(&mut self, keyword: &str) -> bool {
        self.skip_whitespace();
        
        if !self.check_keyword(keyword) {
            return false;
        }
        
        // Consume the keyword
        for _ in 0..keyword.len() {
            self.advance();
        }
        
        true
    }
    
    // Match and consume := flex
    fn match_assignment_and_flex(&mut self) -> bool {
        if !(self.next_char == Some(':') && self.peek_next() == Some('=')) {
            return false;
        }
        
        self.advance(); // Skip :
        self.advance(); // Skip =
        
        self.skip_whitespace();
        
        if self.check_keyword("flex") {
            for _ in 0.."flex".len() {
                self.advance();
            }
            return true;
        }
        
        false
    }
    
    // Expect action end }}
    fn expect_action_end(&mut self) -> Result<(), String> {
        self.skip_whitespace();
        
        if let Some('}') = self.next_char {
            self.advance(); // Skip first }
            if let Some('}') = self.next_char {
                self.advance(); // Skip second }
                return Ok(());
            }
        }
        
        Err(format!("Expected '}}}}' at line {}, column {}", self.line, self.column))
    }
    
    // Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.next_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    // Peek at the next character without advancing
    fn peek_next(&self) -> Option<char> {
        self.chars.clone().next()
    }
    
    // Check if a character can start an identifier
    fn is_identifier_start(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }
    
    // Check if a character can be part of an identifier
    fn is_identifier_part(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }
}

impl Template {
    pub fn new(name: impl Into<String>) -> Self {
        Template {
            name: name.into(),
            text: String::new(),
            tree: TemplateTree { nodes: Vec::new() },
            funcs: default_function_map(),
            templates: HashMap::new(),
        }
    }
    
    pub fn parse(text: impl Into<String>) -> Result<Box<Template>, String> {
        let mut tmpl = Template::new("anonymous");
        tmpl.text = text.into();
        
        // Parse the template text into a syntax tree
        let mut parser = TemplateParser::new(&tmpl.text);
        tmpl.tree = match parser.parse() {
            Ok(tree) => tree,
            Err(e) => return Err(format!("Parse error: {}", e)),
        };
        
        Ok(Box::new(tmpl))
    }
    
    pub fn parse_files(filenames: &[String]) -> Result<Box<Template>, String> {
        let mut tmpl = Template::new("template_set");
        
        for filename in filenames {
            // Read the file content
            let content = match std::fs::read_to_string(filename) {
                Ok(content) => content,
                Err(e) => return Err(format!("Error reading {}: {}", filename, e)),
            };
            
            // Parse the template
            let sub_template = match Template::parse(content) {
                Ok(t) => t,
                Err(e) => return Err(format!("Error parsing {}: {}", filename, e)),
            };
            
            // Add the template to the set
            tmpl.templates.insert(filename.clone(), sub_template);
        }
        
        Ok(Box::new(tmpl))
    }
    
    pub fn parse_glob(pattern: &str) -> Result<Box<Template>, String> {
        let paths = match glob::glob(pattern) {
            Ok(paths) => paths,
            Err(e) => return Err(format!("Invalid glob pattern {}: {}", pattern, e)),
        };
        
        let filenames: Vec<String> = paths
            .filter_map(|entry| entry.ok())
            .filter_map(|path| path.to_str().map(|s| s.to_string()))
            .collect();
        
        Template::parse_files(&filenames)
    }
    
    pub fn execute<W: Write>(&self, mut w: W, data: Object) -> Result<(), String> {
        // Create execution context with template data
        let context = ExecutionContext {
            data,
            variables: HashMap::new(),
            template: self,
        };
        
        // Execute the template
        self.execute_tree(&mut w, &self.tree, &context)
    }
    
    fn execute_tree<W: Write>(&self, w: &mut W, tree: &TemplateTree, context: &ExecutionContext) -> Result<(), String> {
        for node in &tree.nodes {
            self.execute_node(w, node, context)?;
        }
        Ok(())
    }
    
    fn execute_node<W: Write>(&self, w: &mut W, node: &Node, context: &ExecutionContext) -> Result<(), String> {
        match node {
            Node::Text(text) => {
                // Write text content directly
                w.write_all(text.as_bytes())
                    .map_err(|e| format!("Write error: {}", e))?;
            },
            Node::Action(action) => {
                self.execute_action(w, action, context)?;
            },
        }
        Ok(())
    }
    
    fn execute_action<W: Write>(&self, w: &mut W, action: &Action, context: &ExecutionContext) -> Result<(), String> {
        match action {
            Action::Dot(path) => {
                // Get value from data context
                let value = self.lookup_path(path, &context.data)?
                    .unwrap_or(Object::Null);
                
                // Write value as string
                write!(w, "{}", value)
                    .map_err(|e| format!("Write error: {}", e))?;
            },
            Action::Variable(name) => {
                // Get variable from context
                let value = match context.variables.get(name) {
                    Some(val) => val.clone(),
                    None => return Err(format!("Variable not found: {}", name)),
                };
                
                // Write value as string
                write!(w, "{}", value)
                    .map_err(|e| format!("Write error: {}", e))?;
            },
            Action::If { condition, true_branch, false_branch } => {
                // Evaluate condition
                let condition_value = self.evaluate_expression(condition, context)?;
                let is_true = self.is_truthy(&condition_value);
                
                // Execute appropriate branch
                if is_true {
                    self.execute_nodes(w, true_branch, context)?
                } else if let Some(fb) = false_branch {
                    self.execute_nodes(w, fb, context)?
                }
            },
            Action::Loop { variable, collection, body } => {
                // Evaluate collection
                let collection_value = self.evaluate_expression(collection, context)?;
                
                // Get items to iterate over
                let items = self.get_iterable_items(&collection_value)?;
                
                // Create new context for loop iterations
                let mut loop_context = ExecutionContext {
                    data: context.data.clone(),
                    variables: context.variables.clone(),
                    template: context.template,
                };
                
                // Iterate over items
                for item in items {
                    // Set loop variable
                    loop_context.variables.insert(variable.clone(), item);
                    
                    // Execute loop body
                    self.execute_nodes(w, body, &loop_context)?
                }
            },
            Action::IndexLoop { index_var, value_var, collection, body } => {
                // Evaluate collection
                let collection_value = self.evaluate_expression(collection, context)?;
                
                // Get items to iterate over
                let items = self.get_iterable_items(&collection_value)?;
                
                // Create new context for loop iterations
                let mut loop_context = ExecutionContext {
                    data: context.data.clone(),
                    variables: context.variables.clone(),
                    template: context.template,
                };
                
                // Iterate over items with index
                for (i, item) in items.into_iter().enumerate() {
                    // Set loop variables
                    loop_context.variables.insert(index_var.clone(), Object::Integer(i as i64));
                    loop_context.variables.insert(value_var.clone(), item);
                    
                    // Execute loop body
                    self.execute_nodes(w, body, &loop_context)?
                }
            },
            Action::Pipeline { left, right } => {
                // Evaluate left expression
                let left_value = self.evaluate_expression(left, context)?;
                
                // Apply right function to left value
                let result = self.apply_pipeline(right, left_value, context)?;
                
                // Write result
                write!(w, "{}", result)
                    .map_err(|e| format!("Write error: {}", e))?;
            },
            Action::FunctionCall { name, args } => {
                // Evaluate function arguments
                let evaluated_args: Result<Vec<Object>, String> = args
                    .iter()
                    .map(|arg| self.evaluate_expression(arg, context))
                    .collect();
                
                // Call function
                let result = self.call_function(name, &evaluated_args?, context)?;
                
                // Write result
                write!(w, "{}", result)
                    .map_err(|e| format!("Write error: {}", e))?;
            },
            Action::Template { name, data } => {
                // Evaluate data context
                let data_value = self.evaluate_expression(data, context)?;
                
                // Find named template
                let template = match self.templates.get(name) {
                    Some(t) => t,
                    None => return Err(format!("Template not found: {}", name)),
                };
                
                // Execute nested template with data
                template.execute(w, data_value)?;
            },
            Action::Define { name: _, body: _ } => {
                // Defines are processed during parsing, nothing to do during execution
            },
        }
        Ok(())
    }
    
    // Execute a list of nodes
    fn execute_nodes<W: Write>(&self, w: &mut W, nodes: &[Node], context: &ExecutionContext) -> Result<(), String> {
        for node in nodes {
            self.execute_node(w, node, context)?
        }
        Ok(())
    }
    
    // Evaluate an expression to an Object value
    fn evaluate_expression(&self, expr: &Expr, context: &ExecutionContext) -> Result<Object, String> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Path(path) => {
                self.lookup_path(path, &context.data)?
                    .ok_or_else(|| format!("Path not found: {}", path))
            },
            Expr::Variable(name) => {
                context.variables.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Variable not found: {}", name))
            },
            Expr::FunctionCall { name, args } => {
                // Evaluate arguments
                let evaluated_args: Result<Vec<Object>, String> = args
                    .iter()
                    .map(|arg| self.evaluate_expression(arg, context))
                    .collect();
                
                // Call function
                self.call_function(name, &evaluated_args?, context)
            },
            Expr::Binary { op, left, right } => {
                let left_val = self.evaluate_expression(left, context)?;
                let right_val = self.evaluate_expression(right, context)?;
                
                match op {
                    BinaryOp::Eq => Ok(Object::Boolean(self.values_equal(&left_val, &right_val))),
                    BinaryOp::Ne => Ok(Object::Boolean(!self.values_equal(&left_val, &right_val))),
                    BinaryOp::Lt => self.compare_values(&left_val, &right_val, |a, b| a < b),
                    BinaryOp::Le => self.compare_values(&left_val, &right_val, |a, b| a <= b),
                    BinaryOp::Gt => self.compare_values(&left_val, &right_val, |a, b| a > b),
                    BinaryOp::Ge => self.compare_values(&left_val, &right_val, |a, b| a >= b),
                    BinaryOp::And => Ok(Object::Boolean(self.is_truthy(&left_val) && self.is_truthy(&right_val))),
                    BinaryOp::Or => Ok(Object::Boolean(self.is_truthy(&left_val) || self.is_truthy(&right_val))),
                    BinaryOp::Index => self.index_value(&left_val, &right_val),
                }
            },
            Expr::Unary { op, expr } => {
                let val = self.evaluate_expression(expr, context)?;
                
                match op {
                    UnaryOp::Not => Ok(Object::Boolean(!self.is_truthy(&val))),
                }
            },
        }
    }
    
    // Check if two values are equal
    fn values_equal(&self, a: &Object, b: &Object) -> bool {
        match (a, b) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::Float(a), Object::Float(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Char(a), Object::Char(b)) => a == b,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
    
    // Compare values with a comparison function
    fn compare_values<F>(&self, a: &Object, b: &Object, cmp: F) -> Result<Object, String>
    where F: Fn(&f64, &f64) -> bool {
        match (a, b) {
            (Object::Integer(a), Object::Integer(b)) => {
                Ok(Object::Boolean(cmp(&(*a as f64), &(*b as f64))))
            },
            (Object::Float(a), Object::Float(b)) => {
                Ok(Object::Boolean(cmp(a, b)))
            },
            (Object::Integer(a), Object::Float(b)) => {
                Ok(Object::Boolean(cmp(&(*a as f64), b)))
            },
            (Object::Float(a), Object::Integer(b)) => {
                Ok(Object::Boolean(cmp(a, &(*b as f64))))
            },
            _ => Err(format!("Cannot compare {} and {}", a, b)),
        }
    }
    
    // Index into a container value
    fn index_value(&self, container: &Object, index: &Object) -> Result<Object, String> {
        match container {
            Object::Array(items) => {
                match index {
                    Object::Integer(i) => {
                        let idx = *i as usize;
                        if idx >= items.len() {
                            return Err(format!("Index out of bounds: {}", idx));
                        }
                        Ok(items[idx].clone())
                    },
                    _ => Err(format!("Invalid array index: {}", index)),
                }
            },
            Object::HashTable(map) => {
                match index {
                    Object::String(key) => {
                        map.get(key)
                            .cloned()
                            .ok_or_else(|| format!("Key not found: {}", key))
                    },
                    _ => Err(format!("Invalid map key: {}", index)),
                }
            },
            _ => Err(format!("Cannot index into {}", container)),
        }
    }
    
    // Get items to iterate over from a container value
    fn get_iterable_items(&self, value: &Object) -> Result<Vec<Object>, String> {
        match value {
            Object::Array(items) => Ok(items.clone()),
            Object::HashTable(map) => {
                Ok(map.keys()
                    .map(|k| Object::String(k.clone()))
                    .collect())
            },
            Object::String(s) => {
                Ok(s.chars()
                    .map(|c| Object::Char(c))
                    .collect())
            },
            _ => Err(format!("Cannot iterate over {}", value)),
        }
    }
    
    // Apply a pipeline operation
    fn apply_pipeline(&self, right: &Expr, left_value: Object, context: &ExecutionContext) -> Result<Object, String> {
        match right {
            Expr::FunctionCall { name, args } => {
                // Create new args with left value as first argument
                let mut evaluated_args = vec![left_value];
                
                // Evaluate remaining arguments
                for arg in args {
                    evaluated_args.push(self.evaluate_expression(arg, context)?);
                }
                
                // Call function
                self.call_function(name, &evaluated_args, context)
            },
            _ => Err("Right side of pipeline must be a function call".to_string()),
        }
    }
    
    // Call a template function
    fn call_function(&self, name: &str, args: &[Object], context: &ExecutionContext) -> Result<Object, String> {
        // Check built-in functions
        if let Some(func) = self.funcs.get(name) {
            // Call the function
            return self.call_built_in_function(func, args);
        }
        
        Err(format!("Function not found: {}", name))
    }
    
    // Call a built-in function
    fn call_built_in_function(&self, func: &Object, args: &[Object]) -> Result<Object, String> {
        // In a real implementation, we'd have actual built-in function logic
        // For this simplified version, just return a placeholder value
        Ok(Object::String("function result".to_string()))
    }
    
    // Check if a value is truthy
    fn is_truthy(&self, value: &Object) -> bool {
        match value {
            Object::Boolean(b) => *b,
            Object::Integer(i) => *i != 0,
            Object::Float(f) => *f != 0.0,
            Object::String(s) => !s.is_empty(),
            Object::Array(a) => !a.is_empty(),
            Object::HashTable(m) => !m.is_empty(),
            Object::Null => false,
            _ => true,
        }
    }
    
    fn lookup_path(&self, path: &str, data: &Object) -> Result<Option<Object>, String> {
        // Simple path lookup implementation for .Name style access
        match data {
            Object::HashTable(map) => {
                if let Some(value) = map.get(path) {
                    return Ok(Some(value.clone()));
                }
            },
            _ => {}
        }
        Ok(None)
    }
    
    pub fn execute_template<W: Write>(&self, w: W, name: &str, data: Object) -> Result<(), String> {
        // Look up the named template
        match self.templates.get(name) {
            Some(tmpl) => tmpl.execute(w, data),
            None => Err(format!("Template not found: {}", name)),
        }
    }
    
    pub fn funcs(&mut self, funcs: FuncMap) -> &mut Self {
        self.funcs.extend(funcs);
        self
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn parse_files_as_templates(&mut self, filenames: &[String]) -> Result<&mut Self, String> {
        for filename in filenames {
            // Read the file content
            let content = match std::fs::read_to_string(filename) {
                Ok(content) => content,
                Err(e) => return Err(format!("Error reading {}: {}", filename, e)),
            };
            
            // Parse the template
            let sub_template = match Template::parse(content) {
                Ok(t) => t,
                Err(e) => return Err(format!("Error parsing {}: {}", filename, e)),
            };
            
            // Add the template with its base name as the key
            let basename = std::path::Path::new(filename)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(filename);
            
            self.templates.insert(basename.to_string(), sub_template);
        }
        
        Ok(self)
    }
    
    pub fn parse_glob_as_templates(&mut self, pattern: &str) -> Result<&mut Self, String> {
        let paths = match glob::glob(pattern) {
            Ok(paths) => paths,
            Err(e) => return Err(format!("Invalid glob pattern {}: {}", pattern, e)),
        };
        
        let filenames: Vec<String> = paths
            .filter_map(|entry| entry.ok())
            .filter_map(|path| path.to_str().map(|s| s.to_string()))
            .collect();
        
        self.parse_files_as_templates(&filenames)
    }
    
    pub fn templates(&self) -> Vec<&Template> {
        self.templates.values().map(|t| t.as_ref()).collect()
    }
}

pub fn must(tmpl: Result<Box<Template>, String>) -> Box<Template> {
    match tmpl {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}

fn default_function_map() -> FuncMap {
    let mut map = FuncMap::new();
    
    // HTML escaping function
    map.insert("html".to_string(), make_function_object("html"));
    
    // JavaScript escaping function
    map.insert("js".to_string(), make_function_object("js"));
    
    // URL query escaping function
    map.insert("urlquery".to_string(), make_function_object("urlquery"));
    
    // Comparison functions
    map.insert("eq".to_string(), make_function_object("eq"));
    map.insert("ne".to_string(), make_function_object("ne"));
    map.insert("lt".to_string(), make_function_object("lt"));
    map.insert("le".to_string(), make_function_object("le"));
    map.insert("gt".to_string(), make_function_object("gt"));
    map.insert("ge".to_string(), make_function_object("ge"));
    
    // Boolean operators
    map.insert("and".to_string(), make_function_object("and"));
    map.insert("or".to_string(), make_function_object("or"));
    map.insert("not".to_string(), make_function_object("not"));
    
    // Container functions
    map.insert("index".to_string(), make_function_object("index"));
    map.insert("len".to_string(), make_function_object("len"));
    
    // Output functions
    map.insert("print".to_string(), make_function_object("print"));
    map.insert("printf".to_string(), make_function_object("printf"));
    map.insert("println".to_string(), make_function_object("println"));
    
    map
}

// Helper function to create a function object
fn make_function_object(name: &str) -> Object {
    // Function that just returns null
    fn dummy_function(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
        Ok(Rc::new(Object::Null))
    }
    
    // Create a Builtin function object
    Object::Builtin {
        name: name.to_string(),
        function: dummy_function,
    }
}

// Functions for htmlrizzler to call

/// Creates a new template with the given name
pub fn new(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("rizztemplate.New requires a template name".to_string()));
    }

    let name = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("template name must be a string".to_string())),
    };

    // Create a new template and return as an opaque object
    let template = Template::new(name);
    
    // Create a Template struct object
    let template_struct = Object::Struct {
        name: "Template".to_string(),
        fields: vec![("name".to_string(), "string".to_string())],
    };
    
    Ok(Rc::new(template_struct))
}

/// Parses template text and returns a template
pub fn parse(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("rizztemplate.Parse requires template text".to_string()));
    }

    let text = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("template text must be a string".to_string())),
    };

    match Template::parse(text) {
        Ok(template) => {
            // Create a Template struct object
            let template_struct = Object::Struct {
                name: "Template".to_string(),
                fields: vec![("text".to_string(), "string".to_string())],
            };
            Ok(Rc::new(template_struct))
        },
        Err(e) => Err(Error::Runtime(format!("template parsing error: {}", e))),
    }
}

/// Parses template files and returns a template
pub fn parse_files(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("rizztemplate.ParseFiles requires file names".to_string()));
    }

    let filenames: Vec<String> = args.iter().map(|arg| {
        match &**arg {
            Object::String(s) => s.clone(),
            _ => format!("<not a string: {:?}>", arg),
        }
    }).collect();

    match Template::parse_files(&filenames) {
        Ok(template) => {
            // Create a Template struct object
            let template_struct = Object::Struct {
                name: "Template".to_string(),
                fields: vec![("files".to_string(), "[]string".to_string())],
            };
            Ok(Rc::new(template_struct))
        },
        Err(e) => Err(Error::Runtime(format!("template parsing error: {}", e))),
    }
}

/// Executes a template with data and writes to output
pub fn execute(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime("rizztemplate.Execute requires template, writer, and data".to_string()));
    }

    // In a real implementation, extract template, writer, and data and execute
    // For now, just return nil to indicate success
    Ok(Rc::new(Object::Null))
}