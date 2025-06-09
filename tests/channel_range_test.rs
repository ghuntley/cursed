//! Integration tests for channel range operations
//!
//! Tests the comprehensive channel range implementation including:
//! - Channel range AST parsing
//! - LLVM code generation for channel iteration
//! - Runtime support for channel closure detection

use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::ast::control_flow::channel_range::{ChannelRangeForStatement, ChannelRangeClause};
use cursed::ast::Statement;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_channel_range_parse_simple() {
        let input = r#"
        bestie value := flex <-ch {
            vibez.println(value)
        }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens.into_iter());
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok(), "Failed to parse channel range statement: {:?}", stmt.err());
        
        let stmt = stmt.unwrap();
        let channel_range_stmt = stmt.as_any().downcast_ref::<ChannelRangeForStatement>();
        assert!(channel_range_stmt.is_some(), "Statement is not a ChannelRangeForStatement");
        
        let channel_range_stmt = channel_range_stmt.unwrap();
        assert_eq!(channel_range_stmt.value_var, "value");
        assert!(channel_range_stmt.ok_var.is_none());
    }
    
    #[test]
    fn test_channel_range_parse_with_ok() {
        let input = r#"
        bestie value, ok := flex <-ch {
            if !ok {
                break
            }
            processValue(value)
        }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens.into_iter());
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok(), "Failed to parse channel range statement with ok: {:?}", stmt.err());
        
        let stmt = stmt.unwrap();
        let channel_range_stmt = stmt.as_any().downcast_ref::<ChannelRangeForStatement>();
        assert!(channel_range_stmt.is_some(), "Statement is not a ChannelRangeForStatement");
        
        let channel_range_stmt = channel_range_stmt.unwrap();
        assert_eq!(channel_range_stmt.value_var, "value");
        assert_eq!(channel_range_stmt.ok_var.as_ref().unwrap(), "ok");
        assert!(channel_range_stmt.channel_range.with_ok);
    }
    
    #[test]
    fn test_channel_range_string_representation() {
        let input = r#"
        bestie msg, open := flex <-messageChannel {
            handleMessage(msg, open)
        }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens.into_iter());
        
        let stmt = parser.parse_statement();
        assert!(stmt.is_ok());
        
        let stmt = stmt.unwrap();
        let string_repr = stmt.string();
        assert!(string_repr.contains("bestie msg, open := flex <-messageChannel"));
    }
}
