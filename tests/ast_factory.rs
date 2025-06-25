#[cfg(test)]
mod tests {
    use cursed::ast::*;
    use cursed::lexer::{Token, TokenType, SourceLocation};
    
    /// Helper factory for creating AST nodes for testing
    pub struct ASTFactory;
    
    impl ASTFactory {
        pub fn create_identifier(name: &str) -> Identifier {
            Identifier::new(name.to_string())
        }
        
        pub fn create_integer_literal(value: i64) -> IntegerLiteral {
            IntegerLiteral::new(value)
        }
        
        pub fn create_string_literal(value: &str) -> StringLiteral {
            StringLiteral::new(value.to_string())
        }
        
        pub fn create_boolean_literal(value: bool) -> BooleanLiteral {
            BooleanLiteral::new(value)
        }
        
        pub fn create_binary_op(left: Box<dyn Expression>, op: BinaryOperator, right: Box<dyn Expression>) -> BinaryOperatorExpression {
            BinaryOperatorExpression::new(left, op, right)
        }
        
        pub fn create_function_call(name: &str, args: Vec<Box<dyn Expression>>) -> FunctionCall {
            FunctionCall::new(
                Box::new(Self::create_identifier(name)),
                args
            )
        }
        
        pub fn create_variable_declaration(name: &str, value: Option<Box<dyn Expression>>) -> VariableDeclaration {
            VariableDeclaration::new(
                name.to_string(),
                None, // type annotation
                value,
                true // mutable
            )
        }
        
        pub fn create_function_declaration(name: &str, params: Vec<String>, body: Vec<Box<dyn Statement>>) -> FunctionDeclaration {
            FunctionDeclaration::new(
                name.to_string(),
                params.into_iter().map(|p| Parameter { name: p, param_type: "auto".to_string() }).collect(),
                Some("auto".to_string()),
                BlockStatement::new(body),
                Vec::new() // generic parameters
            )
        }
        
        pub fn create_if_statement(condition: Box<dyn Expression>, then_branch: Box<dyn Statement>, else_branch: Option<Box<dyn Statement>>) -> IfStatement {
            IfStatement::new(condition, then_branch, else_branch)
        }
        
        pub fn create_while_statement(condition: Box<dyn Expression>, body: Box<dyn Statement>) -> WhileStatement {
            WhileStatement::new(condition, body)
        }
        
        pub fn create_block_statement(statements: Vec<Box<dyn Statement>>) -> BlockStatement {
            BlockStatement::new(statements)
        }
    }
    
    #[test]
    fn test_create_identifier() {
        let id = ASTFactory::create_identifier("test_var");
        assert_eq!(id.name(), "test_var");
        assert_eq!(format!("{:?}", id).contains("test_var"), true);
    }
    
    #[test]
    fn test_create_literals() {
        let int_lit = ASTFactory::create_integer_literal(42);
        assert_eq!(int_lit.value(), 42);
        
        let str_lit = ASTFactory::create_string_literal("hello");
        assert_eq!(str_lit.value(), "hello");
        
        let bool_lit = ASTFactory::create_boolean_literal(true);
        assert_eq!(bool_lit.value(), true);
    }
    
    #[test]
    fn test_create_binary_operation() {
        let left = Box::new(ASTFactory::create_integer_literal(1));
        let right = Box::new(ASTFactory::create_integer_literal(2));
        let binary_op = ASTFactory::create_binary_op(left, BinaryOperator::Add, right);
        
        let display_str = format!("{}", binary_op);
        assert!(display_str.contains("1"));
        assert!(display_str.contains("2"));
        assert!(display_str.contains("+") || display_str.contains("Add"));
    }
    
    #[test]
    fn test_create_function_call() {
        let args = vec![
            Box::new(ASTFactory::create_integer_literal(1)) as Box<dyn Expression>,
            Box::new(ASTFactory::create_string_literal("test")) as Box<dyn Expression>
        ];
        let call = ASTFactory::create_function_call("my_function", args);
        
        let display_str = format!("{}", call);
        assert!(display_str.contains("my_function"));
        assert!(display_str.contains("1"));
        assert!(display_str.contains("test"));
    }
    
