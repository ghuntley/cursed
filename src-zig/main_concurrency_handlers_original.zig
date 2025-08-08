const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// Use a simplified approach - accept any variable type as anyopaque and cast it

// Global concurrency state  
var global_channels: ?HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage) = null;
var global_goroutines: ?ArrayList(std.Thread) = null;
var global_allocator: ?Allocator = null;

pub fn initGlobalConcurrency(allocator: Allocator) void {
    if (global_channels == null) {
        global_channels = HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
        global_goroutines = ArrayList(std.Thread).init(allocator);
        global_allocator = allocator;
    }
}

pub fn deinitGlobalConcurrency() void {
    if (global_channels) |*channels| {
        channels.deinit();
        global_channels = null;
    }
    if (global_goroutines) |*goroutines| {
        for (goroutines.items) |*thread| {
            thread.join();
        }
        goroutines.deinit();
        global_goroutines = null;
    }
    global_allocator = null;
}

// Handle stan statement (goroutine spawning)
pub fn handleStanStatement(variables: *anyopaque, functions: *anyopaque, allocator: Allocator, source_lines: ArrayList([]const u8), line_index: usize, verbose: bool) !void {
    _ = variables;
    _ = functions;
    
    const line = source_lines.items[line_index];
    const trimmed = std.mem.trim(u8, line, " \t\r\n");
    
    initGlobalConcurrency(allocator);
    
    // Parse stan block: stan { ... }
    if (std.mem.indexOf(u8, trimmed, "{")) |_| {
        if (verbose) print("🚀 Spawning goroutine with block\n", .{});
        
        // Extract the body of the stan block (simplified for now)
        var body_lines = ArrayList([]const u8).init(allocator);
        defer body_lines.deinit();
        
        // Find the corresponding closing brace (simplified)
        var current_line = line_index + 1;
        while (current_line < source_lines.items.len) {
            const block_line = std.mem.trim(u8, source_lines.items[current_line], " \t\r\n");
            if (std.mem.eql(u8, block_line, "}")) {
                break;
            }
            try body_lines.append(block_line);
            current_line += 1;
        }
        
        // Create a simple goroutine that just prints and executes the lines
        const GoroutineContext = struct {
            lines: [][]const u8,
            verb: bool,
            alloc: Allocator,
        };
        
        const context = try allocator.create(GoroutineContext);
        const lines_copy = try allocator.alloc([]const u8, body_lines.items.len);
        for (body_lines.items, 0..) |line_item, i| {
            lines_copy[i] = try allocator.dupe(u8, line_item);
        }
        
        context.* = GoroutineContext{
            .lines = lines_copy,
            .verb = verbose,
            .alloc = allocator,
        };
        
        // Spawn the goroutine
        const goroutine_fn = struct {
            fn run(ctx: *GoroutineContext) void {
                if (ctx.verb) print("🏃 Goroutine executing {} lines\n", .{ctx.lines.len});
                
                // Execute each line in the goroutine
                for (ctx.lines) |block_line| {
                    if (ctx.verb) print("  📝 Goroutine line: {s}\n", .{block_line});
                    
                    // Simple execution - for now just handle basic vibez.spill
                    if (std.mem.indexOf(u8, block_line, "vibez.spill(")) |start| {
                        handleSimpleVibesSpill(block_line, start);
                    }
                    
                    // Handle channel send operations
                    if (std.mem.indexOf(u8, block_line, "<-")) |arrow_pos| {
                        const left_part = std.mem.trim(u8, block_line[0..arrow_pos], " \t");
                        const right_part = std.mem.trim(u8, block_line[arrow_pos + 2..], " \t");
                        print("📤 Goroutine sending '{s}' to channel '{s}'\n", .{ right_part, left_part });
                    }
                }
                
                if (ctx.verb) print("✅ Goroutine completed\n", .{});
                
                // Cleanup
                for (ctx.lines) |line_item| {
                    ctx.alloc.free(line_item);
                }
                ctx.alloc.free(ctx.lines);
                ctx.alloc.destroy(ctx);
            }
        }.run;
        
        const thread = try std.Thread.spawn(.{}, goroutine_fn, .{context});
        if (global_goroutines) |*goroutines| {
            try goroutines.append(thread);
        }
        
        if (verbose) print("✅ Goroutine spawned\n", .{});
    }
}

