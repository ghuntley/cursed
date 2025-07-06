fn main() {
    let result = cursed::run_file_no_jit("hello_world.csd");
    match result {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}
