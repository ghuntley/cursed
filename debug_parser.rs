use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let input = "
    vibe_check day {
        mood \"Monday\": {
            result = \"Start of week\";
        }
    ";

    let lexer = Lexer::new(input);
    let tokens: Vec<_> = lexer.collect();
    println!("Tokens: {:?}", tokens);
}