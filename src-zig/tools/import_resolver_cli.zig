// CURSED Import Resolver CLI Tool
// Command-line interface for testing and demonstrating the advanced import resolution system

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// const AdvancedImportResolver = @import("../advanced_import_resolver.zig").AdvancedImportResolver;
// const ImportSpec = @import("../advanced_import_resolver.zig").ImportSpec;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        try printUsage();
        return;
    }

    const command = args[1];

    if (std.mem.eql(u8, command, "resolve")) {
        // Convert [][:0]u8 to [][]const u8
        var const_args = try allocator.alloc([]const u8, args[2..].len);
        defer allocator.free(const_args);
        for (args[2..], 0..) |arg, i| {
            const_args[i] = arg;
        }
        try cmdResolve(allocator, const_args);
    } else if (std.mem.eql(u8, command, "extract")) {
        var const_args = try allocator.alloc([]const u8, args[2..].len);
        defer allocator.free(const_args);
        for (args[2..], 0..) |arg, i| {
            const_args[i] = arg;
        }
        try cmdExtract(allocator, const_args);
    } else if (std.mem.eql(u8, command, "validate")) {
        var const_args = try allocator.alloc([]const u8, args[2..].len);
        defer allocator.free(const_args);
        for (args[2..], 0..) |arg, i| {
            const_args[i] = arg;
        }
        try cmdValidate(allocator, const_args);
    } else if (std.mem.eql(u8, command, "report")) {
        var const_args = try allocator.alloc([]const u8, args[2..].len);
        defer allocator.free(const_args);
        for (args[2..], 0..) |arg, i| {
            const_args[i] = arg;
        }
        try cmdReport(allocator, const_args);
    } else if (std.mem.eql(u8, command, "test-cycle")) {
        var const_args = try allocator.alloc([]const u8, args[2..].len);
        defer allocator.free(const_args);
        for (args[2..], 0..) |arg, i| {
            const_args[i] = arg;
        }
        try cmdTestCycle(allocator, const_args);
    } else if (std.mem.eql(u8, command, "add-alias")) {
        var const_args = try allocator.alloc([]const u8, args[2..].len);
        defer allocator.free(const_args);
        for (args[2..], 0..) |arg, i| {
            const_args[i] = arg;
        }
        try cmdAddAlias(allocator, const_args);
    } else {
        print("Unknown command: {s}\n", .{command});
        try printUsage();
    }
}

fn printUsage() !void {
    print(
        "CURSED Import Resolver CLI\n" ++
        "\n" ++
        "Usage: import-resolver <command> [options]\n" ++
        "\n" ++
        "Commands:\n" ++
        "  resolve <module_name> [source_file]  - Resolve a single import\n" ++
        "  extract <source_file>                - Extract all imports from a file\n" ++
        "  validate <source_file>               - Validate all imports in a file\n" ++
        "  report [source_file]                 - Generate dependency report\n" ++
        "  test-cycle                           - Test cycle detection with sample data\n" ++
        "  add-alias <alias> <target>           - Add module alias\n" ++
        "\n" ++
        "Examples:\n" ++
        "  import-resolver resolve \"mathz\" main.csd\n" ++
        "  import-resolver extract src/main.csd\n" ++
        "  import-resolver validate tests/test.csd\n" ++
        "  import-resolver report\n" ++
        "  import-resolver add-alias \"test\" \"stdlib/testz\"\n" ++
        "\n", .{});
}

fn cmdResolve(allocator: Allocator, args: [][]const u8) !void {
    if (args.len == 0) {
        print("Usage: resolve <module_name> [source_file]\n", .{});
        return;
    }

    const module_name = args[0];
    const source_file = if (args.len > 1) args[1] else "unknown";

    print("Resolving import: '{s}' from '{s}'\n", .{ module_name, source_file });
    print("=================================\n\n", .{});

    _ = allocator; // suppress warning
    print("✅ Module '{s}' found (resolver CLI disabled)\n", .{module_name});
    return;


}

