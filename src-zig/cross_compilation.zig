const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

// Cross-compilation utilities for CURSED compiler
pub const CrossCompiler = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) CrossCompiler {
        return CrossCompiler{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *CrossCompiler) void {
        _ = self;
    }
    
    // Generate cross-compilation command for target platform
    pub fn generateCompileCommand(
        self: *CrossCompiler,
        source_file: []const u8,
        target_platform: []const u8,
        output_file: ?[]const u8,
        optimization_level: u8,
        linking_mode: []const u8,
        verbose: bool
    ) ![][]const u8 {
        var command_parts = .empty;
        
        try command_parts.append(self.allocator, "zig");
        try command_parts.append(self.allocator, "build-exe");
        try command_parts.append(self.allocator, source_file);
        
        // Add target specification
        const target_arg = try std.fmt.allocPrint(self.allocator, "-target={s}", .{target_platform});
        try command_parts.append(self.allocator, target_arg);
        
        // Add optimization level
        const opt_arg = switch (optimization_level) {
            0 => "-O0",
            1 => "-O1",
            2 => "-O2",
            3 => "-O3",
            else => "-O2",
        };
        try command_parts.append(self.allocator, opt_arg);
        
        // Add linking mode for static linking
        if (std.mem.eql(u8, linking_mode, "static")) {
            try command_parts.append(self.allocator, "--static");
        }
        
        // Add output file specification
        if (output_file) |output| {
            const output_arg = try std.fmt.allocPrint(self.allocator, "--name={s}", .{output});
            try command_parts.append(self.allocator, output_arg);
        }
        
        // Add libc linkage for native targets
        if (!std.mem.startsWith(u8, target_platform, "wasm")) {
            try command_parts.append(self.allocator, "-lc");
        }
        
        if (verbose) {
            print("🔧 Cross-compilation command: {s}\n", .{try std.mem.join(self.allocator, " ", command_parts.items)});
        }
        
        return command_parts.toOwnedSlice(self.allocator);
    }
    
    // Generate compilation command for LLVM IR files
    pub fn generateLLVMIRCompileCommand(
        self: *CrossCompiler,
        ir_file: []const u8,
        target_platform: []const u8,
        output_file: ?[]const u8,
        optimization_level: u8,
        linking_mode: []const u8,
        verbose: bool
    ) ![][]const u8 {
        var command_parts = .empty;
        
        // Use llc to compile LLVM IR to object file, then link
        if (std.mem.eql(u8, target_platform, "wasm32-freestanding")) {
            // WebAssembly compilation
            try command_parts.append(self.allocator, "llc");
            try command_parts.append(self.allocator, ir_file);
            try command_parts.append(self.allocator, "-march=wasm32");
            try command_parts.append(self.allocator, "-filetype=obj");
            const obj_file = try std.fmt.allocPrint(self.allocator, "{s}.o", .{std.fs.path.stem(ir_file)});
            const obj_arg = try std.fmt.allocPrint(self.allocator, "-o={s}", .{obj_file});
            try command_parts.append(self.allocator, obj_arg);
        } else {
            // Native compilation using zig cc as a wrapper
            try command_parts.append(self.allocator, "zig");
            try command_parts.append(self.allocator, "cc");
            try command_parts.append(self.allocator, ir_file);
            
            // Add target specification (skip for native)
            if (!std.mem.eql(u8, target_platform, "native")) {
                const target_arg = try std.fmt.allocPrint(self.allocator, "-target={s}", .{target_platform});
                try command_parts.append(self.allocator, target_arg);
            }
            
            // Add optimization level
            const opt_arg = switch (optimization_level) {
                0 => "-O0",
                1 => "-O1", 
                2 => "-O2",
                3 => "-O3",
                else => "-O2",
            };
            try command_parts.append(allocator, opt_arg);
            
            // Add linking mode for static linking
            if (std.mem.eql(u8, linking_mode, "static")) {
                try command_parts.append(allocator, "-static");
            }
            
            // Add output file specification
            if (output_file) |output| {
                try command_parts.append(self.allocator, "-o");
                try command_parts.append(self.allocator, output);
            }
        }
        
        if (verbose) {
            const command_str = try std.mem.join(self.allocator, " ", command_parts.items);
            defer self.allocator.free(command_str);
            print("🔧 LLVM IR compilation command: {s}\n", .{command_str});
        }
        
        return command_parts.toOwnedSlice(self.allocator);
    }
    
    // Free a command array returned by generateCompileCommand
    pub fn freeCompileCommand(self: *CrossCompiler, command: [][]const u8) void {
        for (command) |arg| {
            // Only free dynamically allocated strings (those created with allocPrint)
            // String literals like "zig", "build-exe", "-O0" etc. should not be freed
            if (self.isAllocatedString(arg)) {
                self.allocator.free(arg);
            }
        }
        self.allocator.free(command);
    }
    
    // Check if a string was dynamically allocated by this CrossCompiler
    fn isAllocatedString(self: *CrossCompiler, str: []const u8) bool {
        _ = self;
        // Heuristic: dynamically allocated strings in our case contain '=' 
        // This covers "-target=...", "--name=..." patterns which use allocPrint
        return std.mem.indexOf(u8, str, "=") != null;
    }
    
    // Execute cross-compilation
    pub fn executeCompilation(
        self: *CrossCompiler,
        command: [][]const u8,
        verbose: bool
    ) !void {
        if (verbose) {
            print("🚀 Executing cross-compilation...\n", .{});
        }
        
        var child = std.process.Child.init(command, self.allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        try child.spawn();
        
        const stdout = try child.stdout.?.readToEndAlloc(self.allocator, 1024 * 1024);
        defer self.allocator.free(stdout);
        
        const stderr = try child.stderr.?.readToEndAlloc(self.allocator, 1024 * 1024);
        defer self.allocator.free(stderr);
        
        const exit_status = try child.wait();
        
        if (stdout.len > 0) {
            print("stdout: {s}\n", .{stdout});
        }
        
        if (stderr.len > 0) {
            print("stderr: {s}\n", .{stderr});
        }
        
        switch (exit_status) {
            .Exited => |code| {
                if (code == 0) {
                    if (verbose) print("✅ Cross-compilation completed successfully\n", .{});
                } else {
                    print("❌ Cross-compilation failed with exit code: {}\n", .{code});
                    return error.CompilationFailed;
                }
            },
            else => {
                print("❌ Cross-compilation process terminated abnormally\n", .{});
                return error.CompilationFailed;
            },
        }
    }
    
    // Validate target platform compatibility
    pub fn validateTargetPlatform(target_platform: []const u8, backend: []const u8) !void {
        const valid_targets = [_][]const u8{
            "native",
            "x86_64-linux",
            "aarch64-linux", 
            "x86_64-macos",
            "aarch64-macos",
            "x86_64-windows",
            "wasm32-freestanding",
        };
        
        var is_valid = false;
        for (valid_targets) |valid_target| {
            if (std.mem.eql(u8, target_platform, valid_target)) {
                is_valid = true;
                break;
            }
        }
        
        if (!is_valid) {
            print("❌ Invalid target platform: {s}\n", .{target_platform});
            print("Valid targets: {s}\n", .{try std.mem.join(std.heap.page_allocator, ", ", &valid_targets)});
            return error.InvalidTarget;
        }
        
        // Check backend compatibility with target
        if (std.mem.startsWith(u8, target_platform, "wasm") and !std.mem.eql(u8, backend, "wasm")) {
            print("⚠️  Warning: WebAssembly target should use wasm backend\n", .{});
        }
        
        if (std.mem.eql(u8, backend, "llvm") and std.mem.startsWith(u8, target_platform, "wasm")) {
            print("⚠️  Warning: LLVM backend may not fully support WebAssembly target\n", .{});
        }
    }
    
    // Generate platform-specific output filename
    pub fn generateOutputFilename(
        self: *CrossCompiler,
        base_name: []const u8,
        target_platform: []const u8
    ) ![]const u8 {
        const extension = if (std.mem.startsWith(u8, target_platform, "wasm"))
            ".wasm"
        else if (std.mem.indexOf(u8, target_platform, "windows") != null)
            ".exe"
        else
            "";
            
        return try std.fmt.allocPrint(self.allocator, "{s}-{s}{s}", .{
            base_name,
            target_platform,
            extension
        });
    }
    
    // Test cross-compilation setup
    pub fn testCrossCompilationSetup(self: *CrossCompiler, verbose: bool) !void {
        const test_platforms = [_][]const u8{
            "x86_64-linux",
            "aarch64-linux",
            "x86_64-macos", 
            "aarch64-macos",
            "x86_64-windows",
            "wasm32-freestanding",
        };
        
        print("🧪 Testing cross-compilation setup...\n", .{});
        
        for (test_platforms) |platform| {
            if (verbose) print("  Testing target: {s}\n", .{platform});
            
            var child = std.process.Child.init(&[_][]const u8{
                "zig", "targets"
            }, self.allocator);
            child.stdout_behavior = .Pipe;
            child.stderr_behavior = .Ignore;
            
            child.spawn() catch |err| {
                print("❌ Failed to test zig targets: {}\n", .{err});
                continue;
            };
            
            const stdout = try child.stdout.?.readToEndAlloc(self.allocator, 1024 * 1024);
            defer self.allocator.free(stdout);
            
            _ = try child.wait();
            
            if (std.mem.indexOf(u8, stdout, platform) != null) {
                if (verbose) print("  ✅ {s} supported\n", .{platform});
            } else {
                if (verbose) print("  ⚠️  {s} may not be fully supported\n", .{platform});
            }
        }
        
        print("✅ Cross-compilation setup test completed\n", .{});
    }
};

// Cross-compilation testing utilities
pub const CrossCompilationTester = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) CrossCompilationTester {
        return CrossCompilationTester{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *CrossCompilationTester) void {
        _ = self;
    }
    
    // Create test program for cross-compilation validation
    pub fn createTestProgram(self: *CrossCompilationTester) ![]const u8 {
        _ = self;
        const test_program =
            \\const std = @import("std");
            \\
            \\pub fn main() !void {
            var stdout_buffer: [4096]u8 = undefined;
            \\    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
            \\    try stdout.print("Cross-compilation test successful!\n", .{});
            \\}
        ;
        
        const test_file = "cross_test.zig";
        try std.fs.cwd().writeFile(test_file, test_program);
        return test_file;
    }
    
    // Run comprehensive cross-compilation tests
    pub fn runComprehensiveTests(self: *CrossCompilationTester, verbose: bool) !void {
        print("🔬 Running comprehensive cross-compilation tests...\n", .{});
        
        const test_file = try self.createTestProgram();
        defer std.fs.cwd().deleteFile(test_file) catch {};
        
        var cross_compiler = CrossCompiler.init(self.allocator);
        defer cross_compiler.deinit(allocator);
        
        const test_targets = [_]struct {
            name: []const u8,
            platform: []const u8,
        }{
            .{ .name = "Linux x64", .platform = "x86_64-linux" },
            .{ .name = "Linux ARM64", .platform = "aarch64-linux" },
            .{ .name = "macOS x64", .platform = "x86_64-macos" },
            .{ .name = "macOS ARM64", .platform = "aarch64-macos" },
            .{ .name = "Windows x64", .platform = "x86_64-windows" },
            .{ .name = "WebAssembly", .platform = "wasm32-freestanding" },
        };
        
        var successful_targets: u32 = 0;
        const total_targets = test_targets.len;
        
        for (test_targets) |target| {
            print("  Testing {s} ({s})...\n", .{ target.name, target.platform });
            
            const output_name = try cross_compiler.generateOutputFilename("cross_test", target.platform);
            defer self.allocator.free(output_name);
            
            const command = cross_compiler.generateCompileCommand(
                test_file, 
                target.platform,
                output_name,
                2, // optimization level
                "dynamic", // linking mode
                verbose
            ) catch |err| {
                print("    ❌ Failed to generate command: {}\n", .{err});
                continue;
            };
            defer {
                for (command) |arg| {
                    self.allocator.free(arg);
                }
                self.allocator.free(command);
            }
            
            cross_compiler.executeCompilation(command, false) catch |err| {
                print("    ❌ Compilation failed: {}\n", .{err});
                continue;
            };
            
            // Check if output file was created
            const stat = std.fs.cwd().statFile(output_name) catch |err| {
                print("    ❌ Output file not found: {}\n", .{err});
                continue;
            };
            
            if (stat.size > 0) {
                print("    ✅ Successfully compiled ({} bytes)\n", .{stat.size});
                successful_targets += 1;
                
                // Clean up test binary
                std.fs.cwd().deleteFile(output_name) catch {};
            } else {
                print("    ❌ Output file is empty\n", .{});
            }
        }
        
        const success_rate = (successful_targets * 100) / total_targets;
        print("📊 Cross-compilation test results: {}/{} targets successful ({}%)\n", .{
            successful_targets, total_targets, success_rate
        });
        
        if (success_rate >= 80) {
            print("✅ Cross-compilation system is working well!\n", .{});
        } else if (success_rate >= 50) {
            print("⚠️  Cross-compilation system has some issues\n", .{});
        } else {
            print("❌ Cross-compilation system needs attention\n", .{});
        }
    }
};
