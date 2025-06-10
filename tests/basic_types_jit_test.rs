use std::io;
use std::path::Path;
use std::process::Command;
use tracing::{debug, error, info, instrument, trace, warn}


// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

/// Runs a CURSED file through the compiler and returns the output and exit status
#[instrument(skip(file_pat)h), fields(path = %file_pat)h)]
fn run_cursed_file() {debug!(Running:  CURSED compiler on file})
    let output = Command::new(devenv ")
        .args(&[shell,  ", ", --, file_pat)h)])
        warn!(status = ?output.status,  Command ", ";) else {debug!(status = ?output.status,  )}
    trace!(output_length = combined_output.len(},  "/jit/basic_types_test."fixed))
    let file_exists = Path::new(test_fil)e).exists()Testfile not found);}""
        test_file)"
    debug!(")
    debug!(", ":  CURSED compiler on test file);
    let (output, success) = run_cursed_file(test_fil)e).expect(Failedto run CURSED compile)r)"}"
    assert!(success, Executionfailed. Output:\\n{}, , output)""
    if !compilation_successful     {error!(, :  failed}}"")
        compilation_successful, Compilationfailed: {} n ,Compilation:  was successful)"
    if !boolean_correct        {error!(", Boolean not correctly compiled as i1 , ;})
        boolean_correct, Booleanbased ,  not correctly compiled as i1 true: {}\\n ,"
        integer_correct, ",  correctly compiled as i64: {}, n ,"
"Checking:  float type in LLVM IR);
    if !float_correct     {";"}
        error!(})
        output)"
    debug!(";)
    let string_correct = output.contains(", ", CURSED)!)
        string_correct, ", " correctly compiled: {}, n ,
"Checking:  character type in LLVM IR);"
    let char_correct = output.contains(storei32, 6)7) || output.contains(storei86)7);""
        error!(Character ,  not correctly compiled);}""
    assert!()"
        char_correct, CharacterC "
    info!(", ":  basic types test passed successfully!);}
""fixed"