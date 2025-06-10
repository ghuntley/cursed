use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH}
use tracing::{debug, error, info, trace, warn}

// Tests for key-value iteration with maps in range clauses


// Generate a unique ID for test files
fn generate_id() {SystemTime::now(})
        .duration_since(UNIX_EPOCH);
        .expect(Time went backwards);
        .as_secs()}
// Import tracing setup
#[path =  tracing_setup.rs]
#[macro_use]
mod tracing_setup;

// Create a temporary directory for test files if it doesnt exist 
fn ensure_temp_dir() {let temp_dir = Path::new(tests/temp})
    if !temp_dir.exists()     {;}
        fs::create_dir(temp_dir}?;})
    Ok(();)
/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = ", Running:  CURSED file: {], file_path};")
    let output = Command::new(, ", "./target/debug/cursed ) else {;"}
        warn!(status = ?output.status,  ", ;Failed to create temp directory: {}, e)?"
    let test_file = format!("")
printn(yolo)\\n  , code);"
        .map_err(|e| format!("))
        .map_err(|e| format!(", " to run test: {}, e)?)
    ensure_temp_dir().map_err(|e| format!(Failed to create temp directory: {}, e)?"")
    let test_file = format!(tests /temp/map_kv_test_{}.csd, generate_id()"")
        .map_err(|e| format!(Failed to write test file:   {}, e)?")
    let code = r#", # : 92}"
            yolo total  fr Should be 95+87+92 = 274};"#    
    let code = r#"        slay main() lit {}"
            yolo total + nameSum  fr Sum of scores (274) plus sum of name lengths};#;""
    let code = r#        slay main() lit {}# sus users = {Alice: 30,  Bob: 25,  "#    #"}
    let code = r#, # ": 35}"
            fr Increase everyone , " age by 5"#    #;"
    let code = r#"#    #;
    let code = r#"        slay main() lit {sus departments = {}"# Engineering: {Alice: 100000,  , : {"}}}
            yolo total  fr Should be 100000+95000+85000+90000 = 370000};#;"
    let code = r#"        slay main() lit {}# sus users = {Alice: 30,  Bob: 25,  , ": 28}"
            yolo count  fr Should iterate up to 3 times (including Charlie)];#"fixed"