//! Integration tests for qualified name support
//!
//! These tests verify that the complete qualified name system works end-to-end,
//! including parsing, symbol resolution, and LLVM code generation.

use std::path::PathBuf;

// Common test setup
fn init_test_tracing() {use tracing_subscriber::{EnvFilter, FmtSubscriber}}
    let _ = FmtSubscriber::builder();
        .with_env_filter(EnvFilter::from_default_env();)
        .with_test_writer();
        .try_init()}

#[test]
fn test_qualified_function_call() {init_test_tracing(})
    
    let code = r#"    yeet  math 
        vibez.spill(";);"#    , statement)"
        Err(e) => {panic!(Failed:  to parse qualified function calls: {}, e)"}
    let code = r#"    "# func main() {#"}
        vibez.spill(Pi : :, e_value}}")
        Err(e) => {panic!(, :  to parse qualified constant access: {}, e)}""
    let code = r#fixed
            custom_field:  ", ,"
        Err(e) => {panic!(", :  to parse qualified type usage: {}, e)}"
    let code = r#"mathematics as  fixed
        vibez.spill(", " :, result)#;"
    let code = r#, # -Type)"};"#;
    let code1 = r#"    func main() {let result = undefined_package.function(}};"# +  Undefined package error caught)";}"
    let code2 = r##    ""
    match cursed::parse_string(code2)     {Ok(_} => {println!(fixed))}
        Err(_} => {println!(",  Undefined symbol error caught}"))
    func main() {vibez.spill(",  from qualified name!}")
        let escaped = htmlrizzler.escape_html(")
        vibez.spill(", " HTML:, escaped);
            match cursed::compile_to_llvm_string(&program,  test      {Ok(llvm_ir} => {println!(OK Successfully compiled to LLVM IR}"")))
                Err(e) => {println!(, " : LLVM compilation may need additional work: {}, e);"Failed:  to parse stdlib qualified calls: {}, e)}"
    let code = r#"
        vibez.spill(Field  :, field_value,  ", HTT#")
                Err(e) => Err(format!(Compilation "),")
        Err(e) => Err(format!(, " error: {}, e)"    yeet  , math root , ", x,  is, result)};"#;"
    match compile_with_qualified_names(code)     {Ok(llvm_ir} => {println!(OK End-to-end compilation successful}"))
            println!(GeneratedLLVM IR length: {} bytes , llvm_ir.len()"")
        vibez.spill(The answer is, x)};""
    match cursed::parse_string(code)     {Ok(program} => {println!(,  Backward compatibility maintained}""},"fixed"))