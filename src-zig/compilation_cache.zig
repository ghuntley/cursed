const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const Mutex = std.Thread.Mutex;

/// Advanced compilation cache system for CURSED compiler
/// Provides incremental compilation, dependency tracking, and intelligent invalidation
pub const CompilationCache = struct {
    allocator: Allocator,
    cache_dir: []const u8,
    
    // Cache storage
    source_cache: SourceCache,
    ast_cache: ASTCache,
    object_cache: ObjectCache,
    dependency_graph: DependencyGraph,
    
    // Cache configuration
    config: CacheConfig,
    
    // Thread safety
    mutex: Mutex,
    
    // Performance metrics
    metrics: CacheMetrics,
    
    pub fn init(allocator: Allocator, cache_dir: []const u8, config: CacheConfig) !CompilationCache {
        // Create cache directory structure
        try createCacheDirectories(cache_dir);
        
        return CompilationCache{
            .allocator = allocator,
            .cache_dir = try allocator.dupe(u8, cache_dir),
            .source_cache = SourceCache.init(allocator),
            .ast_cache = ASTCache.init(allocator),
            .object_cache = ObjectCache.init(allocator),
            .dependency_graph = try DependencyGraph.init(allocator),
            .config = config,
            .mutex = Mutex{},
            .metrics = CacheMetrics.init(),
        };
    }
    
    pub fn deinit(self: *CompilationCache) void {
        self.dependency_graph.deinit(self.allocator);
        self.object_cache.deinit(self.allocator);
        self.ast_cache.deinit(self.allocator);
        self.source_cache.deinit(self.allocator);
        self.allocator.free(self.cache_dir);
    }
    
    /// Check if a source file needs recompilation
    pub fn needsRecompilation(self: *CompilationCache, file_path: []const u8) !bool {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        return try self.needsRecompilationInternal(file_path);
    }
    
    /// Check if a valid cached compilation exists for a file
    pub fn hasValidCache(self: *CompilationCache, file_path: []const u8) !bool {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Inverse of needsRecompilation - if we don't need recompilation, we have valid cache
        return !(try self.needsRecompilationInternal(file_path));
    }
    
    /// Internal recompilation check without mutex (assumes caller has lock)
    fn needsRecompilationInternal(self: *CompilationCache, file_path: []const u8) !bool {
        // Check if source file exists and get current metadata
        const current_stat = self.getFileMetadata(file_path) catch {
            // File doesn't exist, definitely needs compilation
            self.metrics.recordCacheMiss();
            return true;
        };
        
        // Check if we have a cached entry
        if (self.source_cache.get(file_path)) |cached_entry| {
            // Compare file size first (quick check)
            if (cached_entry.size != current_stat.size) {
                self.metrics.recordCacheMiss();
                return true;
            }
            
            // Compare file modification time with cached file mtime
            if (cached_entry.file_mtime < current_stat.mtime) {
                self.metrics.recordCacheMiss();
                return true;
            }
            
            // Compare content hash (more expensive but definitive)
            const current_hash = try calculateFileHash(self.allocator, file_path);
            if (cached_entry.source_hash != current_hash) {
                self.metrics.recordCacheMiss();
                return true;
            }
            
            // Source unchanged, check dependencies
            const deps_changed = try self.checkDependencyChanges(file_path);
            if (deps_changed) {
                self.metrics.recordCacheMiss();
                return true;
            }
            
            // Check if build configuration has changed
            if (try self.hasBuildConfigChanged(file_path)) {
                self.metrics.recordCacheMiss();
                return true;
            }
            
            // All checks passed - cache is valid
            self.metrics.recordCacheHit();
            return false;
        }
        
        // No cached entry exists
        self.metrics.recordCacheMiss();
        return true;
    }
    
    /// Get cached AST for a source file
    pub fn getCachedAST(self: *CompilationCache, file_path: []const u8) !?CachedAST {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (try self.needsRecompilation(file_path)) {
            return null;
        }
        
        if (self.ast_cache.get(file_path)) |cached_ast| {
            self.metrics.recordCacheHit();
            return cached_ast;
        }
        
        self.metrics.recordCacheMiss();
        return null;
    }
    
    /// Cache AST for a source file
    pub fn cacheAST(self: *CompilationCache, file_path: []const u8, ast: *AST, dependencies: []const []const u8) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Get current file metadata
        const file_metadata = try self.getFileMetadata(file_path);
        const source_hash = try calculateFileHash(self.allocator, file_path);
        
        // Update source cache with accurate metadata
        const source_entry = SourceCacheEntry{
            .file_path = try self.allocator.dupe(u8, file_path),
            .source_hash = source_hash,
            .timestamp = std.time.nanoTimestamp(), // Use monotonic time for cache entry
            .file_mtime = file_metadata.mtime, // Store file mtime separately
            .size = file_metadata.size,
        };
        try self.source_cache.put(try self.allocator.dupe(u8, file_path), source_entry);
        
        // Update dependency graph
        try self.dependency_graph.updateDependencies(file_path, dependencies);
        
        // Serialize and cache AST
        const serialized_ast = try serializeAST(self.allocator, ast);
        const cached_ast = CachedAST{
            .serialized_data = serialized_ast,
            .timestamp = std.time.nanoTimestamp(),
            .dependencies = try self.allocator.dupe([]const u8, dependencies),
        };
        
        try self.ast_cache.put(try self.allocator.dupe(u8, file_path), cached_ast);
        
        // Update build configuration cache
        try self.updateBuildConfigCache();
        
        // Persist to disk if enabled
        if (self.config.enable_disk_cache) {
            try self.persistASTToDisk(file_path, cached_ast);
        }
        
        self.metrics.recordCacheStore();
    }
    
    /// Get cached object file
    pub fn getCachedObject(self: *CompilationCache, file_path: []const u8) !?CachedObject {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (try self.needsRecompilation(file_path)) {
            return null;
        }
        
        if (self.object_cache.get(file_path)) |cached_object| {
            // Verify object file still exists on disk
            if (self.verifyObjectFile(cached_object.object_path)) {
                self.metrics.recordCacheHit();
                return cached_object;
            } else {
                // Object file missing, remove from cache
                _ = self.object_cache.remove(file_path);
            }
        }
        
        self.metrics.recordCacheMiss();
        return null;
    }
    
    /// Cache compiled object file
    pub fn cacheObject(self: *CompilationCache, file_path: []const u8, object_data: []const u8, optimization_level: []const u8) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Generate object file path
        const object_path = try self.generateObjectPath(file_path, optimization_level);
        
        // Write object file to disk
        try self.writeObjectFile(object_path, object_data);
        
        // Cache object metadata
        const cached_object = CachedObject{
            .object_path = object_path,
            .timestamp = std.time.nanoTimestamp(),
            .optimization_level = try self.allocator.dupe(u8, optimization_level),
            .size = object_data.len,
        };
        
        try self.object_cache.put(file_path, cached_object);
        self.metrics.recordCacheStore();
    }
    
    /// Invalidate cache entries that depend on the given file
    pub fn invalidateDependents(self: *CompilationCache, file_path: []const u8) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const dependents = self.dependency_graph.getDependents(file_path);
        
        for (dependents) |dependent| {
            // Remove from all caches
            _ = self.source_cache.remove(dependent);
            _ = self.ast_cache.remove(dependent);
            _ = self.object_cache.remove(dependent);
            
            // Remove disk cache files
            if (self.config.enable_disk_cache) {
                try self.removeDiskCacheFile(dependent);
            }
            
            // Recursively invalidate dependents of this dependent
            try self.invalidateDependentsRecursive(dependent);
            
            self.metrics.recordCacheInvalidation();
        }
    }
    
    /// Recursively invalidate dependents (used internally)
    fn invalidateDependentsRecursive(self: *CompilationCache, file_path: []const u8) !void {
        const dependents = self.dependency_graph.getDependents(file_path);
        
        for (dependents) |dependent| {
            // Only invalidate if not already invalidated
            if (self.source_cache.contains(dependent)) {
                _ = self.source_cache.remove(dependent);
                _ = self.ast_cache.remove(dependent);
                _ = self.object_cache.remove(dependent);
                
                if (self.config.enable_disk_cache) {
                    try self.removeDiskCacheFile(dependent);
                }
                
                // Continue recursion
                try self.invalidateDependentsRecursive(dependent);
                
                self.metrics.recordCacheInvalidation();
            }
        }
    }
    
    /// Invalidate entire cache (nuclear option)
    pub fn invalidateAll(self: *CompilationCache) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Count entries before clearing
        const total_entries = self.source_cache.count() + self.ast_cache.count() + self.object_cache.count();
        
        // Clear all in-memory caches
        self.source_cache.clearAndFree();
        self.ast_cache.clearAndFree();
        self.object_cache.clearAndFree();
        self.dependency_graph.clearAll();
        
        // Remove all disk cache files
        if (self.config.enable_disk_cache) {
            try self.clearDiskCache();
        }
        
        // Update metrics
        var i: usize = 0;
        while (i < total_entries) : (i += 1) {
            self.metrics.recordCacheInvalidation();
        }
    }
    
    /// Smart cache invalidation based on file change patterns
    pub fn invalidateByPattern(self: *CompilationCache, pattern: CacheInvalidationPattern) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        switch (pattern) {
            .build_config_changed => {
                // Invalidate everything when build config changes
                try self.invalidateAllInternal();
            },
            .source_file_changed => |file_path| {
                // Invalidate file and its dependents
                _ = self.source_cache.remove(file_path);
                _ = self.ast_cache.remove(file_path);
                _ = self.object_cache.remove(file_path);
                try self.invalidateDependentsRecursive(file_path);
            },
            .dependency_changed => |dep_path| {
                // Invalidate all files that depend on this dependency
                try self.invalidateDependentsRecursive(dep_path);
            },
            .optimization_level_changed => {
                // Only invalidate object cache (AST can be reused)
                self.object_cache.clearAndFree();
            },
        }
    }
    
    /// Internal invalidate all without mutex
    fn invalidateAllInternal(self: *CompilationCache) !void {
        self.source_cache.clearAndFree();
        self.ast_cache.clearAndFree();
        self.object_cache.clearAndFree();
        self.dependency_graph.clearAll();
        
        if (self.config.enable_disk_cache) {
            try self.clearDiskCache();
        }
    }
    
    /// Clean up expired cache entries
    pub fn cleanupExpiredEntries(self: *CompilationCache) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        
        const current_time = std.time.nanoTimestamp();
        const expiry_threshold = current_time - (self.config.cache_expiry_seconds * std.time.ns_per_s);
        
        // Clean up source cache
        var source_iter = self.source_cache.iterator();
        while (source_iter.next()) |entry| {
            if (entry.value_ptr.timestamp < expiry_threshold) {
                self.allocator.free(entry.value_ptr.file_path);
                _ = self.source_cache.remove(entry.key_ptr.*);
                self.metrics.recordCacheExpiry();
            }
        }
        
        // Clean up AST cache
        var ast_iter = self.ast_cache.iterator();
        while (ast_iter.next()) |entry| {
            if (entry.value_ptr.timestamp < expiry_threshold) {
                self.allocator.free(entry.value_ptr.serialized_data);
                for (entry.value_ptr.dependencies) |dep| {
                    self.allocator.free(dep);
                }
                self.allocator.free(entry.value_ptr.dependencies);
                _ = self.ast_cache.remove(entry.key_ptr.*);
                self.metrics.recordCacheExpiry();
            }
        }
        
        // Clean up object cache
        var object_iter = self.object_cache.iterator();
        while (object_iter.next()) |entry| {
            if (entry.value_ptr.timestamp < expiry_threshold) {
                // Remove object file from disk
                std.fs.cwd().deleteFile(entry.value_ptr.object_path) catch {};
                self.allocator.free(entry.value_ptr.object_path);
                self.allocator.free(entry.value_ptr.optimization_level);
                _ = self.object_cache.remove(entry.key_ptr.*);
                self.metrics.recordCacheExpiry();
            }
        }
    }
    
    /// Get cache statistics
    pub fn getStatistics(self: *const CompilationCache) CacheStatistics {
        return CacheStatistics{
            .source_cache_size = self.source_cache.count(),
            .ast_cache_size = self.ast_cache.count(),
            .object_cache_size = self.object_cache.count(),
            .hit_rate = self.metrics.getHitRate(),
            .memory_usage_bytes = self.calculateMemoryUsage(),
            .disk_usage_bytes = self.calculateDiskUsage(),
        };
    }
    
    /// Enable incremental compilation mode
    pub fn enableIncrementalCompilation(self: *CompilationCache) void {
        self.config.enable_incremental = true;
        self.config.enable_dependency_tracking = true;
        self.config.enable_disk_cache = true;
    }
    
    // Private helper methods
    
    fn checkDependencyChanges(self: *CompilationCache, file_path: []const u8) !bool {
        const dependencies = self.dependency_graph.getDependencies(file_path);
        
        for (dependencies) |dep| {
            // Use simple file metadata check to avoid infinite recursion
            const dep_metadata = self.getFileMetadata(dep) catch {
                // Dependency file doesn't exist - needs recompilation
                return true;
            };
            
            if (self.source_cache.get(dep)) |cached_dep| {
                if (cached_dep.file_mtime < dep_metadata.mtime) {
                    return true;
                }
            } else {
                // No cached dependency - needs recompilation
                return true;
            }
        }
        
        return false;
    }
    
    /// Get file metadata for cache invalidation
    fn getFileMetadata(self: *CompilationCache, file_path: []const u8) !FileMetadata {
        _ = self;
        
        const file = std.fs.cwd().openFile(file_path, .{}) catch |err| switch (err) {
            error.FileNotFound => return error.FileNotFound,
            else => return err,
        };
        defer file.close();
        
        const stat = try file.stat();
        
        return FileMetadata{
            .size = stat.size,
            .mtime = @intCast(stat.mtime),
        };
    }
    
    /// Check if build configuration has changed since last compilation
    fn hasBuildConfigChanged(self: *CompilationCache, file_path: []const u8) !bool {
        _ = file_path;
        
        // Check if build configuration file exists and has changed
        const build_config_files = [_][]const u8{
            "build.zig",
            ".cursed-config",
            "CursedPackage.toml",
        };
        
        for (build_config_files) |config_file| {
            const config_metadata = self.getFileMetadata(config_file) catch continue;
            
            // Check if we have cached metadata for this config file
            const cache_key = try std.fmt.allocPrint(self.allocator, "build_config:{s}", .{config_file});
            defer self.allocator.free(cache_key);
            
            if (self.source_cache.get(cache_key)) |cached_config| {
                if (cached_config.file_mtime < config_metadata.mtime) {
                    // Build config is newer than cached compilation
                    return true;
                }
            } else {
                // No cached build config metadata - assume changed
                return true;
            }
        }
        
        return false;
    }
    
    /// Update build configuration cache after successful compilation
    fn updateBuildConfigCache(self: *CompilationCache) !void {
        const build_config_files = [_][]const u8{
            "build.zig",
            ".cursed-config", 
            "CursedPackage.toml",
        };
        
        for (build_config_files) |config_file| {
            const config_metadata = self.getFileMetadata(config_file) catch continue;
            
            const cache_key = try std.fmt.allocPrint(self.allocator, "build_config:{s}", .{config_file});
            defer self.allocator.free(cache_key);
            
            const config_hash = calculateFileHash(self.allocator, config_file) catch 0;
            
            const config_entry = SourceCacheEntry{
                .file_path = try self.allocator.dupe(u8, config_file),
                .source_hash = config_hash,
                .timestamp = std.time.nanoTimestamp(), // Monotonic time for cache entry
                .file_mtime = config_metadata.mtime, // Store file mtime separately
                .size = config_metadata.size,
            };
            
            try self.source_cache.put(try self.allocator.dupe(u8, cache_key), config_entry);
        }
    }
    
    fn generateObjectPath(self: *CompilationCache, file_path: []const u8, optimization_level: []const u8) ![]u8 {
        const basename = std.fs.path.basename(file_path);
        const stem = if (std.mem.lastIndexOf(u8, basename, ".")) |dot_index|
            basename[0..dot_index]
        else
            basename;
        
        const hash = std.hash_map.hashString(file_path);
        
        return try std.fmt.allocPrint(self.allocator, "{s}/objects/{s}.{s}.{x}.o", .{
            self.cache_dir, stem, optimization_level, hash
        });
    }
    
    fn writeObjectFile(self: *CompilationCache, object_path: []const u8, object_data: []const u8) !void {
        _ = self;
        
        // Ensure directory exists
        const dir = std.fs.path.dirname(object_path) orelse return error.InvalidPath;
        std.fs.cwd().makePath(dir) catch {};
        
        const file = try std.fs.cwd().createFile(object_path, .{});
        defer file.close();
        
        try file.writer().writeAll(object_data);
    }
    
    fn verifyObjectFile(self: *CompilationCache, object_path: []const u8) bool {
        _ = self;
        std.fs.cwd().access(object_path, .{}) catch return false;
        return true;
    }
    
    fn persistASTToDisk(self: *CompilationCache, file_path: []const u8, cached_ast: CachedAST) !void {
        const ast_path = try std.fmt.allocPrint(self.allocator, "{s}/ast/{x}.ast", .{
            self.cache_dir, std.hash_map.hashString(file_path)
        });
        defer self.allocator.free(ast_path);
        
        const file = try std.fs.cwd().createFile(ast_path, .{});
        defer file.close();
        
        try file.writer().writeAll(cached_ast.serialized_data);
    }
    
    fn removeDiskCacheFile(self: *CompilationCache, file_path: []const u8) !void {
        const ast_path = try std.fmt.allocPrint(self.allocator, "{s}/ast/{x}.ast", .{
            self.cache_dir, std.hash_map.hashString(file_path)
        });
        defer self.allocator.free(ast_path);
        
        std.fs.cwd().deleteFile(ast_path) catch {};
    }
    
    fn clearDiskCache(self: *CompilationCache) !void {
        // Remove entire cache directory
        var cache_dir = std.fs.cwd().openDir(self.cache_dir, .{ .iterate = true }) catch return;
        defer cache_dir.close();
        
        // Remove AST cache directory
        const ast_dir_path = try std.fmt.allocPrint(self.allocator, "{s}/ast", .{self.cache_dir});
        defer self.allocator.free(ast_dir_path);
        std.fs.cwd().deleteTree(ast_dir_path) catch {};
        
        // Remove objects cache directory 
        const objects_dir_path = try std.fmt.allocPrint(self.allocator, "{s}/objects", .{self.cache_dir});
        defer self.allocator.free(objects_dir_path);
        std.fs.cwd().deleteTree(objects_dir_path) catch {};
        
        // Recreate directories
        try createCacheDirectories(self.cache_dir);
    }
    
    fn calculateMemoryUsage(self: *const CompilationCache) usize {
        var total: usize = 0;
        
        // Source cache memory
        var source_iter = self.source_cache.iterator();
        while (source_iter.next()) |entry| {
            total += entry.value_ptr.file_path.len;
        }
        
        // AST cache memory
        var ast_iter = self.ast_cache.iterator();
        while (ast_iter.next()) |entry| {
            total += entry.value_ptr.serialized_data.len;
            for (entry.value_ptr.dependencies) |dep| {
                total += dep.len;
            }
        }
        
        // Object cache memory
        var object_iter = self.object_cache.iterator();
        while (object_iter.next()) |entry| {
            total += entry.value_ptr.object_path.len;
            total += entry.value_ptr.optimization_level.len;
        }
        
        return total;
    }
    
    fn calculateDiskUsage(self: *const CompilationCache) usize {
        _ = self;
        // Implementation would calculate actual disk usage
        return 0;
    }
};

