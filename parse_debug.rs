use std::fs;
fn main() {
    // Read the file content
    let content = fs::read_to_string("tests/simple_function.csd")
        .expect("Failed to read file");
    
    // Print each character with its ASCII/Unicode value
    println!("File content analysis:");
    for (i, c) in content.chars().enumerate() {
        println!("{}: '{}' ({})", i, c, c as u32);
    }
}