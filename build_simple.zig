const std = @import("std");
const builtin = @import("builtin");

// Simple LLVM linking function that avoids API issues
fn addLlvm(b: *std.Build, exe: *std.Build.Step.Compile, target: std.Build.ResolvedTarget) void {
    // Skip LLVM for WASM targets
    if (target.result.cpu.arch == .wasm32) return;
    
    // Configure LLVM with proper CPU target specification
    const cpu_name = switch (target.result.cpu.arch) {
        .x86_64 => "x86-64",
        .aarch64 => "generic",
        else => "generic",
    };
    
    // Set explicit CPU target to avoid athlon-xp conflicts
    exe.root_module.addCMacro("LLVM_DEFAULT_TARGET_TRIPLE", b.fmt("\"{s}\"", .{@tagName(target.result.cpu.arch)}));
    exe.root_module.addCMacro("LLVM_HOST_TARGET", b.fmt("\"{s}\"", .{cpu_name}));
    
    // Use hardcoded paths for Ubuntu/Debian systems
    exe.addSystemIncludePath(b.path("/usr/include/llvm-c-18"));
    exe.addSystemIncludePath(b.path("/usr/include/llvm-18"));
    exe.addLibraryPath(b.path("/usr/lib/llvm-18/lib"));
    exe.linkSystemLibrary("LLVM-18");
    
    // Link additional required libraries  
    exe.linkSystemLibrary("dl");
    exe.linkSystemLibrary("pthread");
    exe.linkSystemLibrary("z");
    exe.linkSystemLibrary("ncurses");
}

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    const resolved_target = target;
    const is_wasm = resolved_target.result.cpu.arch == .wasm32;
    const is_cross_compile = resolved_target.result.cpu.arch != @import("builtin").target.cpu.arch or
                            resolved_target.result.os.tag != @import("builtin").target.os.tag;
    
    // Create the main CURSED compiler executable
    const exe = b.addExecutable(.{
        .name = "cursed", 
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/minimal_main.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    if (!is_wasm) {
        exe.linkLibC();
    }
    
    b.installArtifact(exe);
    
    // Create syscall-enabled compiler with LLVM support
    if (!is_cross_compile and !is_wasm) {
        const syscall_exe = b.addExecutable(.{
            .name = "cursed-syscall",
            .root_source_file = b.path("src-zig/main_unified.zig"),
            .target = resolved_target,
            .optimize = optimize,
        });
        
        syscall_exe.linkLibC();
        addLlvm(b, syscall_exe, resolved_target);
        b.installArtifact(syscall_exe);
    }
    
    // Create run step
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);
}
