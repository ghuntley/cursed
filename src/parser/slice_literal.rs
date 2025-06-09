//! Parser implementation for slice literal expressions
//!
//! This module contains the parsing logic for slice literals in the CURSED language.
//! Slice literals have the syntax: `[]Type{element1, element2, ...}`

use crate::ast::expressions::SliceLiteral;
use crate::ast::Expression;
use crate::error::Error;
use crate::lexer::Token;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    /// Parse a slice literal expression with syntax: `[]Type{elements...}`
    ///
    /// This method handles the complete slice literal parsing including:
    /// - The opening `[` and `]` brackets
    /// - The element type specification
    /// - The opening and closing braces
    /// - The comma-separated list of element expressions
    ///
    /// # Returns
    ///
    /// A `SliceLiteral` expression on success, or an error if parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// []normie{1, 2, 3}        // Slice of integers
    /// []tea{"key": "value"}    // Slice of maps  
    /// []thicc{}                // Empty slice
    /// ```
    pub fn parse_slice_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let start_token = self.current_token.clone();
        
        // We should be at the '[' token
        if !self.current_token_is(Token::LBracket) {
            return Err(self.error(&format!(
                "Expected '[' to start slice literal, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Skip past '['
        
        // Expect ']' immediately after '['
        if !self.current_token_is(Token::RBracket) {
            return Err(self.error(&format!(
                "Expected ']' after '[' in slice literal, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Skip past ']'
        
        // Parse the element type (should be an identifier like "normie", "thicc", etc.)
        let element_type = self.parse_expression(Precedence::Lowest)?;
        
        // Expect opening brace '{'
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!(
                "Expected '{{' after slice type, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Skip past '{'
        
        // Parse elements until we reach a closing brace
        let mut elements = Vec::new();
        
        // Handle empty slice
        if self.current_token_is(Token::RBrace) {
            self.next_token()?; // Skip past '}'
            return Ok(Box::new(SliceLiteral::new(
                start_token,
                element_type,
                elements,
            )));
        }
        
        // Parse first element
        elements.push(self.parse_expression(Precedence::Lowest)?);
        
        // Parse additional elements separated by commas
        while self.current_token_is(Token::Comma) {
            self.next_token()?; // Skip past comma
            
            // Allow trailing comma before closing brace
            if self.current_token_is(Token::RBrace) {
                break;
            }
            
            // Parse next element
            elements.push(self.parse_expression(Precedence::Lowest)?);
        }
        
        // Expect closing brace '}' - check current token
        if !self.current_token_is(Token::RBrace) {
            return Err(self.error(&format!(
                "Expected '}}' after slice elements, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Skip past '}'
        
        Ok(Box::new(SliceLiteral::new(
            start_token,
            element_type,
            elements,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::ast::expressions::{Identifier, IntegerLiteral, StringLiteral};
    use crate::ast::Node; // Import the Node trait for string() method
    
    fn setup_parser_and_parse(input: &str) -> Result<Box<dyn Expression>, Error> {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer).unwrap();
        // Don't call next_token() here since parse_slice_literal expects to start at the current token
        parser.parse_slice_literal()
    }
    
    #[test]
    fn test_parse_empty_slice_literal() {
        let result = setup_parser_and_parse("[]normie{}").unwrap();
        
        let slice = result.as_any().downcast_ref::<SliceLiteral>().unwrap();
        assert!(slice.is_empty());
        
        // Check that element type is "normie"
        let element_type = slice.element_type.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(element_type.value, "normie");
    }
    
    #[test]
    fn test_parse_integer_slice_literal() {
        let result = setup_parser_and_parse("[]normie{1, 2, 3}").unwrap();
        
        let slice = result.as_any().downcast_ref::<SliceLiteral>().unwrap();
        assert_eq!(slice.len(), 3);
        
        // Check element type
        let element_type = slice.element_type.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(element_type.value, "normie");
        
        // Check elements
        for (i, expected) in [1i64, 2i64, 3i64].iter().enumerate() {
            let elem = slice.elements[i].as_any().downcast_ref::<IntegerLiteral>().unwrap();
            assert_eq!(elem.value, *expected);
        }
    }
    
    #[test]
    fn test_parse_string_slice_literal() {
        let result = setup_parser_and_parse(r#"[]tea{"hello", "world"}"#).unwrap();
        
        let slice = result.as_any().downcast_ref::<SliceLiteral>().unwrap();
        assert_eq!(slice.len(), 2);
        
        // Check element type
        let element_type = slice.element_type.as_any().downcast_ref::<Identifier>().unwrap();
        assert_eq!(element_type.value, "tea");
        
        // Check elements
        let expected = ["hello", "world"];
        for (i, expected_val) in expected.iter().enumerate() {
            let elem = slice.elements[i].as_any().downcast_ref::<StringLiteral>().unwrap();
            assert_eq!(elem.value, *expected_val);
        }
    }
    
    #[test]
    fn test_parse_slice_literal_with_trailing_comma() {
        let result = setup_parser_and_parse("[]thicc{42, 84,}").unwrap();
        
        let slice = result.as_any().downcast_ref::<SliceLiteral>().unwrap();
        assert_eq!(slice.len(), 2);
        
        // Check elements
        let elem1 = slice.elements[0].as_any().downcast_ref::<IntegerLiteral>().unwrap();
        assert_eq!(elem1.value, 42);
        
        let elem2 = slice.elements[1].as_any().downcast_ref::<IntegerLiteral>().unwrap();
        assert_eq!(elem2.value, 84);
    }
    
    #[test]
    fn test_slice_literal_string_representation() {
        let result = setup_parser_and_parse("[]normie{1, 2}").unwrap();
        
        let slice = result.as_any().downcast_ref::<SliceLiteral>().unwrap();
        let string_repr = slice.string();
        assert_eq!(string_repr, "[]normie{1, 2}");
    }
    
    #[test]
    fn test_parse_slice_literal_error_missing_bracket() {
        let result = setup_parser_and_parse("normie{1, 2}");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_slice_literal_error_missing_closing_bracket() {
        let result = setup_parser_and_parse("[normie{1, 2}");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_slice_literal_error_missing_brace() {
        let result = setup_parser_and_parse("[]normie 1, 2");
        assert!(result.is_err());
    }
}
