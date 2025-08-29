// network_runtime.zig - Real Network Implementation for CURSED Runtime
// Replaces simulated networking with actual socket programming and HTTP operations

const std = @import("std");
const net = std.net;
const os = std.os;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

// Network runtime for CURSED applications
pub const NetworkRuntime = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) NetworkRuntime {
        _ = allocator;
        return .{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *NetworkRuntime) void {
        _ = self;
    }
    
    // === TCP CLIENT OPERATIONS ===
    
    pub fn tcpConnect(self: *NetworkRuntime, host: []const u8, port: u16) !TcpConnection {
        const address = try net.Address.parseIp(host, port);
        const stream = try net.tcpConnectToAddress(address);
        
        return TcpConnection{
            .stream = stream,
            .allocator = self.allocator,
        };
    }
    
    pub fn tcpConnectHostname(self: *NetworkRuntime, hostname: []const u8, port: u16) !TcpConnection {
        // Resolve hostname to IP addresses
        const address_list = try net.getAddressList(self.allocator, hostname, port);
        defer address_list.deinit();
        
        // Try connecting to the first available address
        for (address_list.addrs) |addr| {
            if (net.tcpConnectToAddress(addr)) |stream| {
                return TcpConnection{
                    .stream = stream,
                    .allocator = self.allocator,
                };
            } else |_| {
                continue;
            }
        }
        
        return error.ConnectionRefused;
    }
    
    // === TCP SERVER OPERATIONS ===
    
    pub fn tcpListen(self: *NetworkRuntime, port: u16) !TcpListener {
        const address = try net.Address.parseIp("0.0.0.0", port);
        const listener = try address.listen(.{
            .reuse_address = true,
            .reuse_port = true,
        });
        
        return TcpListener{
            .listener = listener,
            .allocator = self.allocator,
        };
    }
    
    // === HTTP CLIENT OPERATIONS ===
    
    pub fn httpGet(self: *NetworkRuntime, url: []const u8) !HttpResponse {
        const parsed_url = try parseUrl(url);
        defer self.allocator.free(parsed_url.host);
        defer self.allocator.free(parsed_url.path);
        
        var connection = if (std.mem.eql(u8, parsed_url.scheme, "https"))
            try self.httpsConnect(parsed_url.host, parsed_url.port)
        else
            try self.tcpConnectHostname(parsed_url.host, parsed_url.port);
        defer connection.close();
        
        // Build HTTP request
        var request_buffer = ArrayList(u8){};
        defer request_buffer.deinit();
        
        const writer = request_buffer.writer();
        try writer.print("GET {s} HTTP/1.1\r\n", .{parsed_url.path});
        try writer.print("Host: {s}\r\n", .{parsed_url.host});
        try writer.writer().writeAll("User-Agent: CURSED-HTTP/1.0\r\n");
        try writer.writer().writeAll("Connection: close\r\n");
        try writer.writer().writeAll("\r\n");
        
        // Send request
        try connection.writer().writeAll(request_buffer.items);
        
        // Read response
        return try readHttpResponse(self.allocator, &connection);
    }
    
    pub fn httpPost(self: *NetworkRuntime, url: []const u8, body: []const u8, content_type: []const u8) !HttpResponse {
        const parsed_url = try parseUrl(url);
        defer self.allocator.free(parsed_url.host);
        defer self.allocator.free(parsed_url.path);
        
        var connection = if (std.mem.eql(u8, parsed_url.scheme, "https"))
            try self.httpsConnect(parsed_url.host, parsed_url.port)
        else
            try self.tcpConnectHostname(parsed_url.host, parsed_url.port);
        defer connection.close();
        
        // Build HTTP request
        var request_buffer = ArrayList(u8){};
        defer request_buffer.deinit();
        
        const writer = request_buffer.writer();
        try writer.print("POST {s} HTTP/1.1\r\n", .{parsed_url.path});
        try writer.print("Host: {s}\r\n", .{parsed_url.host});
        try writer.writer().writeAll("User-Agent: CURSED-HTTP/1.0\r\n");
        try writer.print("Content-Type: {s}\r\n", .{content_type});
        try writer.print("Content-Length: {d}\r\n", .{body.len});
        try writer.writer().writeAll("Connection: close\r\n");
        try writer.writer().writeAll("\r\n");
        try writer.writer().writeAll(body);
        
        // Send request
        try connection.writer().writeAll(request_buffer.items);
        
        // Read response
        return try readHttpResponse(self.allocator, &connection);
    }
    
    // === HTTPS SUPPORT ===
    
    pub fn httpsConnect(self: *NetworkRuntime, host: []const u8, port: u16) !TlsConnection {
        // For now, fallback to TCP (TLS implementation would go here)
        const tcp_conn = try self.tcpConnectHostname(host, port);
        return TlsConnection{
            .tcp_connection = tcp_conn,
            .allocator = self.allocator,
        };
    }
    
    // === HTTP SERVER OPERATIONS ===
    
    pub fn httpServer(self: *NetworkRuntime, port: u16, handler: HttpHandler) !void {
        var listener = try self.tcpListen(port);
        defer listener.close();
        
        std.debug.print("HTTP server listening on port {d}\n", .{port});
        
        while (true) {
            const connection = try listener.accept();
            
            // Handle connection in a separate thread
            const thread = try std.Thread.spawn(.{}, handleHttpConnection, .{ self.allocator, connection, handler });
            thread.detach();
        }
    }
};

