//! CURSED Code Formatter Module
//! 
//! AST-based code formatter with configurable options for consistent code style.

use crate::ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::error::CursedError;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

pub mod config;
pub mod output;
pub mod rules;
pub mod simple;

#[cfg(test)]
mod tests;

pub use config::*;
pub use output::*;
// pub use rules::*;  // Disabled due to AST compatibility issues
pub use simple::*;

/// Main formatter struct
#[derive(Debug, Clone)]
pub struct CursedFormatter {
    pub config: FormatterConfig,
    pub current_indent: usize,
    pub current_line_length: usize,
    pub output: Vec<String>,
    pub comments: HashMap<usize, Vec<String>>,
}

impl Default for CursedFormatter {
    fn default() -> Self {
        Self {
            config: FormatterConfig::default(),
            current_indent: 0,
            current_line_length: 0,
            output: Vec::new(),
            comments: HashMap::new(),
        }
    }
}

impl CursedFormatter {
    /// Create a new formatter with custom configuration
    pub fn new(config: FormatterConfig) -> Self {
        Self {
            config,
            current_indent: 0,
            current_line_length: 0,
            output: Vec::new(),
            comments: HashMap::new(),
        }
    }

    /// Load configuration from file
    pub fn with_config_file<P: AsRef<Path>>(path: P) -> Result<Self, CursedError> {
        let content = fs::read_to_string(path)?;
        let config: FormatterConfig = toml::from_str(&content)
            .map_err(|e| CursedError::ConfigError(format!("Failed to parse config: {}", e)))?;
        Ok(Self::new(config))
    }

    /// Format source code
    pub fn format(&mut self, source: &str) -> Result<String, CursedError> {
        // Reset state
        self.current_indent = 0;
        self.current_line_length = 0;
        self.output.clear();
        self.comments.clear();

        // Extract comments before parsing
        self.extract_comments(source)?;

        // Parse the source code
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse()?;

        // Format the AST
        self.format_program(&program)?;

        Ok(self.output.join("\n"))
    }

    /// Format a program node
    fn format_program(&mut self, program: &Program) -> Result<(), CursedError> {
        // Format package declaration
        if let Some(package) = &program.package {
            self.format_package_declaration(package)?;
            self.add_blank_lines(1);
        }

        // Format imports
        if !program.imports.is_empty() {
            self.format_imports(&program.imports)?;
            self.add_blank_lines(1);
        }

        // Format statements
        for (i, statement) in program.statements.iter().enumerate() {
            if i > 0 {
                self.add_blank_lines(self.config.blank_lines_between_statements);
            }
            self.format_statement(statement)?;
        }

        Ok(())
    }

    /// Format package declaration
    fn format_package_declaration(&mut self, package: &PackageDeclaration) -> Result<(), CursedError> {
        let mut line = format!("vibe {}", package.name);
        if let Some(version) = &package.version {
            line.push_str(&format!(" v{}", version));
        }
        self.add_line(&line);
        Ok(())
    }

    /// Format import statements
    fn format_imports(&mut self, imports: &[ImportStatement]) -> Result<(), CursedError> {
        if self.config.group_imports {
            self.format_grouped_imports(imports)?;
        } else {
            for import in imports {
                self.format_import(import)?;
            }
        }
        Ok(())
    }

    /// Format grouped imports
    fn format_grouped_imports(&mut self, imports: &[ImportStatement]) -> Result<(), CursedError> {
        if imports.len() > 1 {
            self.add_line("yeet (");
            self.increase_indent();
            for import in imports {
                self.format_import_in_group(import)?;
            }
            self.decrease_indent();
            self.add_line(")");
        } else if imports.len() == 1 {
            self.format_import(&imports[0])?;
        }
        Ok(())
    }

    /// Format single import
    fn format_import(&mut self, import: &ImportStatement) -> Result<(), CursedError> {
        let mut line = format!("yeet \"{}\"", import.path);
        if let Some(alias) = &import.alias {
            line.push_str(&format!(" as {}", alias));
        }
        self.add_line(&line);
        Ok(())
    }

    /// Format import within a group
    fn format_import_in_group(&mut self, import: &ImportStatement) -> Result<(), CursedError> {
        let mut line = format!("\"{}\"", import.path);
        if let Some(alias) = &import.alias {
            line.push_str(&format!(" as {}", alias));
        }
        self.add_line(&line);
        Ok(())
    }

