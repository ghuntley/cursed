/// Code formatter for CURSED
use crate::ast::*;
use crate::ast::conditionals::{IfStatement, ForStatement, SwitchStatement, RangeForStatement, WhileStatement};
use crate::ast::statements::{ThrowStatement, TryStatement, CatchStatement};
use crate::error::CursedError;
use crate::parser::Parser;
use crate::lexer::Lexer;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BraceStyle {
    SameLine,
    NextLine,
    NextLineUnindented,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorSpacing {
    WithSpaces,
    WithoutSpaces,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommaSpacing {
    WithSpaces,
    WithoutSpaces,
}

#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub indent_size: usize,
    pub line_width: usize,
    pub brace_style: BraceStyle,
    pub operator_spacing: OperatorSpacing,
    pub comma_spacing: CommaSpacing,
    pub max_empty_lines: usize,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 4,
            line_width: 100,
            brace_style: BraceStyle::SameLine,
            operator_spacing: OperatorSpacing::WithSpaces,
            comma_spacing: CommaSpacing::WithSpaces,
            max_empty_lines: 2,
        }
    }
}

#[derive(Debug)]
pub struct FormatterResult {
    pub formatted_code: String,
    pub changes_made: bool,
    pub lines_changed: usize,
    pub formatting_errors: Vec<String>,
}

impl FormatterResult {
    pub fn new(formatted_code: String, original: &str) -> Self {
        let changes_made = formatted_code != original;
        let lines_changed = if changes_made {
            formatted_code.split("\n").count()
        } else {
            0
        };
        
        Self {
            formatted_code,
            changes_made,
            lines_changed,
            formatting_errors: Vec::new(),
        }
    }
    
    pub fn with_errors(mut self, errors: Vec<String>) -> Self {
        self.formatting_errors = errors;
        self
    }
}

pub struct CursedFormatter {
    config: FormatterConfig,
    current_indent: usize,
    output: String,
    errors: Vec<String>,
}

impl CursedFormatter {
    pub fn new(config: FormatterConfig) -> Self {
        Self { 
            config,
            current_indent: 0,
            output: String::new(),
            errors: Vec::new(),
        }
    }
    
    pub fn format(&mut self, source: &str) -> crate::error::Result<()> {
        // Reset state
        self.current_indent = 0;
        self.output.clear();
        self.errors.clear();
        
        // Parse the source code
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        
        // Format the program
        self.format_program(&program);
        
        // Return result
        let result = FormatterResult::new(self.output.clone(), source)
            .with_errors(self.errors.clone());
        
        Ok(result)
    }
    
    fn format_program(&mut self, program: &Program) {
        // Package declaration
        if let Some(package) = &program.package_name {
            self.write_line(&format!("vibe {}", package));
            self.write_empty_line();
        }
        
        // Import statements
        if !program.imports.is_empty() {
            for import in &program.imports {
                self.format_import_statement(import);
            }
            self.write_empty_line();
        }
        
        // Statements
        for (i, statement) in program.statements.iter().enumerate() {
            if i > 0 {
                self.write_empty_line();
            }
            self.format_statement(statement.as_ref());
        }
    }
    
    fn format_import_statement(&mut self, import: &ImportStatement) {
        if let Some(alias) = &import.alias {
            self.write_line(&format!("yeet {} \"{}\"", alias, import.path));
        } else {
            self.write_line(&format!("yeet \"{}\"", import.path));
        }
    }
    
    fn format_statement(&mut self, statement: &dyn Statement) {
        if let Some(func) = statement.as_any().downcast_ref::<FunctionStatement>() {
            self.format_function_statement(func);
        } else if let Some(squad) = statement.as_any().downcast_ref::<SquadStatement>() {
            self.format_squad_statement(squad);
        } else if let Some(collab) = statement.as_any().downcast_ref::<CollabStatement>() {
            self.format_collab_statement(collab);
        } else if let Some(if_stmt) = statement.as_any().downcast_ref::<IfStatement>() {
            self.format_if_statement(if_stmt);
        } else if let Some(switch) = statement.as_any().downcast_ref::<SwitchStatement>() {
            self.format_switch_statement(switch);
        } else if let Some(for_stmt) = statement.as_any().downcast_ref::<ForStatement>() {
            self.format_for_statement(for_stmt);
        } else if let Some(range_for) = statement.as_any().downcast_ref::<RangeForStatement>() {
            self.format_range_for_statement(range_for);
        } else if let Some(while_stmt) = statement.as_any().downcast_ref::<WhileStatement>() {
            self.format_while_statement(while_stmt);
        } else if let Some(break_stmt) = statement.as_any().downcast_ref::<BreakStatement>() {
            self.write_line("ghosted");
        } else if let Some(continue_stmt) = statement.as_any().downcast_ref::<ContinueStatement>() {
            self.write_line("simp");
        } else if let Some(type_alias) = statement.as_any().downcast_ref::<TypeAliasStatement>() {
            self.format_type_alias_statement(type_alias);
        } else {
            // Fallback to string representation
            self.write_line(&statement.string());
        }
    }
    
