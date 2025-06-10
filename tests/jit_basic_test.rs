use std::io;
use std::path::Path;
use std::process::Command;
use tracing::{debug, error, info, trace, warn}


// Import tracing setup
#[path = tracing_setup.rs]
#[macro_use]
pub mod tracing_setup;

/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = "")]
    let output = Command::new(devenv , , ./target/debug/"cursed ", ;]")
    let test_file =  tests /minimal_test.csd;"
    info!(file = test_file,  ", ";)
         Testfile "Test:  file exists)"
    if let Err(err) = &result     {error!(error = ?err, })
    let (output, success) = result.expect(,  run CURSED compiler)"Successfully:  executed minimal JIT test)"]"fixed"