const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

/// Effect tracking system integrated with borrow checker for comprehensive static analysis
/// This system prevents data races and ensures memory safety through compile-time verification
pub const EffectSystem = struct {
    allocator: Allocator,
    
    // Effect tracking maps
    read_effects: HashMap(u32, ArrayList(u32), std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    write_effects: HashMap(u32, ArrayList(u32), std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    borrow_effects: HashMap(u32, BorrowInfo, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    
    // Effect analysis cache
    effect_cache: HashMap(u32, EffectSet, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    
    // Integration with borrow checker
    borrow_checker: *BorrowChecker,
    
    const Self = @This();
    
    /// Information about borrow operations for effect analysis
    pub const BorrowInfo = struct {
        borrow_type: BorrowType,
        lifetime_id: u32,
        borrowed_from: u32, // ID of the borrowed value
        is_mutable: bool,
        borrow_site: SourceLocation,
        
        pub const BorrowType = enum {
            shared,      // &T
            mutable,     // &mut T  
            moved,       // move semantics
            copied,      // copy semantics
        };
    };
    
    /// Set of effects for a given operation or expression
    pub const EffectSet = struct {
        reads: ArrayList(u32),
        writes: ArrayList(u32),
        borrows: ArrayList(u32),
        calls: ArrayList(u32),
        allocations: ArrayList(u32),
        
        // Safety properties
        is_pure: bool,
        is_safe: bool,
        may_panic: bool,
        may_block: bool,
        
        pub fn init(allocator: Allocator) EffectSet {
            return EffectSet{
                .reads = ArrayList(u32).init(allocator),
                .writes = ArrayList(u32).init(allocator),
                .borrows = ArrayList(u32).init(allocator),
                .calls = ArrayList(u32).init(allocator),
                .allocations = ArrayList(u32).init(allocator),
                .is_pure = true,
                .is_safe = true,
                .may_panic = false,
                .may_block = false,
            };
        }
        
        pub fn deinit(self: *EffectSet) void {
            self.reads.deinit();
            self.writes.deinit();
            self.borrows.deinit();
            self.calls.deinit();
            self.allocations.deinit();
        }
    };
    
    /// Source location for error reporting
    pub const SourceLocation = struct {
        file: []const u8,
        line: u32,
        column: u32,
    };
    
    /// Borrow checker integration
    pub const BorrowChecker = struct {
        allocator: Allocator,
        active_borrows: HashMap(u32, ArrayList(BorrowInfo), std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
        lifetimes: HashMap(u32, LifetimeInfo, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
        
        pub const LifetimeInfo = struct {
            start_location: SourceLocation,
            end_location: ?SourceLocation,
            is_active: bool,
            dependencies: ArrayList(u32), // other lifetimes this depends on
        };
        
        pub fn init(allocator: Allocator) BorrowChecker {
            return BorrowChecker{
                .allocator = allocator,
                .active_borrows = HashMap(u32, ArrayList(BorrowInfo), std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
                .lifetimes = HashMap(u32, LifetimeInfo, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            };
        }
        
        pub fn deinit(self: *BorrowChecker) void {
            var iter = self.active_borrows.iterator();
            while (iter.next()) |entry| {
                entry.value_ptr.deinit();
            }
            self.active_borrows.deinit();
            
            var lifetime_iter = self.lifetimes.iterator();
            while (lifetime_iter.next()) |entry| {
                entry.value_ptr.dependencies.deinit();
            }
            self.lifetimes.deinit();
        }
    };
    
    pub fn init(allocator: Allocator, borrow_checker: *BorrowChecker) EffectSystem {
        return EffectSystem{
            .allocator = allocator,
            .read_effects = HashMap(u32, ArrayList(u32), std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .write_effects = HashMap(u32, ArrayList(u32), std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .borrow_effects = HashMap(u32, BorrowInfo, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .effect_cache = HashMap(u32, EffectSet, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .borrow_checker = borrow_checker,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up read effects
        var read_iter = self.read_effects.iterator();
        while (read_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.read_effects.deinit();
        
        // Clean up write effects
        var write_iter = self.write_effects.iterator();
        while (write_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.write_effects.deinit();
        
        self.borrow_effects.deinit();
        
        // Clean up effect cache
        var cache_iter = self.effect_cache.iterator();
        while (cache_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.effect_cache.deinit();
    }
    
    /// Track a read effect and integrate with borrow analysis
    pub fn trackReadEffect(self: *Self, operation_id: u32, target_id: u32, location: SourceLocation) !void {
        // Check borrow constraints before allowing read
        try self.checkReadPermission(target_id, location);
        
        // Get or create the read effects list for this operation
        const result = try self.read_effects.getOrPut(operation_id);
        if (!result.found_existing) {
            result.value_ptr.* = ArrayList(u32).init(self.allocator);
        }
        
        try result.value_ptr.append(target_id);
    }
    
    /// Track a write effect and integrate with borrow analysis
    pub fn trackWriteEffect(self: *Self, operation_id: u32, target_id: u32, location: SourceLocation) !void {
        // Check borrow constraints before allowing write
        try self.checkWritePermission(target_id, location);
        
        // Get or create the write effects list for this operation
        const result = try self.write_effects.getOrPut(operation_id);
        if (!result.found_existing) {
            result.value_ptr.* = ArrayList(u32).init(self.allocator);
        }
        
        try result.value_ptr.append(target_id);
    }
    
    /// Track a borrow effect and integrate with lifetime analysis
    pub fn trackBorrowEffect(self: *Self, operation_id: u32, borrow_info: BorrowInfo) !void {
        // Validate borrow with borrow checker integration  
        try self.validateBorrow(borrow_info);
        
        try self.borrow_effects.put(operation_id, borrow_info);
        
        // Register with borrow checker
        try self.registerBorrowWithChecker(borrow_info);
    }
    
    /// CRITICAL: Integration point around line 300 - wire effect system into borrow analysis
    /// This function prevents false negatives by ensuring all effects are checked against borrows
    pub fn analyzeEffectsWithBorrowChecking(self: *Self, operation_id: u32) !EffectAnalysisResult {
        // Check cache first
        if (self.effect_cache.get(operation_id)) |cached_effects| {
            return EffectAnalysisResult{
                .effects = cached_effects,
                .is_safe = cached_effects.is_safe,
                .violations = ArrayList(BorrowViolation).init(self.allocator),
            };
        }
        
        var effect_set = EffectSet.init(self.allocator);
        var violations = ArrayList(BorrowViolation).init(self.allocator);
        var is_safe = true;
        
        // Analyze read effects with borrow constraints
        if (self.read_effects.get(operation_id)) |reads| {
            try effect_set.reads.appendSlice(reads.items);
            
            for (reads.items) |target_id| {
                // CRITICAL FIX: Check all active borrows for conflicts
                if (self.checkBorrowConflictOnRead(target_id)) |violation| {
                    try violations.append(violation);
                    is_safe = false;
                    effect_set.is_safe = false;
                }
            }
        }
        
        // Analyze write effects with borrow constraints  
        if (self.write_effects.get(operation_id)) |writes| {
            try effect_set.writes.appendSlice(writes.items);
            effect_set.is_pure = false; // writes make operations impure
            
            for (writes.items) |target_id| {
                // CRITICAL FIX: Check for write-read and write-write conflicts
                if (self.checkBorrowConflictOnWrite(target_id)) |violation| {
                    try violations.append(violation);
                    is_safe = false;
                    effect_set.is_safe = false;
                }
            }
        }
        
        // Analyze borrow effects with lifetime constraints
        if (self.borrow_effects.get(operation_id)) |borrow_info| {
            try effect_set.borrows.append(operation_id);
            
            // CRITICAL FIX: Cross-check with existing borrows for conflicts
            if (self.checkBorrowLifetimeConflict(borrow_info)) |violation| {
                try violations.append(violation);
                is_safe = false;
                effect_set.is_safe = false;
            }
        }
        
        // Cache the analyzed effects for performance
        try self.effect_cache.put(operation_id, effect_set);
        
        return EffectAnalysisResult{
            .effects = effect_set,
            .is_safe = is_safe,
            .violations = violations,
        };
    }
    
    /// Check for borrow conflicts on read operations
    fn checkBorrowConflictOnRead(self: *Self, target_id: u32) ?BorrowViolation {
        // Check if there's an active mutable borrow that conflicts
        if (self.borrow_checker.active_borrows.get(target_id)) |borrows| {
            for (borrows.items) |borrow| {
                if (borrow.is_mutable and borrow.borrow_type == .mutable) {
                    return BorrowViolation{
                        .violation_type = .read_during_mutable_borrow,
                        .target_id = target_id,
                        .conflicting_borrow = borrow,
                        .location = borrow.borrow_site,
                    };
                }
            }
        }
        return null;
    }
    
    /// Check for borrow conflicts on write operations  
    fn checkBorrowConflictOnWrite(self: *Self, target_id: u32) ?BorrowViolation {
        // Check if there's any active borrow that conflicts with writes
        if (self.borrow_checker.active_borrows.get(target_id)) |borrows| {
            for (borrows.items) |borrow| {
                return BorrowViolation{
                    .violation_type = .write_during_borrow,
                    .target_id = target_id,
                    .conflicting_borrow = borrow,
                    .location = borrow.borrow_site,
                };
            }
        }
        return null;
    }
    
    /// Check for borrow lifetime conflicts
    fn checkBorrowLifetimeConflict(self: *Self, borrow_info: BorrowInfo) ?BorrowViolation {
        // Check if borrowing something that's already mutably borrowed
        if (self.borrow_checker.active_borrows.get(borrow_info.borrowed_from)) |existing_borrows| {
            for (existing_borrows.items) |existing| {
                // Conflict cases:
                // 1. Trying to mutably borrow when already borrowed
                // 2. Trying to borrow when already mutably borrowed
                if ((borrow_info.is_mutable) or 
                    (existing.is_mutable and existing.borrow_type == .mutable)) {
                    return BorrowViolation{
                        .violation_type = .conflicting_borrow,
                        .target_id = borrow_info.borrowed_from,
                        .conflicting_borrow = existing,
                        .location = borrow_info.borrow_site,
                    };
                }
            }
        }
        return null;
    }
    
    /// Validate a borrow operation against existing constraints
    fn validateBorrow(self: *Self, borrow_info: BorrowInfo) !void {
        // Check if the borrowed value exists and is accessible
        // This integrates with the lifetime analysis from borrow checker
        
        const lifetime = self.borrow_checker.lifetimes.get(borrow_info.lifetime_id) orelse {
            return EffectError.InvalidLifetime;
        };
        
        if (!lifetime.is_active) {
            return EffectError.BorrowAfterLifetimeEnd;
        }
    }
    
    /// Register borrow with the integrated borrow checker
    fn registerBorrowWithChecker(self: *Self, borrow_info: BorrowInfo) !void {
        // Get or create the borrow list for this target
        const result = try self.borrow_checker.active_borrows.getOrPut(borrow_info.borrowed_from);
        if (!result.found_existing) {
            result.value_ptr.* = ArrayList(BorrowInfo).init(self.allocator);
        }
        
        try result.value_ptr.append(borrow_info);
    }
    
    /// Check read permission against borrow constraints
    fn checkReadPermission(self: *Self, target_id: u32, location: SourceLocation) !void {
        if (self.checkBorrowConflictOnRead(target_id)) |violation| {
            std.log.err("Read permission denied at {}:{}: {}", .{ location.line, location.column, violation.violation_type });
            return EffectError.BorrowViolation;
        }
    }
    
    /// Check write permission against borrow constraints
    fn checkWritePermission(self: *Self, target_id: u32, location: SourceLocation) !void {
        if (self.checkBorrowConflictOnWrite(target_id)) |violation| {
            std.log.err("Write permission denied at {}:{}: {}", .{ location.line, location.column, violation.violation_type });
            return EffectError.BorrowViolation;
        }
    }
    
    /// Generate comprehensive effect analysis report
    pub fn generateEffectReport(self: *Self, operation_id: u32) !EffectReport {
        const analysis = try self.analyzeEffectsWithBorrowChecking(operation_id);
        
        return EffectReport{
            .operation_id = operation_id,
            .total_reads = analysis.effects.reads.items.len,
            .total_writes = analysis.effects.writes.items.len,
            .total_borrows = analysis.effects.borrows.items.len,
            .is_memory_safe = analysis.is_safe,
            .is_data_race_free = analysis.violations.items.len == 0,
            .violations = analysis.violations,
            .safety_level = if (analysis.is_safe) .safe else .unsafe,
        };
    }
};

/// Result of effect analysis with borrow checking integration
pub const EffectAnalysisResult = struct {
    effects: EffectSystem.EffectSet,
    is_safe: bool,
    violations: ArrayList(BorrowViolation),
    
    pub fn deinit(self: *EffectAnalysisResult) void {
        self.effects.deinit();
        self.violations.deinit();
    }
};

/// Borrow violation detected by integrated analysis
pub const BorrowViolation = struct {
    violation_type: ViolationType,
    target_id: u32,
    conflicting_borrow: EffectSystem.BorrowInfo,
    location: EffectSystem.SourceLocation,
    
    pub const ViolationType = enum {
        read_during_mutable_borrow,
        write_during_borrow,
        conflicting_borrow,
        use_after_free,
        double_free,
        data_race,
    };
};

/// Comprehensive effect analysis report
pub const EffectReport = struct {
    operation_id: u32,
    total_reads: usize,
    total_writes: usize,
    total_borrows: usize,
    is_memory_safe: bool,
    is_data_race_free: bool,
    violations: ArrayList(BorrowViolation),
    safety_level: SafetyLevel,
    
    pub const SafetyLevel = enum {
        safe,
        unsafe,
        unknown,
    };
    
    pub fn deinit(self: *EffectReport) void {
        self.violations.deinit();
    }
};

/// Errors that can occur during effect analysis
pub const EffectError = error{
    InvalidLifetime,
    BorrowAfterLifetimeEnd,
    BorrowViolation,
    DataRace,
    UseAfterFree,
    DoubleFree,
    OutOfMemory,
};

test "effect system borrow integration" {
    const gpa = std.testing.allocator;
    
    var borrow_checker = EffectSystem.BorrowChecker.init(gpa);
    defer borrow_checker.deinit();
    
    var effect_system = EffectSystem.init(gpa, &borrow_checker);
    defer effect_system.deinit();
    
    // Test borrow tracking
    const borrow_info = EffectSystem.BorrowInfo{
        .borrow_type = .shared,
        .lifetime_id = 1,
        .borrowed_from = 100,
        .is_mutable = false,
        .borrow_site = EffectSystem.SourceLocation{
            .file = "test.csd",
            .line = 42,
            .column = 10,
        },
    };
    
    try effect_system.trackBorrowEffect(1, borrow_info);
    
    // Test effect analysis with borrow checking
    const analysis = try effect_system.analyzeEffectsWithBorrowChecking(1);
    try std.testing.expect(analysis.is_safe);
}
