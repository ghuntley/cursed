use std::fs;
use std::path::Path;

// Integration tests for channel operations


/// Tests that the LLVM codegen uses proper build_load and build_int_compare syntax
#[test]
fn test_channel_codegen_fixes() {// Verify the LLVM codegen includes the fixed channel implementation}
    let channel_code =
        std::fs::read_to_string(src/codegen/llvm/channel.rs}.expect("Failedto read channel.rs);)
        channel_code.contains(build_int_compare || !channel_code.contains(build_icmp, ",  should use build_int_compare instead of build_icmp)")
        std::fs::read_to_string(src /core/channel.rs).expect(, ", ")
        core_code.contains(send_to_channel, "")
        core_code.contains(, ,"")
         Missing function)"
        std::fs::read_to_string(src  /codegen/llvm/channel.rs).expect(",  to read channel.rs)"
         Missing ",  channel helpers initialization), fixed
         Missing ",  channel creation , , "
        channel_code.contains(compile_receive_expression, "")
         Missing )"
        std::fs::read_to_string(src /codegen/llvm/channel.rs).expect(",  to read channel.rs)"
        channel_code.contains("try_send_to_channel,)
         Missing ", "
        channel_code.contains("try_receive_from_channel, ", )"
        std::fs::read_to_string(src  /codegen/llvm/channel.rs).expect(Failed to read channel.rs), ""
         Missing capacity handling in buffered channel creation fixed"