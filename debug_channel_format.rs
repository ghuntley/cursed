use cursed::formatter::CursedFormatter;

fn main() {
    let formatter = CursedFormatter::default();
    let source = r#"
sus ch chan normie = make(chan normie)
ch<-42
value:=<-ch
"#;
    
    let formatted = formatter.format(source.trim()).unwrap();
    println!("Formatted result:");
    println!("'{}'", formatted);
    
    // Check specific parts
    println!("\nChecking parts:");
    println!("Contains 'ch <- 42': {}", formatted.contains("ch <- 42"));
    println!("Contains 'value := <-ch': {}", formatted.contains("value := <-ch"));
    
    // Print each line
    println!("\nLines:");
    for (i, line) in formatted.lines().enumerate() {
        println!("{}: '{}'", i, line);
    }
}
