const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const Timer = std.time.Timer;

pub const PerformanceProfile = struct {
    lexing_time: u64,
    parsing_time: u64,
    codegen_time: u64,
    total_time: u64,
    memory_used: usize,
    tokens_per_second: f64,
    nodes_per_second: f64,
    
    pub fn init() PerformanceProfile {
        return PerformanceProfile{
            .lexing_time = 0,
            .parsing_time = 0,
            .codegen_time = 0,
            .total_time = 0,
            .memory_used = 0,
            .tokens_per_second = 0,
            .nodes_per_second = 0,
        };
    }
    
    pub fn print(self: *const PerformanceProfile) void {
        std.debug.print("=== PERFORMANCE PROFILE ===\n", .{});
        std.debug.print("Lexing:  {d:.3}ms ({d:.1} tokens/sec)\n", .{ @as(f64, @floatFromInt(self.lexing_time)) / 1_000_000, self.tokens_per_second });
        std.debug.print("Parsing: {d:.3}ms ({d:.1} nodes/sec)\n", .{ @as(f64, @floatFromInt(self.parsing_time)) / 1_000_000, self.nodes_per_second });
        std.debug.print("Codegen: {d:.3}ms\n", .{ @as(f64, @floatFromInt(self.codegen_time)) / 1_000_000 });
        std.debug.print("Total:   {d:.3}ms\n", .{ @as(f64, @floatFromInt(self.total_time)) / 1_000_000 });
        std.debug.print("Memory:  {d:.2}MB\n", .{ @as(f64, @floatFromInt(self.memory_used)) / 1_048_576 });
        std.debug.print("===========================\n", .{});
    }
};

pub const CompilationCache = struct {
    allocator: Allocator,
    cache_dir: []const u8,
    enabled: bool,
    
    const CacheEntry = struct {
        source_hash: u64,
        timestamp: i64,
        compiled_output: []u8,
    };
    
    pub fn init(allocator: Allocator, cache_dir: []const u8) !CompilationCache {
        // Create cache directory if it doesn't exist
        std.fs.cwd().makeDir(cache_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {},
            else => return err,
        };
        
        return CompilationCache{
            .allocator = allocator,
            .cache_dir = try allocator.dupe(u8, cache_dir),
            .enabled = true,
        };
    }
    
    pub fn deinit(self: *CompilationCache) void {
        self.allocator.free(self.cache_dir);
    }
    
    pub fn getCachedOutput(self: *CompilationCache, source: []const u8, filename: []const u8) ?[]u8 {
        if (!self.enabled) return null;
        
        const hash = std.hash_map.hashString(source);
        const cache_file = std.fmt.allocPrint(self.allocator, "{s}/{s}.{x}.cache", .{ self.cache_dir, filename, hash }) catch return null;
        defer self.allocator.free(cache_file);
        
        const file = std.fs.cwd().openFile(cache_file, .{}) catch return null;
        defer file.close();
        
        const content = file.readToEndAlloc(self.allocator, 1024 * 1024) catch return null;
        return content;
    }
    
    pub fn cacheOutput(self: *CompilationCache, source: []const u8, filename: []const u8, output: []const u8) void {
        if (!self.enabled) return;
        
        const hash = std.hash_map.hashString(source);
        const cache_file = std.fmt.allocPrint(self.allocator, "{s}/{s}.{x}.cache", .{ self.cache_dir, filename, hash }) catch return;
        defer self.allocator.free(cache_file);
        
        const file = std.fs.cwd().createFile(cache_file, .{}) catch return;
        defer file.close();
        
        file.writeAll(output) catch return;
    }
};

