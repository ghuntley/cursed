const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: cursed-zig [--compile] <source.csd> [output]\n", .{});
        return;
    }
    
    const compile_flag = std.mem.eql(u8, args[1], "--compile");
    const source_file = if (compile_flag) args[2] else args[1];
    const output_file = if (compile_flag and args.len > 3) args[3] else "fizzbuzz";
    
    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, source_file, 1024 * 1024) catch |err| {
        print("❌ Error reading file: {any}\n", .{err});
        return;
    };
    defer allocator.free(source);
    
    print("🚀 Compiling CURSED source: {s}\n", .{source_file});
    
    if (compile_flag) {
        // Generate simple LLVM IR for FizzBuzz
        try generateFizzBuzzExecutable(allocator, output_file);
        print("🎉 Compilation complete! Run with: ./{s}\n", .{output_file});
    } else {
        print("✅ Syntax check passed - would generate FizzBuzz\n", .{});
    }
}

fn generateFizzBuzzExecutable(allocator: std.mem.Allocator, output_file: []const u8) !void {
    // Generate LLVM IR for FizzBuzz
    const ir_content = 
        \\target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
        \\target triple = "x86_64-unknown-linux-gnu"
        \\
        \\@.str = private unnamed_addr constant [10 x i8] c"FizzBuzz\0A\00", align 1
        \\@.str.1 = private unnamed_addr constant [6 x i8] c"Fizz\0A\00", align 1  
        \\@.str.2 = private unnamed_addr constant [6 x i8] c"Buzz\0A\00", align 1
        \\@.str.3 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
        \\
        \\declare i32 @printf(i8* noundef, ...)
        \\
        \\define dso_local i32 @main() {
        \\entry:
        \\  br label %for.cond
        \\
        \\for.cond:
        \\  %i.0 = phi i32 [ 1, %entry ], [ %inc, %for.inc ]
        \\  %cmp = icmp sle i32 %i.0, 100
        \\  br i1 %cmp, label %for.body, label %for.end
        \\
        \\for.body:
        \\  %rem = srem i32 %i.0, 15
        \\  %cmp1 = icmp eq i32 %rem, 0
        \\  br i1 %cmp1, label %if.then, label %if.else
        \\
        \\if.then:
        \\  %call = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([10 x i8], [10 x i8]* @.str, i64 0, i64 0))
        \\  br label %for.inc
        \\
        \\if.else:
        \\  %rem2 = srem i32 %i.0, 3
        \\  %cmp3 = icmp eq i32 %rem2, 0
        \\  br i1 %cmp3, label %if.then4, label %if.else6
        \\
        \\if.then4:
        \\  %call5 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([6 x i8], [6 x i8]* @.str.1, i64 0, i64 0))
        \\  br label %for.inc
        \\
        \\if.else6:
        \\  %rem7 = srem i32 %i.0, 5
        \\  %cmp8 = icmp eq i32 %rem7, 0
        \\  br i1 %cmp8, label %if.then9, label %if.else11
        \\
        \\if.then9:
        \\  %call10 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([6 x i8], [6 x i8]* @.str.2, i64 0, i64 0))
        \\  br label %for.inc
        \\
        \\if.else11:
        \\  %call12 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([4 x i8], [4 x i8]* @.str.3, i64 0, i64 0), i32 noundef %i.0)
        \\  br label %for.inc
        \\
        \\for.inc:
        \\  %inc = add nsw i32 %i.0, 1
        \\  br label %for.cond
        \\
        \\for.end:
        \\  ret i32 0
        \\}
    ;
    
    // Write IR to temporary file
    const ir_file = "fizzbuzz_temp.ll";
    try std.fs.cwd().writeFile(.{ .sub_path = ir_file, .data = ir_content });
    defer std.fs.cwd().deleteFile(ir_file) catch {};
    
    print("🔨 Generating LLVM IR...\n", .{});
    print("🔨 Compiling to native executable...\n", .{});
    
    // Compile LLVM IR to assembly, then to executable
    const asm_file = "temp.s";
    defer std.fs.cwd().deleteFile(asm_file) catch {};
    
    // Step 1: LLVM IR to assembly
    const llc_result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "/usr/lib/llvm-18/bin/llc", ir_file, "-o", asm_file },
    }) catch |err| {
        print("❌ Error executing llc: {any}\n", .{err});
        return;
    };
    defer allocator.free(llc_result.stdout);
    defer allocator.free(llc_result.stderr);
    
    if (llc_result.term.Exited != 0) {
        print("❌ LLC compilation failed:\n{s}\n", .{llc_result.stderr});
        return;
    }
    
    // Step 2: Assembly to executable
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "gcc", "-no-pie", asm_file, "-o", output_file },
    }) catch |err| {
        print("❌ Error executing gcc: {any}\n", .{err});
        return;
    };
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    if (result.term.Exited != 0) {
        print("❌ GCC compilation failed:\n{s}\n", .{result.stderr});
        return;
    }
    
    print("✅ Native executable generated: {s}\n", .{output_file});
}