    #[test]
    fn test_create_variable_declaration() {
        let value = Some(Box::new(ASTFactory::create_integer_literal(42)) as Box<dyn Expression>);
        let var_decl = ASTFactory::create_variable_declaration("my_var", value);
        
        let display_str = format!("{}", var_decl);
        assert!(display_str.contains("my_var"));
        assert!(display_str.contains("42"));
    }
    
    #[test]
    fn test_create_function_declaration() {
        let params = vec!["param1".to_string(), "param2".to_string()];
        let body = vec![
            Box::new(ASTFactory::create_variable_declaration("local_var", Some(Box::new(ASTFactory::create_integer_literal(10))))) as Box<dyn Statement>
        ];
        let func_decl = ASTFactory::create_function_declaration("my_function", params, body);
        
        let display_str = format!("{}", func_decl);
        assert!(display_str.contains("my_function"));
        assert!(display_str.contains("param1"));
        assert!(display_str.contains("param2"));
    }
    
    #[test]
    fn test_create_control_flow() {
        let condition = Box::new(ASTFactory::create_boolean_literal(true)) as Box<dyn Expression>;
        let then_stmt = Box::new(ASTFactory::create_variable_declaration("x", Some(Box::new(ASTFactory::create_integer_literal(1))))) as Box<dyn Statement>;
        let else_stmt = Some(Box::new(ASTFactory::create_variable_declaration("y", Some(Box::new(ASTFactory::create_integer_literal(2))))) as Box<dyn Statement>);
        
        let if_stmt = ASTFactory::create_if_statement(condition, then_stmt, else_stmt);
        let display_str = format!("{}", if_stmt);
        assert!(display_str.contains("true"));
    }
    
    #[test]
    fn test_create_while_loop() {
        let condition = Box::new(ASTFactory::create_boolean_literal(true)) as Box<dyn Expression>;
        let body = Box::new(ASTFactory::create_variable_declaration("i", Some(Box::new(ASTFactory::create_integer_literal(0))))) as Box<dyn Statement>;
        
        let while_stmt = ASTFactory::create_while_statement(condition, body);
        let display_str = format!("{}", while_stmt);
        assert!(display_str.contains("true"));
    }
    
    #[test]
    fn test_create_block_statement() {
        let statements = vec![
            Box::new(ASTFactory::create_variable_declaration("a", Some(Box::new(ASTFactory::create_integer_literal(1))))) as Box<dyn Statement>,
            Box::new(ASTFactory::create_variable_declaration("b", Some(Box::new(ASTFactory::create_integer_literal(2))))) as Box<dyn Statement>
        ];
        let block = ASTFactory::create_block_statement(statements);
        
        let display_str = format!("{}", block);
        assert!(display_str.contains("a"));
        assert!(display_str.contains("b"));
    }
    
    #[test]
    fn test_complex_ast_structure() {
        // Create a complex nested AST structure
        let inner_expr = Box::new(ASTFactory::create_binary_op(
            Box::new(ASTFactory::create_integer_literal(1)),
            BinaryOperator::Add,
            Box::new(ASTFactory::create_integer_literal(2))
        )) as Box<dyn Expression>;
        
        let call_expr = Box::new(ASTFactory::create_function_call("calculate", vec![inner_expr])) as Box<dyn Expression>;
        
        let var_decl = ASTFactory::create_variable_declaration("result", Some(call_expr));
        
        let display_str = format!("{}", var_decl);
        assert!(display_str.contains("result"));
        assert!(display_str.contains("calculate"));
        assert!(display_str.contains("1"));
        assert!(display_str.contains("2"));
    }
    
    #[test]
    fn test_ast_node_traits() {
        let id = ASTFactory::create_identifier("test");
        
        // Test that all AST nodes implement required traits
        let _debug = format!("{:?}", id);
        let _display = format!("{}", id);
        let _cloned = id.clone();
        
        // Verify it implements Expression trait
        let expr: Box<dyn Expression> = Box::new(id);
        let _expr_debug = format!("{:?}", expr);
        let _expr_display = format!("{}", expr);
    }
}
