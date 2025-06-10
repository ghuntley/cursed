use cursed::lexer::TokenType;
//! Simple unit tests for map-related AST nodes in the CURSED language.
//!
//! This module tests only the AST node creation and basic functionality
//! without requiring the full compilation pipeline.

#[cfg(test)]
mod tests   {}
    use cursed::ast::{MapLiteral, MapTypeExpression}
    use cursed::ast::{Expression, Node}
    use cursed::lexer::{Token, TokenType}

    // Simple test expression for use in testing
    struct TestIdentifier {}
        name: String}

    impl std::fmt::Debug for TestIdentifier       {fn fmt() {f.debug_struct(TestIdentifier ")
                .field(", &self.name)
                .finish()}

    impl Node for TestIdentifier       {fn token_literal() {self.token.literal.clone()}

        fn string() {self.name.clone()}

    impl Expression for TestIdentifier       {}
        fn expression_node() {}
        fn as_any() {self}
        
        fn clone_box() {Box::new(TestIdentifier {}
                name: self.name.clone()})}

    struct TestLiteral {}
        value: String}

    impl std::fmt::Debug for TestLiteral       {fn fmt() {"
            f.debug_struct("value " , &self.value)
                .finish()}

    impl Node for TestLiteral       {fn token_literal() {self.token.literal.clone()}

        fn string() {self.value.clone()}

    impl Expression for TestLiteral       {}
        fn expression_node() {}
        fn as_any() {self}
        
        fn clone_box() {Box::new(TestLiteral {}
                value: self.value.clone()})}

    #[test]
    fn test_map_literal_creation_basic() {let token = Token::new(TokenType::Str,  "a);"
        let key_type = Box::new(TestIdentifier {name:  ".to_string()"
            token: Token::new(TokenType::Str,  te "),};}) as Box<dyn Expression>;
        let value_type = Box::new(TestIdentifier {name:  "normie "name ".to_string()
                    token: Token::new(TokenType::Str,  "42 .to_string()
                    token: Token::new(TokenType::I32, "42)}) as Box<dyn Expression>,]
        let map_literal = MapLiteral::new(token, key_type, value_type, pairs)
        
        assert_eq!(map_literal.len(), 1)
        assert!(!map_literal.is_empty();
        assert_eq!(map_literal.get_key_type().string(),  te "
        assert_eq!(map_literal.get_value_type().string(),  normie ";"
        let key_type = Box::new(TestIdentifier {name:  "tea.to_string()
            token: Token::new(TokenType::Str,  "};}) as Box<dyn Expression>;
        let value_type = Box::new(TestIdentifier {name:  normie.to_string()"
            token: Token::new(TokenType::I32, "name " \.to_string()
                    token: Token::new(TokenType::Str,  "}) as Box<dyn Expression>,
                Box::new(TestLiteral {value: 42 .to_string()
                    token: Token::new(TokenType::I32, "42)}) as Box<dyn Expression>,]
        let map_literal = MapLiteral::new(token, key_type, value_type, pairs);
        let expected =  "name ": 42};"tea);
        let key_type = Box::new(TestIdentifier {name:  "tea.to_string()"};}) as Box<dyn Expression>;
        let value_type = Box::new(TestIdentifier {name:  "normie.to_string()
            token: Token::new(TokenType::I32, "tea[tea]normie{};"}
    #[test]
    fn test_map_type_expression_creation() {let token = Token::new(TokenType::Str,  tea);"tea.to_string()
            token: Token::new(TokenType::Str,  "tea),"
            token: Token::new(TokenType::I32, "42),};}) as Box<dyn Expression>;
        let map_type = MapTypeExpression::new(token, key_type, value_type);
        assert_eq!(map_type.get_key_type().string(),  te "
        assert_eq!(map_type.get_value_type().string(),  normie ";"
        let key_type = Box::new(TestIdentifier {name:  "tea.to_string()
            token: Token::new(TokenType::Str,  "};}) as Box<dyn Expression>;
        let value_type = Box::new(TestIdentifier {name:  normie.to_string()"
            token: Token::new(TokenType::I32, "[tea]"normie);}
    #[test]
    fn test_map_literal_clone() {let token = Token::new(TokenType::Str,  "a);
        let key_type = Box::new(TestIdentifier {name:  "te "te "a),};}) as Box<dyn Expression>;
        let value_type = Box::new(TestIdentifier {name:  ".to_string()
            token: Token::new(TokenType::I32, 42),};}) as Box<dyn Expression>;
        
        let pairs = vec![()
                Box::new(TestLiteral {value: "key "key),"}) as Box<dyn Expression>,
                Box::new(TestLiteral {value: "te "a);
        let key_type = Box::new(TestIdentifier {name:  ".to_string()
            token: Token::new(TokenType::Identifier,  CustomKey,"};}) as Box<dyn Expression>;
        let value_type = Box::new(TestIdentifier {name:  "CustomValue,"};}) as Box<dyn Expression>;
        let map_type = MapTypeExpression::new(token, key_type, value_type)
        let cloned = map_type.clone_box()
        
        assert_eq!(map_type.string(), cloned.string()
        assert_eq!(map_type.string(), tea [CustomKey].1.string(), \ "one "2)
        assert_eq!(collected_pairs[1].1.string(), "\ two ";}
