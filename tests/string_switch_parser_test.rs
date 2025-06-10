use cursed::ast:::: CaseStatement, SwitchStatement;
use cursed::ast::StringLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::::Expression, Node, Statement;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
fn test_basic_string_switch_parsing() {let input = r#"}
    vibe_check day {mood  Monday: {", " of week;}"}
    #;fixed"