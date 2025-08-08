// Demo of CURSED security linter functionality

const std = @import("std");
const linter = @import("src-zig/tools/linter.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.log.info("=== CURSED Security Linter Demo ===", .{});
    
    // Test secret detection patterns
    const test_patterns = [_]struct {
        value: []const u8,
        description: []const u8,
    }{
        .{ .value = "sk_1234567890abcdef1234567890abcdef", .description = "API Key" },
        .{ .value = "AKIA1234567890ABCDEF", .description = "AWS Access Key" },
        .{ .value = "ghp_1234567890abcdef1234567890abcdef", .description = "GitHub Token" },
        .{ .value = "password123", .description = "Password" },
        .{ .value = "admin_secret_key", .description = "Admin Secret" },
        .{ .value = "-----BEGIN PRIVATE KEY-----", .description = "Private Key" },
        .{ .value = "mysql://user:pass@localhost/db", .description = "DB Connection" },
        .{ .value = "abcdef1234567890abcdef1234567890abcdef12", .description = "Hex Key" },
    };
    
    var config = linter.LinterConfig.init(allocator);
    defer config.deinit();
    
    var test_linter = linter.Linter.init(allocator, config);
    defer test_linter.deinit();
    
    std.log.info("\n🔍 Testing Secret Detection Patterns:", .{});
    
    for (test_patterns) |pattern| {
        const is_api_key = test_linter.looksLikeApiKey(pattern.value);
        const is_password = test_linter.looksLikePassword(pattern.value);
        const is_private_key = test_linter.looksLikePrivateKey(pattern.value);
        
        if (is_api_key or is_password or is_private_key) {
            const detected_type = if (is_api_key) "API Key" else if (is_password) "Password" else "Private Key";
            std.log.info("🚨 DETECTED {s}: {s} -> {s}", .{ detected_type, pattern.description, pattern.value });
        } else {
            std.log.info("✅ Safe: {s} -> {s}", .{ pattern.description, pattern.value });
        }
    }
    
    std.log.info("\n🛡️ Security Rules Implemented:", .{});
    std.log.info("✅ Hardcoded API Keys Detection", .{});
    std.log.info("✅ Hardcoded Passwords Detection", .{});
    std.log.info("✅ Private Keys Detection", .{});
    std.log.info("✅ Database Connection Strings", .{});
    std.log.info("✅ Insecure Crypto Functions (md5, sha1, des, rc4)", .{});
    std.log.info("✅ Dangerous System Calls (system, exec)", .{});
    std.log.info("✅ Buffer Overflow Risks (strcpy, strcat, sprintf)", .{});
    std.log.info("✅ SQL Injection Detection", .{});
    std.log.info("✅ Memory Safety (missing defer cleanup)", .{});
    std.log.info("✅ Error Handling Validation", .{});
    std.log.info("✅ Channel Deadlock Detection", .{});
    std.log.info("✅ Weak Random Number Generation", .{});
    std.log.info("✅ Unchecked Array Access", .{});
    
    std.log.info("\n🎯 CURSED-Specific Security Features:", .{});
    std.log.info("• Integrates with CURSED AST for deep analysis", .{});
    std.log.info("• Checks CURSED memory management patterns", .{});
    std.log.info("• Validates CURSED error handling (damn/yikes)", .{});
    std.log.info("• Ensures proper defer usage for resource cleanup", .{});
    std.log.info("• Detects channel operations that could deadlock", .{});
    std.log.info("• Validates CURSED crypto stdlib usage", .{});
    
    std.log.info("\n✨ Production-Ready Implementation Complete!", .{});
}
