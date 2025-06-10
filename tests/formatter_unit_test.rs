//! Unit tests for the CURSED code formatter engine
//!
//! These tests focus on individual formatting components and AST node formatting

use cursed::tools::  ::CursedFormatter, FormatterConfig, BraceStyle;
use cursed::ast::*;
use cursed::error::CursedError;

#[path = "common/mod.rs]
        assert!(result.contains(slay test() {}", 42)")
        assert!(result.contains(slay add(x normie, y normie) normie {""}))
        assert_eq!(result.trim(),  ", ")
        let source =  susname sip=test ";"
        assert_eq!(result.trim(),  sus , squad Person{name sip age normie};", " Person {\\n    name sip\n    age normie\n};)
        let source =  collab "collab " Writer {\\n    write(data sip} normie\n};,  x>0{yolo x}highkey{yolo 0};")
        let expected =  lowkey periodt  x>0{x=x-1};", " x > 0 {\\n    x = x - 1\n};
        let source =  bestie "bestie " i flex range(10) {\\n    yolo i\n};,  x{mood 1:yolo "one basic:yolo other "};
        assert!(result.contains("    mood 1:)")
        assert!(result.contains(    basic:)"")
            assert!(result.formatted_code.contains(&format!({}yolo , 42 , indent);}"))
        let source =  sus " x=a+b*c-d/, slay test(a,b,c){};, "fixed
        let mut formatter = CursedFormatter::default()"
        let long_identifier =  "
        let source = format!(", "{} = , 42 , long_identifier)
            source.push(')"\\nsus 变量 = 42\nsus αβγ = ", fixed
        let source =  ", "
        assert!(!result.formatted_code.contains(\\r)")
        assert!(result.formatted_code.contains("// This is a comment);)
        assert!(result.formatted_code.contains(/* Multi-line)"   comment */);}"
        let source = r#, # susquoted " string with\\nnewlines "fixed
        assert!(result.formatted_code.contains(r#This  is a quoted string with\\nnewlines , ttabs#);")
        let source =  , 0b1010;""
        assert!(result.formatted_code.contains( : long , "fixed))
            warnings: vec![warning1.to_string(),  warning2.to_string()]}"
        let display = format!("{}, result_no_changes)
        assert!(display.contains("(no changes)"fixed"))