// Simple vibez.spill handler for goroutines
fn handleSimpleVibesSpill(line: []const u8, start: usize) void {
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            
            // Remove quotes if present
            if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                print("{s}\n", .{content[1..content.len - 1]});
            } else {
                print("{s}\n", .{content});
            }
        }
    }
}

// Handle channel operations
pub fn handleChannelOperation(variables: *anyopaque, functions: *anyopaque, allocator: Allocator, line: []const u8, arrow_pos: usize, verbose: bool) !void {
    _ = variables;
    _ = functions; // unused for now
    
    initGlobalConcurrency(allocator);
    
    const left_part = std.mem.trim(u8, line[0..arrow_pos], " \t");
    const right_part = std.mem.trim(u8, line[arrow_pos + 2..], " \t");
    
    if (verbose) print("📡 Channel operation: left='{s}', right='{s}'\n", .{ left_part, right_part });
    
    // For now, just simulate the channel operations
    if (std.mem.startsWith(u8, left_part, "sus ")) {
        // Receive operation (sus value = <-ch)
        const var_decl = left_part[4..]; // Remove "sus "
        if (std.mem.indexOf(u8, var_decl, " ")) |space_pos| {
            const var_name = std.mem.trim(u8, var_decl[0..space_pos], " \t");
            if (verbose) print("📥 Receiving from channel '{s}' into {s}\n", .{ right_part, var_name });
            if (verbose) print("✅ Value received from channel\n", .{});
        }
    } else {
        // Send operation (ch <- value)
        if (verbose) print("📤 Sending value '{s}' to channel '{s}'\n", .{ right_part, left_part });
        if (verbose) print("✅ Value sent to channel\n", .{});
    }
}

// Handle wait functions
pub fn handleWaitFunction(variables: *anyopaque, allocator: Allocator, line: []const u8, verbose: bool) !void {
    _ = variables; // unused for now
    _ = allocator; // unused for now
    
    if (std.mem.startsWith(u8, line, "wait_all()")) {
        if (verbose) print("⏳ Waiting for all goroutines to complete...\n", .{});
        
        // Wait for all spawned goroutines
        if (global_goroutines) |*goroutines| {
            for (goroutines.items) |*thread| {
                thread.join();
            }
            goroutines.clearRetainingCapacity();
            
            if (verbose) print("✅ All goroutines completed\n", .{});
        }
    } else if (std.mem.startsWith(u8, line, "wait(")) {
        // Extract wait time
        if (std.mem.indexOf(u8, line, "(")) |start| {
            if (std.mem.indexOf(u8, line, ")")) |end| {
                const time_str = line[start + 1..end];
                const wait_ms = std.fmt.parseInt(u64, time_str, 10) catch 100;
                
                if (verbose) print("⏳ Waiting for {}ms...\n", .{wait_ms});
                std.time.sleep(wait_ms * 1_000_000); // Convert ms to ns
                if (verbose) print("✅ Wait completed\n", .{});
            }
        }
    }
}

// Handle make_channel function
pub fn handleMakeChannel(variables: *anyopaque, allocator: Allocator, var_name: []const u8, verbose: bool) !void {
    _ = variables; // We'll simplify this for now
    initGlobalConcurrency(allocator);
    
    if (verbose) print("🔧 Creating channel for variable: {s}\n", .{var_name});
    
    // Generate a unique channel ID
    const channel_id = std.time.timestamp();
    
    // Store in global registry
    if (global_channels) |*channels| {
        try channels.put(var_name, @as(u64, @intCast(channel_id)));
    }
    
    if (verbose) print("✅ Channel created with ID: {}\n", .{channel_id});
}
