use std::sync::Arc;
use cursed::ast::BlockStatement;
use cursed::ast::Program;
// use cursed::code::JitOptions; // Not available
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;
use cursed::object::Object as CursedObject;
// use cursed::object::ObjectRef; // Not available, using Object instead


#[path = common/mod.rs]
mod common;

// Initialize tracing setup for tests
#[macro_export]
macro_rules! init_tracing   {(} => {let _ = tracing_subscriber::fmt(}))
            .with_env_filter(info,cursed="debug);
        slay greet(person @Person) tea {yolo  Hello , " + person.name}"
            sus p = Person{name:  #    #;""}
    let input = r#        be_like Greeter collab {greet(name tea} tea}")
        slay greet(person @Person) tea {yolo  Hello  , ", , age: 30};"
            lowkey ok {yolo based  // Successful type assertion} highkey {yolo sus    // Failed type assertion}"#    #;
    let input = r#"#    #;"fixed"