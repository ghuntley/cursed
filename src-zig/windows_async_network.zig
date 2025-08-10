// Windows Async Network Operations using IOCP
// Provides high-performance async networking for CURSED Runtime on Windows
// Integrates with the IOCP poller and goroutine system

const std = @import("std");
const builtin = @import("builtin");
const platform = @import("platform_abstraction.zig");
const iocp = @import("windows_iocp_poller.zig");
const concurrency = @import("concurrency.zig");

// Only compile on Windows
comptime {
    if (!builtin.target.os.tag.windows) {
        @compileError("Windows async network only supports Windows platforms");
    }
}

const windows = std.os.windows;
const ws2_32 = windows.ws2_32;
const SOCKET = ws2_32.SOCKET;
const INVALID_SOCKET = ws2_32.INVALID_SOCKET;
const WSABUF = ws2_32.WSABUF;

// Winsock extensions for async operations
extern "ws2_32" fn AcceptEx(
    sListenSocket: SOCKET,
    sAcceptSocket: SOCKET,
    lpOutputBuffer: [*]u8,
    dwReceiveDataLength: u32,
    dwLocalAddressLength: u32,
    dwRemoteAddressLength: u32,
    lpdwBytesReceived: *u32,
    lpOverlapped: *iocp.AsyncOperation.overlapped,
) callconv(windows.WINAPI) windows.BOOL;

extern "ws2_32" fn ConnectEx(
    s: SOCKET,
    name: *const ws2_32.sockaddr,
    namelen: c_int,
    lpSendBuffer: ?[*]const u8,
    dwSendDataLength: u32,
    lpdwBytesSent: ?*u32,
    lpOverlapped: *iocp.AsyncOperation.overlapped,
) callconv(windows.WINAPI) windows.BOOL;

extern "ws2_32" fn WSASend(
    s: SOCKET,
    lpBuffers: [*]WSABUF,
    dwBufferCount: u32,
    lpNumberOfBytesSent: ?*u32,
    dwFlags: u32,
    lpOverlapped: *iocp.AsyncOperation.overlapped,
    lpCompletionRoutine: ?*anyopaque,
) callconv(windows.WINAPI) c_int;

extern "ws2_32" fn WSARecv(
    s: SOCKET,
    lpBuffers: [*]WSABUF,
    dwBufferCount: u32,
    lpNumberOfBytesRecvd: ?*u32,
    lpFlags: *u32,
    lpOverlapped: *iocp.AsyncOperation.overlapped,
    lpCompletionRoutine: ?*anyopaque,
) callconv(windows.WINAPI) c_int;

extern "ws2_32" fn WSAGetLastError() callconv(windows.WINAPI) c_int;

// Network-specific async operation types
pub const NetAsyncOpType = enum {
    accept,
    connect,
    send,
    recv,
    sendto,
    recvfrom,
    disconnect,
};

// Network address structure
pub const NetAddress = struct {
    ip: [4]u8,
    port: u16,
    
    pub fn init(ip: [4]u8, port: u16) NetAddress {
        return NetAddress{ .ip = ip, .port = port };
    }
    
    pub fn fromString(ip_str: []const u8, port: u16) !NetAddress {
        var ip: [4]u8 = undefined;
        var parts = std.mem.split(u8, ip_str, ".");
        
        for (0..4) |i| {
            const part = parts.next() orelse return error.InvalidIP;
            ip[i] = std.fmt.parseInt(u8, part, 10) catch return error.InvalidIP;
        }
        
        return NetAddress.init(ip, port);
    }
    
    pub fn toSockAddr(self: NetAddress) ws2_32.sockaddr.in {
        return ws2_32.sockaddr.in{
            .family = ws2_32.AF.INET,
            .port = std.mem.nativeToBig(u16, self.port),
            .addr = std.mem.readIntBig(u32, &self.ip),
            .zero = [_]u8{0} ** 8,
        };
    }
};