    /// Format a statement
    fn format_statement(&mut self, statement: &Statement) -> Result<(), CursedError> {
        match statement {
            Statement::Expression(expr) => {
                let formatted = self.format_expression(expr)?;
                self.add_line(&formatted);
            }
            Statement::Let(let_stmt) => self.format_let_statement(let_stmt)?,
            Statement::Assignment(assign_stmt) => self.format_assignment_statement(assign_stmt)?,
            Statement::Return(return_stmt) => self.format_return_statement(return_stmt)?,
            Statement::If(if_stmt) => self.format_if_statement(if_stmt)?,
            Statement::Function(func_stmt) => self.format_function_statement(func_stmt)?,
            Statement::While(while_stmt) => self.format_while_statement(while_stmt)?,
            Statement::For(for_stmt) => self.format_for_statement(for_stmt)?,
            Statement::ForIn(for_in_stmt) => self.format_for_in_statement(for_in_stmt)?,
            Statement::Switch(switch_stmt) => self.format_switch_statement(switch_stmt)?,
            Statement::Goroutine(goroutine_stmt) => self.format_goroutine_statement(goroutine_stmt)?,
            Statement::Channel(channel_stmt) => self.format_channel_statement(channel_stmt)?,
            Statement::Select(select_stmt) => self.format_select_statement(select_stmt)?,
            Statement::Struct(struct_stmt) => self.format_struct_statement(struct_stmt)?,
            Statement::Interface(interface_stmt) => self.format_interface_statement(interface_stmt)?,
            Statement::Panic(panic_stmt) => self.format_panic_statement(panic_stmt)?,
            Statement::Catch(catch_stmt) => self.format_catch_statement(catch_stmt)?,
            Statement::Defer(defer_stmt) => self.format_defer_statement(defer_stmt)?,
            Statement::Break(break_stmt) => self.format_break_statement(break_stmt)?,
            Statement::Continue(continue_stmt) => self.format_continue_statement(continue_stmt)?,
            Statement::Increment(inc_stmt) => self.format_increment_statement(inc_stmt)?,
            Statement::Decrement(dec_stmt) => self.format_decrement_statement(dec_stmt)?,
            Statement::ShortDeclaration(short_decl) => self.format_short_declaration(short_decl)?,
            Statement::Yikes(yikes_stmt) => self.format_yikes_statement(yikes_stmt)?,
            Statement::Fam(fam_stmt) => self.format_fam_statement(fam_stmt)?,
        }
        Ok(())
    }

    /// Format a let statement
    fn format_let_statement(&mut self, let_stmt: &LetStatement) -> Result<(), CursedError> {
        let mut line = format!("sus {} ", let_stmt.name);
        
        if let Some(type_name) = &let_stmt.type_name {
            line.push_str(&format!("{} ", type_name));
        }
        
        line.push_str("= ");
        line.push_str(&self.format_expression(&let_stmt.value)?);
        
        self.add_line(&line);
        Ok(())
    }

    /// Format an assignment statement
    fn format_assignment_statement(&mut self, assign_stmt: &AssignmentStatement) -> Result<(), CursedError> {
        let left = self.format_expression(&assign_stmt.left)?;
        let right = self.format_expression(&assign_stmt.right)?;
        let line = format!("{} = {}", left, right);
        self.add_line(&line);
        Ok(())
    }

    /// Format a return statement
    fn format_return_statement(&mut self, return_stmt: &ReturnStatement) -> Result<(), CursedError> {
        let mut line = "damn".to_string();
        if let Some(value) = &return_stmt.value {
            line.push(' ');
            line.push_str(&self.format_expression(value)?);
        }
        self.add_line(&line);
        Ok(())
    }

    /// Format an if statement
    fn format_if_statement(&mut self, if_stmt: &IfStatement) -> Result<(), CursedError> {
        let condition = self.format_expression(&if_stmt.condition)?;
        self.add_line(&format!("nah {} {{", condition));
        
        self.increase_indent();
        for stmt in &if_stmt.then_branch {
            self.format_statement(stmt)?;
        }
        self.decrease_indent();
        
        if let Some(else_branch) = &if_stmt.else_branch {
            self.add_line("} lowkey {");
            self.increase_indent();
            for stmt in else_branch {
                self.format_statement(stmt)?;
            }
            self.decrease_indent();
        }
        
        self.add_line("}");
        Ok(())
    }

