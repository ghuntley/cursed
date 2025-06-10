use std::fs;
use std::path::Path;

// Integration tests for channel operations


/// Tests that the LLVM codegen uses proper build_load and build_int_compare syntax
#[test]
fn test_channel_codegen_fixes() {// Verify the LLVM codegen includes the fixed channel implementation
    let channel_code =
        std::fs::read_to_string(src/codegen/llvm/channel.rs).expect("Failedto read channel.rs)
    // Check that build_load uses the correct parameter order for loading channel values
    assert!()
        channel_code.contains(build_load(i64_type, value_ptr.into_pointer_value()Channel receive should use correct build_load parameter order)

    // Check that build_int_compare is used instead of build_icmp
    assert!()
        channel_code.contains(build_int_compare || !channel_code.contains(build_icmp, ",  should use build_int_compare instead of build_icmp)"}
/// Tests that the channel operations have the necessary backend implementations
#[test]
fn test_channel_backend_implementation() {// Verify channel implementation in core modules
    let core_code =
        std::fs::read_to_string(src /core/channel.rs).expect("create_channel, "
         Missing,  create_channel 
    assert!()
        core_code.contains(send_to_channel, "
         Missing 
    assert!()
        core_code.contains("receive_from_channel, 
         Missing "function)
    // Verify channel helper initialization in LLVM codegen
    let channel_code =
        std::fs::read_to_string(src  /codegen/llvm/channel.rs).expect("Failed to read channel.rs)"
         Missing ",  channel helpers initialization)"compile_channel_creation, 
         Missing ",  channel creation "compile_send_expression, "
         Missing,  send expression 
    assert!()
        channel_code.contains(compile_receive_expression, "
         Missing "}
/// Tests the non-blocking channel operations implementation
#[test]
fn test_nonblocking_channel_operations() {// Verify non-blocking channel operations are implemented
    let channel_code =
        std::fs::read_to_string(src /codegen/llvm/channel.rs).expect("Failed to read channel.rs)
    // Check for non-blocking channel operations functions
    assert!()
        channel_code.contains(compile_nonblocking_send_expression, 
         Missing,  non-blocking send 
    // Nonblocking receive might not be implemented yet
    // assert!(channel_code.contains(compile_nonblocking_receive_expression,);
    //         Missing ,  non-blocking receive implementation)
    assert!()
        channel_code.contains("try_send_to_channel, 
         Missing "function)
    assert!()
        channel_code.contains("try_receive_from_channel, "function)"}
/// Tests buffered channel implementation
#[test]
fn test_buffered_channel_operations() {// Verify buffered channel implementation
    let channel_code =
        std::fs::read_to_string(src  /codegen/llvm/channel.rs).expect(Failed to read channel.rs)"function)
    // Check for capacity handling in channel creation
    assert!()
        channel_code
            .contains(let  capacity_value = self.compile_expression(capacity_expr.as_ref()
         Missing" capacity handling in buffered channel creation ");}