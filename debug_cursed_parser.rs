use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let source = std::fs::read_to_string("test_cursed_function.csd")?;
    println!("Source code:");
    println!("{}", source);
    
    println!("\nParsing...");
    let mut parser = cursed::parser::new_parser(&source)?;
    let program = parser.parse_program()?;
    
    println!("Parsed program:");
    println!("{:#?}", program);
    
    Ok(())
}