    /// Format a function statement
    fn format_function_statement(&mut self, func_stmt: &FunctionStatement) -> Result<(), CursedError> {
        let mut line = format!("slay {}", func_stmt.name);
        
        // Parameters
        line.push('(');
        for (i, param) in func_stmt.parameters.iter().enumerate() {
            if i > 0 {
                line.push_str(", ");
            }
            line.push_str(&format!("{} {}", param.name, param.type_name));
        }
        line.push(')');
        
        // Return type
        if let Some(return_type) = &func_stmt.return_type {
            line.push_str(&format!(" {} ", return_type));
        } else {
            line.push(' ');
        }
        
        line.push('{');
        self.add_line(&line);
        
        // Body
        self.increase_indent();
        for stmt in &func_stmt.body {
            self.format_statement(stmt)?;
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format a while statement
    fn format_while_statement(&mut self, while_stmt: &WhileStatement) -> Result<(), CursedError> {
        let condition = self.format_expression(&while_stmt.condition)?;
        self.add_line(&format!("lol {} {{", condition));
        
        self.increase_indent();
        for stmt in &while_stmt.body {
            self.format_statement(stmt)?;
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format a for statement
    fn format_for_statement(&mut self, for_stmt: &ForStatement) -> Result<(), CursedError> {
        let mut line = "bestie ".to_string();
        
        if let Some(init) = &for_stmt.init {
            line.push_str(&self.format_expression(init)?);
        }
        line.push_str("; ");
        
        if let Some(condition) = &for_stmt.condition {
            line.push_str(&self.format_expression(condition)?);
        }
        line.push_str("; ");
        
        if let Some(update) = &for_stmt.update {
            line.push_str(&self.format_expression(update)?);
        }
        line.push_str(" {");
        
        self.add_line(&line);
        
        self.increase_indent();
        for stmt in &for_stmt.body {
            self.format_statement(stmt)?;
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format a for-in statement
    fn format_for_in_statement(&mut self, for_in_stmt: &ForInStatement) -> Result<(), CursedError> {
        let iterable = self.format_expression(&for_in_stmt.iterable)?;
        let line = format!("bestie {} in {} {{", for_in_stmt.variable, iterable);
        self.add_line(&line);
        
        self.increase_indent();
        for stmt in &for_in_stmt.body {
            self.format_statement(stmt)?;
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format a switch statement
    fn format_switch_statement(&mut self, switch_stmt: &SwitchStatement) -> Result<(), CursedError> {
        let expr = self.format_expression(&switch_stmt.expression)?;
        self.add_line(&format!("periodt {} {{", expr));
        
        self.increase_indent();
        for case in &switch_stmt.cases {
            let case_expr = self.format_expression(&case.value)?;
            self.add_line(&format!("case {}:", case_expr));
            
            self.increase_indent();
            for stmt in &case.body {
                self.format_statement(stmt)?;
            }
            self.decrease_indent();
        }
        
        if let Some(default) = &switch_stmt.default {
            self.add_line("default:");
            self.increase_indent();
            for stmt in default {
                self.format_statement(stmt)?;
            }
            self.decrease_indent();
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format a goroutine statement
    fn format_goroutine_statement(&mut self, goroutine_stmt: &GoroutineStatement) -> Result<(), CursedError> {
        let call = self.format_expression(&goroutine_stmt.call)?;
        self.add_line(&format!("yolo {}", call));
        Ok(())
    }

    /// Format a channel statement
    fn format_channel_statement(&mut self, channel_stmt: &ChannelStatement) -> Result<(), CursedError> {
        let line = format!("sus {} chan {} = make(chan {})", 
            channel_stmt.name, 
            channel_stmt.type_name, 
            channel_stmt.type_name);
        self.add_line(&line);
        Ok(())
    }

    /// Format a select statement
    fn format_select_statement(&mut self, select_stmt: &SelectStatement) -> Result<(), CursedError> {
        self.add_line("ready {");
        
        self.increase_indent();
        for case in &select_stmt.cases {
            let case_expr = self.format_expression(&case.expression)?;
            self.add_line(&format!("case {}:", case_expr));
            
            self.increase_indent();
            for stmt in &case.body {
                self.format_statement(stmt)?;
            }
            self.decrease_indent();
        }
        
        if let Some(default) = &select_stmt.default {
            self.add_line("default:");
            self.increase_indent();
            for stmt in default {
                self.format_statement(stmt)?;
            }
            self.decrease_indent();
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format a struct statement
    fn format_struct_statement(&mut self, struct_stmt: &StructStatement) -> Result<(), CursedError> {
        self.add_line(&format!("flex {} {{", struct_stmt.name));
        
        self.increase_indent();
        for field in &struct_stmt.fields {
            self.add_line(&format!("{} {}", field.name, field.type_name));
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format an interface statement
    fn format_interface_statement(&mut self, interface_stmt: &InterfaceStatement) -> Result<(), CursedError> {
        self.add_line(&format!("vibes {} {{", interface_stmt.name));
        
        self.increase_indent();
        for method in &interface_stmt.methods {
            let mut line = format!("{}(", method.name);
            for (i, param) in method.parameters.iter().enumerate() {
                if i > 0 {
                    line.push_str(", ");
                }
                line.push_str(&format!("{} {}", param.name, param.type_name));
            }
            line.push(')');
            
            if let Some(return_type) = &method.return_type {
                line.push_str(&format!(" {}", return_type));
            }
            
            self.add_line(&line);
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format a panic statement
    fn format_panic_statement(&mut self, panic_stmt: &PanicStatement) -> Result<(), CursedError> {
        let message = self.format_expression(&panic_stmt.message)?;
        self.add_line(&format!("cringe {}", message));
        Ok(())
    }

    /// Format a catch statement
    fn format_catch_statement(&mut self, catch_stmt: &CatchStatement) -> Result<(), CursedError> {
        let error_var = &catch_stmt.error_variable;
        self.add_line(&format!("caught {} {{", error_var));
        
        self.increase_indent();
        for stmt in &catch_stmt.body {
            self.format_statement(stmt)?;
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format a defer statement
    fn format_defer_statement(&mut self, defer_stmt: &DeferStatement) -> Result<(), CursedError> {
        let call = self.format_expression(&defer_stmt.call)?;
        self.add_line(&format!("defer {}", call));
        Ok(())
    }

    /// Format a break statement
    fn format_break_statement(&mut self, break_stmt: &BreakStatement) -> Result<(), CursedError> {
        if let Some(label) = &break_stmt.label {
            self.add_line(&format!("ghosted {}", label));
        } else {
            self.add_line("ghosted");
        }
        Ok(())
    }

    /// Format a continue statement
    fn format_continue_statement(&mut self, continue_stmt: &ContinueStatement) -> Result<(), CursedError> {
        if let Some(label) = &continue_stmt.label {
            self.add_line(&format!("simp {}", label));
        } else {
            self.add_line("simp");
        }
        Ok(())
    }

    /// Format an increment statement
    fn format_increment_statement(&mut self, inc_stmt: &IncrementStatement) -> Result<(), CursedError> {
        let target = self.format_expression(&inc_stmt.target)?;
        self.add_line(&format!("{}++", target));
        Ok(())
    }

    /// Format a decrement statement
    fn format_decrement_statement(&mut self, dec_stmt: &DecrementStatement) -> Result<(), CursedError> {
        let target = self.format_expression(&dec_stmt.target)?;
        self.add_line(&format!("{}--", target));
        Ok(())
    }

    /// Format a short declaration
    fn format_short_declaration(&mut self, short_decl: &ShortDeclarationStatement) -> Result<(), CursedError> {
        let mut line = String::new();
        
        // Handle tuple destructuring
        if short_decl.names.len() > 1 {
            line.push('(');
            for (i, name) in short_decl.names.iter().enumerate() {
                if i > 0 {
                    line.push_str(", ");
                }
                line.push_str(name);
            }
            line.push(')');
        } else {
            line.push_str(&short_decl.names[0]);
        }
        
        line.push_str(" := ");
        line.push_str(&self.format_expression(&short_decl.value)?);
        
        self.add_line(&line);
        Ok(())
    }

    /// Format a yikes statement
    fn format_yikes_statement(&mut self, yikes_stmt: &YikesStatement) -> Result<(), CursedError> {
        let variable = &yikes_stmt.variable;
        let expr = self.format_expression(&yikes_stmt.expression)?;
        self.add_line(&format!("yikes {} := {}", variable, expr));
        Ok(())
    }

    /// Format a fam statement
    fn format_fam_statement(&mut self, fam_stmt: &FamStatement) -> Result<(), CursedError> {
        let error_var = &fam_stmt.error_variable;
        self.add_line(&format!("fam {} {{", error_var));
        
        self.increase_indent();
        for stmt in &fam_stmt.body {
            self.format_statement(stmt)?;
        }
        self.decrease_indent();
        
        self.add_line("}");
        Ok(())
    }

    /// Format an expression
    fn format_expression(&mut self, expr: &Expression) -> Result<String, CursedError> {
        match expr {
            Expression::Identifier(name) => Ok(name.clone()),
            Expression::Variable(name) => Ok(name.clone()),
            Expression::Integer(value) => Ok(value.to_string()),
            Expression::Float(value) => Ok(value.to_string()),
            Expression::String(value) => Ok(format!("\"{}\"", value)),
            Expression::Boolean(value) => Ok(if *value { "based".to_string() } else { "cap".to_string() }),
            Expression::Character(value) => Ok(format!("'{}'", value)),
            Expression::Binary(binary) => self.format_binary_expression(binary),
            Expression::Call(call) => self.format_call_expression(call),
            Expression::MemberAccess(member) => self.format_member_access_expression(member),
            Expression::Literal(literal) => self.format_literal(literal),
            Expression::Unary(unary) => self.format_unary_expression(unary),
            Expression::Array(elements) => self.format_array_expression(elements),
            Expression::Map(pairs) => self.format_map_expression(pairs),
            Expression::CompositeLiteral(composite) => self.format_composite_literal(composite),
            Expression::ChannelSend(send) => self.format_channel_send_expression(send),
            Expression::ChannelReceive(receive) => self.format_channel_receive_expression(receive),
            Expression::ChannelCreation(creation) => self.format_channel_creation_expression(creation),
            Expression::StructLiteral(struct_lit) => self.format_struct_literal_expression(struct_lit),
            Expression::Lambda(lambda) => self.format_lambda_expression(lambda),
            Expression::Tuple(tuple) => self.format_tuple_expression(tuple),
            Expression::TupleAccess(tuple_access) => self.format_tuple_access_expression(tuple_access),
            Expression::ArrayAccess(array_access) => self.format_array_access_expression(array_access),
            Expression::SliceAccess(slice_access) => self.format_slice_access_expression(slice_access),
            Expression::TypeAssertion(type_assertion) => self.format_type_assertion_expression(type_assertion),
            Expression::Increment(inc) => self.format_increment_expression(inc),
            Expression::Decrement(dec) => self.format_decrement_expression(dec),
            Expression::Shook(shook) => self.format_shook_expression(shook),
            Expression::ErrorValue(error_value) => self.format_error_value_expression(error_value),
        }
    }

    /// Format a binary expression
    fn format_binary_expression(&mut self, binary: &BinaryExpression) -> Result<String, CursedError> {
        let left = self.format_expression(&binary.left)?;
        let right = self.format_expression(&binary.right)?;
        
        let op = match binary.operator.as_str() {
            "+" => "+",
            "-" => "-",
            "*" => "*",
            "/" => "/",
            "%" => "%",
            "==" => "==",
            "!=" => "!=",
            "<" => "<",
            ">" => ">",
            "<=" => "<=",
            ">=" => ">=",
            "&&" => "&&",
            "||" => "||",
            "&" => "&",
            "|" => "|",
            "^" => "^",
            "<<" => "<<",
            ">>" => ">>",
            _ => &binary.operator,
        };
        
        if self.config.spaces_around_operators {
            Ok(format!("{} {} {}", left, op, right))
        } else {
            Ok(format!("{}{}{}", left, op, right))
        }
    }

    /// Format a call expression
    fn format_call_expression(&mut self, call: &CallExpression) -> Result<String, CursedError> {
        let function = self.format_expression(&call.function)?;
        let mut result = format!("{}(", function);
        
        for (i, arg) in call.arguments.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&self.format_expression(arg)?);
        }
        
        result.push(')');
        Ok(result)
    }

    /// Format a member access expression
    fn format_member_access_expression(&mut self, member: &MemberAccessExpression) -> Result<String, CursedError> {
        let object = self.format_expression(&member.object)?;
        Ok(format!("{}.{}", object, member.member))
    }

    /// Format a literal
    fn format_literal(&mut self, literal: &Literal) -> Result<String, CursedError> {
        match literal {
            Literal::Integer(value) => Ok(value.to_string()),
            Literal::Float(value) => Ok(value.to_string()),
            Literal::String(value) => Ok(format!("\"{}\"", value)),
            Literal::Boolean(value) => Ok(if *value { "based".to_string() } else { "cap".to_string() }),
            Literal::Character(value) => Ok(format!("'{}'", value)),
            Literal::Nil => Ok("cringe".to_string()),
        }
    }

    /// Format a unary expression
    fn format_unary_expression(&mut self, unary: &UnaryExpression) -> Result<String, CursedError> {
        let operand = self.format_expression(&unary.operand)?;
        match unary.operator.as_str() {
            "!" => Ok(format!("!{}", operand)),
            "-" => Ok(format!("-{}", operand)),
            "+" => Ok(format!("+{}", operand)),
            "&" => Ok(format!("&{}", operand)),
            "*" => Ok(format!("*{}", operand)),
            _ => Ok(format!("{}{}", unary.operator, operand)),
        }
    }

    /// Format an array expression
    fn format_array_expression(&mut self, elements: &[Expression]) -> Result<String, CursedError> {
        if elements.is_empty() {
            return Ok("[]".to_string());
        }
        
        let mut result = String::from("[");
        
        if self.should_format_array_multiline(elements) {
            result.push('\n');
            self.increase_indent();
            
            for (i, element) in elements.iter().enumerate() {
                if i > 0 {
                    result.push_str(",\n");
                }
                result.push_str(&self.get_indent());
                result.push_str(&self.format_expression(element)?);
            }
            
            self.decrease_indent();
            result.push('\n');
            result.push_str(&self.get_indent());
        } else {
            for (i, element) in elements.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&self.format_expression(element)?);
            }
        }
        
        result.push(']');
        Ok(result)
    }

    /// Format a map expression
    fn format_map_expression(&mut self, pairs: &[(Expression, Expression)]) -> Result<String, CursedError> {
        if pairs.is_empty() {
            return Ok("{}".to_string());
        }
        
        let mut result = String::from("{");
        
        if self.should_format_map_multiline(pairs) {
            result.push('\n');
            self.increase_indent();
            
            for (i, (key, value)) in pairs.iter().enumerate() {
                if i > 0 {
                    result.push_str(",\n");
                }
                result.push_str(&self.get_indent());
                result.push_str(&self.format_expression(key)?);
                result.push_str(": ");
                result.push_str(&self.format_expression(value)?);
            }
            
            self.decrease_indent();
            result.push('\n');
            result.push_str(&self.get_indent());
        } else {
            for (i, (key, value)) in pairs.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&self.format_expression(key)?);
                result.push_str(": ");
                result.push_str(&self.format_expression(value)?);
            }
        }
        
        result.push('}');
        Ok(result)
    }

    /// Format a composite literal
    fn format_composite_literal(&mut self, composite: &CompositeLiteralExpression) -> Result<String, CursedError> {
        let mut result = format!("{}{{", composite.type_name);
        
        if composite.fields.is_empty() {
            result.push('}');
            return Ok(result);
        }
        
        if self.should_format_composite_multiline(&composite.fields) {
            result.push('\n');
            self.increase_indent();
            
            for (i, field) in composite.fields.iter().enumerate() {
                if i > 0 {
                    result.push_str(",\n");
                }
                result.push_str(&self.get_indent());
                result.push_str(&field.name);
                result.push_str(": ");
                result.push_str(&self.format_expression(&field.value)?);
            }
            
            self.decrease_indent();
            result.push('\n');
            result.push_str(&self.get_indent());
        } else {
            for (i, field) in composite.fields.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&field.name);
                result.push_str(": ");
                result.push_str(&self.format_expression(&field.value)?);
            }
        }
        
        result.push('}');
        Ok(result)
    }

    /// Format a channel send expression
    fn format_channel_send_expression(&mut self, send: &ChannelSendExpression) -> Result<String, CursedError> {
        let channel = self.format_expression(&send.channel)?;
        let value = self.format_expression(&send.value)?;
        Ok(format!("{} <- {}", channel, value))
    }

    /// Format a channel receive expression
    fn format_channel_receive_expression(&mut self, receive: &ChannelReceiveExpression) -> Result<String, CursedError> {
        let channel = self.format_expression(&receive.channel)?;
        Ok(format!("<-{}", channel))
    }

    /// Format a channel creation expression
    fn format_channel_creation_expression(&mut self, creation: &ChannelCreationExpression) -> Result<String, CursedError> {
        let mut result = format!("make(chan {}", creation.type_name);
        if let Some(size) = &creation.size {
            result.push_str(&format!(", {}", self.format_expression(size)?));
        }
        result.push(')');
        Ok(result)
    }

    /// Format a struct literal expression
    fn format_struct_literal_expression(&mut self, struct_lit: &StructLiteralExpression) -> Result<String, CursedError> {
        let mut result = format!("{}{{", struct_lit.type_name);
        
        if struct_lit.fields.is_empty() {
            result.push('}');
            return Ok(result);
        }
        
        if self.should_format_struct_literal_multiline(&struct_lit.fields) {
            result.push('\n');
            self.increase_indent();
            
            for (i, field) in struct_lit.fields.iter().enumerate() {
                if i > 0 {
                    result.push_str(",\n");
                }
                result.push_str(&self.get_indent());
                result.push_str(&field.name);
                result.push_str(": ");
                result.push_str(&self.format_expression(&field.value)?);
            }
            
            self.decrease_indent();
            result.push('\n');
            result.push_str(&self.get_indent());
        } else {
            for (i, field) in struct_lit.fields.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&field.name);
                result.push_str(": ");
                result.push_str(&self.format_expression(&field.value)?);
            }
        }
        
        result.push('}');
        Ok(result)
    }

    /// Format a lambda expression
    fn format_lambda_expression(&mut self, lambda: &LambdaExpression) -> Result<String, CursedError> {
        let mut result = String::from("slay(");
        
        for (i, param) in lambda.parameters.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{} {}", param.name, param.type_name));
        }
        
        result.push(')');
        
        if let Some(return_type) = &lambda.return_type {
            result.push_str(&format!(" {} ", return_type));
        } else {
            result.push(' ');
        }
        
        result.push('{');
        
        if lambda.body.len() == 1 {
            result.push(' ');
            match &lambda.body[0] {
                Statement::Expression(expr) => {
                    result.push_str(&self.format_expression(expr)?);
                }
                _ => {
                    // For complex statements, format multiline
                    result.push('\n');
                    self.increase_indent();
                    result.push_str(&self.get_indent());
                    // Format statement would go here
                    self.decrease_indent();
                    result.push('\n');
                    result.push_str(&self.get_indent());
                }
            }
            result.push_str(" }");
        } else {
            result.push('\n');
            self.increase_indent();
            for stmt in &lambda.body {
                self.format_statement(stmt)?;
            }
            self.decrease_indent();
            result.push('\n');
            result.push_str(&self.get_indent());
            result.push('}');
        }
        
        Ok(result)
    }

    /// Format a tuple expression
    fn format_tuple_expression(&mut self, tuple: &TupleExpression) -> Result<String, CursedError> {
        let mut result = String::from("(");
        
        for (i, element) in tuple.elements.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&self.format_expression(element)?);
        }
        
        result.push(')');
        Ok(result)
    }

    /// Format a tuple access expression
    fn format_tuple_access_expression(&mut self, tuple_access: &TupleAccessExpression) -> Result<String, CursedError> {
        let tuple = self.format_expression(&tuple_access.tuple)?;
        Ok(format!("{}.{}", tuple, tuple_access.index))
    }

    /// Format an array access expression
    fn format_array_access_expression(&mut self, array_access: &ArrayAccessExpression) -> Result<String, CursedError> {
        let array = self.format_expression(&array_access.array)?;
        let index = self.format_expression(&array_access.index)?;
        Ok(format!("{}[{}]", array, index))
    }

    /// Format a slice access expression
    fn format_slice_access_expression(&mut self, slice_access: &SliceAccessExpression) -> Result<String, CursedError> {
        let array = self.format_expression(&slice_access.array)?;
        let start = slice_access.start.as_ref().map(|s| self.format_expression(s)).transpose()?;
        let end = slice_access.end.as_ref().map(|e| self.format_expression(e)).transpose()?;
        
        let slice_expr = match (start, end) {
            (Some(s), Some(e)) => format!("{}:{}", s, e),
            (Some(s), None) => format!("{}:", s),
            (None, Some(e)) => format!(":{}", e),
            (None, None) => ":".to_string(),
        };
        
        Ok(format!("{}[{}]", array, slice_expr))
    }

    /// Format a type assertion expression
    fn format_type_assertion_expression(&mut self, type_assertion: &TypeAssertionExpression) -> Result<String, CursedError> {
        let value = self.format_expression(&type_assertion.value)?;
        Ok(format!("{}.({}) ", value, type_assertion.type_name))
    }

    /// Format an increment expression
    fn format_increment_expression(&mut self, inc: &IncrementExpression) -> Result<String, CursedError> {
        let operand = self.format_expression(&inc.operand)?;
        if inc.prefix {
            Ok(format!("++{}", operand))
        } else {
            Ok(format!("{}++", operand))
        }
    }

    /// Format a decrement expression
    fn format_decrement_expression(&mut self, dec: &DecrementExpression) -> Result<String, CursedError> {
        let operand = self.format_expression(&dec.operand)?;
        if dec.prefix {
            Ok(format!("--{}", operand))
        } else {
            Ok(format!("{}--", operand))
        }
    }

    /// Format a shook expression
    fn format_shook_expression(&mut self, shook: &ShookExpression) -> Result<String, CursedError> {
        let expr = self.format_expression(&shook.expression)?;
        Ok(format!("shook {}", expr))
    }

    /// Format an error value expression
    fn format_error_value_expression(&mut self, error_value: &ErrorValueExpression) -> Result<String, CursedError> {
        let message = self.format_expression(&error_value.message)?;
        Ok(format!("error({})", message))
    }

    /// Extract comments from source code
    fn extract_comments(&mut self, source: &str) -> Result<(), CursedError> {
        let lines: Vec<&str> = source.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            if let Some(comment_start) = line.find("//") {
                let comment = line[comment_start..].trim();
                self.comments.entry(line_num).or_insert_with(Vec::new).push(comment.to_string());
            }
            
            if let Some(comment_start) = line.find("/*") {
                if let Some(comment_end) = line.find("*/") {
                    let comment = &line[comment_start..=comment_end];
                    self.comments.entry(line_num).or_insert_with(Vec::new).push(comment.to_string());
                }
            }
        }
        
        Ok(())
    }

    /// Check if array should be formatted multiline
    fn should_format_array_multiline(&self, elements: &[Expression]) -> bool {
        if elements.len() > self.config.max_array_elements_single_line {
            return true;
        }
        
        // Estimate line length
        let estimated_length: usize = elements.iter()
            .map(|e| self.estimate_expression_length(e))
            .sum::<usize>() + (elements.len() - 1) * 2 + 2; // commas and brackets
        
        estimated_length > self.config.max_line_length
    }

    /// Check if map should be formatted multiline
    fn should_format_map_multiline(&self, pairs: &[(Expression, Expression)]) -> bool {
        if pairs.len() > self.config.max_map_elements_single_line {
            return true;
        }
        
        // Estimate line length
        let estimated_length: usize = pairs.iter()
            .map(|(k, v)| self.estimate_expression_length(k) + self.estimate_expression_length(v) + 2)
            .sum::<usize>() + (pairs.len() - 1) * 2 + 2; // commas and brackets
        
        estimated_length > self.config.max_line_length
    }

    /// Check if composite literal should be formatted multiline
    fn should_format_composite_multiline(&self, fields: &[crate::ast::FieldExpression]) -> bool {
        if fields.len() > self.config.max_struct_fields_single_line {
            return true;
        }
        
        // Estimate line length
        let estimated_length: usize = fields.iter()
            .map(|f| f.name.len() + self.estimate_expression_length(&f.value) + 2)
            .sum::<usize>() + (fields.len() - 1) * 2 + 2;
        
        estimated_length > self.config.max_line_length
    }

    /// Check if struct literal should be formatted multiline
    fn should_format_struct_literal_multiline(&self, fields: &[crate::ast::StructLiteralField]) -> bool {
        if fields.len() > self.config.max_struct_fields_single_line {
            return true;
        }
        
        // Estimate line length
        let estimated_length: usize = fields.iter()
            .map(|f| f.name.len() + self.estimate_expression_length(&f.value) + 2)
            .sum::<usize>() + (fields.len() - 1) * 2 + 2;
        
        estimated_length > self.config.max_line_length
    }

    /// Estimate the length of an expression when formatted
    fn estimate_expression_length(&self, expr: &Expression) -> usize {
        match expr {
            Expression::Identifier(name) => name.len(),
            Expression::Variable(name) => name.len(),
            Expression::Integer(value) => value.to_string().len(),
            Expression::Float(value) => value.to_string().len(),
            Expression::String(value) => value.len() + 2, // quotes
            Expression::Boolean(_) => 5, // "based" or "cap"
            Expression::Character(_) => 3, // single quotes
            Expression::Binary(binary) => {
                self.estimate_expression_length(&binary.left) + 
                self.estimate_expression_length(&binary.right) + 
                binary.operator.len() + 2 // spaces
            }
            Expression::Call(call) => {
                let func_len = self.estimate_expression_length(&call.function);
                let args_len: usize = call.arguments.iter()
                    .map(|arg| self.estimate_expression_length(arg))
                    .sum();
                func_len + args_len + call.arguments.len() * 2 + 2 // commas and parens
            }
            Expression::Array(elements) => {
                let elements_len: usize = elements.iter()
                    .map(|e| self.estimate_expression_length(e))
                    .sum();
                elements_len + elements.len() * 2 + 2 // commas and brackets
            }
            Expression::Tuple(tuple) => {
                let elements_len: usize = tuple.elements.iter()
                    .map(|e| self.estimate_expression_length(e))
                    .sum();
                elements_len + tuple.elements.len() * 2 + 2 // commas and parens
            }
            _ => 10, // Default estimate for complex expressions
        }
    }

    /// Add a line to the output
    fn add_line(&mut self, line: &str) {
        let indented_line = if line.trim().is_empty() {
            String::new()
        } else {
            format!("{}{}", self.get_indent(), line)
        };
        
        self.output.push(indented_line);
        self.current_line_length = line.len() + self.current_indent * self.config.indent_size;
    }

    /// Add blank lines
    fn add_blank_lines(&mut self, count: usize) {
        for _ in 0..count {
            self.output.push(String::new());
        }
    }

    /// Get current indentation string
    fn get_indent(&self) -> String {
        if self.config.use_tabs {
            "\t".repeat(self.current_indent)
        } else {
            " ".repeat(self.current_indent * self.config.indent_size)
        }
    }

    /// Increase indentation level
    fn increase_indent(&mut self) {
        self.current_indent += 1;
    }

    /// Decrease indentation level
    fn decrease_indent(&mut self) {
        if self.current_indent > 0 {
            self.current_indent -= 1;
        }
    }

    /// Format source code and compare with original
    pub fn format_diff(&mut self, source: &str) -> Result<String, CursedError> {
        let formatted = self.format(source)?;
        
        if source == formatted {
            Ok(String::new()) // No changes
        } else {
            Ok(self.generate_diff(source, &formatted))
        }
    }

    /// Generate a diff between original and formatted code
    fn generate_diff(&self, original: &str, formatted: &str) -> String {
        let original_lines: Vec<&str> = original.lines().collect();
        let formatted_lines: Vec<&str> = formatted.lines().collect();
        
        let mut diff = String::new();
        let mut i = 0;
        let mut j = 0;
        
        while i < original_lines.len() || j < formatted_lines.len() {
            if i < original_lines.len() && j < formatted_lines.len() {
                if original_lines[i] == formatted_lines[j] {
                    diff.push_str(&format!("  {}\n", original_lines[i]));
                    i += 1;
                    j += 1;
                } else {
                    diff.push_str(&format!("- {}\n", original_lines[i]));
                    diff.push_str(&format!("+ {}\n", formatted_lines[j]));
                    i += 1;
                    j += 1;
                }
            } else if i < original_lines.len() {
                diff.push_str(&format!("- {}\n", original_lines[i]));
                i += 1;
            } else {
                diff.push_str(&format!("+ {}\n", formatted_lines[j]));
                j += 1;
            }
        }
        
        diff
    }

    /// Check if source code is already formatted
    pub fn is_formatted(&mut self, source: &str) -> Result<bool, CursedError> {
        let formatted = self.format(source)?;
        Ok(source == formatted)
    }
}
