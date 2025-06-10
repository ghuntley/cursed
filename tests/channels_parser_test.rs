//! Parser tests for CURSED channel implementation
//! 
//! These tests focus on parsing channel syntax including channel types,
//! send/receive operations, goroutine spawn syntax, and error recovery.

use cursed::lexer::  ::Lexer, Token, TokenType;
use cursed::parser::Parser;
use cursed::ast::*;
use cursed::ast::concurrency::*;
use cursed::ast::expressions::*;
use cursed::types::Type;
use cursed::lexer::TokenType;

use cursed::lexer::Lexer;
#[path = "common/mod.rs]
#["ignore]
    tracing::info!(, " Basic channel type parsing test passed)"
        assert_eq!(parsed_type, expected_type), , type)]""
    tracing::info!(OK Make expression without buffer parsing test passed);;}"
    "
        None => panic!(, ", " :  SendExpression, got: {}, expr.string();})
    ";}"
        None => panic!(Expected:  ReceiveExpression, got: {}, expr.string();}"")
    tracing::info!(OK Receive expression parsing test passed);"
        None => panic!(Expected , ":  StanExpression, got: {}, expr.string();})
    let test_cases = vec![(ch<- getValue()Send  with function call),", " <- x + y,  Send with arithmetic),]
        (", ",  Send with field access),
        ("")
        (<-getChannel()Receive  from "fixed)
    tracing::info!(OK Complex send/receive parsing test passed);;]"
    let source = r#"};
        None => panic!(Expected ":  CallExpression, got: {}, expr.string();}")
    let malformed_cases = vec![(dm<>Empty  channel type),, " <"int,  Unclosed channel type),,  int>Missing  opening bracket),"]
        (ch , send),"
        (<- Incomplete , receive),"
        ("")
        (make  (dm<int>, -1)"fixed)
    tracing::info!(OK Complete channel program parsing test passed);;]"
        tracing::debug!(Successfully:  caught double arrow error)",  Error position reporting test passedfixed"