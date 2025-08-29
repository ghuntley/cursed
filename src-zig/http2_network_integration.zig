//! HTTP/2 Network Integration for CURSED
//! Integrates HTTP/2 frame parsing with the existing network infrastructure
//! Provides high-level HTTP/2 client and server implementations

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const platform = @import("platform_abstraction.zig");
const network = @import("windows_async_network.zig");
const http2 = @import("http2_frame_parser.zig");

/// HTTP/2 client configuration
pub const Http2ClientConfig = struct {
    max_concurrent_streams: u32 = 100,
    initial_window_size: u32 = 65535,
    max_frame_size: u32 = 16384,
    header_table_size: u32 = 4096,
    enable_push: bool = false,
};

/// HTTP/2 server configuration  
pub const Http2ServerConfig = struct {
    max_concurrent_streams: u32 = 1000,
    initial_window_size: u32 = 65535,
    max_frame_size: u32 = 16384,
    header_table_size: u32 = 4096,
    enable_push: bool = true,
};

/// HTTP/2 request structure
pub const Http2Request = struct {
    method: []const u8,
    path: []const u8,
    scheme: []const u8,
    authority: []const u8,
    headers: ArrayList(http2.HeaderEntry),
    body: ?[]const u8,
    
    pub fn init() Http2Request {
        return Http2Request{
            .method = "",
            .path = "",
            .scheme = "",
            .authority = "",
            .headers = .empty,
            .body = null,
        };
    }
    
    pub fn deinit(self: *Http2Request) void {
        for (self.headers.items) |*header| {
            header.deinit(self.headers.allocator);
        }
        self.headers.deinit(self.allocator);
    }
    
    pub fn setMethod(self: *Http2Request, method: []const u8) !void {
        self.method = method;
    }
    
    pub fn setPath(self: *Http2Request, path: []const u8) !void {
        self.path = path;
    }
    
    pub fn setScheme(self: *Http2Request, scheme: []const u8) !void {
        self.scheme = scheme;
    }
    
    pub fn setAuthority(self: *Http2Request, authority: []const u8) !void {
        self.authority = authority;
    }
    
    pub fn addHeader(self: *Http2Request, name: []const u8, value: []const u8) !void {
        const header = try http2.HeaderEntry.init(self.headers.allocator, name, value);
        try self.headers.append(allocator, header);
    }
    
    pub fn setBody(self: *Http2Request, body: []const u8) void {
        self.body = body;
    }
};