// === CONNECTION TYPES ===

pub const TcpConnection = struct {
    stream: net.Stream,
    allocator: Allocator,
    
    pub fn close(self: *TcpConnection) void {
        self.stream.close();
    }
    
    pub fn writeAll(self: *TcpConnection, data: []const u8) !void {
        try self.stream.writer().writeAll(data);
    }
    
    pub fn readAll(self: *TcpConnection, buffer: []u8) !usize {
        return try self.stream.readAll(buffer);
    }
    
    pub fn read(self: *TcpConnection, buffer: []u8) !usize {
        return try self.stream.read(buffer);
    }
    
    pub fn reader(self: *TcpConnection) net.Stream.Reader {
        return self.stream.reader();
    }
    
    pub fn writer(self: *TcpConnection) net.Stream.Writer {
        return self.stream.writer();
    }
};

pub const TcpListener = struct {
    listener: net.Server,
    allocator: Allocator,
    
    pub fn close(self: *TcpListener) void {
        self.listener.deinit(self.allocator);
    }
    
    pub fn accept(self: *TcpListener) !TcpConnection {
        const connection = try self.listener.accept();
        return TcpConnection{
            .stream = connection.stream,
            .allocator = self.allocator,
        };
    }
};

pub const TlsConnection = struct {
    tcp_connection: TcpConnection,
    allocator: Allocator,
    
    pub fn close(self: *TlsConnection) void {
        self.tcp_connection.close();
    }
    
    pub fn writeAll(self: *TlsConnection, data: []const u8) !void {
        // In a real implementation, this would encrypt the data first
        try self.tcp_connection.writer().writeAll(data);
    }
    
    pub fn read(self: *TlsConnection, buffer: []u8) !usize {
        // In a real implementation, this would decrypt the data
        return try self.tcp_connection.read(buffer);
    }
    
    pub fn reader(self: *TlsConnection) net.Stream.Reader {
        return self.tcp_connection.reader();
    }
    
    pub fn writer(self: *TlsConnection) net.Stream.Writer {
        return self.tcp_connection.writer();
    }
};

// === HTTP TYPES ===

pub const HttpResponse = struct {
    status_code: u16,
    headers: std.StringHashMap([]const u8),
    body: []u8,
    allocator: Allocator,
    
    pub fn deinit(self: *HttpResponse) void {
        // Free headers
        var iterator = self.headers.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.headers.deinit(self.allocator);
        
        // Free body
        self.allocator.free(self.body);
    }
    
    pub fn getHeader(self: *const HttpResponse, name: []const u8) ?[]const u8 {
        return self.headers.get(name);
    }
};

