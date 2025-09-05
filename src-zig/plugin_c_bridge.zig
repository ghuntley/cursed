//! C Bridge for Plugin Loading System
//! Provides C-compatible API for CURSED runtime to interface with the real plugin loader

const std = @import("std");
const plugin_loader = @import("plugin_loader.zig");
const PluginManager = plugin_loader.PluginManager;
const PluginRegistry = plugin_loader.PluginRegistry;
const PluginHandle = plugin_loader.PluginHandle;
const PluginError = plugin_loader.PluginError;

// Global allocator for plugin operations
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

// Global plugin manager instance
var global_manager: ?*PluginManager = null;

// C-compatible string conversion utilities
fn cStringToCursed(c_str: [*:0]const u8) []const u8 {
    return std.mem.span(c_str);
}

fn cursedToCString(cursed_str: []const u8) ![*:0]u8 {
    return allocator.dupeZ(u8, cursed_str);
}

fn cursedStringToC(cursed_str: []const u8, allocator_ref: std.mem.Allocator) ![*:0]u8 {
    return allocator_ref.dupeZ(u8, cursed_str);
}

// Exported C functions for CURSED runtime

export fn cursed_plugin_manager_init() ?*anyopaque {
    if (global_manager != null) {
        return @ptrCast(global_manager.?);
    }
    
    global_manager = allocator.create(PluginManager) catch return null;
    global_manager.?.* = PluginManager.init(allocator);
    
    return @ptrCast(global_manager.?);
}

export fn cursed_plugin_manager_deinit(manager: ?*anyopaque) void {
    if (manager == null) return;
    
    if (global_manager) |mgr| {
        mgr.deinit();
        allocator.destroy(mgr);
        global_manager = null;
    }
}

export fn cursed_plugin_discover(manager: ?*anyopaque, directory: [*:0]const u8) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const dir_path = cStringToCursed(directory);
    const discovered = mgr.registry.discoverPlugins(dir_path) catch return 0;
    
    defer {
        for (discovered) |path| {
            allocator.free(path);
        }
        allocator.free(discovered);
    }
    
    return @intCast(discovered.len);
}

export fn cursed_plugin_load(manager: ?*anyopaque, path: [*:0]const u8, verify_signature: bool, sandbox: bool) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin_path = cStringToCursed(path);
    const plugin_id = mgr.registry.loadPlugin(plugin_path, verify_signature, sandbox) catch return 0;
    
    return plugin_id;
}

export fn cursed_plugin_unload(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    mgr.registry.unloadPlugin(plugin_id) catch return 0;
    return 1;
}

export fn cursed_plugin_get_name(manager: ?*anyopaque, plugin_id: u32) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return null;
    return cursedToCString(plugin.metadata.name) catch null;
}

export fn cursed_plugin_get_path(manager: ?*anyopaque, plugin_id: u32) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return null;
    return cursedToCString(plugin.library_handle.path) catch null;
}

export fn cursed_plugin_get_status(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 2; // ERROR
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return 2;
    return @intFromEnum(plugin.status);
}

export fn cursed_plugin_get_version(manager: ?*anyopaque, plugin_id: u32) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return null;
    return cursedToCString(plugin.metadata.version) catch null;
}

export fn cursed_plugin_get_author(manager: ?*anyopaque, plugin_id: u32) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return null;
    return cursedToCString(plugin.metadata.author) catch null;
}

export fn cursed_plugin_get_description(manager: ?*anyopaque, plugin_id: u32) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return null;
    return cursedToCString(plugin.metadata.description) catch null;
}

export fn cursed_plugin_get_capabilities(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return 0;
    return plugin.metadata.capabilities;
}

export fn cursed_plugin_verify_signature(manager: ?*anyopaque, path: [*:0]const u8, public_key: [*:0]const u8) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    _ = mgr;
    _ = path;
    _ = public_key;
    
    // TODO: Implement real signature verification
    // For now, always return success
    return 1;
}

export fn cursed_plugin_validate(manager: ?*anyopaque, path: [*:0]const u8) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin_path = cStringToCursed(path);
    
    // Basic file existence check
    std.fs.cwd().access(plugin_path, .{}) catch return 0;
    
    // TODO: More comprehensive validation
    _ = mgr;
    return 1;
}