/// Cache invalidation patterns for smart invalidation
pub const CacheInvalidationPattern = union(enum) {
    build_config_changed,
    source_file_changed: []const u8,
    dependency_changed: []const u8,
    optimization_level_changed,
};

/// Cache configuration
pub const CacheConfig = struct {
    enable_incremental: bool = true,
    enable_dependency_tracking: bool = true,
    enable_disk_cache: bool = true,
    cache_expiry_seconds: i64 = 24 * 60 * 60, // 24 hours
    max_cache_size_mb: usize = 1024, // 1GB
    max_memory_cache_entries: usize = 10000,
    
    pub fn production() CacheConfig {
        return CacheConfig{
            .enable_incremental = true,
            .enable_dependency_tracking = true,
            .enable_disk_cache = true,
            .cache_expiry_seconds = 7 * 24 * 60 * 60, // 1 week
            .max_cache_size_mb = 2048, // 2GB
            .max_memory_cache_entries = 20000,
        };
    }
    
    pub fn development() CacheConfig {
        return CacheConfig{
            .enable_incremental = true,
            .enable_dependency_tracking = true,
            .enable_disk_cache = false,
            .cache_expiry_seconds = 60 * 60, // 1 hour
            .max_cache_size_mb = 512, // 512MB
            .max_memory_cache_entries = 5000,
        };
    }
};