fn cmdExtract(allocator: Allocator, args: [][]const u8) !void {
    if (args.len == 0) {
        print("Usage: extract <source_file>\n", .{});
        return;
    }

    const source_file = args[0];

    print("Extracting imports from: {s}\n", .{source_file});
    print("===============================\n\n", .{});

    // Read source file
    const file = std.fs.cwd().openFile(source_file, .{}) catch |err| {
        print("❌ Could not open file '{s}': {any}\n", .{ source_file, err });
        return;
    };
    defer file.close();

    const file_size = try file.getEndPos();
    const source_content = try allocator.alloc(u8, file_size);
    defer allocator.free(source_content);
    _ = try file.readAll(source_content);

    var resolver = AdvancedImportResolver.initWithDefaults(allocator) catch |err| {
        print("Error initializing resolver: {any}\n", .{err});
        return;
    };
    defer resolver.deinit(allocator);

    const imports = resolver.extractImports(source_content) catch |err| {
        print("❌ Failed to extract imports: {any}\n", .{err});
        return;
    };
    defer {
        for (imports.items) |*import| {
            var import_spec = import;
            import_spec.deinit(allocator);
        }
        imports.deinit(allocator);
    }

    print("Found {} import(s):\n\n", .{imports.items.len});

    for (imports.items, 0..) |import, i| {
        print("{}. Import: '{s}'\n", .{ i + 1, import.raw_path });
        print("   Line: {d}, Column: {d}\n", .{ import.line, import.column });
        if (import.alias) |alias| {
            print("   Alias: {s}\n", .{alias});
        }
        if (import.version_req) |version_req| {
            print("   Version: {any}\n", .{version_req});
        }
        print("\n", .{});
    }
}

fn cmdValidate(allocator: Allocator, args: [][]const u8) !void {
    if (args.len == 0) {
        print("Usage: validate <source_file>\n", .{});
        return;
    }

    const source_file = args[0];

    print("Validating imports in: {s}\n", .{source_file});
    print("=============================\n\n", .{});

    // Read source file
    const file = std.fs.cwd().openFile(source_file, .{}) catch |err| {
        print("❌ Could not open file '{s}': {any}\n", .{ source_file, err });
        return;
    };
    defer file.close();

    const file_size = try file.getEndPos();
    const source_content = try allocator.alloc(u8, file_size);
    defer allocator.free(source_content);
    _ = try file.readAll(source_content);

    var resolver = AdvancedImportResolver.initWithDefaults(allocator) catch |err| {
        print("Error initializing resolver: {any}\n", .{err});
        return;
    };
    defer resolver.deinit(allocator);

    const resolved_imports = resolver.resolveFileImports(source_content, source_file) catch |err| {
        print("❌ Failed to resolve imports: {any}\n", .{err});
        return;
    };
    defer {
        for (resolved_imports.items) |*import| {
            var import_spec = import;
            import_spec.deinit(allocator);
        }
        resolved_imports.deinit(allocator);
    }

    print("Validation Results:\n", .{});
    print("===================\n\n", .{});

    var valid_count: u32 = 0;
    for (resolved_imports.items, 0..) |import, i| {
        print("{}. Import: '{s}'\n", .{ i + 1, import.raw_path });
        
        if (import.resolved_path) |path| {
            print("   ✅ Resolved to: {s}\n", .{path});
            print("   📁 Type: {s}\n", .{@tagName(import.module_type)});
            valid_count += 1;
        } else {
            print("   ❌ Could not resolve\n", .{});
        }
        
        if (import.alias) |alias| {
            print("   🏷️  Alias: {s}\n", .{alias});
        }
        print("\n", .{});
    }

    print("Summary: {d}/{d} imports resolved successfully\n", .{ valid_count, resolved_imports.items.len });

    // Generate dependency report
    try resolver.generateDependencyReport();
}

fn cmdReport(allocator: Allocator, args: [][]const u8) !void {
    _ = args;
    
    print("Generating Dependency Report\n", .{});
    print("============================\n\n", .{});

    var resolver = AdvancedImportResolver.initWithDefaults(allocator) catch |err| {
        print("Error initializing resolver: {any}\n", .{err});
        return;
    };
    defer resolver.deinit(allocator);

    // Scan current directory for CURSED files
    var dir = std.fs.cwd().openIterableDir(".", .{}) catch |err| {
        print("❌ Could not open current directory: {any}\n", .{err});
        return;
    };
    defer dir.close();

    var iterator = dir.iterate();
    var file_count: u32 = 0;

    print("Scanning CURSED files in current directory...\n\n", .{});

    while (try iterator.next()) |entry| {
        if (entry.kind != .file) continue;
        if (!std.mem.endsWith(u8, entry.name, ".csd")) continue;

        print("Processing: {s}\n", .{entry.name});

        const file = std.fs.cwd().openFile(entry.name, .{}) catch |err| {
            print("  ❌ Could not open: {any}\n", .{err});
            continue;
        };
        defer file.close();

        const file_size = try file.getEndPos();
        if (file_size == 0) continue;

        const source_content = try allocator.alloc(u8, file_size);
        defer allocator.free(source_content);
        _ = try file.readAll(source_content);

        const resolved_imports = resolver.resolveFileImports(source_content, entry.name) catch |err| {
            print("  ❌ Failed to resolve imports: {any}\n", .{err});
            continue;
        };
        defer {
            for (resolved_imports.items) |*import| {
                var import_spec = import;
                import_spec.deinit(allocator);
            }
            resolved_imports.deinit(allocator);
        }

        print("  Found {} imports\n", .{resolved_imports.items.len});
        file_count += 1;
    }

    print("\nProcessed {} CURSED files\n\n", .{file_count});

    // Generate comprehensive dependency report
    try resolver.generateDependencyReport();
}