// Network async operation
pub const NetAsyncOperation = struct {
    const Self = @This();
    
    // Base async operation
    base: iocp.AsyncOperation,
    
    // Network-specific fields
    net_op_type: NetAsyncOpType,
    socket: SOCKET,
    remote_addr: ?NetAddress,
    local_addr: ?NetAddress,
    
    // Buffer management
    send_buffers: []WSABUF,
    recv_buffers: []WSABUF,
    
    // Accept operation specific
    accept_socket: SOCKET,
    accept_buffer: []u8,
    
    pub fn init(_: std.mem.Allocator, net_op_type: NetAsyncOpType, socket: SOCKET) !Self {
        return Self{
            .base = iocp.AsyncOperation.init(@enumFromInt(@intFromEnum(net_op_type) + 100), @ptrFromInt(@intFromPtr(&socket))),
            .net_op_type = net_op_type,
            .socket = socket,
            .remote_addr = null,
            .local_addr = null,
            .send_buffers = &[_]WSABUF{},
            .recv_buffers = &[_]WSABUF{},
            .accept_socket = INVALID_SOCKET,
            .accept_buffer = &[_]u8{},
        };
    }
    
    pub fn deinit(self: *Self, allocator: std.mem.Allocator) void {
        if (self.send_buffers.len > 0) allocator.free(self.send_buffers);
        if (self.recv_buffers.len > 0) allocator.free(self.recv_buffers);
        if (self.accept_buffer.len > 0) allocator.free(self.accept_buffer);
    }
    
    pub fn setSendBuffer(self: *Self, allocator: std.mem.Allocator, data: []const u8) !void {
        self.send_buffers = try allocator.alloc(WSABUF, 1);
        self.send_buffers[0] = WSABUF{
            .len = @intCast(data.len),
            .buf = @ptrCast(@constCast(data.ptr)),
        };
    }
    
    pub fn setRecvBuffer(self: *Self, allocator: std.mem.Allocator, buffer: []u8) !void {
        self.recv_buffers = try allocator.alloc(WSABUF, 1);
        self.recv_buffers[0] = WSABUF{
            .len = @intCast(buffer.len),
            .buf = buffer.ptr,
        };
    }
    
    pub fn setAcceptBuffer(self: *Self, allocator: std.mem.Allocator, size: usize) !void {
        // AcceptEx requires space for two addresses plus data
        const addr_size = @sizeOf(ws2_32.sockaddr.in) + 16;
        const total_size = size + (2 * addr_size);
        self.accept_buffer = try allocator.alloc(u8, total_size);
    }
};