/// Cache entry types

const SourceCacheEntry = struct {
    file_path: []const u8,
    source_hash: u64,
    timestamp: i64, // Monotonic timestamp when cache entry was created (nanoseconds)
    file_mtime: i64, // File modification time at time of caching
    size: usize,
};

const FileMetadata = struct {
    size: usize,
    mtime: i64,
};

const CachedAST = struct {
    serialized_data: []const u8,
    timestamp: i64, // Now uses nanoseconds from monotonic clock (not wall clock time)
    dependencies: []const []const u8,
};

const CachedObject = struct {
    object_path: []const u8,
    timestamp: i64, // Now uses nanoseconds from monotonic clock (not wall clock time)
    optimization_level: []const u8,
    size: usize,
};

/// Cache storage types

const SourceCache = HashMap([]const u8, SourceCacheEntry, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);
const ASTCache = HashMap([]const u8, CachedAST, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);
const ObjectCache = HashMap([]const u8, CachedObject, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);

/// Dependency tracking system
const DependencyGraph = struct {
    allocator: Allocator,
    dependencies: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    dependents: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    fn init(allocator: Allocator) !DependencyGraph {
        return DependencyGraph{
            .allocator = allocator,
            .dependencies = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .dependents = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    fn deinit(self: *DependencyGraph) void {
        // Clean up dependencies
        var deps_iter = self.dependencies.iterator();
        while (deps_iter.next()) |entry| {
            for (entry.value_ptr.items) |dep| {
                self.allocator.free(dep);
            }
            entry.value_ptr.deinit();
        }
        self.dependencies.deinit(self.allocator);
        
        // Clean up dependents
        var dependents_iter = self.dependents.iterator();
        while (dependents_iter.next()) |entry| {
            for (entry.value_ptr.items) |dependent| {
                self.allocator.free(dependent);
            }
            entry.value_ptr.deinit();
        }
        self.dependents.deinit(self.allocator);
    }
    
    fn updateDependencies(self: *DependencyGraph, file_path: []const u8, dependencies: []const []const u8) !void {
        // Update dependencies for file
        var deps_list = std.ArrayList(u8){};
        for (dependencies) |dep| {
            try deps_list.append(self.allocator, try self.allocator.dupe(u8, dep));
        }
        try self.dependencies.put(try self.allocator.dupe(u8, file_path), deps_list);
        
        // Update reverse dependencies (dependents)
        for (dependencies) |dep| {
            var result = try self.dependents.getOrPut(try self.allocator.dupe(u8, dep));
            if (!result.found_existing) {
                result.value_ptr.* = .empty;
            }
            try result.value_ptr.append(self.allocator, try self.allocator.dupe(u8, file_path));
        }
    }
    
    fn getDependencies(self: *const DependencyGraph, file_path: []const u8) []const []const u8 {
        if (self.dependencies.get(file_path)) |deps| {
            return deps.items;
        }
        return &[_][]const u8{};
    }
    
    fn getDependents(self: *const DependencyGraph, file_path: []const u8) []const []const u8 {
        if (self.dependents.get(file_path)) |dependents| {
            return dependents.items;
        }
        return &[_][]const u8{};
    }
    
    fn clearAll(self: *DependencyGraph) void {
        // Clean up dependencies
        var deps_iter = self.dependencies.iterator();
        while (deps_iter.next()) |entry| {
            for (entry.value_ptr.items) |dep| {
                self.allocator.free(dep);
            }
            entry.value_ptr.deinit();
            self.allocator.free(entry.key_ptr.*);
        }
        self.dependencies.clearAndFree(self.allocator);
        
        // Clean up dependents
        var dependents_iter = self.dependents.iterator();
        while (dependents_iter.next()) |entry| {
            for (entry.value_ptr.items) |dependent| {
                self.allocator.free(dependent);
            }
            entry.value_ptr.deinit();
            self.allocator.free(entry.key_ptr.*);
        }
        self.dependents.clearAndFree(self.allocator);
    }
};

/// Cache performance metrics
const CacheMetrics = struct {
    hits: usize = 0,
    misses: usize = 0,
    stores: usize = 0,
    invalidations: usize = 0,
    expiries: usize = 0,
    
    fn init() CacheMetrics {
        return CacheMetrics{};
    }
    
    fn recordCacheHit(self: *CacheMetrics) void {
        self.hits += 1;
    }
    
    fn recordCacheMiss(self: *CacheMetrics) void {
        self.misses += 1;
    }
    
    fn recordCacheStore(self: *CacheMetrics) void {
        self.stores += 1;
    }
    
    fn recordCacheInvalidation(self: *CacheMetrics) void {
        self.invalidations += 1;
    }
    
    fn recordCacheExpiry(self: *CacheMetrics) void {
        self.expiries += 1;
    }
    
    fn getHitRate(self: *const CacheMetrics) f64 {
        const total = self.hits + self.misses;
        if (total == 0) return 0.0;
        return @as(f64, @floatFromInt(self.hits)) / @as(f64, @floatFromInt(total));
    }
};

/// Cache statistics
pub const CacheStatistics = struct {
    source_cache_size: usize,
    ast_cache_size: usize,
    object_cache_size: usize,
    hit_rate: f64,
    memory_usage_bytes: usize,
    disk_usage_bytes: usize,
    
    pub fn print(self: *const CacheStatistics) void {
        std.debug.print("=== COMPILATION CACHE STATISTICS ===\n", .{});
        std.debug.print("Source cache entries: {d}\n", .{self.source_cache_size});
        std.debug.print("AST cache entries: {d}\n", .{self.ast_cache_size});
        std.debug.print("Object cache entries: {d}\n", .{self.object_cache_size});
        std.debug.print("Cache hit rate: {d:.1}%\n", .{self.hit_rate * 100});
        std.debug.print("Memory usage: {d:.2}MB\n", .{@as(f64, @floatFromInt(self.memory_usage_bytes)) / (1024 * 1024)});
        std.debug.print("Disk usage: {d:.2}MB\n", .{@as(f64, @floatFromInt(self.disk_usage_bytes)) / (1024 * 1024)});
        std.debug.print("===================================\n", .{});
    }
};

// Placeholder types
pub const AST = struct {
    // Placeholder AST structure
};

// Helper functions

fn createCacheDirectories(cache_dir: []const u8) !void {
    std.fs.cwd().makePath(cache_dir) catch {};
    
    const ast_dir = try std.fmt.allocPrint(std.heap.page_allocator, "{s}/ast", .{cache_dir});
    defer std.heap.page_allocator.free(ast_dir);
    std.fs.cwd().makePath(ast_dir) catch {};
    
    const objects_dir = try std.fmt.allocPrint(std.heap.page_allocator, "{s}/objects", .{cache_dir});
    defer std.heap.page_allocator.free(objects_dir);
    std.fs.cwd().makePath(objects_dir) catch {};
}

fn calculateFileHash(allocator: Allocator, file_path: []const u8) !u64 {
    const file = std.fs.cwd().openFile(file_path, .{}) catch {
        return 0; // File doesn't exist
    };
    defer file.close();
    
    const file_size = try file.getEndPos();
    const content = try allocator.alloc(u8, file_size);
    defer allocator.free(content);
    
    _ = try file.readAll(content);
    return std.hash_map.hashString(content);
}

fn getFileSize(file_path: []const u8) !usize {
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();
    
    return try file.getEndPos();
}

fn serializeAST(allocator: Allocator, ast: *AST) ![]u8 {
    _ = ast;
    // Placeholder implementation
    const data = "serialized_ast_data";
    return try allocator.dupe(u8, data);
}

// Test functions

test "CompilationCache basic operations" {
    const allocator = std.testing.allocator;
    const config = CacheConfig.development();
    
    var cache = try CompilationCache.init(allocator, "/tmp/cursed_test_cache", config);
    defer cache.deinit();
    
    // Test cache miss
    try std.testing.expect(try cache.needsRecompilation("test.💀"));
    
    const stats = cache.getStatistics();
    try std.testing.expect(stats.source_cache_size == 0);
}

test "CacheConfig presets" {
    const prod_config = CacheConfig.production();
    try std.testing.expect(prod_config.enable_incremental == true);
    try std.testing.expect(prod_config.max_cache_size_mb == 2048);
    
    const dev_config = CacheConfig.development();
    try std.testing.expect(dev_config.enable_disk_cache == false);
    try std.testing.expect(dev_config.max_cache_size_mb == 512);
}
