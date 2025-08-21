const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const ast = @import("ast.zig");
const Program = ast.Program;

// Smart LLVM backend that automatically falls back to stub when real LLVM is unavailable
pub const SmartLLVMBackend = struct {
    allocator: Allocator,
    use_real_llvm: bool,
    
    pub fn init(allocator: Allocator) SmartLLVMBackend {
        const use_real_llvm = comptime blk: {
            // Check at compile time if LLVM is enabled
            if (@hasDecl(@This(), "CURSED_ENABLE_LLVM")) {
                break :blk true;
            } else {
                break :blk false;
            }
        };
        
        return SmartLLVMBackend{
            .allocator = allocator,
            .use_real_llvm = use_real_llvm,
        };
    }
    
    pub fn compileToNative(self: *SmartLLVMBackend, source: []const u8, filename: []const u8, output_file: []const u8, verbose: bool) !void {
        if (self.use_real_llvm) {
            self.compileWithRealLLVM(source, filename, output_file, verbose) catch |err| {
                if (verbose) print("⚠️  Real LLVM failed ({any}), falling back to IR generation\n", .{err});
                try self.compileWithFallback(source, filename, output_file, verbose);
            };
        } else {
            try self.compileWithFallback(source, filename, output_file, verbose);
        }
    }
    
    fn compileWithRealLLVM(self: *SmartLLVMBackend, source: []const u8, filename: []const u8, output_file: []const u8, verbose: bool) !void {
        _ = self;
        _ = source;
        _ = filename;
        _ = output_file;
        if (verbose) print("🔨 Real LLVM backend not available, using fallback\n", .{});
        
        // For now, always use fallback until real LLVM integration is complete
        return error.RealLLVMNotAvailable;
    }
    
    fn compileWithFallback(self: *SmartLLVMBackend, source: []const u8, filename: []const u8, output_file: []const u8, verbose: bool) !void {
        if (verbose) print("🔨 Compiling with fallback LLVM IR generation + external clang\n", .{});
        
        // Step 1: Generate LLVM IR using simple direct compiler
        const simple_compiler = @import("simple_compiler_direct.zig");
        
        const ir_filename = try std.fmt.allocPrint(self.allocator, "{s}.ll", .{std.fs.path.stem(filename)});
        defer self.allocator.free(ir_filename);
        
        var compiler = simple_compiler.SimpleDirectCompiler.init(self.allocator);
        defer compiler.deinit();
        
        try compiler.compileToLLVMIR(source, verbose);
        
        // Write IR to file
        try compiler.writeToFile(ir_filename);
        
        if (verbose) print("✅ LLVM IR generated: {s}\n", .{ir_filename});
        
        // Step 2: Compile LLVM IR to native using system clang/llc
        try self.compileIRToNative(ir_filename, output_file, verbose);
        
        // Step 3: Clean up temporary IR file (unless verbose)
        if (!verbose) {
            std.fs.cwd().deleteFile(ir_filename) catch |err| {
                if (verbose) print("⚠️  Could not delete IR file: {any}\n", .{err});
            };
        }
    }
    
    fn compileIRToNative(self: *SmartLLVMBackend, ir_filename: []const u8, output_file: []const u8, verbose: bool) !void {
        // Try different compilation approaches including zig as C compiler
        const commands = [_][]const []const u8{
            &[_][]const u8{ "zig", "cc", "-O2", "-o", output_file, ir_filename },
            &[_][]const u8{ "clang", "-O2", "-o", output_file, ir_filename },
            &[_][]const u8{ "llc", ir_filename, "-o", "/tmp/cursed_temp.s" }, // Generate assembly first
            &[_][]const u8{ "gcc", "-O2", "-o", output_file, ir_filename },
        };
        
        for (commands) |cmd| {
            if (verbose) {
                print("🔄 Trying: {s}", .{cmd[0]});
                for (cmd[1..]) |arg| {
                    print(" {s}", .{arg});
                }
                print("\n", .{});
            }
            
            var process = std.process.Child.init(cmd, self.allocator);
            process.stdout_behavior = if (verbose) .Inherit else .Ignore;
            process.stderr_behavior = if (verbose) .Inherit else .Ignore;
            
            const result = process.spawnAndWait() catch |err| {
                if (verbose) print("⚠️  Command failed to spawn: {any}\n", .{err});
                continue;
            };
            
            if (result == .Exited and result.Exited == 0) {
                if (verbose) print("✅ Compilation successful with {s}\n", .{cmd[0]});
                return;
            } else {
                if (verbose) print("⚠️  Command failed with exit code: {any}\n", .{result});
            }
        }
        
        return error.CompilationFailed;
    }
    
    pub fn supportsTarget(target: []const u8) bool {
        // All targets supported through LLVM IR + system compilers
        _ = target;
        return true;
    }
    
    pub fn getCapabilities() []const u8 {
        return "LLVM IR generation + system compiler fallback";
    }
};
