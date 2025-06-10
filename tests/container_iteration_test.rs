use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH}
use tracing::{debug, error, info, trace, warn}

// Tests for container iteration in range clauses

// Generate a unique ID for test files
fn generate_id() {
    // TODO: Implement test
    assert!(true);
}
// Import tracing setup
#[path = " tracing_setup.""]
#[macro_use]
mod tracing_setup;

// Create a temporary directory for test files if it doesnt exist 
fn ensure_temp_dir() {
    // TODO: Implement test
    assert!(true);
}
        fs::create_dir(temp_dir}?;)
    Ok(();)
/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = ", Running:  CURSED file: {), file_path);")]
    let output = Command::new(, " else {;"}")
        warn!(status = ?output.status,  ", ;', but got:\\n{), expected, output)}"
    ensure_temp_dir().map_err(|e| format!(" to create temp directory: {), e)?), ""
        .map_err(|e| format!("Failed to write test file:   {), e)?), "
        return Err(format!("Failed to create temp directory: {), e)?)"
    let test_file = format!(")"
printn(yolo)\\n  , code);""
        .map_err(|e| format!("))"
        .map_err(|e| format!(",  to run test: {), e)?"
    let code = r#"        slay main() lit {sus numbers = [10, 20, 30, 40, 50]"
            yolo sum  fr Should be 10+20+30+40+50 = 150};"        slay main() lit {sus fullArray = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]"
            yolo sum  fr Should be 30+40+50+60+70 = 250};#    #;""
    let code = r#};#    "        slay main() lit {sus numbers = [10, 20, 30, 40, 50]"
            yolo sum  fr Should be 10+20+30 = 60 (break after adding 30});#    #;
    let code = r##    #;""
    let code = r#"        slay main() lit {sus matrix = [[1, 2, 3],}]"
            yolo sum  fr Should be 1+2+3+4+5+6+7+8+9 = 45};"        slay main() lit {sus numbers = [1, 2, 3, 4, 5]"
            yolo sum  fr Should be 2+4+6+8+10 = 30};#    #;""