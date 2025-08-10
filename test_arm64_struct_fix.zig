const std = @import("std");
const testing = std.testing;

// Simplified ARM64 calling convention test without LLVM
const ARM64CallingConvention = struct {
    const RegisterType = enum {
        General,
        FloatingPoint, 
        Vector,
        Stack,
        IndirectResult,
    };
    
    const ParameterClass = struct {
        register_type: RegisterType,
        register_index: u8,
        stack_offset: u32,
        is_indirect: bool,
        
        pub fn init(reg_type: RegisterType, index: u8) ParameterClass {
            return ParameterClass{
                .register_type = reg_type,
                .register_index = index,
                .stack_offset = 0,
                .is_indirect = false,
            };
        }
    };
    
    /// Fixed ARM64 AAPCS64 struct return classification
    pub fn classifyStructReturn(struct_size: usize, field_count: usize) ParameterClass {
        _ = field_count; // Field count doesn't matter for AAPCS64 struct returns
        
        // ARM64 AAPCS64: structs ≤16 bytes returned in X0/X1 registers
        // regardless of field count or alignment
        if (struct_size <= 16) {
            return ParameterClass.init(.General, 0);
        } else {
            // Large structs (>16 bytes) returned via X8 (indirect result)
            return ParameterClass{
                .register_type = .IndirectResult,
                .register_index = 8,
                .stack_offset = 0,
                .is_indirect = true,
            };
        }
    }
};

test "P0 issue #10: ARM64 struct return fix validation" {
    // Test that small structs (≤16 bytes) use registers regardless of field count
    // This was the bug - field count was incorrectly restricting register usage
    
    // 1-field struct, 8 bytes - should use registers
    const small_1field = ARM64CallingConvention.classifyStructReturn(8, 1);
    try testing.expect(small_1field.register_type == .General);
    try testing.expect(!small_1field.is_indirect);
    
    // 4-field struct, 16 bytes - should use registers (was failing before fix)
    const small_4fields = ARM64CallingConvention.classifyStructReturn(16, 4);
    try testing.expect(small_4fields.register_type == .General);
    try testing.expect(!small_4fields.is_indirect);
    
    // Many-field struct, 16 bytes - should use registers (was failing before fix)
    const many_fields = ARM64CallingConvention.classifyStructReturn(16, 10);
    try testing.expect(many_fields.register_type == .General);
    try testing.expect(!many_fields.is_indirect);
    
    // Exactly 16-byte struct - should use registers
    const exact_16 = ARM64CallingConvention.classifyStructReturn(16, 100);
    try testing.expect(exact_16.register_type == .General);
    try testing.expect(!exact_16.is_indirect);
    
    // Large struct (>16 bytes) - should use X8 indirect
    const large_struct = ARM64CallingConvention.classifyStructReturn(17, 1);
    try testing.expect(large_struct.register_type == .IndirectResult);
    try testing.expect(large_struct.register_index == 8);
    try testing.expect(large_struct.is_indirect);
    
    // SQLite driver typical struct size (e.g., sqlite3_stmt*) - should use registers
    const sqlite_struct = ARM64CallingConvention.classifyStructReturn(12, 3);
    try testing.expect(sqlite_struct.register_type == .General);
    try testing.expect(!sqlite_struct.is_indirect);
}

test "ARM64 parameter classification improvement" {
    // Verify that parameter classification respects the 16-byte boundary correctly
    // This affects how structs are passed as parameters to C functions
    
    const small_param = ARM64CallingConvention.classifyStructReturn(8, 2);
    try testing.expect(small_param.register_type == .General);
    
    const medium_param = ARM64CallingConvention.classifyStructReturn(16, 8);
    try testing.expect(medium_param.register_type == .General);
    
    const large_param = ARM64CallingConvention.classifyStructReturn(32, 2);
    try testing.expect(large_param.register_type == .IndirectResult);
    try testing.expect(large_param.is_indirect);
}
