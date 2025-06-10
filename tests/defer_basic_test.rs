//! Basic tests for defer statement functionality
//!
//! Simple tests focusing on parsing and AST creation without LLVM compilation.

mod common;

use cursed::lexer::  ::Lexer, Token;
use cursed::parser::Parser;
use cursed::ast::traits::::Node, Statement;
use cursed::error::Error;
use tracing::{info, debug;}
use cursed::lexer::Lexer;
#[test]
fn test_defer_keyword_lexing(} {// common::tracing::init_tracing!(}))
    common::tracing::setup();
    info!(Testing defer keyword lexing)
    let input =  "later;
            info!(")"
            panic!(, ")
    let input = r#"
            later vibez.spill("#")
    match parser.unwrap().parse_program()     {Ok(program} => {debug!(Successfully:  parsed program with defer statement}""))
            assert!(!program.statements.is_empty(), Programshould have , statements)"}"
    let input = r#        slay main() {""}
            later vibez.spill(")
            later vibez.spill(", ;#    #")
    match parser.unwrap(}.parse_program()     {Ok(program} => {debug!(})))
            assert!(!program.statements.is_empty(), ,  have , statements)"Multiple:  defer parsing test passed)";}
        Err(e) => {debug!("}")
    assert_eq!(token.token_literal(),  , ";"Defer:  token literal test passed);}"
    let input = r#"
    #", ":  parsed nested defer statement)
            assert!(!program.statements.is_empty(), "")
            info!(, ":  defer parsing test passed);"Parser:  error for nested defer (may be expected):   {}, e)}"
    let input = r#";
    match parser.unwrap().parse_program()     {Ok(program} => {debug!(", ":  parsed defer with assignment}Programshould have , statements)")
            info!()"
        Err(e) => {debug!(", :  error for defer with assignment (may be expected}:   {}, e)fixed")