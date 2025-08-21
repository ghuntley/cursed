const std = @import("std");
const print = std.debug.print;

/// Architecture mismatch detection and resolution for CURSED binary generation
pub const ArchitectureMismatchDetector = struct {
    allocator: std.mem.Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .allocator = allocator,
        };
    }
    
    /// Detect if a binary has architecture mismatches that prevent execution
    pub fn detectBinaryMismatch(self: Self, binary_path: []const u8) !BinaryAnalysis {
        var analysis = BinaryAnalysis{
            .has_mismatch = false,
            .expected_interpreter = null,
            .actual_interpreter = null,
            .target_arch = null,
            .issues = .{},
        };
        
        // Use `file` command to analyze the binary
        const result = std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{ "file", binary_path },
        }) catch |err| {
            try analysis.issues.append(try std.fmt.allocPrint(self.allocator, "Failed to analyze binary: {}", .{err}));
            return analysis;
        };
        
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);
        
        if (result.term != .Exited or result.term.Exited != 0) {
            try analysis.issues.append(try std.fmt.allocPrint(self.allocator, "file command failed: {s}", .{result.stderr}));
            return analysis;
        }
        
        // Parse the output for architecture info
        const output = result.stdout;
        
        // Check for musl vs glibc interpreter mismatch
        if (std.mem.containsAtLeast(u8, output, 1, "ld-musl")) {
            analysis.actual_interpreter = try self.allocator.dupe(u8, "/lib/ld-musl-x86_64.so.1");
            analysis.expected_interpreter = try self.allocator.dupe(u8, "/lib64/ld-linux-x86-64.so.2");
            analysis.has_mismatch = true;
            try analysis.issues.append(self.allocator, try self.allocator.dupe(u8, "Binary compiled with musl libc but system uses glibc"));
        } else if (std.mem.containsAtLeast(u8, output, 1, "ld-linux-x86-64")) {
            analysis.actual_interpreter = try self.allocator.dupe(u8, "/lib64/ld-linux-x86-64.so.2");
            analysis.expected_interpreter = try self.allocator.dupe(u8, "/lib64/ld-linux-x86-64.so.2");
        }
        
        // Check for architecture mismatch
        if (std.mem.containsAtLeast(u8, output, 1, "x86-64")) {
            analysis.target_arch = try self.allocator.dupe(u8, "x86_64");
        } else if (std.mem.containsAtLeast(u8, output, 1, "aarch64") or std.mem.containsAtLeast(u8, output, 1, "ARM64")) {
            analysis.target_arch = try self.allocator.dupe(u8, "aarch64");
        }
        
        return analysis;
    }
    
    /// Check if the current system can execute binaries with the given architecture
    pub fn checkSystemCompatibility(self: Self) !SystemCompatibility {
        var compat = SystemCompatibility{
            .has_musl = false,
            .has_glibc = false,
            .supported_arches = .{},
            .dynamic_linkers = .{},
        };
        
        // Check for glibc
        std.fs.cwd().access("/lib64/ld-linux-x86-64.so.2", .{}) catch {
            // Try alternative locations
            std.fs.cwd().access("/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2", .{}) catch {
                // glibc not found
            } else {
                compat.has_glibc = true;
                try compat.dynamic_linkers.append(self.allocator, try self.allocator.dupe(u8, "/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2"));
            };
        } else {
            compat.has_glibc = true;
            try compat.dynamic_linkers.append(self.allocator, try self.allocator.dupe(u8, "/lib64/ld-linux-x86-64.so.2"));
        }
        
        // Check for musl
        std.fs.cwd().access("/lib/ld-musl-x86_64.so.1", .{}) catch {
            // musl not found
        } else {
            compat.has_musl = true;
            try compat.dynamic_linkers.append(self.allocator, try self.allocator.dupe(u8, "/lib/ld-musl-x86_64.so.1"));
        }
        
        // Determine supported architectures from current system
        try compat.supported_arches.append(self.allocator, try self.allocator.dupe(u8, @tagName(@import("builtin").target.cpu.arch)));
        
        return compat;
    }
    
    /// Provide fix recommendations for architecture mismatches
    pub fn provideFix(self: Self, analysis: BinaryAnalysis, system: SystemCompatibility) ![]const u8 {
        var fix_message = std.ArrayList(u8).init(self.allocator);
        const writer = fix_message.writer();
        
        try writer.writeAll("🔧 Architecture Mismatch Fix Recommendations:\n\n");
        
        if (analysis.has_mismatch) {
            for (analysis.issues.items) |issue| {
                try writer.print("❌ Issue: {s}\n", .{issue});
            }
            
            try writer.writeAll("\n💡 Solutions:\n");
            
            if (analysis.actual_interpreter != null and analysis.expected_interpreter != null) {
                if (std.mem.containsAtLeast(u8, analysis.actual_interpreter.?, "musl")) {
                    try writer.writeAll("1. Rebuild with glibc instead of musl:\n");
                    try writer.writeAll("   zig build -Dtarget=x86_64-linux-gnu -Ddynamic-linker=/lib64/ld-linux-x86-64.so.2\n\n");
                    
                    try writer.writeAll("2. Or install musl compatibility:\n");
                    try writer.writeAll("   sudo apt install musl-dev musl-tools\n");
                    try writer.writeAll("   sudo ln -s /usr/lib/x86_64-linux-musl/libc.so /lib/ld-musl-x86_64.so.1\n\n");
                }
            }
            
            try writer.writeAll("3. Force glibc target in build.zig (recommended fix applied):\n");
            try writer.writeAll("   Default target now uses .gnu ABI to prevent musl linking\n\n");
        } else {
            try writer.writeAll("✅ No architecture mismatches detected!\n");
        }
        
        if (system.has_glibc) {
            try writer.writeAll("✅ System has glibc support\n");
        }
        
        if (system.has_musl) {
            try writer.writeAll("✅ System has musl support\n");
        }
        
        try writer.writeAll("\n🚀 Available dynamic linkers:\n");
        for (system.dynamic_linkers.items) |linker| {
            try writer.print("   - {s}\n", .{linker});
        }
        
        return fix_message.toOwnedSlice();
    }
};

pub const BinaryAnalysis = struct {
    has_mismatch: bool,
    expected_interpreter: ?[]const u8,
    actual_interpreter: ?[]const u8,
    target_arch: ?[]const u8,
    issues: std.ArrayList([]const u8),
    
    pub fn deinit(self: *BinaryAnalysis, allocator: std.mem.Allocator) void {
        if (self.expected_interpreter) |interp| allocator.free(interp);
        if (self.actual_interpreter) |interp| allocator.free(interp);  
        if (self.target_arch) |arch| allocator.free(arch);
        
        for (self.issues.items) |issue| {
            allocator.free(issue);
        }
        self.issues.deinit(self);
    }
};

pub const SystemCompatibility = struct {
    has_musl: bool,
    has_glibc: bool,
    supported_arches: std.ArrayList([]const u8),
    dynamic_linkers: std.ArrayList([]const u8),
    
    pub fn deinit(self: *SystemCompatibility, allocator: std.mem.Allocator) void {
        for (self.supported_arches.items) |arch| {
            allocator.free(arch);
        }
        self.supported_arches.deinit(self);
        
        for (self.dynamic_linkers.items) |linker| {
            allocator.free(linker);
        }
        self.dynamic_linkers.deinit(self);
    }
};