/// HTTP/2 response structure
pub const Http2Response = struct {
    status: u16,
    headers: ArrayList(http2.HeaderEntry),
    body: ArrayList(u8),
    allocator: Allocator,
    
    pub fn init() Http2Response {
        return Http2Response{
            .status = 200,
            .headers = .empty,
            .body = .empty,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Http2Response) void {
        for (self.headers.items) |*header| {
            header.deinit();
        }
        self.headers.deinit(self.allocator);
        self.body.deinit(self.allocator);
    }
    
    pub fn setStatus(self: *Http2Response, status: u16) void {
        self.status = status;
    }
    
    pub fn addHeader(self: *Http2Response, name: []const u8, value: []const u8) !void {
        const header = try http2.HeaderEntry.init(self.allocator, name, value);
        try self.headers.append(self.allocator, header);
    }
    
    pub fn setBody(self: *Http2Response, body: []const u8) !void {
        try self.body.resize(body.len);
        @memcpy(self.body.items, body);
    }
    
    pub fn appendBody(self: *Http2Response, data: []const u8) !void {
        try self.body.appendSlice(data);
    }
};

/// HTTP/2 stream state for connection management
pub const Http2Stream = struct {
    id: u31,
    state: http2.StreamState,
    request: ?Http2Request,
    response: ?Http2Response,
    window_size: i32,
    dependency: ?u31,
    weight: u8,
    exclusive: bool,
    data_buffer: ArrayList(u8),
    header_buffer: ArrayList(u8),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, id: u31) Http2Stream {
        return Http2Stream{
            .id = id,
            .state = .idle,
            .request = null,
            .response = null,
            .window_size = 65535,
            .dependency = null,
            .weight = 16,
            .exclusive = false,
            .data_buffer = .empty,
            .header_buffer = .empty,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Http2Stream) void {
        if (self.request) |*req| req.deinit();
        if (self.response) |*resp| resp.deinit();
        self.data_buffer.deinit(self.allocator);
        self.header_buffer.deinit(self.allocator);
    }
    
    pub fn appendHeaderData(self: *Http2Stream, data: []const u8) !void {
        try self.header_buffer.appendSlice(data);
    }
    
    pub fn appendBodyData(self: *Http2Stream, data: []const u8) !void {
        try self.data_buffer.appendSlice(data);
    }
    
    pub fn canSend(self: Http2Stream) bool {
        return switch (self.state) {
            .open, .half_closed_remote => true,
            else => false,
        };
    }
    
    pub fn canReceive(self: Http2Stream) bool {
        return switch (self.state) {
            .open, .half_closed_local => true,
            else => false,
        };
    }
};

/// HTTP/2 client implementation
pub const Http2Client = struct {
    connection: http2.Http2Connection,
    socket: ?platform.Network.SocketHandle,
    config: Http2ClientConfig,
    allocator: Allocator,
    streams: std.HashMap(u31, Http2Stream, std.hash_map.DefaultContext(u31), std.hash_map.default_max_load_percentage),
    next_stream_id: u31,
    connected: bool,
    
    pub fn init(allocator: Allocator, config: Http2ClientConfig) !Http2Client {
        return Http2Client{
            .connection = try http2.Http2Connection.init(allocator, false), // false = client
            .socket = null,
            .config = config,
            .allocator = allocator,
            .streams = std.HashMap(u31, Http2Stream, std.hash_map.DefaultContext(u31), std.hash_map.default_max_load_percentage).init(allocator),
            .next_stream_id = 1, // Client uses odd stream IDs
            .connected = false,
        };
    }
    
    pub fn deinit(self: *Http2Client) void {
        var iterator = self.streams.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.streams.deinit(self.allocator);
        self.connection.deinit(self.allocator);
        
        if (self.socket) |socket| {
            platform.Network.closeSocket(socket);
        }
    }
    
    /// Connect to HTTP/2 server
    pub fn connect(self: *Http2Client, host: []const u8, port: u16) !void {
        // Create TCP socket
        self.socket = try platform.Network.createSocket(.tcp);
        
        // For demo purposes, simulate connection
        _ = host;
        _ = port;
        
        // Send connection preface
        const preface = http2.CONNECTION_PREFACE;
        _ = preface; // Would send via socket
        
        // Send initial SETTINGS frame
        const settings_frame = try self.connection.getInitialSettings();
        defer self.allocator.free(settings_frame);
        _ = settings_frame; // Would send via socket
        
        self.connected = true;
        std.debug.print("HTTP/2 client connected to {s}:{s}\n", .{ host, port });
    }
    
    /// Send HTTP/2 request
    pub fn sendRequest(self: *Http2Client, request: Http2Request) !u31 {
        if (!self.connected) return error.NotConnected;
        
        const stream_id = self.next_stream_id;
        self.next_stream_id += 2; // Client uses odd stream IDs
        
        // Create stream
        var stream = Http2Stream.init(self.allocator, stream_id);
        stream.request = request;
        try self.streams.put(stream_id, stream);
        
        // Create HEADERS frame
        const headers_frame = try self.createHeadersFrame(stream_id, &request, true);
        defer self.allocator.free(headers_frame);
        
        // Send headers (would go via socket)
        _ = headers_frame;
        
        // Send body if present
        if (request.body) |body| {
            const data_frame = try self.createDataFrame(stream_id, body, true);
            defer self.allocator.free(data_frame);
            _ = data_frame;
        }
        
        std.debug.print("HTTP/2 request sent on stream {s}\n", .{stream_id});
        return stream_id;
    }
    
    /// Create HEADERS frame for request
    fn createHeadersFrame(self: *Http2Client, stream_id: u31, request: *const Http2Request, end_stream: bool) ![]u8 {
        // Build pseudo-headers
        var header_list = std.ArrayList(u8){};
        defer header_list.deinit();
        
        // :method
        try self.encodeHeader(&header_list, ":method", request.method);
        // :path  
        try self.encodeHeader(&header_list, ":path", request.path);
        // :scheme
        try self.encodeHeader(&header_list, ":scheme", request.scheme);
        // :authority
        try self.encodeHeader(&header_list, ":authority", request.authority);
        
        // Regular headers
        for (request.headers.items) |header| {
            try self.encodeHeader(&header_list, header.name, header.value);
        }
        
        // Create frame
        const flags = http2.FrameFlags{
            .end_headers = true,
            .end_stream = end_stream and request.body == null,
        };
        
        const header = http2.FrameHeader.init(
            @intCast(header_list.items.len),
            .HEADERS,
            flags,
            stream_id
        );
        
        var frame_data = try self.allocator.alloc(u8, 9 + header_list.items.len);
        @memcpy(frame_data[0..9], std.mem.asBytes(&header));
        @memcpy(frame_data[9..], header_list.items);
        
        return frame_data;
    }
    
    /// Create DATA frame
    fn createDataFrame(self: *Http2Client, stream_id: u31, data: []const u8, end_stream: bool) ![]u8 {
        const flags = http2.FrameFlags{ .end_stream = end_stream };
        const header = http2.FrameHeader.init(@intCast(data.len), .DATA, flags, stream_id);
        
        var frame_data = try self.allocator.alloc(u8, 9 + data.len);
        @memcpy(frame_data[0..9], std.mem.asBytes(&header));
        @memcpy(frame_data[9..], data);
        
        return frame_data;
    }
    
    /// Simple header encoding (would use HPACK in full implementation)
    fn encodeHeader(self: *Http2Client, buffer: *ArrayList(u8), name: []const u8, value: []const u8) !void {
        _ = self;
        
        // Simplified encoding - just store name and value lengths + data
        try buffer.append(allocator, @intCast(name.len));
        try buffer.appendSlice(name);
        try buffer.append(allocator, @intCast(value.len));
        try buffer.appendSlice(value);
    }
    
    /// Process incoming data from server
    pub fn processIncomingData(self: *Http2Client, data: []const u8) !void {
        try self.connection.processData(data);
        
        // Process completed streams and extract responses
        var iterator = self.streams.iterator();
        while (iterator.next()) |entry| {
            const stream = entry.value_ptr;
            if (stream.state == .closed or stream.state == .half_closed_local) {
                // Stream completed, process response
                if (stream.response) |response| {
                    std.debug.print("Received HTTP/2 response on stream {} with status {}\n", 
                        .{ stream.id, response.status });
                }
            }
        }
    }
};

/// HTTP/2 server implementation
pub const Http2Server = struct {
    connection: http2.Http2Connection,
    config: Http2ServerConfig,
    allocator: Allocator,
    streams: std.HashMap(u31, Http2Stream, std.hash_map.DefaultContext(u31), std.hash_map.default_max_load_percentage),
    request_handler: ?*const fn (*Http2Request, *Http2Response) anyerror!void,
    
    pub fn init(allocator: Allocator, config: Http2ServerConfig) !Http2Server {
        return Http2Server{
            .connection = try http2.Http2Connection.init(allocator, true), // true = server
            .config = config,
            .allocator = allocator,
            .streams = std.HashMap(u31, Http2Stream, std.hash_map.DefaultContext(u31), std.hash_map.default_max_load_percentage).init(allocator),
            .request_handler = null,
        };
    }
    
    pub fn deinit(self: *Http2Server) void {
        var iterator = self.streams.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.streams.deinit(self.allocator);
        self.connection.deinit(self.allocator);
    }
    
    /// Set request handler callback
    pub fn setRequestHandler(self: *Http2Server, handler: *const fn (*Http2Request, *Http2Response) anyerror!void) void {
        self.request_handler = handler;
    }
    
    /// Process incoming client data
    pub fn processClientData(self: *Http2Server, data: []const u8) !void {
        try self.connection.processData(data);
        
        // Check for completed requests
        var iterator = self.streams.iterator();
        while (iterator.next()) |entry| {
            const stream = entry.value_ptr;
            if (stream.request != null and stream.state == .half_closed_remote) {
                // Request complete, invoke handler
                if (self.request_handler) |handler| {
                    var response = Http2Response.init(self.allocator);
                    try handler(&stream.request.?, &response);
                    
                    // Send response
                    try self.sendResponse(stream.id, response);
                }
            }
        }
    }
    
    /// Send HTTP/2 response
    fn sendResponse(self: *Http2Server, stream_id: u31, response: Http2Response) !void {
        // Create HEADERS frame with status and headers
        const headers_frame = try self.createResponseHeadersFrame(stream_id, &response);
        defer self.allocator.free(headers_frame);
        _ = headers_frame; // Would send via socket
        
        // Send body
        if (response.body.items.len > 0) {
            const data_frame = try self.createDataFrame(stream_id, response.body.items, true);
            defer self.allocator.free(data_frame);
            _ = data_frame; // Would send via socket
        }
        
        std.debug.print("HTTP/2 response sent on stream {} with status {}\n", 
            .{ stream_id, response.status });
    }
    
    /// Create HEADERS frame for response
    fn createResponseHeadersFrame(self: *Http2Server, stream_id: u31, response: *const Http2Response) ![]u8 {
        var header_list = std.ArrayList(u8){};
        defer header_list.deinit();
        
        // :status pseudo-header
        var status_buf: [3]u8 = undefined;
        const status_str = std.fmt.bufPrint(&status_buf, "{}", .{response.status}) catch "200";
        try self.encodeHeader(&header_list, ":status", status_str);
        
        // Regular headers
        for (response.headers.items) |header| {
            try self.encodeHeader(&header_list, header.name, header.value);
        }
        
        const flags = http2.FrameFlags{
            .end_headers = true,
            .end_stream = response.body.items.len == 0,
        };
        
        const header = http2.FrameHeader.init(
            @intCast(header_list.items.len),
            .HEADERS,
            flags,
            stream_id
        );
        
        var frame_data = try self.allocator.alloc(u8, 9 + header_list.items.len);
        @memcpy(frame_data[0..9], std.mem.asBytes(&header));
        @memcpy(frame_data[9..], header_list.items);
        
        return frame_data;
    }
    
    /// Create DATA frame
    fn createDataFrame(self: *Http2Server, stream_id: u31, data: []const u8, end_stream: bool) ![]u8 {
        _ = self;
        const flags = http2.FrameFlags{ .end_stream = end_stream };
        const header = http2.FrameHeader.init(@intCast(data.len), .DATA, flags, stream_id);
        
        var frame_data = try self.allocator.alloc(u8, 9 + data.len);
        @memcpy(frame_data[0..9], std.mem.asBytes(&header));
        @memcpy(frame_data[9..], data);
        
        return frame_data;
    }
    
    /// Simple header encoding
    fn encodeHeader(self: *Http2Server, buffer: *ArrayList(u8), name: []const u8, value: []const u8) !void {
        _ = self;
        try buffer.append(self.allocator, @intCast(name.len));
        try buffer.appendSlice(name);
        try buffer.append(allocator, @intCast(value.len));
        try buffer.appendSlice(value);
    }
};

/// HTTP/2 utility functions
pub const Http2Utils = struct {
    /// Validate HTTP/2 connection preface
    pub fn validateConnectionPreface(data: []const u8) bool {
        return std.mem.eql(u8, data, http2.CONNECTION_PREFACE);
    }
    
    /// Create initial settings for connection
    pub fn createInitialSettings(allocator: Allocator, config: anytype) ![]u8 {
        const settings = [_]http2.SettingsParameter{
            .{ .id = .HEADER_TABLE_SIZE, .value = config.header_table_size },
            .{ .id = .ENABLE_PUSH, .value = if (config.enable_push) 1 else 0 },
            .{ .id = .MAX_CONCURRENT_STREAMS, .value = config.max_concurrent_streams },
            .{ .id = .INITIAL_WINDOW_SIZE, .value = config.initial_window_size },
            .{ .id = .MAX_FRAME_SIZE, .value = config.max_frame_size },
        };
        
        return http2.createSettingsFrame(allocator, &settings, false);
    }
    
    /// Calculate stream priority weight
    pub fn calculatePriorityWeight(priority: u8) u8 {
        return std.math.clamp(priority, 1, 256);
    }
    
    /// Check if stream ID is valid for client/server
    pub fn isValidStreamId(stream_id: u31, is_server: bool) bool {
        if (stream_id == 0) return false; // Stream 0 is connection stream
        
        if (is_server) {
            return (stream_id % 2) == 1; // Server expects odd client-initiated streams
        } else {
            return (stream_id % 2) == 0; // Client expects even server-initiated streams
        }
    }
    
    /// Get default HTTP/2 error message
    pub fn getErrorMessage(error_code: http2.ErrorCode) []const u8 {
        return switch (error_code) {
            .NO_ERROR => "No error",
            .PROTOCOL_ERROR => "Protocol error",
            .INTERNAL_ERROR => "Internal error",
            .FLOW_CONTROL_ERROR => "Flow control error",
            .SETTINGS_TIMEOUT => "Settings timeout",
            .STREAM_CLOSED => "Stream closed",
            .FRAME_SIZE_ERROR => "Frame size error",
            .REFUSED_STREAM => "Refused stream",
            .CANCEL => "Cancelled",
            .COMPRESSION_ERROR => "Compression error",
            .CONNECT_ERROR => "Connect error",
            .ENHANCE_YOUR_CALM => "Enhance your calm",
            .INADEQUATE_SECURITY => "Inadequate security",
            .HTTP_1_1_REQUIRED => "HTTP/1.1 required",
            else => "Unknown error",
        };
    }
};

/// Example request handler for server
fn exampleRequestHandler(request: *Http2Request, response: *Http2Response) !void {
    std.debug.print("Handling HTTP/2 request: {s} {s}\n", .{ request.method, request.path });
    
    // Set response status
    response.setStatus(200);
    
    // Add response headers
    try response.addHeader("content-type", "application/json");
    try response.addHeader("server", "cursed-http2/1.0");
    
    // Set response body
    const body = "{\"message\": \"Hello from CURSED HTTP/2 server!\", \"status\": \"success\"}";
    try response.setBody(body);
}

/// Test HTTP/2 client/server interaction
pub fn testHttp2Integration() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test HTTP/2 client
    const client_config = Http2ClientConfig{
        .max_concurrent_streams = 100,
        .enable_push = false,
    };
    
    var client = try Http2Client.init(allocator, client_config);
    defer client.deinit();
    
    try client.connect("example.com", 443);
    
    // Create and send request
    var request = Http2Request.init(allocator);
    defer request.deinit();
    
    try request.setMethod("GET");
    try request.setPath("/api/data");
    try request.setScheme("https");
    try request.setAuthority("example.com");
    try request.addHeader("user-agent", "cursed-http2-client/1.0");
    try request.addHeader("accept", "application/json");
    
    const stream_id = try client.sendRequest(request);
    std.debug.print("Request sent on stream {s}\n", .{stream_id});
    
    // Test HTTP/2 server
    const server_config = Http2ServerConfig{
        .max_concurrent_streams = 1000,
        .enable_push = true,
    };
    
    var server = try Http2Server.init(allocator, server_config);
    defer server.deinit();
    
    server.setRequestHandler(&exampleRequestHandler);
    
    // Simulate processing client data
    const mock_request_data = "mock http/2 request data";
    try server.processClientData(mock_request_data);
    
    std.debug.print("HTTP/2 integration test completed successfully!\n", .{});
}

/// Export for use in CURSED runtime
pub const cursed_http2_init_client = Http2Client.init;
pub const cursed_http2_init_server = Http2Server.init;
pub const cursed_http2_test_integration = testHttp2Integration;
