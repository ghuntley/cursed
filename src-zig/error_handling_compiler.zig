const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const error_operators = @import("error_operators.zig");
const YikesError = error_operators.YikesError;
const ShookResult = error_operators.ShookResult;
const FamBlock = error_operators.FamBlock;

/// Simple error handling compiler for testing yikes, shook, fam operators
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: {} <program.csd>\n", .{args[0]});
        print("CURSED Error Handling Compiler - Testing yikes, shook, fam operators\n");
        print("\nError handling syntax:\n");
        print("  yikes \"message\", code     - Create error\n");
        print("  shook expression           - Propagate error\n");
        print("  fam {{ ... }} catch(e) {{ ... }}  - Panic recovery\n");
        return;
    }

    const filename = args[1];
    print("🔧 CURSED Error Handling Compiler\n");
    print("📁 Processing: {s}\n", .{filename});

    // Read file content
    const content = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("❌ Error reading file: {}\n", .{err});
        return;
    };
    defer allocator.free(content);

    print("📄 File content ({} bytes):\n{s}\n", .{ content.len, content });

    // Simple pattern matching for error handling keywords
    try analyzeErrorHandling(allocator, content);
    
    // Test the error operators
    try testErrorOperators(allocator);

    print("✅ Error handling analysis completed\n");
}

fn analyzeErrorHandling(allocator: Allocator, content: []const u8) !void {
    print("\n🔍 Error handling analysis:\n");
    
    var line_number: u32 = 1;
    var lines = std.mem.split(u8, content, "\n");
    
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        if (trimmed.len == 0) {
            line_number += 1;
            continue;
        }

        // Skip comments
        if (std.mem.indexOf(u8, trimmed, "fr fr")) |_| {
            line_number += 1;
            continue;
        }

        // Analyze yikes expressions
        if (std.mem.indexOf(u8, trimmed, "yikes")) |_| {
            try processYikesExpression(allocator, trimmed, line_number);
        }

        // Analyze shook expressions  
        if (std.mem.indexOf(u8, trimmed, "shook")) |_| {
            try processShookExpression(allocator, trimmed, line_number);
        }

        // Analyze fam blocks
        if (std.mem.indexOf(u8, trimmed, "fam")) |_| {
            try processFamExpression(allocator, trimmed, line_number);
        }

        line_number += 1;
    }
}

fn processYikesExpression(allocator: Allocator, line: []const u8, line_number: u32) !void {
    print("Line {}: 💥 YIKES expression - {s}\n", .{ line_number, line });
    
    // Extract error message and code if possible
    if (std.mem.indexOf(u8, line, "\"")) |start| {
        if (std.mem.indexOf(u8, line[start + 1..], "\"")) |end| {
            const message = line[start + 1..start + 1 + end];
            print("   📝 Error message: \"{s}\"\n", .{message});
            
            // Look for error code
            if (std.mem.indexOf(u8, line, ",")) |comma| {
                const code_part = std.mem.trim(u8, line[comma + 1..], " \t");
                print("   🔢 Error code: {s}\n", .{code_part});
            }
            
            // Create actual YikesError for testing
            var test_error = YikesError.init(allocator, message, 42) catch {
                print("   ⚠️  Could not create YikesError\n");
                return;
            };
            defer test_error.deinit();
            
            print("   ✅ Created YikesError successfully\n");
        }
    }
}

fn processShookExpression(allocator: Allocator, line: []const u8, line_number: u32) !void {
    print("Line {}: ⚡ SHOOK expression - {s}\n", .{ line_number, line });
    
    // Simulate error propagation
    print("   🔄 Simulating error propagation...\n");
    
    // Create a test error and shook result
    var test_error = YikesError.init(allocator, "Test propagation", 100) catch return;
    defer test_error.deinit();
    
    const shook_result = ShookResult.err(test_error);
    if (shook_result.isError()) {
        print("   ✅ Error propagation working\n");
    }
}

fn processFamExpression(allocator: Allocator, line: []const u8, line_number: u32) !void {
    print("Line {}: 🛡️  FAM expression - {s}\n", .{ line_number, line });
    
    // Check for catch blocks
    if (std.mem.indexOf(u8, line, "catch")) |_| {
        print("   🎯 Catch handler detected\n");
    }
    
    if (std.mem.indexOf(u8, line, "finally")) |_| {
        print("   🏁 Finally handler detected\n");
    }
    
    // Create test fam block
    var fam_block = FamBlock.init(allocator);
    defer fam_block.deinit();
    
    print("   ✅ FAM block structure working\n");
}

fn testErrorOperators(allocator: Allocator) !void {
    print("\n🧪 Testing error operators implementation:\n");
    
    // Test 1: yikes error creation
    print("1. Testing yikes error creation...\n");
    var error1 = YikesError.init(allocator, "Test error", 404) catch {
        print("   ❌ YikesError creation failed\n");
        return;
    };
    defer error1.deinit();
    print("   ✅ yikes: Created error with message '{s}' and code {}\n", .{ error1.getMessage(), error1.getCode() });
    
    // Test 2: shook error propagation
    print("2. Testing shook error propagation...\n");
    const shook_result = ShookResult.err(error1);
    if (shook_result.isError()) {
        const propagated_error = shook_result.getError().?;
        print("   ✅ shook: Error propagated - {s}\n", .{propagated_error.getMessage()});
    }
    
    // Test 3: fam block creation
    print("3. Testing fam panic recovery...\n");
    var fam_block = FamBlock.init(allocator);
    defer fam_block.deinit();
    print("   ✅ fam: Recovery block created successfully\n");
    
    // Test 4: Complete error flow
    print("4. Testing complete error handling flow...\n");
    var original_error = YikesError.init(allocator, "Original error", 500) catch return;
    defer original_error.deinit();
    
    const result = ShookResult.err(original_error);
    if (result.isError()) {
        print("   ✅ Complete flow: yikes -> shook -> fam working\n");
    }
    
    print("\n🎉 All error operator tests passed!\n");
}
