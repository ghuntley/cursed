const std = @import("std");
const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");
const interpreter = @import("src-zig/interpreter.zig");
const arena_allocator = @import("src-zig/arena_allocator.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = 
        \\sus x drip = 42
        \\vibez.spill("Hello World! x =", x)
        \\slay add(a drip, b drip) drip {
        \\    damn a + b
        \\}
        \\sus result drip = add(10, 20)
        \\vibez.spill("Result:", result)
    ;
    
    std.debug.print("🚀 Testing proper interpreter pipeline directly\n", .{});
    std.debug.print("📝 Source code:\n{s}\n", .{source});
    
    // Tokenize
    var lexer_instance = lexer.Lexer.init(allocator, source);
    var tokens = try lexer_instance.tokenize();
    defer tokens.deinit(allocator);
    
    std.debug.print("✅ Tokenized {} tokens\n", .{tokens.items.len});
    
    // Initialize arena manager
    var arena_manager = try arena_allocator.CursedArenaManager.init(allocator);
    defer arena_manager.deinit();
    
    // Parse to AST
    var p = parser.Parser.init(arena_manager.getASTAllocator(), tokens.items);
    var program = p.parseProgram() catch |err| {
        std.debug.print("❌ Parse error: {any}\n", .{err});
        return;
    };
    defer program.deinit(arena_manager.getASTAllocator());
    
    std.debug.print("🌳 AST created with {} statements\n", .{program.statements.items.len});
    
    // Initialize interpreter
    var cursed_interpreter = interpreter.Interpreter.init(arena_manager.getRuntimeAllocator());
    defer cursed_interpreter.deinit();
    
    std.debug.print("🚀 Executing with proper interpreter\n", .{});
    
    // Execute
    cursed_interpreter.execute(program) catch |err| {
        std.debug.print("❌ Execution error: {any}\n", .{err});
        return;
    };
    
    std.debug.print("✅ Execution completed successfully!\n", .{});
}
