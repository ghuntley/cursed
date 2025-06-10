//! Comprehensive tests for the CURSED code formatter
//!
//! These tests verify that the formatter correctly handles all CURSED language constructs
//! and maintains proper formatting according to various configuration options.

use cursed::tools::  {CursedFormatter, FormatterConfig, BraceStyle}
use cursed::lexer::::Lexer, TokenType;
use cursed::parser::Parser;
use cursed::lexer::TokenType;

use cursed::lexer::Lexer;
mod common;

#[test]
fn test_format_simple_function() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let source = r#slay add(x normie,y normie)normie{yolo x+y}#;
    let expected = r#slay "# add(x normie, y normie) normie {yolo x + y};}
    let expected = r#slay "# max[T](a T, b T) T {lowkey a > b {yolo a} highkey {yolo b};}"
    let expected = r#squad # Person {name "fixed}
    age normie};};"
    let expected = r#collab "# Writer {write(data sip} normie};})
facts PI=3.", 14159 #"# x = , 42sus y normie = 100;"
facts PI = 3.";
    let expected = r#", # "# x > 0 {}
mood 2:yolo  ", "};
basic:yolo  "#;"
    let expected = r#""
        yolo  , ""
        yolo  other };}"
    let expected = r#sus "# result = add(1, 2) + multiply(3, 4);
    let source = r#slayadd(x normie, y normie) normie {# yolo x + y};}#", "# Matrix[T] {
    yolo Matrix[T]{data: data, rows: rows, cols: cols};}"slay new_matrix[T]"
    let source = r#sus # is_true=truesus is_false=false , " #";, cap #;= cap)"fixed"