    fn format_function_statement(&mut self, func: &FunctionStatement) {
        let mut line = String::new();
        
        // Function keyword and name
        line.push_str("slay ");
        line.push_str(&func.name.string());
        
        // Parameters
        line.push('(');
        for (i, param) in func.parameters.iter().enumerate() {
            if i > 0 {
                if self.config.comma_spacing == CommaSpacing::WithSpaces {
                    line.push_str(", ");
                } else {
                    line.push(',');
                }
            }
            line.push_str(&param.string());
        }
        line.push(')');
        
        // Return type
        if let Some(return_type) = &func.return_type {
            line.push(' ');
            line.push_str(&return_type.string());
        }
        
        // Opening brace
        match self.config.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
                self.write_line(&line);
            }
            BraceStyle::NextLine => {
                self.write_line(&line);
                self.write_line("{");
            }
            BraceStyle::NextLineUnindented => {
                self.write_line(&line);
                self.write_unindented_line("{");
            }
        }
        
        // Body
        self.current_indent += 1;
        self.format_block_statement(&func.body);
        self.current_indent -= 1;
        
        // Closing brace
        self.write_line("}");
    }
    
    fn format_squad_statement(&mut self, squad: &SquadStatement) {
        let mut line = format!("squad {}", squad.name.string());
        
        // Opening brace
        match self.config.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
                self.write_line(&line);
            }
            BraceStyle::NextLine => {
                self.write_line(&line);
                self.write_line("{");
            }
            BraceStyle::NextLineUnindented => {
                self.write_line(&line);
                self.write_unindented_line("{");
            }
        }
        
        // Fields
        self.current_indent += 1;
        for field in &squad.fields {
            self.write_line(&format!("{} {}", field.name.string(), field.type_name.string()));
        }
        self.current_indent -= 1;
        
        // Closing brace
        self.write_line("}");
    }
    
    fn format_collab_statement(&mut self, collab: &CollabStatement) {
        let mut line = format!("collab {}", collab.name.string());
        
        // Opening brace
        match self.config.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
                self.write_line(&line);
            }
            BraceStyle::NextLine => {
                self.write_line(&line);
                self.write_line("{");
            }
            BraceStyle::NextLineUnindented => {
                self.write_line(&line);
                self.write_unindented_line("{");
            }
        }
        
        // Methods
        self.current_indent += 1;
        for method in &collab.methods {
            let mut method_line = method.name.string();
            method_line.push('(');
            for (i, param) in method.parameters.iter().enumerate() {
                if i > 0 {
                    if self.config.comma_spacing == CommaSpacing::WithSpaces {
                        method_line.push_str(", ");
                    } else {
                        method_line.push(',');
                    }
                }
                method_line.push_str(&param.string());
            }
            method_line.push(')');
            
            if let Some(return_type) = &method.return_type {
                method_line.push(' ');
                method_line.push_str(&return_type.string());
            }
            
            self.write_line(&method_line);
        }
        self.current_indent -= 1;
        
        // Closing brace
        self.write_line("}");
    }
    
    fn format_if_statement(&mut self, if_stmt: &IfStatement) {
        let mut line = format!("lowkey {}", if_stmt.condition.string());
        
        // Opening brace
        match self.config.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
                self.write_line(&line);
            }
            BraceStyle::NextLine => {
                self.write_line(&line);
                self.write_line("{");
            }
            BraceStyle::NextLineUnindented => {
                self.write_line(&line);
                self.write_unindented_line("{");
            }
        }
        
        // Consequence
        self.current_indent += 1;
        self.format_block_statement(&if_stmt.consequence);
        self.current_indent -= 1;
        
        // Alternative
        if let Some(alternative) = &if_stmt.alternative {
            match self.config.brace_style {
                BraceStyle::SameLine => {
                    self.write_line("} highkey {");
                }
                BraceStyle::NextLine => {
                    self.write_line("}");
                    self.write_line("highkey");
                    self.write_line("{");
                }
                BraceStyle::NextLineUnindented => {
                    self.write_line("}");
                    self.write_line("highkey");
                    self.write_unindented_line("{");
                }
            }
            
            self.current_indent += 1;
            self.format_statement(alternative.as_ref());
            self.current_indent -= 1;
            self.write_line("}");
        } else {
            self.write_line("}");
        }
    }
    
    fn format_switch_statement(&mut self, switch: &SwitchStatement) {
        let mut line = format!("vibe_check {}", switch.value.string());
        
        // Opening brace
        match self.config.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
                self.write_line(&line);
            }
            BraceStyle::NextLine => {
                self.write_line(&line);
                self.write_line("{");
            }
            BraceStyle::NextLineUnindented => {
                self.write_line(&line);
                self.write_unindented_line("{");
            }
        }
        
        // Cases
        self.current_indent += 1;
        for case in &switch.cases {
            let values: Vec<String> = case.values.iter()
                .map(|v| v.string())
                .collect();
            self.write_line(&format!("mood {}:", values.join(", ")));
            
            self.current_indent += 1;
            for stmt in &case.statements {
                self.format_statement(stmt.as_ref());
            }
            self.current_indent -= 1;
        }
        
        // Default case
        if let Some(default) = &switch.default {
            self.write_line("basic:");
            self.current_indent += 1;
            for stmt in default {
                self.format_statement(stmt.as_ref());
            }
            self.current_indent -= 1;
        }
        
        self.current_indent -= 1;
        self.write_line("}");
    }
    
    fn format_for_statement(&mut self, for_stmt: &ForStatement) {
        let init_str = for_stmt.init.as_ref().map(|i| i.string()).unwrap_or_default();
        let cond_str = for_stmt.condition.as_ref().map(|c| c.string()).unwrap_or_default();
        let post_str = for_stmt.post.as_ref().map(|p| p.string()).unwrap_or_default();
        
        let spacing = if self.config.operator_spacing == OperatorSpacing::WithSpaces { " " } else { "" };
        let mut line = format!("bestie{}{};{}{};{}{}", spacing, init_str, spacing, cond_str, spacing, post_str);
        
        // Opening brace
        match self.config.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
                self.write_line(&line);
            }
            BraceStyle::NextLine => {
                self.write_line(&line);
                self.write_line("{");
            }
            BraceStyle::NextLineUnindented => {
                self.write_line(&line);
                self.write_unindented_line("{");
            }
        }
        
        // Body
        self.current_indent += 1;
        self.format_block_statement(&for_stmt.body);
        self.current_indent -= 1;
        
        // Closing brace
        self.write_line("}");
    }
    
    fn format_range_for_statement(&mut self, range_for: &RangeForStatement) {
        let vars = match (&range_for.key_var, &range_for.value_var) {
            (Some(k), Some(v)) => {
                if self.config.comma_spacing == CommaSpacing::WithSpaces {
                    format!("{}, {}", k, v)
                } else {
                    format!("{},{}", k, v)
                }
            }
            (Some(k), None) => k.clone(),
            (None, Some(v)) => v.clone(),
            (None, None) => "_".to_string(),
        };
        
        let spacing = if self.config.operator_spacing == OperatorSpacing::WithSpaces { " " } else { "" };
        let mut line = format!("bestie{}{}{}:={}flex{}{}", spacing, vars, spacing, spacing, spacing, range_for.iterable.string());
        
        // Opening brace
        match self.config.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
                self.write_line(&line);
            }
            BraceStyle::NextLine => {
                self.write_line(&line);
                self.write_line("{");
            }
            BraceStyle::NextLineUnindented => {
                self.write_line(&line);
                self.write_unindented_line("{");
            }
        }
        
        // Body
        self.current_indent += 1;
        self.format_block_statement(&range_for.body);
        self.current_indent -= 1;
        
        // Closing brace
        self.write_line("}");
    }
    
    fn format_while_statement(&mut self, while_stmt: &WhileStatement) {
        let mut line = format!("periodt {}", while_stmt.condition.string());
        
        // Opening brace
        match self.config.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
                self.write_line(&line);
            }
            BraceStyle::NextLine => {
                self.write_line(&line);
                self.write_line("{");
            }
            BraceStyle::NextLineUnindented => {
                self.write_line(&line);
                self.write_unindented_line("{");
            }
        }
        
        // Body
        self.current_indent += 1;
        self.format_block_statement(&while_stmt.body);
        self.current_indent -= 1;
        
        // Closing brace
        self.write_line("}");
    }
    
    fn format_type_alias_statement(&mut self, type_alias: &TypeAliasStatement) {
        self.write_line(&format!("be_like {} {}", type_alias.name, type_alias.target_type));
    }
    
    fn format_block_statement(&mut self, block: &BlockStatement) {
        for statement in &block.statements {
            self.format_statement(statement.as_ref());
        }
    }
    
    fn write_line(&mut self, content: &str) {
        self.output.push_str(&self.get_indent());
        self.output.push_str(content);
        self.output.push('\n');
    }
    
    fn write_unindented_line(&mut self, content: &str) {
        self.output.push_str(content);
        self.output.push('\n');
    }
    
    fn write_empty_line(&mut self) {
        self.output.push('\n');
    }
    
    fn get_indent(&self) -> String {
        " ".repeat(self.current_indent * self.config.indent_size)
    }
}

impl Default for CursedFormatter {
    fn default() -> Self {
        Self::new(FormatterConfig::default())
    }
}