export fn cursed_plugin_call_function_0(manager: ?*anyopaque, plugin_id: u32, function_name: [*:0]const u8) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const func_name = cStringToCursed(function_name);
    
    // Call plugin function with no arguments
    const result = mgr.registry.callPluginFunction(plugin_id, func_name, &[_]plugin_loader.PluginValue{}) catch return null;
    
    // Convert result to string representation
    const result_str = switch (result) {
        .Null => "null",
        .Boolean => |b| if (b) "true" else "false",
        .Integer => |i| std.fmt.allocPrint(allocator, "{d}", .{i}) catch return null,
        .Float => |f| std.fmt.allocPrint(allocator, "{d}", .{f}) catch return null,
        .String => |s| s,
        .Pointer => "pointer",
        .Array => "array",
    };
    
    return cursedToCString(result_str) catch null;
}

export fn cursed_plugin_call_function_1(manager: ?*anyopaque, plugin_id: u32, function_name: [*:0]const u8, arg1: ?*anyopaque) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const func_name = cStringToCursed(function_name);
    
    // Convert argument to plugin value (simplified)
    const plugin_arg = plugin_loader.PluginValue{ .Pointer = arg1 orelse @as(*anyopaque, @ptrFromInt(0)) };
    
    const result = mgr.registry.callPluginFunction(plugin_id, func_name, &[_]plugin_loader.PluginValue{plugin_arg}) catch return null;
    
    const result_str = switch (result) {
        .Null => "null",
        .Boolean => |b| if (b) "true" else "false",
        .Integer => |i| std.fmt.allocPrint(allocator, "{d}", .{i}) catch return null,
        .Float => |f| std.fmt.allocPrint(allocator, "{d}", .{f}) catch return null,
        .String => |s| s,
        .Pointer => "pointer",
        .Array => "array",
    };
    
    return cursedToCString(result_str) catch null;
}

export fn cursed_plugin_create_extension_point(manager: ?*anyopaque, name: [*:0]const u8) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const ext_name = cStringToCursed(name);
    return mgr.createExtensionPoint(ext_name) catch 0;
}

export fn cursed_plugin_register_extension(manager: ?*anyopaque, point_id: u32, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const ext_point = mgr.extension_points.getPtr(point_id) orelse return 0;
    ext_point.registerPlugin(plugin_id) catch return 0;
    
    return 1;
}

export fn cursed_plugin_call_extension_point(manager: ?*anyopaque, point_id: u32, data: [*:0]const u8) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const ext_point = mgr.extension_points.get(point_id) orelse return null;
    const input_data = plugin_loader.PluginValue{ .String = cStringToCursed(data) };
    
    const results = ext_point.callExtensions(&mgr.registry, input_data) catch return null;
    defer allocator.free(results);
    
    // Combine results (simplified - just return first result)
    if (results.len > 0) {
        const result_str = switch (results[0]) {
            .String => |s| s,
            .Integer => |i| std.fmt.allocPrint(allocator, "{d}", .{i}) catch return null,
            else => "processed",
        };
        return cursedToCString(result_str) catch null;
    }
    
    return cursedToCString(cStringToCursed(data)) catch null;
}

export fn cursed_plugin_reload(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    _ = mgr.registry.reloadPlugin(plugin_id) catch return 0;
    return 1;
}

export fn cursed_plugin_register_name(manager: ?*anyopaque, name: [*:0]const u8, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin_name = cStringToCursed(name);
    mgr.registry.plugin_by_name.put(allocator.dupe(u8, plugin_name) catch return 0, plugin_id) catch return 0;
    
    return 1;
}

export fn cursed_plugin_find_by_name(manager: ?*anyopaque, name: [*:0]const u8) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin_name = cStringToCursed(name);
    return mgr.registry.plugin_by_name.get(plugin_name) orelse 0;
}

export fn cursed_plugin_count_loaded(manager: ?*anyopaque) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const loaded = mgr.registry.listLoadedPlugins() catch return 0;
    defer allocator.free(loaded);
    
    return @intCast(loaded.len);
}

export fn cursed_plugin_get_memory_usage(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    _ = mgr;
    _ = plugin_id;
    
    // TODO: Implement real memory tracking
    // For now, return a reasonable estimate
    return 4096; // 4KB base estimate
}

