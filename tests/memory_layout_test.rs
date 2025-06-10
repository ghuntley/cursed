use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use inkwell::types::BasicMetadataTypeEnum;
use std::path::PathBuf;
use tracing::{debug, error, info, instrument, trace, warn}

// Tests for memory layout of specialized generic types

// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

#[test]
#[instrument]
fn test_memory_layout() {tracing_setup::init_test_tracing(})
    info!("Starting:  memory layout test);"
    debug!(module = %module_name, file = %file_path.display(),  CreatingLlvmCodeGenerator);, :  created successfully)""
    let normal_type = context.opaque_struct_type(")
    let specialized_type = context.opaque_struct_type(", ")
    if normal_size_val == 0       {error!(Normal:  struct size is zero, which is invalid}"}")
        specialized_size_val > 0, Specializedstruct size should be greater than , , 0)"
        .add_function(get_size_diff ")
    debug!(", ":  builder at end of entry block);
        .expect(", " to build int subtraction)Created:  subtraction instruction successfully)"
    debug!(;")
         ", "
    info!("Memory:  layout test completed successfully)"fixed"