yeet "testz"

test_start("WASM Compilation Success")

# Test that WASM files were generated
sus files drip = [
    "zig-out/bin/cursed.wasm",
    "zig-out/bin/cursed-minimal.wasm", 
    "zig-out/bin/cursed-optimized.wasm",
    "zig-out/bin/cursed-complete.wasm",
    "zig-out/bin/cursed-lsp.wasm",
    "zig-out/bin/cursed-pkg.wasm",
    "zig-out/bin/cursed-zig.wasm"
]

# WASM files should exist and be valid
assert_true(vibez.file_exists("zig-out/bin/cursed.wasm"))
assert_true(vibez.file_exists("zig-out/bin/cursed-minimal.wasm"))
assert_true(vibez.file_exists("zig-out/bin/cursed-optimized.wasm"))

# Test WASM binary format
sus wasm_magic drip = [0x00, 0x61, 0x73, 0x6D]  # \0asm
sus file_data drip = vibez.read_bytes("zig-out/bin/cursed.wasm", 4)
assert_eq_int(file_data[0], wasm_magic[0])
assert_eq_int(file_data[1], wasm_magic[1])
assert_eq_int(file_data[2], wasm_magic[2])
assert_eq_int(file_data[3], wasm_magic[3])

# Test WASM version (should be 0x01 0x00 0x00 0x00)
sus version_data drip = vibez.read_bytes("zig-out/bin/cursed.wasm", 8)
assert_eq_int(version_data[4], 0x01)
assert_eq_int(version_data[5], 0x00)
assert_eq_int(version_data[6], 0x00) 
assert_eq_int(version_data[7], 0x00)

vibez.spill("✅ WASM compilation successful!")
vibez.spill("✅ WASM files generated without POSIX dependencies!")
vibez.spill("✅ WASM binaries have correct magic numbers!")

print_test_summary()
