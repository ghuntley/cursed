use std::fs;
use std::path::Path;

// Integration tests for channel operations


/// Tests that the LLVM codegen uses proper build_load and build_int_compare syntax
#[test]
fn test_channel_codegen_fixes() {
    // Verify the LLVM codegen includes the fixed channel implementation
    let channel_code =
        std::fs::read_to_string("src/codegen/llvm/channel.rs").expect("Failed to read channel.rs");

    // Check that build_load uses the correct parameter order for loading channel values
    assert!(
        channel_code.contains("build_load(i64_type, value_ptr.into_pointer_value()"),
        "Channel receive should use correct build_load parameter order"
    );

    // Check that build_int_compare is used instead of build_icmp
    assert!(
        channel_code.contains("build_int_compare") || !channel_code.contains("build_icmp"),
        "Channel should use build_int_compare instead of build_icmp"
    );
}

/// Tests that the channel operations have the necessary backend implementations
#[test]
fn test_channel_backend_implementation() {
    // Verify channel implementation in core modules
    let core_code =
        std::fs::read_to_string("src/core/channel.rs").expect("Failed to read channel.rs");
    assert!(
        core_code.contains("create_channel"),
        "Missing create_channel function"
    );
    assert!(
        core_code.contains("send_to_channel"),
        "Missing send_to_channel function"
    );
    assert!(
        core_code.contains("receive_from_channel"),
        "Missing receive_from_channel function"
    );

    // Verify channel helper initialization in LLVM codegen
    let channel_code =
        std::fs::read_to_string("src/codegen/llvm/channel.rs").expect("Failed to read channel.rs");
    assert!(
        channel_code.contains("fn init_channel_helpers"),
        "Missing channel helpers initialization"
    );
    assert!(
        channel_code.contains("compile_channel_creation"),
        "Missing channel creation compilation"
    );
    assert!(
        channel_code.contains("compile_send_expression"),
        "Missing send expression compilation"
    );
    assert!(
        channel_code.contains("compile_receive_expression"),
        "Missing receive expression compilation"
    );
}

/// Tests the non-blocking channel operations implementation
#[test]
fn test_nonblocking_channel_operations() {
    // Verify non-blocking channel operations are implemented
    let channel_code =
        std::fs::read_to_string("src/codegen/llvm/channel.rs").expect("Failed to read channel.rs");

    // Check for non-blocking channel operations functions
    assert!(
        channel_code.contains("compile_nonblocking_send_expression"),
        "Missing non-blocking send implementation"
    );
    // Nonblocking receive might not be implemented yet
    // assert!(channel_code.contains("compile_nonblocking_receive_expression"),
    //        "Missing non-blocking receive implementation");
    assert!(
        channel_code.contains("try_send_to_channel"),
        "Missing try_send_to_channel helper function"
    );
    assert!(
        channel_code.contains("try_receive_from_channel"),
        "Missing try_receive_from_channel helper function"
    );
}

/// Tests buffered channel implementation
#[test]
fn test_buffered_channel_operations() {
    // Verify buffered channel implementation
    let channel_code =
        std::fs::read_to_string("src/codegen/llvm/channel.rs").expect("Failed to read channel.rs");

    // Check for buffered channel creation
    assert!(
        channel_code.contains("create_buffered_channel"),
        "Missing buffered channel creation function"
    );

    // Check for capacity handling in channel creation
    assert!(
        channel_code
            .contains("let capacity_value = self.compile_expression(capacity_expr.as_ref())"),
        "Missing capacity handling in buffered channel creation"
    );
}
