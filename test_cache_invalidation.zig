const std = @import("std");
const CompilationCache = @import("src-zig/compilation_cache.zig").CompilationCache;
const CacheConfig = @import("src-zig/compilation_cache.zig").CacheConfig;
const CacheInvalidationPattern = @import("src-zig/compilation_cache.zig").CacheInvalidationPattern;

test "cache invalidation functionality" {
    const allocator = std.testing.allocator;
    const config = CacheConfig.development();
    
    var cache = try CompilationCache.init(allocator, "/tmp/cursed_test_cache", config);
    defer cache.deinit();
    
    // Test 1: Fresh file should need recompilation
    const test_file = "/tmp/test_source.csd";
    try std.fs.cwd().writeFile(.{ .sub_path = test_file, .data = "sus x drip = 42;" });
    defer std.fs.cwd().deleteFile(test_file) catch {};
    
    // First check - should need recompilation (no cache)
    try std.testing.expect(try cache.needsRecompilation(test_file));
    try std.testing.expect(!(try cache.hasValidCache(test_file)));
    
    // Cache some fake AST data
    var fake_ast: @import("src-zig/compilation_cache.zig").AST = .{};
    const deps = [_][]const u8{"dependency1.csd"};
    try cache.cacheAST(test_file, &fake_ast, &deps);
    
    // Second check - should NOT need recompilation (cache hit)
    try std.testing.expect(!(try cache.needsRecompilation(test_file)));
    try std.testing.expect(try cache.hasValidCache(test_file));
    
    // Test 2: Modify file and check invalidation
    try std.fs.cwd().writeFile(.{ .sub_path = test_file, .data = "sus y drip = 84;" });
    
    // Sleep to ensure different timestamp
    std.time.sleep(1_000_000_000); // 1 second
    
    // Third check - should need recompilation (file changed)
    try std.testing.expect(try cache.needsRecompilation(test_file));
    try std.testing.expect(!(try cache.hasValidCache(test_file)));
    
    // Test 3: Pattern-based invalidation
    try cache.invalidateByPattern(.build_config_changed);
    
    // Test 4: Dependency invalidation
    const dep_file = "/tmp/dependency1.csd";
    try std.fs.cwd().writeFile(.{ .sub_path = dep_file, .data = "sus dep drip = 1;" });
    defer std.fs.cwd().deleteFile(dep_file) catch {};
    
    try cache.cacheAST(test_file, &fake_ast, &deps);
    try cache.invalidateByPattern(.{ .dependency_changed = dep_file });
    
    // Should need recompilation due to dependency change
    try std.testing.expect(try cache.needsRecompilation(test_file));
    
    std.debug.print("✅ All cache invalidation tests passed!\n", .{});
}

test "cache performance and statistics" {
    const allocator = std.testing.allocator;
    const config = CacheConfig.development();
    
    var cache = try CompilationCache.init(allocator, "/tmp/cursed_perf_cache", config);
    defer cache.deinit();
    
    // Create multiple test files
    const test_files = [_][]const u8{
        "/tmp/file1.csd",
        "/tmp/file2.csd", 
        "/tmp/file3.csd",
    };
    
    for (test_files) |file| {
        try std.fs.cwd().writeFile(.{ .sub_path = file, .data = "sus x drip = 42;" });
        defer std.fs.cwd().deleteFile(file) catch {};
        
        var fake_ast: @import("src-zig/compilation_cache.zig").AST = .{};
        try cache.cacheAST(file, &fake_ast, &[_][]const u8{});
    }
    
    // Check cache statistics
    const stats = cache.getStatistics();
    try std.testing.expect(stats.source_cache_size > 0);
    try std.testing.expect(stats.ast_cache_size > 0);
    
    // Test cache hit rate
    for (test_files) |file| {
        _ = try cache.hasValidCache(file);
    }
    
    const updated_stats = cache.getStatistics();
    try std.testing.expect(updated_stats.hit_rate > 0.0);
    
    std.debug.print("✅ Cache performance tests passed!\n", .{});
    stats.print();
}