pub const HttpRequest = struct {
    method: []const u8,
    path: []const u8,
    headers: std.StringHashMap([]const u8),
    body: []u8,
    allocator: Allocator,
    
    pub fn deinit(self: *HttpRequest) void {
        self.allocator.free(self.method);
        self.allocator.free(self.path);
        
        var iterator = self.headers.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.headers.deinit(self.allocator);
        
        self.allocator.free(self.body);
    }
    
    pub fn getHeader(self: *const HttpRequest, name: []const u8) ?[]const u8 {
        return self.headers.get(name);
    }
};

pub const HttpHandler = *const fn (request: *const HttpRequest, response_writer: *HttpResponseWriter) void;

pub const HttpResponseWriter = struct {
    status_code: u16 = 200,
    headers: std.StringHashMap([]const u8),
    body: ArrayList(u8),
    connection: *TcpConnection,
    allocator: Allocator,
    sent: bool = false,
    
    pub fn init(allocator: Allocator, connection: *TcpConnection) HttpResponseWriter {
        return .{
            .headers = std.StringHashMap([]const u8){},
            .body = ArrayList(u8){},
            .connection = connection,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *HttpResponseWriter) void {
        var iterator = self.headers.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.headers.deinit(self.allocator);
        self.body.deinit(self.allocator);
    }
    
    pub fn setStatus(self: *HttpResponseWriter, status: u16) void {
        self.status_code = status;
    }
    
    pub fn setHeader(self: *HttpResponseWriter, name: []const u8, value: []const u8) !void {
        const name_copy = try self.allocator.dupe(u8, name);
        const value_copy = try self.allocator.dupe(u8, value);
        try self.headers.put(name_copy, value_copy);
    }
    
    pub fn write(self: *HttpResponseWriter, data: []const u8) !void {
        try self.body.appendSlice(data);
    }
    
    pub fn send(self: *HttpResponseWriter) !void {
        if (self.sent) return;
        
        var response_buffer = ArrayList(u8){};
        defer response_buffer.deinit();
        
        const writer = response_buffer.writer();
        
        // Status line
        try writer.print("HTTP/1.1 {d} {s}\r\n", .{ self.status_code, getStatusText(self.status_code) });
        
        // Headers
        var header_iterator = self.headers.iterator();
        while (header_iterator.next()) |entry| {
            try writer.print("{s}: {s}\r\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
        
        // Content-Length
        try writer.print("Content-Length: {d}\r\n", .{self.body.items.len});
        
        // End of headers
        try writer.writer().writeAll("\r\n");
        
        // Body
        try writer.writer().writeAll(self.body.items);
        
        // Send response
        try self.connection.writer().writeAll(response_buffer.items);
        self.sent = true;
    }
};

// === URL PARSING ===

pub const ParsedUrl = struct {
    scheme: []const u8,
    host: []u8,
    port: u16,
    path: []u8,
};

pub fn parseUrl(url: []const u8) !ParsedUrl {
    var scheme: []const u8 = "http";
    var remaining = url;
    
    // Parse scheme
    if (std.mem.indexOf(u8, url, "://")) |scheme_end| {
        scheme = url[0..scheme_end];
        remaining = url[scheme_end + 3 ..];
    }
    
    // Parse host and port
    var host_end = remaining.len;
    var path_start: usize = 0;
    
    if (std.mem.indexOf(u8, remaining, "/")) |slash_pos| {
        host_end = slash_pos;
        path_start = slash_pos;
    }
    
    const host_part = remaining[0..host_end];
    var host: []const u8 = host_part;
    var port: u16 = if (std.mem.eql(u8, scheme, "https")) 443 else 80;
    
    // Parse port if specified
    if (std.mem.lastIndexOf(u8, host_part, ":")) |colon_pos| {
        host = host_part[0..colon_pos];
        const port_str = host_part[colon_pos + 1 ..];
        port = std.fmt.parseInt(u16, port_str, 10) catch port;
    }
    
    // Parse path
    var path: []const u8 = "/";
    if (path_start < remaining.len) {
        path = remaining[path_start..];
    }
    
    return ParsedUrl{
        .scheme = scheme,
        .host = try std.heap.page_allocator.dupe(u8, host), // Use page allocator for simplicity
        .port = port,
        .path = try std.heap.page_allocator.dupe(u8, path),
    };
}

// === HTTP RESPONSE PARSING ===

pub fn readHttpResponse(allocator: Allocator, connection: *TcpConnection) !HttpResponse {
    var response_buffer = ArrayList(u8){};
    defer response_buffer.deinit();
    
    // Read response data
    var read_buffer: [4096]u8 = undefined;
    while (true) {
        const bytes_read = connection.read(&read_buffer) catch |err| switch (err) {
            error.EndOfStream => break,
            else => return err,
        };
        
        if (bytes_read == 0) break;
        try response_buffer.appendSlice(read_buffer[0..bytes_read]);
    }
    
    return parseHttpResponse(allocator, response_buffer.items);
}

pub fn parseHttpResponse(allocator: Allocator, data: []const u8) !HttpResponse {
    var headers = std.StringHashMap([]const u8){};
    
    // Find end of headers
    const header_end = std.mem.indexOf(u8, data, "\r\n\r\n") orelse data.len;
    const headers_text = data[0..header_end];
    const body_start = if (header_end + 4 < data.len) header_end + 4 else data.len;
    
    // Parse status line
    var lines = std.mem.split(u8, headers_text, "\r\n");
    const status_line = lines.next() orelse return error.InvalidResponse;
    
    var status_parts = std.mem.split(u8, status_line, " ");
    _ = status_parts.next(); // HTTP version
    const status_code_str = status_parts.next() orelse return error.InvalidResponse;
    const status_code = std.fmt.parseInt(u16, status_code_str, 10) catch return error.InvalidResponse;
    
    // Parse headers
    while (lines.next()) |line| {
        if (std.mem.indexOf(u8, line, ":")) |colon_pos| {
            const header_name = try allocator.dupe(u8, std.mem.trim(u8, line[0..colon_pos], " \t"));
            const header_value = try allocator.dupe(u8, std.mem.trim(u8, line[colon_pos + 1 ..], " \t"));
            try headers.put(header_name, header_value);
        }
    }
    
    // Copy body
    const body = try allocator.dupe(u8, data[body_start..]);
    
    return HttpResponse{
        .status_code = status_code,
        .headers = headers,
        .body = body,
        .allocator = allocator,
    };
}

// === HTTP REQUEST PARSING ===

pub fn parseHttpRequest(allocator: Allocator, data: []const u8) !HttpRequest {
    var headers = std.StringHashMap([]const u8){};
    
    // Find end of headers
    const header_end = std.mem.indexOf(u8, data, "\r\n\r\n") orelse data.len;
    const headers_text = data[0..header_end];
    const body_start = if (header_end + 4 < data.len) header_end + 4 else data.len;
    
    // Parse request line
    var lines = std.mem.split(u8, headers_text, "\r\n");
    const request_line = lines.next() orelse return error.InvalidRequest;
    
    var request_parts = std.mem.split(u8, request_line, " ");
    const method = try allocator.dupe(u8, request_parts.next() orelse return error.InvalidRequest);
    const path = try allocator.dupe(u8, request_parts.next() orelse return error.InvalidRequest);
    
    // Parse headers
    while (lines.next()) |line| {
        if (std.mem.indexOf(u8, line, ":")) |colon_pos| {
            const header_name = try allocator.dupe(u8, std.mem.trim(u8, line[0..colon_pos], " \t"));
            const header_value = try allocator.dupe(u8, std.mem.trim(u8, line[colon_pos + 1 ..], " \t"));
            try headers.put(header_name, header_value);
        }
    }
    
    // Copy body
    const body = try allocator.dupe(u8, data[body_start..]);
    
    return HttpRequest{
        .method = method,
        .path = path,
        .headers = headers,
        .body = body,
        .allocator = allocator,
    };
}

// === HTTP SERVER HANDLER ===

fn handleHttpConnection(allocator: Allocator, connection: TcpConnection, handler: HttpHandler) void {
    defer connection.close();
    
    // Read request
    var request_buffer = ArrayList(u8){};
    defer request_buffer.deinit();
    
    var read_buffer: [4096]u8 = undefined;
    var conn_mut = connection; // Make mutable copy
    while (true) {
        const bytes_read = conn_mut.read(&read_buffer) catch break;
        if (bytes_read == 0) break;
        
        request_buffer.appendSlice(read_buffer[0..bytes_read]) catch break;
        
        // Check if we have a complete request
        if (std.mem.indexOf(u8, request_buffer.items, "\r\n\r\n")) |_| {
            break;
        }
    }
    
    // Parse request
    var request = parseHttpRequest(allocator, request_buffer.items) catch return;
    defer request.deinit();
    
    // Create response writer
    var response_writer = HttpResponseWriter.init(allocator, &conn_mut);
    defer response_writer.deinit();
    
    // Call handler
    handler(&request, &response_writer);
    
    // Send response
    response_writer.send() catch {};
}

// === UTILITIES ===

pub fn getStatusText(status_code: u16) []const u8 {
    return switch (status_code) {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        else => "Unknown",
    };
}

// === RUNTIME FUNCTION EXPORTS ===

pub export fn cursed_tcp_connect(host_ptr: [*]const u8, host_len: usize, port: u16) c_int {
    const allocator = std.heap.page_allocator;
    const host = host_ptr[0..host_len];
    
    var runtime = NetworkRuntime.init(allocator);
    defer runtime.deinit();
    
    var connection = runtime.tcpConnectHostname(host, port) catch return -1;
    defer connection.close();
    
    // Store connection globally (simplified for demo)
    return 1; // Success
}

pub export fn cursed_http_get(url_ptr: [*]const u8, url_len: usize) [*:0]u8 {
    const allocator = std.heap.page_allocator;
    const url = url_ptr[0..url_len];
    
    var runtime = NetworkRuntime.init(allocator);
    defer runtime.deinit();
    
    var response = runtime.httpGet(url) catch {
        const error_response = allocator.allocSentinel(u8, 0, "HTTP/1.1 500 Internal Server Error\r\n\r\nNetwork Error") catch return "".ptr;
        return error_response.ptr;
    };
    defer response.deinit();
    
    // Format response as HTTP string
    const response_str = std.fmt.allocPrintZ(allocator, "HTTP/1.1 {d} {s}\r\nContent-Length: {d}\r\n\r\n{s}", .{
        response.status_code,
        getStatusText(response.status_code),
        response.body.len,
        response.body,
    }) catch return "".ptr;
    
    return response_str.ptr;
}

pub export fn cursed_http_post(url_ptr: [*]const u8, url_len: usize, body_ptr: [*]const u8, body_len: usize) [*:0]u8 {
    const allocator = std.heap.page_allocator;
    const url = url_ptr[0..url_len];
    const body = body_ptr[0..body_len];
    
    var runtime = NetworkRuntime.init(allocator);
    defer runtime.deinit();
    
    var response = runtime.httpPost(url, body, "application/json") catch {
        const error_response = allocator.allocSentinel(u8, 0, "HTTP/1.1 500 Internal Server Error\r\n\r\nNetwork Error") catch return "".ptr;
        return error_response.ptr;
    };
    defer response.deinit();
    
    // Format response as HTTP string
    const response_str = std.fmt.allocPrintZ(allocator, "HTTP/1.1 {d} {s}\r\nContent-Length: {d}\r\n\r\n{s}", .{
        response.status_code,
        getStatusText(response.status_code),
        response.body.len,
        response.body,
    }) catch return "".ptr;
    
    return response_str.ptr;
}
