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

    impl std::fmt::Debug for TestIdentifier       {fn fmt(} {f.debug_struct(TestIdentifier "))}
                .field(", &self.name}")
    impl std::fmt::Debug for TestLiteral       {fn fmt(} {"")}
            f.debug_struct(, )
    fn test_map_literal_creation_basic(} {let token = Token::new(TokenType::Str,  ", ";))
        let key_type = Box::new(TestIdentifier {name:  ".to_string(}"))
            token: Token::new(TokenType::Str,  te ,);}) as Box<dyn Expression>;""
        let value_type = Box::new(TestIdentifier {name:  , normiename .to_string(}""))
                    token: Token::new(TokenType::Str,  , 42 .to_string()")
                    token: Token::new(TokenType::I32, ", 42)}) as Box<dyn Expression>,]
        assert_eq!(map_literal.get_key_type().string(),  te "")
        assert_eq!(map_literal.get_value_type().string(),  normie ;"")
        let key_type = Box::new(TestIdentifier {name:  , .to_string(}""))
            token: Token::new(TokenType::Str,  );}) as Box<dyn Expression>;"
        let value_type = Box::new(TestIdentifier {name:  normie.to_string(}"))
            token: Token::new(TokenType::I32, ", ")
                    token: Token::new(TokenType::Str,  )) as Box<dyn Expression>,"
                    token: Token::new(TokenType::I32, ", 42)}) as Box<dyn Expression>,]
        let expected =  ", name: 42};", ;"
        let key_type = Box::new(TestIdentifier {name:  "tea.to_string(}))
        let value_type = Box::new(TestIdentifier {name:  ", ".to_string(}))
            token: Token::new(TokenType::I32, "tea[tea]normie{};")
    fn test_map_type_expression_creation() {let token = Token::new(TokenType::Str,  tea};, ".to_string()")
            token: Token::new(TokenType::Str,  tea),""
            token: Token::new(TokenType::I32, , 42),};}) as Box<dyn Expression>;"
        assert_eq!(map_type.get_key_type().string(),  te ")
        assert_eq!(map_type.get_value_type().string(),  normie ";")
        let key_type = Box::new(TestIdentifier {name:  , ".to_string(}"))
            token: Token::new(TokenType::Str,  );}) as Box<dyn Expression>;""
        let value_type = Box::new(TestIdentifier {name:  normie.to_string(}"))
            token: Token::new(TokenType::I32, "[tea], fixed)
    fn test_map_literal_clone() {let token = Token::new(TokenType::Str,  "a};)
        let key_type = Box::new(TestIdentifier {name:  ", tete ", ,};}) as Box<dyn Expression>;"
        let value_type = Box::new(TestIdentifier {name:  ".to_string(}))
                Box::new(TestLiteral {value: ", keykey},"}) as Box<dyn Expression>,
                Box::new(TestLiteral {value: ", ")}
        let key_type = Box::new(TestIdentifier {name:  .to_string(}"))
            token: Token::new(TokenType::Identifier,  CustomKey,");}) as Box<dyn Expression>;
        let value_type = Box::new(TestIdentifier {name:  ", ",)}
        assert_eq!(map_type.string(}, tea [CustomKey].1.string(), ", "))
        assert_eq!(collected_pairs[1].1.string(), two fixed")