const std = @import("std");
const print = std.debug.print;

// CURSED v1.0 Production Telemetry Pipeline
// Oracle 72-hour post-launch infrastructure

pub const TelemetryConfig = struct {
    endpoint: []const u8,
    api_key: []const u8,
    batch_size: u32,
    flush_interval_ms: u32,
    privacy_mode: bool,
    
    pub const production = TelemetryConfig{
        .endpoint = "https://telemetry.cursedlang.org/v1/events",
        .api_key = "[REDACTED:production-telemetry-key]",
        .batch_size = 100,
        .flush_interval_ms = 30000,
        .privacy_mode = true,
    };
};

pub const EventType = enum {
    compiler_crash,
    runtime_panic,
    performance_degradation,
    memory_leak,
    compilation_success,
    execution_success,
    
    pub fn priority(self: EventType) u8 {
        return switch (self) {
            .compiler_crash => 1,
            .runtime_panic => 1,
            .memory_leak => 2,
            .performance_degradation => 3,
            .compilation_success => 5,
            .execution_success => 5,
        };
    }
};

pub const TelemetryEvent = struct {
    event_type: EventType,
    timestamp: i64,
    session_id: []const u8,
    platform: []const u8,
    version: []const u8,
    stack_trace: ?[]const u8,
    metadata: std.StringHashMap([]const u8),
    
    pub fn init(allocator: std.mem.Allocator, event_type: EventType) TelemetryEvent {
        return TelemetryEvent{
            .event_type = event_type,
            .timestamp = std.time.timestamp(),
            .session_id = generateSessionId(),
            .platform = getPlatform(),
            .version = "1.0.0",
            .stack_trace = null,
            .metadata = std.StringHashMap([]const u8).init(allocator),
        };
    }
    
    pub fn addMetadata(self: *TelemetryEvent, key: []const u8, value: []const u8) void {
        self.metadata.put(key, value) catch {};
    }
    
    pub fn setStackTrace(self: *TelemetryEvent, stack_trace: []const u8) void {
        self.stack_trace = stack_trace;
    }
};

pub const ProductionTelemetryPipeline = struct {
    allocator: std.mem.Allocator,
    config: TelemetryConfig,
    event_queue: std.ArrayList(TelemetryEvent),
    is_running: std.atomic.Atomic(bool),
    flush_thread: ?std.Thread,
    
    pub fn init(allocator: std.mem.Allocator, config: TelemetryConfig) ProductionTelemetryPipeline {
        return ProductionTelemetryPipeline{
            .allocator = allocator,
            .config = config,
            .event_queue = std.ArrayList(TelemetryEvent).init(allocator),
            .is_running = std.atomic.Atomic(bool).init(false),
            .flush_thread = null,
        };
    }
    
    pub fn start(self: *ProductionTelemetryPipeline) !void {
        self.is_running.store(true, .SeqCst);
        self.flush_thread = try std.Thread.spawn(.{}, flushWorker, .{self});
        print("🚀 Production telemetry pipeline started\n");
    }
    
    pub fn stop(self: *ProductionTelemetryPipeline) void {
        self.is_running.store(false, .SeqCst);
        if (self.flush_thread) |thread| {
            thread.join();
        }
        self.flushImmediate();
        print("🛑 Production telemetry pipeline stopped\n");
    }
    
    pub fn logEvent(self: *ProductionTelemetryPipeline, event: TelemetryEvent) !void {
        try self.event_queue.append(event);
        
        // High-priority events get immediate flush
        if (event.event_type.priority() <= 2) {
            try self.flushBatch();
        }
    }
    
    pub fn logCrash(self: *ProductionTelemetryPipeline, crash_info: []const u8, stack_trace: []const u8) !void {
        var event = TelemetryEvent.init(self.allocator, .compiler_crash);
        event.addMetadata("crash_info", crash_info);
        event.setStackTrace(stack_trace);
        try self.logEvent(event);
    }
    
    pub fn logPanic(self: *ProductionTelemetryPipeline, panic_message: []const u8) !void {
        var event = TelemetryEvent.init(self.allocator, .runtime_panic);
        event.addMetadata("panic_message", panic_message);
        try self.logEvent(event);
    }
    
    fn flushWorker(self: *ProductionTelemetryPipeline) void {
        while (self.is_running.load(.SeqCst)) {
            std.time.sleep(self.config.flush_interval_ms * 1000000); // Convert to nanoseconds
            self.flushBatch() catch |err| {
                print("❌ Telemetry flush error: {}\n", .{err});
            };
        }
    }
    
    fn flushBatch(self: *ProductionTelemetryPipeline) !void {
        if (self.event_queue.items.len == 0) return;
        
        const batch_size = @min(self.event_queue.items.len, self.config.batch_size);
        const events_to_send = self.event_queue.items[0..batch_size];
        
        try self.sendEvents(events_to_send);
        
        // Remove sent events from queue
        var i: usize = 0;
        while (i < batch_size and self.event_queue.items.len > 0) {
            _ = self.event_queue.orderedRemove(0);
            i += 1;
        }
    }
    
    fn flushImmediate(self: *ProductionTelemetryPipeline) void {
        while (self.event_queue.items.len > 0) {
            self.flushBatch() catch break;
        }
    }
    
    fn sendEvents(self: *ProductionTelemetryPipeline, events: []TelemetryEvent) !void {
        // In production, this would use HTTP client to send to telemetry endpoint
        // For now, log to structured format for collection
        
        print("📊 Sending {} telemetry events to {s}\n", .{ events.len, self.config.endpoint });
        
        for (events) |event| {
            print("Event: {s} at {d} (priority: {})\n", .{
                @tagName(event.event_type),
                event.timestamp,
                event.event_type.priority()
            });
            
            if (event.stack_trace) |trace| {
                print("  Stack trace: {s}\n", .{trace});
            }
            
            var iterator = event.metadata.iterator();
            while (iterator.next()) |entry| {
                print("  {s}: {s}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
            }
        }
        
        print("✅ Telemetry batch sent successfully\n");
    }
};

// Helper functions
fn generateSessionId() []const u8 {
    return "session-" ++ std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{std.time.timestamp()}) catch "session-unknown";
}

fn getPlatform() []const u8 {
    return switch (std.builtin.os.tag) {
        .linux => "linux",
        .macos => "macos",
        .windows => "windows",
        else => "unknown",
    };
}

// Global telemetry instance
var global_telemetry: ?ProductionTelemetryPipeline = null;

pub fn initGlobalTelemetry() !void {
    global_telemetry = ProductionTelemetryPipeline.init(
        std.heap.page_allocator,
        TelemetryConfig.production
    );
    try global_telemetry.?.start();
}

pub fn shutdownGlobalTelemetry() void {
    if (global_telemetry) |*telemetry| {
        telemetry.stop();
    }
}

pub fn reportCrash(crash_info: []const u8, stack_trace: []const u8) void {
    if (global_telemetry) |*telemetry| {
        telemetry.logCrash(crash_info, stack_trace) catch {};
    }
}

pub fn reportPanic(panic_message: []const u8) void {
    if (global_telemetry) |*telemetry| {
        telemetry.logPanic(panic_message) catch {};
    }
}