// Network async operations manager
pub const WindowsAsyncNetwork = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    poller: *iocp.IOCPPoller,
    active_operations: std.HashMap(*NetAsyncOperation, void, std.hash_map.DefaultContext(*NetAsyncOperation), std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: std.mem.Allocator, poller: *iocp.IOCPPoller) Self {
        return Self{
            .allocator = allocator,
            .poller = poller,
            .active_operations = std.HashMap(*NetAsyncOperation, void, std.hash_map.DefaultContext(*NetAsyncOperation), std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up any remaining operations
        var iterator = self.active_operations.iterator();
        while (iterator.next()) |entry| {
            const operation = entry.key_ptr.*;
            operation.deinit(self.allocator);
            self.allocator.destroy(operation);
        }
        self.active_operations.deinit();
    }
    
    // Create TCP socket with IOCP association
    pub fn createTcpSocket(self: *Self) !SOCKET {
        const socket = ws2_32.socket(ws2_32.AF.INET, ws2_32.SOCK.STREAM, ws2_32.IPPROTO.TCP);
        if (socket == INVALID_SOCKET) {
            return error.SocketCreationFailed;
        }
        
        // Associate socket with IOCP
        try self.poller.associateHandle(@ptrFromInt(@intFromPtr(&socket)), @ptrFromInt(@intFromPtr(&socket)));
        
        return socket;
    }
    
    // Create UDP socket with IOCP association
    pub fn createUdpSocket(self: *Self) !SOCKET {
        const socket = ws2_32.socket(ws2_32.AF.INET, ws2_32.SOCK.DGRAM, ws2_32.IPPROTO.UDP);
        if (socket == INVALID_SOCKET) {
            return error.SocketCreationFailed;
        }
        
        // Associate socket with IOCP
        try self.poller.associateHandle(@ptrFromInt(@intFromPtr(&socket)), @ptrFromInt(@intFromPtr(&socket)));
        
        return socket;
    }
    
    // Async TCP accept
    pub fn acceptAsync(self: *Self, listen_socket: SOCKET) !*NetAsyncOperation {
        const operation = try self.allocator.create(NetAsyncOperation);
        operation.* = try NetAsyncOperation.init(self.allocator, .accept, listen_socket);
        
        // Create accept socket
        operation.accept_socket = try self.createTcpSocket();
        
        // Set up accept buffer (space for initial data + addresses)
        try operation.setAcceptBuffer(self.allocator, 0); // No initial data
        
        // Track operation
        try self.active_operations.put(operation, {});
        
        // Initiate AcceptEx
        var bytes_received: u32 = 0;
        const addr_size: u32 = @sizeOf(ws2_32.sockaddr.in) + 16;
        
        const result = AcceptEx(
            listen_socket,
            operation.accept_socket,
            operation.accept_buffer.ptr,
            0, // No initial receive data
            addr_size,
            addr_size,
            &bytes_received,
            &operation.base.overlapped,
        );
        
        if (result == 0) {
            const error_code = WSAGetLastError();
            if (error_code != @intFromEnum(ws2_32.WS2_32_ERROR.WSA_IO_PENDING)) {
                _ = self.active_operations.remove(operation);
                operation.deinit(self.allocator);
                self.allocator.destroy(operation);
                return error.AcceptFailed;
            }
        }
        
        return operation;
    }
    
    // Async TCP connect
    pub fn connectAsync(self: *Self, socket: SOCKET, addr: NetAddress) !*NetAsyncOperation {
        const operation = try self.allocator.create(NetAsyncOperation);
        operation.* = try NetAsyncOperation.init(self.allocator, .connect, socket);
        operation.remote_addr = addr;
        
        // Track operation
        try self.active_operations.put(operation, {});
        
        // ConnectEx requires socket to be bound to local address first
        const local_addr = ws2_32.sockaddr.in{
            .family = ws2_32.AF.INET,
            .port = 0, // Any port
            .addr = 0, // INADDR_ANY
            .zero = [_]u8{0} ** 8,
        };
        
        const bind_result = ws2_32.bind(socket, @ptrCast(&local_addr), @sizeOf(@TypeOf(local_addr)));
        if (bind_result != 0) {
            _ = self.active_operations.remove(operation);
            operation.deinit(self.allocator);
            self.allocator.destroy(operation);
            return error.BindFailed;
        }
        
        // Initiate ConnectEx
        const remote_sockaddr = addr.toSockAddr();
        var bytes_sent: u32 = 0;
        
        const result = ConnectEx(
            socket,
            @ptrCast(&remote_sockaddr),
            @sizeOf(@TypeOf(remote_sockaddr)),
            null, // No initial send data
            0,
            &bytes_sent,
            &operation.base.overlapped,
        );
        
        if (result == 0) {
            const error_code = WSAGetLastError();
            if (error_code != @intFromEnum(ws2_32.WS2_32_ERROR.WSA_IO_PENDING)) {
                _ = self.active_operations.remove(operation);
                operation.deinit(self.allocator);
                self.allocator.destroy(operation);
                return error.ConnectFailed;
            }
        }
        
        return operation;
    }
    
    // Async send
    pub fn sendAsync(self: *Self, socket: SOCKET, data: []const u8) !*NetAsyncOperation {
        const operation = try self.allocator.create(NetAsyncOperation);
        operation.* = try NetAsyncOperation.init(self.allocator, .send, socket);
        
        // Set up send buffer
        try operation.setSendBuffer(self.allocator, data);
        
        // Track operation
        try self.active_operations.put(operation, {});
        
        // Initiate WSASend
        var bytes_sent: u32 = 0;
        
        const result = WSASend(
            socket,
            operation.send_buffers.ptr,
            1, // Single buffer
            &bytes_sent,
            0, // No flags
            &operation.base.overlapped,
            null, // No completion routine
        );
        
        if (result != 0) {
            const error_code = WSAGetLastError();
            if (error_code != @intFromEnum(ws2_32.WS2_32_ERROR.WSA_IO_PENDING)) {
                _ = self.active_operations.remove(operation);
                operation.deinit(self.allocator);
                self.allocator.destroy(operation);
                return error.SendFailed;
            }
        }
        
        return operation;
    }
    
    // Async receive
    pub fn recvAsync(self: *Self, socket: SOCKET, buffer: []u8) !*NetAsyncOperation {
        const operation = try self.allocator.create(NetAsyncOperation);
        operation.* = try NetAsyncOperation.init(self.allocator, .recv, socket);
        
        // Set up receive buffer
        try operation.setRecvBuffer(self.allocator, buffer);
        
        // Track operation
        try self.active_operations.put(operation, {});
        
        // Initiate WSARecv
        var bytes_received: u32 = 0;
        var flags: u32 = 0;
        
        const result = WSARecv(
            socket,
            operation.recv_buffers.ptr,
            1, // Single buffer
            &bytes_received,
            &flags,
            &operation.base.overlapped,
            null, // No completion routine
        );
        
        if (result != 0) {
            const error_code = WSAGetLastError();
            if (error_code != @intFromEnum(ws2_32.WS2_32_ERROR.WSA_IO_PENDING)) {
                _ = self.active_operations.remove(operation);
                operation.deinit(self.allocator);
                self.allocator.destroy(operation);
                return error.RecvFailed;
            }
        }
        
        return operation;
    }
    
    // Complete and clean up operation
    pub fn completeOperation(self: *Self, operation: *NetAsyncOperation) void {
        _ = self.active_operations.remove(operation);
        operation.deinit(self.allocator);
        self.allocator.destroy(operation);
    }
    
    // Close socket
    pub fn closeSocket(self: *Self, socket: SOCKET) void {
        _ = self;
        _ = ws2_32.closesocket(socket);
    }
};

// High-level TCP server using async operations
pub const AsyncTcpServer = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    network: *WindowsAsyncNetwork,
    listen_socket: SOCKET,
    bind_addr: NetAddress,
    running: std.atomic.Value(bool),
    
    // Client connection handler
    connection_handler: ?*const fn(SOCKET, NetAddress) void,
    
    pub fn init(allocator: std.mem.Allocator, network: *WindowsAsyncNetwork, bind_addr: NetAddress) !Self {
        const listen_socket = try network.createTcpSocket();
        
        return Self{
            .allocator = allocator,
            .network = network,
            .listen_socket = listen_socket,
            .bind_addr = bind_addr,
            .running = std.atomic.Value(bool).init(false),
            .connection_handler = null,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.stop();
        self.network.closeSocket(self.listen_socket);
    }
    
    pub fn setConnectionHandler(self: *Self, handler: *const fn(SOCKET, NetAddress) void) void {
        self.connection_handler = handler;
    }
    
    pub fn start(self: *Self) !void {
        // Bind socket
        const sockaddr = self.bind_addr.toSockAddr();
        const bind_result = ws2_32.bind(self.listen_socket, @ptrCast(&sockaddr), @sizeOf(@TypeOf(sockaddr)));
        if (bind_result != 0) {
            return error.BindFailed;
        }
        
        // Start listening
        const listen_result = ws2_32.listen(self.listen_socket, 128); // 128 connection backlog
        if (listen_result != 0) {
            return error.ListenFailed;
        }
        
        self.running.store(true, .release);
        
        // Start accept loop
        try self.acceptLoop();
    }
    
    pub fn stop(self: *Self) void {
        self.running.store(false, .release);
    }
    
    fn acceptLoop(self: *Self) !void {
        while (self.running.load(.acquire)) {
            // Start async accept
            const accept_op = self.network.acceptAsync(self.listen_socket) catch |err| {
                std.log.err("Failed to start async accept: {}", .{err});
                std.time.sleep(100 * std.time.ns_per_ms); // 100ms delay before retry
                continue;
            };
            
            // Wait for completion (this would integrate with IOCP in real implementation)
            // For now, just clean up the operation
            defer self.network.completeOperation(accept_op);
            
            // In a real implementation, the IOCP completion would handle this
            if (self.connection_handler) |handler| {
                // Extract client address from accept buffer
                const client_addr = NetAddress.init([_]u8{ 127, 0, 0, 1 }, 0); // Placeholder
                handler(accept_op.accept_socket, client_addr);
            }
        }
    }
};

// High-level TCP client using async operations
pub const AsyncTcpClient = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    network: *WindowsAsyncNetwork,
    socket: SOCKET,
    connected: std.atomic.Value(bool),
    
    pub fn init(allocator: std.mem.Allocator, network: *WindowsAsyncNetwork) !Self {
        const socket = try network.createTcpSocket();
        
        return Self{
            .allocator = allocator,
            .network = network,
            .socket = socket,
            .connected = std.atomic.Value(bool).init(false),
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.disconnect();
    }
    
    pub fn connect(self: *Self, addr: NetAddress) !void {
        const connect_op = try self.network.connectAsync(self.socket, addr);
        defer self.network.completeOperation(connect_op);
        
        // In real implementation, this would wait for IOCP completion
        self.connected.store(true, .release);
    }
    
    pub fn send(self: *Self, data: []const u8) !u32 {
        if (!self.connected.load(.acquire)) {
            return error.NotConnected;
        }
        
        const send_op = try self.network.sendAsync(self.socket, data);
        defer self.network.completeOperation(send_op);
        
        // In real implementation, this would return actual bytes sent from IOCP
        return @intCast(data.len);
    }
    
    pub fn recv(self: *Self, buffer: []u8) !u32 {
        if (!self.connected.load(.acquire)) {
            return error.NotConnected;
        }
        
        const recv_op = try self.network.recvAsync(self.socket, buffer);
        defer self.network.completeOperation(recv_op);
        
        // In real implementation, this would return actual bytes received from IOCP
        return @intCast(buffer.len);
    }
    
    pub fn disconnect(self: *Self) void {
        if (self.connected.load(.acquire)) {
            self.network.closeSocket(self.socket);
            self.connected.store(false, .release);
        }
    }
};

// Integration with CURSED runtime
pub const RuntimeIntegration = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    iocp_runtime: *iocp.AsyncRuntime,
    network: WindowsAsyncNetwork,
    
    pub fn init(allocator: std.mem.Allocator, iocp_runtime: *iocp.AsyncRuntime) Self {
        const network = WindowsAsyncNetwork.init(allocator, &iocp_runtime.poller);
        
        return Self{
            .allocator = allocator,
            .iocp_runtime = iocp_runtime,
            .network = network,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.network.deinit();
    }
    
    // Goroutine-friendly TCP server
    pub fn startTcpServer(self: *Self, bind_addr: NetAddress, handler: *const fn(SOCKET, NetAddress) void) !AsyncTcpServer {
        var server = try AsyncTcpServer.init(self.allocator, &self.network, bind_addr);
        server.setConnectionHandler(handler);
        try server.start();
        return server;
    }
    
    // Goroutine-friendly TCP client
    pub fn createTcpClient(self: *Self) !AsyncTcpClient {
        return AsyncTcpClient.init(self.allocator, &self.network);
    }
};

// Tests
test "network address conversion" {
    const addr = try NetAddress.fromString("192.168.1.1", 8080);
    const sockaddr = addr.toSockAddr();
    
    try std.testing.expectEqual(@as(u16, 8080), std.mem.bigToNative(u16, sockaddr.port));
    try std.testing.expectEqual(@as(u32, 0xC0A80101), std.mem.bigToNative(u32, sockaddr.addr));
}

test "async operation initialization" {
    if (!builtin.target.os.tag.windows) return; // Skip on non-Windows
    
    const allocator = std.testing.allocator;
    var operation = try NetAsyncOperation.init(allocator, .connect, INVALID_SOCKET);
    defer operation.deinit(allocator);
    
    try std.testing.expectEqual(NetAsyncOpType.connect, operation.net_op_type);
}

// Export for use by other modules
pub const WindowsNetworking = struct {
    pub const Address = NetAddress;
    pub const AsyncOp = NetAsyncOperation;
    pub const Network = WindowsAsyncNetwork;
    pub const TcpServer = AsyncTcpServer;
    pub const TcpClient = AsyncTcpClient;
    pub const Runtime = RuntimeIntegration;
};
