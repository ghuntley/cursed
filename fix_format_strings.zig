const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const patterns = [_][]const u8{
        "items.len",
        ".len",
        "count()",
        "size()",
    };
    
    // Find all zig files in src-zig
    var dir = std.fs.cwd().openIterableDir("src-zig", .{}) catch {
        print("Could not open src-zig directory\n");
        return;
    };
    defer dir.close();
    
    var iterator = dir.iterate();
    while (try iterator.next()) |entry| {
        if (entry.kind != .file) continue;
        if (!std.mem.endsWith(u8, entry.name, ".zig")) continue;
        
        const file_path = try std.fmt.allocPrint(allocator, "src-zig/{s}", .{entry.name});
        defer allocator.free(file_path);
        
        try fixFileFormatStrings(allocator, file_path);
    }
    
    print("✅ Format string fixes completed\n");
}

fn fixFileFormatStrings(allocator: std.mem.Allocator, file_path: []const u8) !void {
    const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
        print("Could not open file {s}: {}\n", .{file_path, err});
        return;
    };
    defer file.close();
    
    const content = file.readToEndAlloc(allocator, std.math.maxInt(usize)) catch |err| {
        print("Could not read file {s}: {}\n", .{file_path, err});
        return;
    };
    defer allocator.free(content);
    
    var modified = false;
    var lines = std.mem.split(u8, content, "\n");
    var new_content = std.ArrayList(u8).init(allocator);
    defer new_content.deinit();
    
    while (lines.next()) |line| {
        var new_line = std.ArrayList(u8).init(allocator);
        defer new_line.deinit();
        
        if (needsFormatStringFix(line)) {
            try new_line.appendSlice(fixFormatString(allocator, line) catch line);
            modified = true;
        } else {
            try new_line.appendSlice(line);
        }
        
        try new_content.appendSlice(new_line.items);
        try new_content.append('\n');
    }
    
    if (modified) {
        const out_file = std.fs.cwd().createFile(file_path, .{}) catch |err| {
            print("Could not write to file {s}: {}\n", .{file_path, err});
            return;
        };
        defer out_file.close();
        
        try out_file.writeAll(new_content.items[0..new_content.items.len-1]); // Remove last newline
        print("Fixed format strings in {s}\n", .{file_path});
    }
}

fn needsFormatStringFix(line: []const u8) bool {
    if (std.mem.indexOf(u8, line, "print(") == null and 
        std.mem.indexOf(u8, line, ".print(") == null) return false;
    if (std.mem.indexOf(u8, line, "{s}") == null) return false;
    
    // Check if this line has numeric format issues
    return std.mem.indexOf(u8, line, ".len") != null or
           std.mem.indexOf(u8, line, "items.len") != null or
           std.mem.indexOf(u8, line, "count()") != null;
}

fn fixFormatString(allocator: std.mem.Allocator, line: []const u8) ![]const u8 {
    var result = std.ArrayList(u8).init(allocator);
    defer result.deinit();
    
    var i: usize = 0;
    while (i < line.len) {
        if (i + 2 < line.len and std.mem.eql(u8, line[i..i+3], "{s}")) {
            // Check context to see if this should be changed
            if (shouldConvertToNumericFormat(line, i)) {
                try result.appendSlice("{}");
                i += 3;
            } else {
                try result.append(line[i]);
                i += 1;
            }
        } else {
            try result.append(line[i]);
            i += 1;
        }
    }
    
    return result.toOwnedSlice();
}

fn shouldConvertToNumericFormat(line: []const u8, pos: usize) bool {
    // Look for numeric patterns in the argument list after this format specifier
    var paren_count: i32 = 0;
    var in_args = false;
    var i = pos + 3; // Start after {s}
    
    while (i < line.len) {
        switch (line[i]) {
            '(' => {
                paren_count += 1;
                if (paren_count == 1 and !in_args) {
                    in_args = true;
                }
            },
            ')' => {
                paren_count -= 1;
                if (paren_count == 0) break;
            },
            else => {},
        }
        i += 1;
    }
    
    if (!in_args) return false;
    
    // Check for numeric patterns in the args section
    const args_section = line[pos..i];
    return std.mem.indexOf(u8, args_section, ".len") != null or
           std.mem.indexOf(u8, args_section, "items.len") != null or
           std.mem.indexOf(u8, args_section, "count()") != null;
}