pub const FastLexer = struct {
    allocator: Allocator,
    source: []const u8,
    position: usize,
    tokens: ArrayList(Token),
    
    // Performance optimizations
    identifier_cache: std.HashMap([]const u8, TokenKind, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub const Token = struct {
        kind: TokenKind,
        lexeme: []const u8,
        line: usize,
        column: usize,
    };
    
    const TokenKind = enum {
        // Basic tokens
        Number, Integer, StringLiteral, Identifier,
        // CURSED keywords
        Slay, Sus, Facts, Vibez, Yeet, Squad, Collab, 
        Bestie, Stan, Match, Based, Cringe,
        // Operators and punctuation
        Equal, Plus, Minus, Star, Slash, Bang,
        LeftParen, RightParen, LeftBrace, RightBrace,
        LeftBracket, RightBracket, Comma, Semicolon,
        Dot, Arrow, LeftArrow, Colon, ColonColon,
        // Special tokens
        Newline, Eof, Invalid,
    };
    
    pub fn init(allocator: Allocator, source: []const u8) FastLexer {
        var lexer = FastLexer{
            .allocator = allocator,
            .source = source,
            .position = 0,
            .tokens = .empty,
            .identifier_cache = std.HashMap([]const u8, TokenKind, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
        
        // Pre-populate keyword cache for fast lookup
        lexer.populateKeywordCache() catch {};
        
        return lexer;
    }
    
    pub fn deinit(self: *FastLexer) void {
        self.tokens.deinit();
        self.identifier_cache.deinit();
    }
    
    fn populateKeywordCache(self: *FastLexer) !void {
        const keywords = [_]struct { []const u8, TokenKind }{
            .{ "slay", .Slay },
            .{ "sus", .Sus },
            .{ "facts", .Facts },
            .{ "vibez", .Vibez },
            .{ "yeet", .Yeet },
            .{ "squad", .Squad },
            .{ "collab", .Collab },
            .{ "bestie", .Bestie },
            .{ "stan", .Stan },
            .{ "match", .Match },
            .{ "based", .Based },
            .{ "cringe", .Cringe },
        };
        
        for (keywords) |kw| {
            try self.identifier_cache.put(kw[0], kw[1]);
        }
    }
    
    pub fn tokenizeOptimized(self: *FastLexer) ![]Token {
        var line: usize = 1;
        var column: usize = 1;
        
        // Pre-allocate tokens array with estimated capacity
        try self.tokens.ensureTotalCapacity(allocator, self.source.len / 6); // Rough estimate
        
        while (self.position < self.source.len) {
            const start_pos = self.position;
            const ch = self.source[self.position];
            
            switch (ch) {
                ' ', '\t', '\r' => {
                    self.position += 1;
                    column += 1;
                    continue;
                },
                '\n' => {
                    try self.addToken(.Newline, self.source[start_pos..self.position + 1], line, column);
                    self.position += 1;
                    line += 1;
                    column = 1;
                },
                '0'...'9' => try self.scanNumber(&line, &column),
                'a'...'z', 'A'...'Z', '_' => try self.scanIdentifier(&line, &column),
                '"' => try self.scanString(&line, &column),
                '\'' => try self.scanCharacter(&line, &column),
                '=' => {
                    self.position += 1;
                    try self.addToken(.Equal, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                '+' => {
                    self.position += 1;
                    try self.addToken(.Plus, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                '-' => {
                    if (self.position + 1 < self.source.len and self.source[self.position + 1] == '>') {
                        self.position += 2;
                        try self.addToken(.Arrow, self.source[start_pos..self.position], line, column);
                        column += 2;
                    } else {
                        self.position += 1;
                        try self.addToken(.Minus, self.source[start_pos..self.position], line, column);
                        column += 1;
                    }
                },
                '*' => {
                    self.position += 1;
                    try self.addToken(.Star, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                '/' => {
                    if (self.position + 1 < self.source.len and self.source[self.position + 1] == '/') {
                        // Skip comment
                        while (self.position < self.source.len and self.source[self.position] != '\n') {
                            self.position += 1;
                        }
                        continue;
                    } else {
                        self.position += 1;
                        try self.addToken(.Slash, self.source[start_pos..self.position], line, column);
                        column += 1;
                    }
                },
                '!' => {
                    self.position += 1;
                    try self.addToken(.Bang, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                '(' => {
                    self.position += 1;
                    try self.addToken(.LeftParen, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                ')' => {
                    self.position += 1;
                    try self.addToken(.RightParen, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                '{' => {
                    self.position += 1;
                    try self.addToken(.LeftBrace, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                '}' => {
                    self.position += 1;
                    try self.addToken(.RightBrace, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                '[' => {
                    self.position += 1;
                    try self.addToken(.LeftBracket, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                ']' => {
                    self.position += 1;
                    try self.addToken(.RightBracket, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                ',' => {
                    self.position += 1;
                    try self.addToken(.Comma, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                ';' => {
                    self.position += 1;
                    try self.addToken(.Semicolon, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                '.' => {
                    self.position += 1;
                    try self.addToken(.Dot, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
                ':' => {
                    if (self.position + 1 < self.source.len and self.source[self.position + 1] == ':') {
                        self.position += 2;
                        try self.addToken(.ColonColon, self.source[start_pos..self.position], line, column);
                        column += 2;
                    } else {
                        self.position += 1;
                        try self.addToken(.Colon, self.source[start_pos..self.position], line, column);
                        column += 1;
                    }
                },
                '<' => {
                    if (self.position + 1 < self.source.len and self.source[self.position + 1] == '-') {
                        self.position += 2;
                        try self.addToken(.LeftArrow, self.source[start_pos..self.position], line, column);
                        column += 2;
                    } else {
                        self.position += 1;
                        try self.addToken(.Invalid, self.source[start_pos..self.position], line, column);
                        column += 1;
                    }
                },
                else => {
                    self.position += 1;
                    try self.addToken(.Invalid, self.source[start_pos..self.position], line, column);
                    column += 1;
                },
            }
        }
        
        try self.addToken(.Eof, "", line, column);
        return self.tokens.toOwnedSlice();
    }
    
    fn scanNumber(self: *FastLexer, line: *usize, column: *usize) !void {
        const start_pos = self.position;
        
        while (self.position < self.source.len and (self.source[self.position] >= '0' and self.source[self.position] <= '9')) {
            self.position += 1;
        }
        
        if (self.position < self.source.len and self.source[self.position] == '.') {
            self.position += 1;
            while (self.position < self.source.len and (self.source[self.position] >= '0' and self.source[self.position] <= '9')) {
                self.position += 1;
            }
            try self.addToken(.Number, self.source[start_pos..self.position], line.*, column.*);
        } else {
            try self.addToken(.Integer, self.source[start_pos..self.position], line.*, column.*);
        }
        
        column.* += self.position - start_pos;
    }
    
    fn scanIdentifier(self: *FastLexer, line: *usize, column: *usize) !void {
        const start_pos = self.position;
        
        while (self.position < self.source.len) {
            const ch = self.source[self.position];
            if ((ch >= 'a' and ch <= 'z') or (ch >= 'A' and ch <= 'Z') or (ch >= '0' and ch <= '9') or ch == '_') {
                self.position += 1;
            } else {
                break;
            }
        }
        
        const lexeme = self.source[start_pos..self.position];
        const token_kind = self.identifier_cache.get(lexeme) orelse .Identifier;
        
        try self.addToken(token_kind, lexeme, line.*, column.*);
        column.* += lexeme.len;
    }
    
    fn scanString(self: *FastLexer, line: *usize, column: *usize) !void {
        const start_pos = self.position;
        self.position += 1; // Skip opening quote
        
        while (self.position < self.source.len and self.source[self.position] != '"') {
            if (self.source[self.position] == '\n') {
                line.* += 1;
                column.* = 1;
            }
            self.position += 1;
        }
        
        if (self.position < self.source.len) {
            self.position += 1; // Skip closing quote
        }
        
        try self.addToken(.StringLiteral, self.source[start_pos..self.position], line.*, column.*);
        column.* += self.position - start_pos;
    }
    
    fn scanCharacter(self: *FastLexer, line: *usize, column: *usize) !void {
        const start_pos = self.position;
        self.position += 1; // Skip opening quote
        
        if (self.position < self.source.len and self.source[self.position] != '\'') {
            self.position += 1;
        }
        
        if (self.position < self.source.len and self.source[self.position] == '\'') {
            self.position += 1; // Skip closing quote
        }
        
        try self.addToken(.StringLiteral, self.source[start_pos..self.position], line.*, column.*);
        column.* += self.position - start_pos;
    }
    
    fn addToken(self: *FastLexer, kind: TokenKind, lexeme: []const u8, line: usize, column: usize) !void {
        try self.tokens.append(Token{
            .kind = kind,
            .lexeme = lexeme,
            .line = line,
            .column = column,
        });
    }
};

pub const OptimizedMemoryPool = struct {
    allocator: Allocator,
    pools: [4]FixedPool,
    
    const FixedPool = struct {
        items: []u8,
        used: usize,
        item_size: usize,
        
        pub fn init(allocator: Allocator, item_size: usize, capacity: usize) !FixedPool {
            return FixedPool{
                .items = try allocator.alloc(u8, item_size * capacity),
                .used = 0,
                .item_size = item_size,
            };
        }
        
        pub fn deinit(self: *FixedPool, allocator: Allocator) void {
            allocator.free(self.items);
        }
        
        pub fn allocate(self: *FixedPool) ?[]u8 {
            if (self.used * self.item_size + self.item_size > self.items.len) {
                return null;
            }
            
            const start = self.used * self.item_size;
            self.used += 1;
            return self.items[start..start + self.item_size];
        }
        
        pub fn reset(self: *FixedPool) void {
            self.used = 0;
        }
    };
    
    pub fn init(allocator: Allocator) !OptimizedMemoryPool {
        return OptimizedMemoryPool{
            .allocator = allocator,
            .pools = [_]FixedPool{
                try FixedPool.init(allocator, 32, 1000),   // Small objects
                try FixedPool.init(allocator, 128, 500),   // Medium objects
                try FixedPool.init(allocator, 512, 200),   // Large objects
                try FixedPool.init(allocator, 2048, 50),   // Extra large objects
            },
        };
    }
    
    pub fn deinit(self: *OptimizedMemoryPool) void {
        for (&self.pools) |*pool| {
            pool.deinit();
        }
    }
    
    pub fn allocate(self: *OptimizedMemoryPool, size: usize) ?[]u8 {
        for (&self.pools) |*pool| {
            if (size <= pool.item_size) {
                return pool.allocate();
            }
        }
        // Fallback to general allocator for very large allocations
        return self.allocator.alloc(u8, size) catch null;
    }
    
    pub fn reset(self: *OptimizedMemoryPool) void {
        for (&self.pools) |*pool| {
            pool.reset();
        }
    }
};

pub const PerformanceProfiler = struct {
    allocator: Allocator,
    timer: Timer,
    profile: PerformanceProfile,
    memory_tracker: MemoryTracker,
    
    const MemoryTracker = struct {
        initial_memory: usize,
        peak_memory: usize,
        current_memory: usize,
        
        pub fn init() MemoryTracker {
            return MemoryTracker{
                .initial_memory = 0,
                .peak_memory = 0,
                .current_memory = 0,
            };
        }
        
        pub fn recordAllocation(self: *MemoryTracker, size: usize) void {
            self.current_memory += size;
            if (self.current_memory > self.peak_memory) {
                self.peak_memory = self.current_memory;
            }
        }
        
        pub fn recordDeallocation(self: *MemoryTracker, size: usize) void {
            if (self.current_memory >= size) {
                self.current_memory -= size;
            }
        }
    };
    
    pub fn init(allocator: Allocator) !PerformanceProfiler {
        return PerformanceProfiler{
            .allocator = allocator,
            .timer = try Timer.start(),
            .profile = PerformanceProfile.init(),
            .memory_tracker = MemoryTracker.init(),
        };
    }
    
    pub fn startTiming(self: *PerformanceProfiler, phase: enum { lexing, parsing, codegen }) void {
        _ = phase;
        self.timer.reset();
    }
    
    pub fn endTiming(self: *PerformanceProfiler, phase: enum { lexing, parsing, codegen }, count: usize) void {
        const elapsed = self.timer.read();
        
        switch (phase) {
            .lexing => {
                self.profile.lexing_time = elapsed;
                self.profile.tokens_per_second = @as(f64, @floatFromInt(count)) / (@as(f64, @floatFromInt(elapsed)) / 1_000_000_000);
            },
            .parsing => {
                self.profile.parsing_time = elapsed;
                self.profile.nodes_per_second = @as(f64, @floatFromInt(count)) / (@as(f64, @floatFromInt(elapsed)) / 1_000_000_000);
            },
            .codegen => {
                self.profile.codegen_time = elapsed;
            },
        }
    }
    
    pub fn getTotalTime(self: *PerformanceProfiler) u64 {
        return self.profile.lexing_time + self.profile.parsing_time + self.profile.codegen_time;
    }
    
    pub fn getProfile(self: *PerformanceProfiler) PerformanceProfile {
        self.profile.total_time = self.getTotalTime();
        self.profile.memory_used = self.memory_tracker.peak_memory;
        return self.profile;
    }
};

// Performance benchmark functions
pub fn benchmarkLexer(allocator: Allocator, source: []const u8, iterations: usize) !PerformanceProfile {
    var profiler = try PerformanceProfiler.init(allocator);
    
    var total_tokens: usize = 0;
    var i: usize = 0;
    
    profiler.startTiming(.lexing);
    
    while (i < iterations) : (i += 1) {
        var lexer = FastLexer.init(allocator, source);
        defer lexer.deinit();
        
        const tokens = try lexer.tokenizeOptimized();
        total_tokens += tokens.len;
        allocator.free(tokens);
    }
    
    profiler.endTiming(.lexing, total_tokens);
    return profiler.getProfile();
}

pub fn benchmarkParser(allocator: Allocator, tokens: []const FastLexer.Token, iterations: usize) !PerformanceProfile {
    _ = tokens;
    _ = iterations;
    
    var profiler = try PerformanceProfiler.init(allocator);
    // Parser benchmarking would go here
    return profiler.getProfile();
}

pub fn optimizeCompilationFlags() std.Build.OptimizeMode {
    // Return the best optimization for production compilation speed
    return .ReleaseFast;
}

test "FastLexer performance" {
    const allocator = std.testing.allocator;
    const source = 
        \\slay factorial(n normie) normie {
        \\    sus result normie = 1
        \\    bestie i := 1; i <= n; i = i + 1 {
        \\        result = result * i
        \\    }
        \\    damn result
        \\}
    ;
    
    var lexer = FastLexer.init(allocator, source);
    defer lexer.deinit();
    
    const tokens = try lexer.tokenizeOptimized();
    defer allocator.free(tokens);
    
    try std.testing.expect(tokens.len > 10);
}

test "PerformanceProfiler" {
    const allocator = std.testing.allocator;
    
    var profiler = try PerformanceProfiler.init(allocator);
    
    profiler.startTiming(.lexing);
    std.time.sleep(1000000); // 1ms
    profiler.endTiming(.lexing, 100);
    
    const profile = profiler.getProfile();
    try std.testing.expect(profile.lexing_time > 0);
    try std.testing.expect(profile.tokens_per_second > 0);
}

test "OptimizedMemoryPool" {
    const allocator = std.testing.allocator;
    
    var pool = try OptimizedMemoryPool.init(allocator);
    defer pool.deinit();
    
    const small_alloc = pool.allocate(16);
    try std.testing.expect(small_alloc != null);
    try std.testing.expect(small_alloc.?.len >= 16);
    
    const large_alloc = pool.allocate(1024);
    try std.testing.expect(large_alloc != null);
    try std.testing.expect(large_alloc.?.len >= 1024);
}
