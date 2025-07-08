use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::{Parser, AstNode};

/// Production-ready code formatter for CURSED
#[derive(Debug, Clone)]
pub struct CursedFormatter {
    pub config: FormatterConfig,
    pub style_rules: StyleRules,
    pub indent_level: usize,
    pub current_line: String,
    pub output: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub indent_size: usize,
    pub use_tabs: bool,
    pub max_line_length: usize,
    pub preserve_blank_lines: bool,
    pub max_blank_lines: usize,
    pub space_before_parens: bool,
    pub space_after_comma: bool,
    pub space_around_operators: bool,
    pub newline_before_brace: bool,
    pub align_assignments: bool,
    pub sort_imports: bool,
    pub remove_trailing_whitespace: bool,
    pub ensure_newline_at_eof: bool,
    pub format_comments: bool,
    pub comment_column: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct StyleRules {
    pub function_declaration_style: FunctionStyle,
    pub variable_declaration_style: VariableStyle,
    pub import_style: ImportStyle,
    pub comment_style: CommentStyle,
    pub brace_style: BraceStyle,
    pub operator_style: OperatorStyle,
    pub type_annotation_style: TypeStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionStyle {
    Compact,      // slay func() type
    Verbose,      // slay func() -> type
    Multiline,    // Parameters on separate lines
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableStyle {
    Compact,      // sus x drip = 42
    Aligned,      // sus x     drip = 42
    TypeFirst,    // drip x = 42
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportStyle {
    OnePerLine,   // yeet "a"; yeet "b"
    Grouped,      // yeet ( "a"; "b" )
    Sorted,       // Alphabetically sorted
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommentStyle {
    Preserve,     // Keep original formatting
    Align,        // Align to specific column
    Reflow,       // Reflow long comments
}

#[derive(Debug, Clone, PartialEq)]
pub enum BraceStyle {
    SameLine,     // func() {
    NewLine,      // func()\n{
    Compact,      // func(){
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorStyle {
    Spaced,       // a + b
    Compact,      // a+b
    Aligned,      // a  + b (aligned)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeStyle {
    Explicit,     // sus x normie = 42
    Inferred,     // sus x := 42
    Compact,      // normie x = 42
}

#[derive(Debug, Clone)]
pub struct FormattingContext {
    pub in_function: bool,
    pub in_type_declaration: bool,
    pub in_import_block: bool,
    pub in_comment: bool,
    pub brace_depth: usize,
    pub paren_depth: usize,
    pub current_function: Option<String>,
    pub pending_newlines: usize,
}

impl CursedFormatter {
    /// Create new formatter with config
    pub fn new(config: FormatterConfig) -> Self {
        Self {
            config,
            style_rules: StyleRules::default(),
            indent_level: 0,
            current_line: String::new(),
            output: Vec::new(),
        }
    }

    /// Format CURSED source file
    pub fn format_file(&mut self, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        self.format_source(&content)
    }

    /// Format CURSED source code
    pub fn format_source(&mut self, source: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("🎨 Formatting CURSED source code...");

        // Reset formatter state
        self.output.clear();
        self.current_line.clear();
        self.indent_level = 0;

        // Tokenize source
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        // Parse into AST for semantic formatting
        let mut parser = Parser::new(tokens.clone());
        let ast = parser.parse().unwrap_or_else(|_| {
            // If parsing fails, fall back to token-based formatting
            eprintln!("⚠️  Parsing failed, using token-based formatting");
            Default::default()
        });

        // Format using AST-aware approach
        if !ast.nodes.is_empty() {
            self.format_ast(&ast)?;
        } else {
            self.format_tokens(&tokens)?;
        }

        // Post-processing
        self.post_process()?;

        let formatted = self.output.join("\n");
        println!("✅ Source code formatted successfully");
        Ok(formatted)
    }

    /// Format multiple files
    pub fn format_directory(&mut self, dir_path: &Path, recursive: bool) -> Result<Vec<FormattingResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() && recursive {
                results.extend(self.format_directory(&path, recursive)?);
            } else if path.extension().map_or(false, |ext| ext == "csd") {
                match self.format_file(&path) {
                    Ok(formatted) => {
                        if self.should_update_file(&path, &formatted)? {
                            fs::write(&path, &formatted)?;
                            results.push(FormattingResult {
                                file_path: path,
                                status: FormatStatus::Formatted,
                                changes: self.count_changes(&formatted),
                            });
                        } else {
                            results.push(FormattingResult {
                                file_path: path,
                                status: FormatStatus::NoChanges,
                                changes: 0,
                            });
                        }
                    }
                    Err(e) => {
                        results.push(FormattingResult {
                            file_path: path,
                            status: FormatStatus::Error(e.to_string()),
                            changes: 0,
                        });
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// Format using AST-aware approach
    fn format_ast(&mut self, ast: &crate::ast::Program) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = FormattingContext::new();
        
        for node in &ast.nodes {
            self.format_ast_node(node, &mut context)?;
        }
        
        Ok(())
    }

    /// Format individual AST node
    fn format_ast_node(&mut self, node: &AstNode, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        match node {
            AstNode::FunctionDeclaration(func) => {
                self.format_function_declaration(func, context)?;
            }
            AstNode::VariableDeclaration(var) => {
                self.format_variable_declaration(var, context)?;
            }
            AstNode::ImportDeclaration(import) => {
                self.format_import_declaration(import, context)?;
            }
            AstNode::TypeDeclaration(type_decl) => {
                self.format_type_declaration(type_decl, context)?;
            }
            AstNode::Block(statements) => {
                self.format_block(statements, context)?;
            }
            AstNode::IfStatement(if_stmt) => {
                self.format_if_statement(if_stmt, context)?;
            }
            AstNode::WhileLoop(while_loop) => {
                self.format_while_loop(while_loop, context)?;
            }
            AstNode::ForLoop(for_loop) => {
                self.format_for_loop(for_loop, context)?;
            }
            AstNode::Expression(expr) => {
                self.format_expression(expr, context)?;
            }
            AstNode::Comment(comment) => {
                self.format_comment(comment, context)?;
            }
            _ => {
                // Handle other node types
                self.add_line(&format!("# Unhandled node: {:?}", node));
            }
        }
        
        Ok(())
    }

    /// Format function declaration
    fn format_function_declaration(&mut self, func: &crate::ast::FunctionDeclaration, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        context.in_function = true;
        context.current_function = Some(func.name.clone());
        
        let mut declaration = String::new();
        
        // Function keyword
        declaration.push_str("slay ");
        
        // Function name
        declaration.push_str(&func.name);
        
        // Parameters
        if self.config.space_before_parens {
            declaration.push(' ');
        }
        declaration.push('(');
        
        // Format parameters
        for (i, param) in func.parameters.iter().enumerate() {
            if i > 0 {
                declaration.push_str(", ");
            }
            
            match self.style_rules.variable_declaration_style {
                VariableStyle::Compact => {
                    declaration.push_str(&format!("{} {:?}", param.name, param.param_type));
                }
                VariableStyle::TypeFirst => {
                    declaration.push_str(&format!("{:?} {}", param.param_type, param.name));
                }
                VariableStyle::Aligned => {
                    // Implement alignment logic
                    declaration.push_str(&format!("{} {:?}", param.name, param.param_type));
                }
            }
        }
        
        declaration.push(')');
        
        // Return type
        if let Some(return_type) = &func.return_type {
            match self.style_rules.function_declaration_style {
                FunctionStyle::Compact => {
                    declaration.push_str(&format!(" {:?}", return_type));
                }
                FunctionStyle::Verbose => {
                    declaration.push_str(&format!(" -> {:?}", return_type));
                }
                FunctionStyle::Multiline => {
                    declaration.push_str(" ->");
                    self.add_line(&declaration);
                    self.indent_level += 1;
                    self.add_line(&format!("{:?}", return_type));
                    self.indent_level -= 1;
                    declaration.clear();
                }
            }
        }
        
        // Opening brace
        match self.style_rules.brace_style {
            BraceStyle::SameLine => {
                declaration.push_str(" {");
            }
            BraceStyle::NewLine => {
                self.add_line(&declaration);
                declaration = "{".to_string();
            }
            BraceStyle::Compact => {
                declaration.push('{');
            }
        }
        
        if !declaration.is_empty() {
            self.add_line(&declaration);
        }
        
        // Function body
        self.indent_level += 1;
        for stmt in &func.body {
            self.format_ast_node(stmt, context)?;
        }
        self.indent_level -= 1;
        
        // Closing brace
        self.add_line("}");
        self.add_blank_line();
        
        context.in_function = false;
        context.current_function = None;
        
        Ok(())
    }

    /// Format variable declaration
    fn format_variable_declaration(&mut self, var: &crate::ast::VariableDeclaration, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        let mut declaration = String::new();
        
        match self.style_rules.variable_declaration_style {
            VariableStyle::Compact => {
                declaration.push_str(&format!("sus {} {:?}", var.name, var.var_type));
                if let Some(value) = &var.initial_value {
                    if self.config.space_around_operators {
                        declaration.push_str(" = ");
                    } else {
                        declaration.push_str("=");
                    }
                    declaration.push_str(&self.format_expression_to_string(value));
                }
            }
            VariableStyle::TypeFirst => {
                declaration.push_str(&format!("{:?} {}", var.var_type, var.name));
                if let Some(value) = &var.initial_value {
                    if self.config.space_around_operators {
                        declaration.push_str(" = ");
                    } else {
                        declaration.push_str("=");
                    }
                    declaration.push_str(&self.format_expression_to_string(value));
                }
            }
            VariableStyle::Aligned => {
                // Implement alignment logic
                declaration.push_str(&format!("sus {} {:?}", var.name, var.var_type));
                if let Some(value) = &var.initial_value {
                    declaration.push_str(" = ");
                    declaration.push_str(&self.format_expression_to_string(value));
                }
            }
        }
        
        self.add_line(&declaration);
        Ok(())
    }

    /// Format import declaration
    fn format_import_declaration(&mut self, import: &crate::ast::ImportDeclaration, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        let import_str = match self.style_rules.import_style {
            ImportStyle::OnePerLine => {
                format!("yeet \"{}\"", import.module_name)
            }
            ImportStyle::Grouped => {
                if context.in_import_block {
                    format!("    \"{}\"", import.module_name)
                } else {
                    format!("yeet \"{}\"", import.module_name)
                }
            }
            ImportStyle::Sorted => {
                format!("yeet \"{}\"", import.module_name)
            }
        };
        
        self.add_line(&import_str);
        Ok(())
    }

    /// Format type declaration
    fn format_type_declaration(&mut self, type_decl: &crate::ast::TypeDeclaration, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        context.in_type_declaration = true;
        
        let mut declaration = format!("{} {}", type_decl.kind, type_decl.name);
        
        match self.style_rules.brace_style {
            BraceStyle::SameLine => {
                declaration.push_str(" {");
            }
            BraceStyle::NewLine => {
                self.add_line(&declaration);
                declaration = "{".to_string();
            }
            BraceStyle::Compact => {
                declaration.push('{');
            }
        }
        
        self.add_line(&declaration);
        
        // Format fields
        self.indent_level += 1;
        for field in &type_decl.fields {
            let field_str = format!("{} {:?}", field.name, field.field_type);
            self.add_line(&field_str);
        }
        self.indent_level -= 1;
        
        self.add_line("}");
        self.add_blank_line();
        
        context.in_type_declaration = false;
        Ok(())
    }

    /// Format block statement
    fn format_block(&mut self, statements: &[AstNode], context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        for stmt in statements {
            self.format_ast_node(stmt, context)?;
        }
        Ok(())
    }

    /// Format if statement
    fn format_if_statement(&mut self, if_stmt: &crate::ast::IfStatement, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        let mut line = String::from("bestie ");
        line.push_str(&self.format_expression_to_string(&if_stmt.condition));
        
        match self.style_rules.brace_style {
            BraceStyle::SameLine => {
                line.push_str(" {");
            }
            BraceStyle::NewLine => {
                self.add_line(&line);
                line = "{".to_string();
            }
            BraceStyle::Compact => {
                line.push('{');
            }
        }
        
        self.add_line(&line);
        
        // Then block
        self.indent_level += 1;
        for stmt in &if_stmt.then_branch {
            self.format_ast_node(stmt, context)?;
        }
        self.indent_level -= 1;
        
        // Else block
        if let Some(else_branch) = &if_stmt.else_branch {
            self.add_line("} salty {");
            self.indent_level += 1;
            for stmt in else_branch {
                self.format_ast_node(stmt, context)?;
            }
            self.indent_level -= 1;
        }
        
        self.add_line("}");
        Ok(())
    }

    /// Format while loop
    fn format_while_loop(&mut self, while_loop: &crate::ast::WhileLoop, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        let mut line = String::from("bestie ");
        line.push_str(&self.format_expression_to_string(&while_loop.condition));
        line.push_str(" {");
        
        self.add_line(&line);
        
        self.indent_level += 1;
        for stmt in &while_loop.body {
            self.format_ast_node(stmt, context)?;
        }
        self.indent_level -= 1;
        
        self.add_line("}");
        Ok(())
    }

    /// Format for loop
    fn format_for_loop(&mut self, for_loop: &crate::ast::ForLoop, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        let mut line = String::from("bestie ");
        
        if let Some(init) = &for_loop.init {
            line.push_str(&self.format_expression_to_string(init));
        }
        line.push_str("; ");
        
        if let Some(condition) = &for_loop.condition {
            line.push_str(&self.format_expression_to_string(condition));
        }
        line.push_str("; ");
        
        if let Some(update) = &for_loop.update {
            line.push_str(&self.format_expression_to_string(update));
        }
        
        line.push_str(" {");
        self.add_line(&line);
        
        self.indent_level += 1;
        for stmt in &for_loop.body {
            self.format_ast_node(stmt, context)?;
        }
        self.indent_level -= 1;
        
        self.add_line("}");
        Ok(())
    }

    /// Format expression
    fn format_expression(&mut self, expr: &crate::ast::Expression, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        let expr_str = self.format_expression_to_string(expr);
        self.add_line(&expr_str);
        Ok(())
    }

    /// Format expression to string
    fn format_expression_to_string(&self, expr: &crate::ast::Expression) -> String {
        match expr {
            crate::ast::Expression::Literal(lit) => {
                format!("{:?}", lit)
            }
            crate::ast::Expression::Identifier(id) => {
                id.clone()
            }
            crate::ast::Expression::BinaryOp { left, operator, right } => {
                let left_str = self.format_expression_to_string(left);
                let right_str = self.format_expression_to_string(right);
                
                if self.config.space_around_operators {
                    format!("{} {} {}", left_str, operator, right_str)
                } else {
                    format!("{}{}{}", left_str, operator, right_str)
                }
            }
            crate::ast::Expression::UnaryOp { operator, operand } => {
                let operand_str = self.format_expression_to_string(operand);
                format!("{}{}", operator, operand_str)
            }
            crate::ast::Expression::FunctionCall { name, args } => {
                let mut call = name.clone();
                call.push('(');
                
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        if self.config.space_after_comma {
                            call.push_str(", ");
                        } else {
                            call.push(',');
                        }
                    }
                    call.push_str(&self.format_expression_to_string(arg));
                }
                
                call.push(')');
                call
            }
            _ => {
                format!("{:?}", expr)
            }
        }
    }

    /// Format comment
    fn format_comment(&mut self, comment: &str, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        match self.style_rules.comment_style {
            CommentStyle::Preserve => {
                self.add_line(comment);
            }
            CommentStyle::Align => {
                if let Some(column) = self.config.comment_column {
                    let padded_comment = self.pad_to_column(comment, column);
                    self.add_line(&padded_comment);
                } else {
                    self.add_line(comment);
                }
            }
            CommentStyle::Reflow => {
                let reflowed = self.reflow_comment(comment);
                self.add_line(&reflowed);
            }
        }
        Ok(())
    }

    /// Format using token-based approach (fallback)
    fn format_tokens(&mut self, tokens: &[Token]) -> Result<(), Box<dyn std::error::Error>> {
        let mut i = 0;
        let mut context = FormattingContext::new();
        
        while i < tokens.len() {
            let token = &tokens[i];
            
            match &token.token_type {
                TokenType::Keyword(keyword) => {
                    self.handle_keyword(keyword, &tokens, &mut i, &mut context)?;
                }
                TokenType::Identifier(name) => {
                    self.current_line.push_str(name);
                }
                TokenType::Number(num) => {
                    self.current_line.push_str(&num.to_string());
                }
                TokenType::StringLiteral(s) => {
                    self.current_line.push_str(&format!("\"{}\"", s));
                }
                TokenType::Operator(op) => {
                    if self.config.space_around_operators {
                        self.current_line.push_str(&format!(" {} ", op));
                    } else {
                        self.current_line.push_str(op);
                    }
                }
                TokenType::LeftParen => {
                    if self.config.space_before_parens && !self.current_line.is_empty() {
                        self.current_line.push(' ');
                    }
                    self.current_line.push('(');
                    context.paren_depth += 1;
                }
                TokenType::RightParen => {
                    self.current_line.push(')');
                    context.paren_depth = context.paren_depth.saturating_sub(1);
                }
                TokenType::LeftBrace => {
                    match self.style_rules.brace_style {
                        BraceStyle::SameLine => {
                            self.current_line.push_str(" {");
                        }
                        BraceStyle::NewLine => {
                            self.finish_line();
                            self.current_line.push('{');
                        }
                        BraceStyle::Compact => {
                            self.current_line.push('{');
                        }
                    }
                    context.brace_depth += 1;
                }
                TokenType::RightBrace => {
                    self.finish_line();
                    context.brace_depth = context.brace_depth.saturating_sub(1);
                    self.add_line("}");
                }
                TokenType::Semicolon => {
                    self.current_line.push(';');
                    self.finish_line();
                }
                TokenType::Comma => {
                    self.current_line.push(',');
                    if self.config.space_after_comma {
                        self.current_line.push(' ');
                    }
                }
                TokenType::Comment(comment) => {
                    self.format_comment(comment, &mut context)?;
                }
                TokenType::Newline => {
                    self.finish_line();
                }
                _ => {
                    // Handle other token types
                    self.current_line.push_str(&format!("{:?}", token));
                }
            }
            
            i += 1;
        }
        
        // Finish any remaining line
        if !self.current_line.trim().is_empty() {
            self.finish_line();
        }
        
        Ok(())
    }

    /// Handle keyword tokens
    fn handle_keyword(&mut self, keyword: &str, tokens: &[Token], index: &mut usize, context: &mut FormattingContext) -> Result<(), Box<dyn std::error::Error>> {
        match keyword {
            "slay" => {
                context.in_function = true;
                self.current_line.push_str("slay ");
            }
            "sus" => {
                self.current_line.push_str("sus ");
            }
            "yeet" => {
                context.in_import_block = true;
                self.current_line.push_str("yeet ");
            }
            "bestie" => {
                self.current_line.push_str("bestie ");
            }
            "damn" => {
                self.current_line.push_str("damn ");
            }
            _ => {
                self.current_line.push_str(keyword);
                self.current_line.push(' ');
            }
        }
        
        Ok(())
    }

    /// Add line with proper indentation
    fn add_line(&mut self, line: &str) {
        let indented = self.create_indent() + line;
        self.output.push(indented);
    }

    /// Add blank line
    fn add_blank_line(&mut self) {
        if self.config.preserve_blank_lines {
            self.output.push(String::new());
        }
    }

    /// Finish current line
    fn finish_line(&mut self) {
        if !self.current_line.trim().is_empty() {
            self.add_line(&self.current_line);
        }
        self.current_line.clear();
    }

    /// Create indentation string
    fn create_indent(&self) -> String {
        if self.config.use_tabs {
            "\t".repeat(self.indent_level)
        } else {
            " ".repeat(self.indent_level * self.config.indent_size)
        }
    }

    /// Post-process formatted output
    fn post_process(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Remove trailing whitespace
        if self.config.remove_trailing_whitespace {
            for line in &mut self.output {
                *line = line.trim_end().to_string();
            }
        }
        
        // Limit consecutive blank lines
        if self.config.max_blank_lines > 0 {
            let mut filtered_output = Vec::new();
            let mut consecutive_blank_lines = 0;
            
            for line in &self.output {
                if line.trim().is_empty() {
                    consecutive_blank_lines += 1;
                    if consecutive_blank_lines <= self.config.max_blank_lines {
                        filtered_output.push(line.clone());
                    }
                } else {
                    consecutive_blank_lines = 0;
                    filtered_output.push(line.clone());
                }
            }
            
            self.output = filtered_output;
        }
        
        // Ensure newline at EOF
        if self.config.ensure_newline_at_eof && !self.output.is_empty() {
            if let Some(last_line) = self.output.last() {
                if !last_line.is_empty() {
                    self.output.push(String::new());
                }
            }
        }
        
        // Sort imports if requested
        if self.config.sort_imports {
            self.sort_imports()?;
        }
        
        Ok(())
    }

    /// Sort import statements
    fn sort_imports(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut import_lines = Vec::new();
        let mut other_lines = Vec::new();
        let mut in_imports = false;
        
        for line in &self.output {
            if line.trim().starts_with("yeet") {
                in_imports = true;
                import_lines.push(line.clone());
            } else if in_imports && line.trim().is_empty() {
                // End of import block
                in_imports = false;
                import_lines.sort();
                other_lines.extend(import_lines.drain(..));
                other_lines.push(line.clone());
            } else {
                other_lines.push(line.clone());
            }
        }
        
        // Handle case where imports are at the end
        if !import_lines.is_empty() {
            import_lines.sort();
            other_lines.extend(import_lines);
        }
        
        self.output = other_lines;
        Ok(())
    }

    /// Check if file should be updated
    fn should_update_file(&self, file_path: &Path, formatted: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if !file_path.exists() {
            return Ok(true);
        }
        
        let original = fs::read_to_string(file_path)?;
        Ok(original != formatted)
    }

    /// Count formatting changes
    fn count_changes(&self, formatted: &str) -> usize {
        // Simple change counting - in production, use proper diff algorithm
        formatted.lines().count()
    }

    /// Pad comment to specific column
    fn pad_to_column(&self, comment: &str, column: usize) -> String {
        if comment.len() >= column {
            comment.to_string()
        } else {
            format!("{}{}", " ".repeat(column - comment.len()), comment)
        }
    }

    /// Reflow long comment
    fn reflow_comment(&self, comment: &str) -> String {
        if comment.len() <= self.config.max_line_length {
            return comment.to_string();
        }
        
        // Simple word wrapping
        let words: Vec<&str> = comment.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();
        
        for word in words {
            if current_line.len() + word.len() + 1 > self.config.max_line_length {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }
            }
            
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        lines.join("\n# ")
    }
}

impl FormattingContext {
    fn new() -> Self {
        Self {
            in_function: false,
            in_type_declaration: false,
            in_import_block: false,
            in_comment: false,
            brace_depth: 0,
            paren_depth: 0,
            current_function: None,
            pending_newlines: 0,
        }
    }
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 4,
            use_tabs: false,
            max_line_length: 100,
            preserve_blank_lines: true,
            max_blank_lines: 2,
            space_before_parens: false,
            space_after_comma: true,
            space_around_operators: true,
            newline_before_brace: false,
            align_assignments: false,
            sort_imports: true,
            remove_trailing_whitespace: true,
            ensure_newline_at_eof: true,
            format_comments: true,
            comment_column: None,
        }
    }
}

impl Default for StyleRules {
    fn default() -> Self {
        Self {
            function_declaration_style: FunctionStyle::Compact,
            variable_declaration_style: VariableStyle::Compact,
            import_style: ImportStyle::Sorted,
            comment_style: CommentStyle::Preserve,
            brace_style: BraceStyle::SameLine,
            operator_style: OperatorStyle::Spaced,
            type_annotation_style: TypeStyle::Explicit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormattingResult {
    pub file_path: PathBuf,
    pub status: FormatStatus,
    pub changes: usize,
}

#[derive(Debug, Clone)]
pub enum FormatStatus {
    Formatted,
    NoChanges,
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_formatter_creation() {
        let config = FormatterConfig::default();
        let formatter = CursedFormatter::new(config);
        
        assert_eq!(formatter.config.indent_size, 4);
        assert!(!formatter.config.use_tabs);
    }

    #[test]
    fn test_basic_formatting() {
        let mut formatter = CursedFormatter::new(FormatterConfig::default());
        let source = "slay test(){vibez.spill(\"hello\")}";
        
        let result = formatter.format_source(source);
        assert!(result.is_ok());
        
        let formatted = result.unwrap();
        assert!(formatted.contains("slay test"));
        assert!(formatted.contains("vibez.spill"));
    }

    #[test]
    fn test_indentation() {
        let mut formatter = CursedFormatter::new(FormatterConfig::default());
        
        let indent = formatter.create_indent();
        assert_eq!(indent, "");
        
        formatter.indent_level = 1;
        let indent = formatter.create_indent();
        assert_eq!(indent, "    ");
    }

    #[test]
    fn test_style_rules_default() {
        let rules = StyleRules::default();
        
        assert_eq!(rules.function_declaration_style, FunctionStyle::Compact);
        assert_eq!(rules.brace_style, BraceStyle::SameLine);
        assert_eq!(rules.import_style, ImportStyle::Sorted);
    }
}
