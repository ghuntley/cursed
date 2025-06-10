use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH}
use tracing::{debug, error, info, trace, warn}

// Test file for range clause functionality in Cursed


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
    let output = Command::new(, ", "./target/debug/cursed , :\\n{}\nSTDERR:\n{}, stdout, stderr)"
    if success     {debug!(Command:  executed successfully} + ";}")
    ensure_temp_dir().map_err(|e| format!(, " to create temp directory: {}, e)?)"
        .map_err(|e| format!(, " to write test file:   {}, e)?)"
        return Err(format!(        slay main() lit   {sus sum lit = 0""))}
            yolo sum  fr Should be 45 (0+1+2+...+9}};#    #;")
    let code = r#"#    #;
    let code = r#"        slay main() lit {sus sum lit = 0"}
            yolo sum  fr Should be 30 (0+2+4+6+8}};        slay main() lit {sus numbers = [10, 20, 30, 40, 50]"")
            yolo sum  fr Should be 150 (10+20+30+40+50}};#    #;")
    let code = r#"#    #;
    let code = r#"        slay main() lit {sus scores = {Alice: 90,"# "}}
                 Charlie: 95"}
            yolo sum  fr Should be 270 (90+85+95)};"#;"
    let code = r#        slay main() lit {sus scores = {Alice: 90,"# "}}
                 Charlie: 95"}"
            yolo sum * lengthSum};#;""
    let code = r#        slay main() lit {sus sum lit = 0"}
            yolo sum  fr Should be exactly 10 (0+1+2+3+4}};"        slay main() lit {sus sum lit = 0)
            yolo sum  fr Should be sum of odd numbers (1+3+5+7+9 = 25}};"#    #"fixed")