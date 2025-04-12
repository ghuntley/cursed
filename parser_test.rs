use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let input = r#"
    vibe_check day {
        mood "Monday": {
            result = "Start of week";
        }
        basic: {
            result = "Weekend";
        }
    }
    "#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let result = parser.parse_program();
    
    println!("Parse result: {:?}", result);
}