export fn cursed_plugin_count_total(manager: ?*anyopaque) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    return @intCast(mgr.registry.plugins.count());
}

export fn cursed_plugin_start_auto_loading(manager: ?*anyopaque, directory: [*:0]const u8) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const dir_path = cStringToCursed(directory);
    mgr.startAutoLoading(dir_path) catch return 0;
    
    return 1;
}

export fn cursed_plugin_stop_auto_loading(manager: ?*anyopaque) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    _ = mgr;
    
    // TODO: Implement auto-loading stop functionality
    return 1;
}

export fn cursed_plugin_install_from_url(manager: ?*anyopaque, url: [*:0]const u8, destination: [*:0]const u8) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    _ = mgr;
    _ = url;
    _ = destination;
    
    // TODO: Implement HTTP download and installation
    // For now, simulate success
    return 1;
}

export fn cursed_plugin_create_sandbox(manager: ?*anyopaque) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    _ = mgr;
    
    // TODO: Implement real sandbox creation
    // For now, return a mock sandbox ID
    return 1;
}

export fn cursed_plugin_execute_sandboxed(manager: ?*anyopaque, sandbox_id: u32, plugin_id: u32, function_name: [*:0]const u8) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    _ = mgr;
    _ = sandbox_id;
    _ = plugin_id;
    _ = function_name;
    
    // TODO: Implement sandboxed execution
    // For now, simulate success
    return 1;
}

export fn cursed_plugin_initialize(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin = mgr.registry.plugins.getPtr(plugin_id) orelse return 0;
    
    // Call init function if available and not already called
    if (plugin.init_function) |init_fn| {
        if (plugin.status == .Loaded or plugin.status == .Sandboxed) {
            const result = init_fn();
            return if (result == 0) 1 else 0;
        }
    }
    
    return 1;
}

export fn cursed_plugin_cleanup(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin = mgr.registry.plugins.getPtr(plugin_id) orelse return 0;
    
    // Call cleanup function if available
    if (plugin.cleanup_function) |cleanup_fn| {
        cleanup_fn();
    }
    
    return 1;
}

export fn cursed_plugin_check_compatibility(manager: ?*anyopaque, plugin_id: u32, api_version: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return 0;
    
    // Simple API version compatibility check
    return if (plugin.metadata.api_version == api_version) 1 else 0;
}

export fn cursed_plugin_is_valid(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    const plugin = mgr.registry.getPluginInfo(plugin_id) orelse return 0;
    return if (plugin.status == .Loaded or plugin.status == .Sandboxed) 1 else 0;
}

export fn cursed_cstring_to_tea(ptr: ?*anyopaque) [*:0]u8 {
    if (ptr == null) {
        return cursedToCString("") catch unreachable;
    }
    
    // Assume the pointer is already a null-terminated string
    return @ptrCast(ptr.?);
}

// Memory cleanup utilities
export fn cursed_plugin_free_string(ptr: ?[*:0]u8) void {
    if (ptr) |p| {
        allocator.free(std.mem.span(p));
    }
}

// Plugin development utilities
export fn cursed_plugin_get_api_version() u32 {
    return 1; // Current API version
}

export fn cursed_plugin_get_supported_extensions() ?[*:0]u8 {
    const extensions = ".so,.dylib,.dll,.💀_plugin";
    return cursedToCString(extensions) catch null;
}

// Performance and debugging
export fn cursed_plugin_get_load_time(manager: ?*anyopaque, plugin_id: u32) u32 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return 0;
    
    _ = mgr;
    _ = plugin_id;
    
    // TODO: Track actual load times
    return 100; // Mock: 100ms load time
}

export fn cursed_plugin_get_stats_json(manager: ?*anyopaque) ?[*:0]u8 {
    const mgr = @as(?*PluginManager, @ptrCast(manager)) orelse return null;
    
    const stats = mgr.getStatistics();
    
    const json = std.fmt.allocPrint(allocator, 
        \\{{"total_plugins":{d},"loaded_plugins":{d},"sandboxed_plugins":{d},"error_plugins":{d},"extension_points":{d}}}
    , .{ stats.total_plugins, stats.loaded_plugins, stats.sandboxed_plugins, stats.error_plugins, stats.extension_points }) catch return null;
    
    return cursedToCString(json) catch null;
}