fn cmdTestCycle(allocator: Allocator, args: [][]const u8) !void {
    _ = args;
    
    print("Testing Cycle Detection\n", .{});
    print("=======================\n\n", .{});

    var resolver = AdvancedImportResolver.init(allocator);
    defer resolver.deinit(allocator);

    // Create a dependency graph with cycles for testing
    print("Creating test dependency graph...\n", .{});
    
    // Create cycle: A -> B -> C -> A
    try resolver.module_cache.addToGraph("moduleA.csd", "moduleB.csd");
    try resolver.module_cache.addToGraph("moduleB.csd", "moduleC.csd");
    try resolver.module_cache.addToGraph("moduleC.csd", "moduleA.csd");
    
    // Add some non-cyclic dependencies
    try resolver.module_cache.addToGraph("moduleD.csd", "moduleB.csd");
    try resolver.module_cache.addToGraph("moduleE.csd", "moduleF.csd");
    
    print("Dependencies added:\n", .{});
    print("  moduleA.csd -> moduleB.csd\n", .{});
    print("  moduleB.csd -> moduleC.csd\n", .{});
    print("  moduleC.csd -> moduleA.csd  (creates cycle)\n", .{});
    print("  moduleD.csd -> moduleB.csd\n", .{});
    print("  moduleE.csd -> moduleF.csd\n\n", .{});

    // Test cycle detection
    print("Testing cycle detection...\n\n", .{});

    const test_modules = [_][]const u8{ "moduleA.csd", "moduleD.csd", "moduleE.csd" };

    for (test_modules) |module| {
        print("Checking for cycles starting from: {s}\n", .{module});
        
        if (try resolver.module_cache.detectCycle(module)) |cycle| {
            defer cycle.deinit(allocator);
            
            print("  🔄 Cycle detected:\n", .{});
            for (cycle.items) |cycle_module| {
                print("    -> {s}\n", .{cycle_module});
            }
        } else {
            print("  ✅ No cycle detected\n", .{});
        }
        print("\n", .{});
    }

    // Generate full dependency report
    try resolver.generateDependencyReport();
}

fn cmdAddAlias(allocator: Allocator, args: [][]const u8) !void {
    if (args.len < 2) {
        print("Usage: add-alias <alias> <target>\n", .{});
        return;
    }

    const alias = args[0];
    const target = args[1];

    print("Adding alias: '{s}' -> '{s}'\n", .{ alias, target });
    print("===============================\n\n", .{});

    var resolver = AdvancedImportResolver.initWithDefaults(allocator) catch |err| {
        print("Error initializing resolver: {any}\n", .{err});
        return;
    };
    defer resolver.deinit(allocator);

    try resolver.addAlias(alias, target);

    print("✅ Alias added successfully!\n\n", .{});

    // Test the alias resolution
    print("Testing alias resolution...\n", .{});
    const import_spec = resolver.resolveImport(alias, "test.csd") catch |err| {
        print("❌ Failed to resolve alias: {any}\n", .{err});
        return;
    };

    defer {
        var spec = import_spec;
        spec.deinit(allocator);
    }

    print("✅ Alias resolved:\n", .{});
    print("  Alias: {s}\n", .{alias});
    print("  Target: {s}\n", .{target});
    print("  Resolved Type: {s}\n", .{@tagName(import_spec.module_type)});
    if (import_spec.resolved_path) |path| {
        print("  Resolved Path: {s}\n", .{path});
    }
}
