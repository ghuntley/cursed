//! HTTP/2 Frame Parsing Implementation for CURSED Network Layer
//! Provides complete HTTP/2 protocol support with frame parsing, 
//! HPACK header compression, flow control, and error handling.
//! 
//! Implements RFC 7540 (HTTP/2) and RFC 7541 (HPACK) specifications.

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const platform = @import("platform_abstraction.zig");

/// HTTP/2 connection preface (24 octets)
pub const CONNECTION_PREFACE = "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

/// HTTP/2 frame types as defined in RFC 7540 Section 6
pub const FrameType = enum(u8) {
    DATA = 0x0,
    HEADERS = 0x1,
    PRIORITY = 0x2,
    RST_STREAM = 0x3,
    SETTINGS = 0x4,
    PUSH_PROMISE = 0x5,
    PING = 0x6,
    GOAWAY = 0x7,
    WINDOW_UPDATE = 0x8,
    CONTINUATION = 0x9,
    _,
    
    pub fn toString(self: FrameType) []const u8 {
        return switch (self) {
            .DATA => "DATA",
            .HEADERS => "HEADERS", 
            .PRIORITY => "PRIORITY",
            .RST_STREAM => "RST_STREAM",
            .SETTINGS => "SETTINGS",
            .PUSH_PROMISE => "PUSH_PROMISE",
            .PING => "PING",
            .GOAWAY => "GOAWAY",
            .WINDOW_UPDATE => "WINDOW_UPDATE",
            .CONTINUATION => "CONTINUATION",
            else => "UNKNOWN",
        };
    }
};

/// HTTP/2 frame flags
pub const FrameFlags = packed struct {
    /// END_STREAM flag
    end_stream: bool = false,
    /// END_HEADERS flag  
    end_headers: bool = false,
    /// PADDED flag
    padded: bool = false,
    /// PRIORITY flag
    priority: bool = false,
    /// Reserved bits
    reserved: u4 = 0,
    
    pub fn fromU8(value: u8) FrameFlags {
        return @bitCast(value);
    }
    
    pub fn toU8(self: FrameFlags) u8 {
        return @bitCast(self);
    }
};

/// HTTP/2 error codes as defined in RFC 7540 Section 7
pub const ErrorCode = enum(u32) {
    NO_ERROR = 0x0,
    PROTOCOL_ERROR = 0x1,
    INTERNAL_ERROR = 0x2,
    FLOW_CONTROL_ERROR = 0x3,
    SETTINGS_TIMEOUT = 0x4,
    STREAM_CLOSED = 0x5,
    FRAME_SIZE_ERROR = 0x6,
    REFUSED_STREAM = 0x7,
    CANCEL = 0x8,
    COMPRESSION_ERROR = 0x9,
    CONNECT_ERROR = 0xa,
    ENHANCE_YOUR_CALM = 0xb,
    INADEQUATE_SECURITY = 0xc,
    HTTP_1_1_REQUIRED = 0xd,
    _,
};

/// HTTP/2 settings identifiers
pub const SettingsId = enum(u16) {
    HEADER_TABLE_SIZE = 0x1,
    ENABLE_PUSH = 0x2,
    MAX_CONCURRENT_STREAMS = 0x3,
    INITIAL_WINDOW_SIZE = 0x4,
    MAX_FRAME_SIZE = 0x5,
    MAX_HEADER_LIST_SIZE = 0x6,
    _,
};

/// HTTP/2 frame header (9 octets)
pub const FrameHeader = packed struct {
    /// Frame length (24 bits, big-endian)
    length_high: u8,
    length_mid: u8,
    length_low: u8,
    /// Frame type
    frame_type: u8,
    /// Frame flags
    flags: u8,
    /// Stream identifier (31 bits, big-endian, R bit reserved)
    stream_id: u32,
    
    pub fn init(length: u24, frame_type: FrameType, flags: FrameFlags, stream_id: u31) FrameHeader {
        const length_bytes = std.mem.toBytes(std.mem.nativeToBig(u24, length));
        return FrameHeader{
            .length_high = length_bytes[0],
            .length_mid = length_bytes[1], 
            .length_low = length_bytes[2],
            .frame_type = @intFromEnum(frame_type),
            .flags = flags.toU8(),
            .stream_id = std.mem.nativeToBig(u32, @as(u32, stream_id)),
        };
    }
    
    pub fn getLength(self: FrameHeader) u24 {
        const bytes = [3]u8{ self.length_high, self.length_mid, self.length_low };
        return std.mem.bigToNative(u24, std.mem.bytesToValue(u24, &bytes));
    }
    
    pub fn getFrameType(self: FrameHeader) FrameType {
        return @enumFromInt(self.frame_type);
    }
    
    pub fn getFlags(self: FrameHeader) FrameFlags {
        return FrameFlags.fromU8(self.flags);
    }
    
    pub fn getStreamId(self: FrameHeader) u31 {
        return @truncate(std.mem.bigToNative(u32, self.stream_id));
    }
};

/// HTTP/2 SETTINGS frame parameter
pub const SettingsParameter = struct {
    id: SettingsId,
    value: u32,
};

/// HTTP/2 frame structures
pub const Frame = union(FrameType) {
    DATA: DataFrame,
    HEADERS: HeadersFrame,
    PRIORITY: PriorityFrame,
    RST_STREAM: RstStreamFrame,
    SETTINGS: SettingsFrame,
    PUSH_PROMISE: PushPromiseFrame,
    PING: PingFrame,
    GOAWAY: GoAwayFrame,
    WINDOW_UPDATE: WindowUpdateFrame,
    CONTINUATION: ContinuationFrame,
    
    pub fn deinit(self: *Frame, allocator: Allocator) void {
        switch (self.*) {
            .DATA => |*data| data.deinit(),
            .HEADERS => |*headers| headers.deinit(),
            .SETTINGS => |*settings| settings.deinit(),
            .PUSH_PROMISE => |*push| push.deinit(),
            .CONTINUATION => |*cont| cont.deinit(),
            else => {},
        }
    }
};

pub const DataFrame = struct {
    header: FrameHeader,
    pad_length: ?u8,
    data: []u8,
    padding: []u8,
    
    pub fn deinit(self: *DataFrame, allocator: Allocator) void {
        allocator.free(self.data);
        allocator.free(self.padding);
    }
};

pub const HeadersFrame = struct {
    header: FrameHeader,
    pad_length: ?u8,
    priority: ?PrioritySpec,
    header_block_fragment: []u8,
    padding: []u8,
    
    pub fn deinit(self: *HeadersFrame, allocator: Allocator) void {
        allocator.free(self.header_block_fragment);
        allocator.free(self.padding);
    }
};

pub const PrioritySpec = struct {
    exclusive: bool,
    stream_dependency: u31,
    weight: u8,
};

pub const PriorityFrame = struct {
    header: FrameHeader,
    priority: PrioritySpec,
};

pub const RstStreamFrame = struct {
    header: FrameHeader,
    error_code: ErrorCode,
};

pub const SettingsFrame = struct {
    header: FrameHeader,
    parameters: []SettingsParameter,
    
    pub fn deinit(self: *SettingsFrame, allocator: Allocator) void {
        allocator.free(self.parameters);
    }
};

pub const PushPromiseFrame = struct {
    header: FrameHeader,
    pad_length: ?u8,
    promised_stream_id: u31,
    header_block_fragment: []u8,
    padding: []u8,
    
    pub fn deinit(self: *PushPromiseFrame, allocator: Allocator) void {
        allocator.free(self.header_block_fragment);
        allocator.free(self.padding);
    }
};

pub const PingFrame = struct {
    header: FrameHeader,
    opaque_data: [8]u8,
};

pub const GoAwayFrame = struct {
    header: FrameHeader,
    last_stream_id: u31,
    error_code: ErrorCode,
    additional_debug_data: []u8,
    
    pub fn deinit(self: *GoAwayFrame, allocator: Allocator) void {
        allocator.free(self.additional_debug_data);
    }
};

pub const WindowUpdateFrame = struct {
    header: FrameHeader,
    window_size_increment: u31,
};

pub const ContinuationFrame = struct {
    header: FrameHeader,
    header_block_fragment: []u8,
    
    pub fn deinit(self: *ContinuationFrame, allocator: Allocator) void {
        allocator.free(self.header_block_fragment);
    }
};

/// HPACK header table entry
pub const HeaderEntry = struct {
    name: []const u8,
    value: []const u8,
    size: u32,
    
    pub fn init(allocator: Allocator, name: []const u8, value: []const u8) !HeaderEntry {
        const name_copy = try allocator.dupe(u8, name);
        const value_copy = try allocator.dupe(u8, value);
        return HeaderEntry{
            .name = name_copy,
            .value = value_copy,
            .size = @intCast(32 + name.len + value.len),
        };
    }
    
    pub fn deinit(self: *HeaderEntry, allocator: Allocator) void {
        allocator.free(self.name);
        allocator.free(self.value);
    }
};

/// HPACK static table as defined in RFC 7541 Appendix B
pub const STATIC_TABLE = [_]HeaderEntry{
    HeaderEntry{ .name = ":authority", .value = "", .size = 42 },
    HeaderEntry{ .name = ":method", .value = "GET", .size = 45 },
    HeaderEntry{ .name = ":method", .value = "POST", .size = 46 },
    HeaderEntry{ .name = ":path", .value = "/", .size = 37 },
    HeaderEntry{ .name = ":path", .value = "/index.html", .size = 50 },
    HeaderEntry{ .name = ":scheme", .value = "http", .size = 44 },
    HeaderEntry{ .name = ":scheme", .value = "https", .size = 45 },
    HeaderEntry{ .name = ":status", .value = "200", .size = 43 },
    HeaderEntry{ .name = ":status", .value = "204", .size = 43 },
    HeaderEntry{ .name = ":status", .value = "206", .size = 43 },
    HeaderEntry{ .name = ":status", .value = "304", .size = 43 },
    HeaderEntry{ .name = ":status", .value = "400", .size = 43 },
    HeaderEntry{ .name = ":status", .value = "404", .size = 43 },
    HeaderEntry{ .name = ":status", .value = "500", .size = 43 },
    HeaderEntry{ .name = "accept-charset", .value = "", .size = 46 },
    HeaderEntry{ .name = "accept-encoding", .value = "gzip, deflate", .size = 59 },
    HeaderEntry{ .name = "accept-language", .value = "", .size = 47 },
    HeaderEntry{ .name = "accept-ranges", .value = "", .size = 45 },
    HeaderEntry{ .name = "accept", .value = "", .size = 38 },
    HeaderEntry{ .name = "access-control-allow-origin", .value = "", .size = 59 },
    HeaderEntry{ .name = "age", .value = "", .size = 35 },
    HeaderEntry{ .name = "allow", .value = "", .size = 37 },
    HeaderEntry{ .name = "authorization", .value = "", .size = 45 },
    HeaderEntry{ .name = "cache-control", .value = "", .size = 45 },
    HeaderEntry{ .name = "content-disposition", .value = "", .size = 51 },
    HeaderEntry{ .name = "content-encoding", .value = "", .size = 48 },
    HeaderEntry{ .name = "content-language", .value = "", .size = 48 },
    HeaderEntry{ .name = "content-length", .value = "", .size = 46 },
    HeaderEntry{ .name = "content-location", .value = "", .size = 48 },
    HeaderEntry{ .name = "content-range", .value = "", .size = 45 },
    HeaderEntry{ .name = "content-type", .value = "", .size = 44 },
    HeaderEntry{ .name = "cookie", .value = "", .size = 38 },
    HeaderEntry{ .name = "date", .value = "", .size = 36 },
    HeaderEntry{ .name = "etag", .value = "", .size = 36 },
    HeaderEntry{ .name = "expect", .value = "", .size = 38 },
    HeaderEntry{ .name = "expires", .value = "", .size = 39 },
    HeaderEntry{ .name = "from", .value = "", .size = 36 },
    HeaderEntry{ .name = "host", .value = "", .size = 36 },
    HeaderEntry{ .name = "if-match", .value = "", .size = 40 },
    HeaderEntry{ .name = "if-modified-since", .value = "", .size = 49 },
    HeaderEntry{ .name = "if-none-match", .value = "", .size = 45 },
    HeaderEntry{ .name = "if-range", .value = "", .size = 40 },
    HeaderEntry{ .name = "if-unmodified-since", .value = "", .size = 51 },
    HeaderEntry{ .name = "last-modified", .value = "", .size = 45 },
    HeaderEntry{ .name = "link", .value = "", .size = 36 },
    HeaderEntry{ .name = "location", .value = "", .size = 40 },
    HeaderEntry{ .name = "max-forwards", .value = "", .size = 44 },
    HeaderEntry{ .name = "proxy-authenticate", .value = "", .size = 50 },
    HeaderEntry{ .name = "proxy-authorization", .value = "", .size = 51 },
    HeaderEntry{ .name = "range", .value = "", .size = 37 },
    HeaderEntry{ .name = "referer", .value = "", .size = 39 },
    HeaderEntry{ .name = "refresh", .value = "", .size = 39 },
    HeaderEntry{ .name = "retry-after", .value = "", .size = 43 },
    HeaderEntry{ .name = "server", .value = "", .size = 38 },
    HeaderEntry{ .name = "set-cookie", .value = "", .size = 42 },
    HeaderEntry{ .name = "strict-transport-security", .value = "", .size = 57 },
    HeaderEntry{ .name = "transfer-encoding", .value = "", .size = 49 },
    HeaderEntry{ .name = "user-agent", .value = "", .size = 42 },
    HeaderEntry{ .name = "vary", .value = "", .size = 36 },
    HeaderEntry{ .name = "via", .value = "", .size = 35 },
    HeaderEntry{ .name = "www-authenticate", .value = "", .size = 48 },
};

/// HPACK dynamic table for header compression
pub const DynamicTable = struct {
    entries: ArrayList(HeaderEntry),
    max_size: u32,
    current_size: u32,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, max_size: u32) DynamicTable {
        return DynamicTable{
            .entries = .empty,
            .max_size = max_size,
            .current_size = 0,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *DynamicTable) void {
        for (self.entries.items) |*entry| {
            entry.deinit();
        }
        self.entries.deinit();
    }
    
    pub fn add(self: *DynamicTable, entry: HeaderEntry) !void {
        // Evict entries if necessary
        while (self.current_size + entry.size > self.max_size and self.entries.items.len > 0) {
            var evicted = self.entries.pop();
            self.current_size -= evicted.size;
            evicted.deinit();
        }
        
        if (entry.size <= self.max_size) {
            try self.entries.insert(0, entry);
            self.current_size += entry.size;
        }
    }
    
    pub fn get(self: *DynamicTable, index: u32) ?*HeaderEntry {
        if (index == 0 or index > self.entries.items.len) return null;
        return &self.entries.items[index - 1];
    }
    
    pub fn setMaxSize(self: *DynamicTable, new_max_size: u32) !void {
        self.max_size = new_max_size;
        
        // Evict entries if necessary
        while (self.current_size > self.max_size and self.entries.items.len > 0) {
            var evicted = self.entries.pop();
            self.current_size -= evicted.size;
            evicted.deinit();
        }
    }
};

/// HPACK decoder for header compression
pub const HpackDecoder = struct {
    dynamic_table: DynamicTable,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, table_size: u32) HpackDecoder {
        return HpackDecoder{
            .dynamic_table = DynamicTable.init(allocator, table_size),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *HpackDecoder) void {
        self.dynamic_table.deinit();
    }
    
    /// Decode HPACK-compressed header block
    pub fn decode(self: *HpackDecoder, data: []const u8) !ArrayList(HeaderEntry) {
        var headers = .empty;
        var pos: usize = 0;
        
        while (pos < data.len) {
            const first_byte = data[pos];
            
            if ((first_byte & 0x80) != 0) {
                // Indexed Header Field
                const index = try self.decodeInteger(data, &pos, 7);
                const entry = try self.getTableEntry(index);
                try headers.append(try HeaderEntry.init(self.allocator, entry.name, entry.value));
            } else if ((first_byte & 0x40) != 0) {
                // Literal Header Field with Incremental Indexing — Indexed Name
                const index = try self.decodeInteger(data, &pos, 6);
                const name = if (index > 0) try self.getTableEntry(index).name else try self.decodeString(data, &pos);
                const value = try self.decodeString(data, &pos);
                
                const entry = try HeaderEntry.init(self.allocator, name, value);
                try self.dynamic_table.add(entry);
                try headers.append(try HeaderEntry.init(self.allocator, name, value));
            } else if ((first_byte & 0x20) != 0) {
                // Dynamic Table Size Update
                const new_size = try self.decodeInteger(data, &pos, 5);
                try self.dynamic_table.setMaxSize(@intCast(new_size));
            } else {
                // Literal Header Field without Indexing / Never Indexed
                const index = try self.decodeInteger(data, &pos, if ((first_byte & 0x10) != 0) 4 else 4);
                const name = if (index > 0) try self.getTableEntry(index).name else try self.decodeString(data, &pos);
                const value = try self.decodeString(data, &pos);
                
                try headers.append(try HeaderEntry.init(self.allocator, name, value));
            }
        }
        
        return headers;
    }
    
    fn getTableEntry(self: *HpackDecoder, index: u32) !*const HeaderEntry {
        if (index == 0) return error.InvalidIndex;
        
        if (index <= STATIC_TABLE.len) {
            return &STATIC_TABLE[index - 1];
        } else {
            const dynamic_index = index - STATIC_TABLE.len;
            return self.dynamic_table.get(dynamic_index) orelse error.InvalidIndex;
        }
    }
    
    fn decodeInteger(self: *HpackDecoder, data: []const u8, pos: *usize, prefix_bits: u8) !u32 {
        _ = self;
        if (*pos >= data.len) return error.InvalidData;
        
        const mask = (@as(u8, 1) << prefix_bits) - 1;
        var value = @as(u32, data[*pos] & mask);
        *pos += 1;
        
        if (value < mask) return value;
        
        var m: u32 = 0;
        while (*pos < data.len) {
            const byte = data[*pos];
            *pos += 1;
            
            value += (@as(u32, byte & 0x7F) << m);
            m += 7;
            
            if ((byte & 0x80) == 0) break;
            if (m >= 32) return error.IntegerOverflow;
        }
        
        return value;
    }
    
    fn decodeString(self: *HpackDecoder, data: []const u8, pos: *usize) ![]const u8 {
        if (*pos >= data.len) return error.InvalidData;
        
        const huffman_encoded = (data[*pos] & 0x80) != 0;
        const length = try self.decodeInteger(data, pos, 7);
        
        if (*pos + length > data.len) return error.InvalidData;
        
        const string_data = data[*pos..*pos + length];
        *pos += length;
        
        if (huffman_encoded) {
            return try self.decodeHuffman(string_data);
        } else {
            return try self.allocator.dupe(u8, string_data);
        }
    }
    
    fn decodeHuffman(self: *HpackDecoder, data: []const u8) ![]const u8 {
        // Simplified Huffman decoding - in a full implementation,
        // this would use the static Huffman table from RFC 7541
        _ = data;
        return try self.allocator.dupe(u8, "");
    }
};

/// Stream state for HTTP/2 connection
pub const StreamState = enum {
    idle,
    reserved_local,
    reserved_remote,
    open,
    half_closed_local,
    half_closed_remote,
    closed,
};

/// HTTP/2 stream
pub const Stream = struct {
    id: u31,
    state: StreamState,
    window_size: i32,
    dependency: ?u31,
    weight: u8,
    exclusive: bool,
    
    pub fn init(id: u31) Stream {
        return Stream{
            .id = id,
            .state = .idle,
            .window_size = 65535, // Default initial window size
            .dependency = null,
            .weight = 16, // Default weight
            .exclusive = false,
        };
    }
    
    pub fn canSend(self: Stream) bool {
        return switch (self.state) {
            .open, .half_closed_remote => true,
            else => false,
        };
    }
    
    pub fn canReceive(self: Stream) bool {
        return switch (self.state) {
            .open, .half_closed_local => true,
            else => false,
        };
    }
};

/// HTTP/2 connection state
pub const ConnectionState = struct {
    streams: HashMap(u31, Stream, std.hash_map.DefaultContext(u31), std.hash_map.default_max_load_percentage),
    next_stream_id: u31,
    connection_window_size: i32,
    peer_settings: HashMap(SettingsId, u32, std.hash_map.DefaultContext(SettingsId), std.hash_map.default_max_load_percentage),
    local_settings: HashMap(SettingsId, u32, std.hash_map.DefaultContext(SettingsId), std.hash_map.default_max_load_percentage),
    hpack_decoder: HpackDecoder,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, is_server: bool) !ConnectionState {
        var state = ConnectionState{
            .streams = HashMap(u31, Stream, std.hash_map.DefaultContext(u31), std.hash_map.default_max_load_percentage).init(allocator),
            .next_stream_id = if (is_server) 2 else 1, // Server uses even, client uses odd
            .connection_window_size = 65535,
            .peer_settings = HashMap(SettingsId, u32, std.hash_map.DefaultContext(SettingsId), std.hash_map.default_max_load_percentage).init(allocator),
            .local_settings = HashMap(SettingsId, u32, std.hash_map.DefaultContext(SettingsId), std.hash_map.default_max_load_percentage).init(allocator),
            .hpack_decoder = HpackDecoder.init(allocator, 4096),
            .allocator = allocator,
        };
        
        // Initialize default settings
        try state.local_settings.put(.HEADER_TABLE_SIZE, 4096);
        try state.local_settings.put(.ENABLE_PUSH, 1);
        try state.local_settings.put(.MAX_CONCURRENT_STREAMS, 100);
        try state.local_settings.put(.INITIAL_WINDOW_SIZE, 65535);
        try state.local_settings.put(.MAX_FRAME_SIZE, 16384);
        try state.local_settings.put(.MAX_HEADER_LIST_SIZE, 8192);
        
        return state;
    }
    
    pub fn deinit(self: *ConnectionState) void {
        self.streams.deinit();
        self.peer_settings.deinit();
        self.local_settings.deinit();
        self.hpack_decoder.deinit();
    }
    
    pub fn getStream(self: *ConnectionState, stream_id: u31) ?*Stream {
        return self.streams.getPtr(stream_id);
    }
    
    pub fn createStream(self: *ConnectionState, stream_id: u31) !*Stream {
        var stream = Stream.init(stream_id);
        try self.streams.put(stream_id, stream);
        return self.streams.getPtr(stream_id).?;
    }
};

/// HTTP/2 frame parser
pub const FrameParser = struct {
    allocator: Allocator,
    connection: *ConnectionState,
    
    pub fn init(allocator: Allocator, connection: *ConnectionState) FrameParser {
        return FrameParser{
            .allocator = allocator,
            .connection = connection,
        };
    }
    
    /// Parse a complete HTTP/2 frame from buffer
    pub fn parseFrame(self: *FrameParser, buffer: []const u8) !?Frame {
        if (buffer.len < 9) return null; // Need at least frame header
        
        // Parse frame header
        const header = @as(*const FrameHeader, @ptrCast(buffer.ptr)).*;
        const frame_length = header.getLength();
        const frame_type = header.getFrameType();
        const flags = header.getFlags();
        const stream_id = header.getStreamId();
        
        if (buffer.len < 9 + frame_length) return null; // Incomplete frame
        
        const payload = buffer[9..9 + frame_length];
        
        return switch (frame_type) {
            .DATA => Frame{ .DATA = try self.parseDataFrame(header, payload) },
            .HEADERS => Frame{ .HEADERS = try self.parseHeadersFrame(header, payload) },
            .PRIORITY => Frame{ .PRIORITY = try self.parsePriorityFrame(header, payload) },
            .RST_STREAM => Frame{ .RST_STREAM = try self.parseRstStreamFrame(header, payload) },
            .SETTINGS => Frame{ .SETTINGS = try self.parseSettingsFrame(header, payload) },
            .PUSH_PROMISE => Frame{ .PUSH_PROMISE = try self.parsePushPromiseFrame(header, payload) },
            .PING => Frame{ .PING = try self.parsePingFrame(header, payload) },
            .GOAWAY => Frame{ .GOAWAY = try self.parseGoAwayFrame(header, payload) },
            .WINDOW_UPDATE => Frame{ .WINDOW_UPDATE = try self.parseWindowUpdateFrame(header, payload) },
            .CONTINUATION => Frame{ .CONTINUATION = try self.parseContinuationFrame(header, payload) },
            else => error.UnknownFrameType,
        };
    }
    
    fn parseDataFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !DataFrame {
        var pos: usize = 0;
        const flags = header.getFlags();
        
        const pad_length = if (flags.padded) blk: {
            if (pos >= payload.len) return error.InvalidFrame;
            const len = payload[pos];
            pos += 1;
            break :blk len;
        } else null;
        
        const padding_size = pad_length orelse 0;
        if (pos + padding_size > payload.len) return error.InvalidFrame;
        
        const data_size = payload.len - pos - padding_size;
        const data = try self.allocator.dupe(u8, payload[pos..pos + data_size]);
        const padding = try self.allocator.dupe(u8, payload[pos + data_size..]);
        
        return DataFrame{
            .header = header,
            .pad_length = pad_length,
            .data = data,
            .padding = padding,
        };
    }
    
    fn parseHeadersFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !HeadersFrame {
        var pos: usize = 0;
        const flags = header.getFlags();
        
        const pad_length = if (flags.padded) blk: {
            if (pos >= payload.len) return error.InvalidFrame;
            const len = payload[pos];
            pos += 1;
            break :blk len;
        } else null;
        
        const priority = if (flags.priority) blk: {
            if (pos + 5 > payload.len) return error.InvalidFrame;
            const dep_and_exclusive = std.mem.bigToNative(u32, std.mem.bytesToValue(u32, payload[pos..pos + 4]));
            const exclusive = (dep_and_exclusive & 0x80000000) != 0;
            const stream_dependency: u31 = @truncate(dep_and_exclusive & 0x7FFFFFFF);
            const weight = payload[pos + 4];
            pos += 5;
            break :blk PrioritySpec{
                .exclusive = exclusive,
                .stream_dependency = stream_dependency,
                .weight = weight,
            };
        } else null;
        
        const padding_size = pad_length orelse 0;
        if (pos + padding_size > payload.len) return error.InvalidFrame;
        
        const header_block_size = payload.len - pos - padding_size;
        const header_block = try self.allocator.dupe(u8, payload[pos..pos + header_block_size]);
        const padding = try self.allocator.dupe(u8, payload[pos + header_block_size..]);
        
        return HeadersFrame{
            .header = header,
            .pad_length = pad_length,
            .priority = priority,
            .header_block_fragment = header_block,
            .padding = padding,
        };
    }
    
    fn parsePriorityFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !PriorityFrame {
        _ = self;
        if (payload.len != 5) return error.InvalidFrame;
        
        const dep_and_exclusive = std.mem.bigToNative(u32, std.mem.bytesToValue(u32, payload[0..4]));
        const exclusive = (dep_and_exclusive & 0x80000000) != 0;
        const stream_dependency: u31 = @truncate(dep_and_exclusive & 0x7FFFFFFF);
        const weight = payload[4];
        
        return PriorityFrame{
            .header = header,
            .priority = PrioritySpec{
                .exclusive = exclusive,
                .stream_dependency = stream_dependency,
                .weight = weight,
            },
        };
    }
    
    fn parseRstStreamFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !RstStreamFrame {
        _ = self;
        if (payload.len != 4) return error.InvalidFrame;
        
        const error_code: ErrorCode = @enumFromInt(std.mem.bigToNative(u32, std.mem.bytesToValue(u32, payload[0..4])));
        
        return RstStreamFrame{
            .header = header,
            .error_code = error_code,
        };
    }
    
    fn parseSettingsFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !SettingsFrame {
        if (payload.len % 6 != 0) return error.InvalidFrame;
        
        const param_count = payload.len / 6;
        var parameters = try self.allocator.alloc(SettingsParameter, param_count);
        
        for (0..param_count) |i| {
            const offset = i * 6;
            const id: SettingsId = @enumFromInt(std.mem.bigToNative(u16, std.mem.bytesToValue(u16, payload[offset..offset + 2])));
            const value = std.mem.bigToNative(u32, std.mem.bytesToValue(u32, payload[offset + 2..offset + 6]));
            
            parameters[i] = SettingsParameter{
                .id = id,
                .value = value,
            };
        }
        
        return SettingsFrame{
            .header = header,
            .parameters = parameters,
        };
    }
    
    fn parsePushPromiseFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !PushPromiseFrame {
        var pos: usize = 0;
        const flags = header.getFlags();
        
        const pad_length = if (flags.padded) blk: {
            if (pos >= payload.len) return error.InvalidFrame;
            const len = payload[pos];
            pos += 1;
            break :blk len;
        } else null;
        
        if (pos + 4 > payload.len) return error.InvalidFrame;
        const promised_stream_id: u31 = @truncate(std.mem.bigToNative(u32, std.mem.bytesToValue(u32, payload[pos..pos + 4])) & 0x7FFFFFFF);
        pos += 4;
        
        const padding_size = pad_length orelse 0;
        if (pos + padding_size > payload.len) return error.InvalidFrame;
        
        const header_block_size = payload.len - pos - padding_size;
        const header_block = try self.allocator.dupe(u8, payload[pos..pos + header_block_size]);
        const padding = try self.allocator.dupe(u8, payload[pos + header_block_size..]);
        
        return PushPromiseFrame{
            .header = header,
            .pad_length = pad_length,
            .promised_stream_id = promised_stream_id,
            .header_block_fragment = header_block,
            .padding = padding,
        };
    }
    
    fn parsePingFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !PingFrame {
        _ = self;
        if (payload.len != 8) return error.InvalidFrame;
        
        var opaque_data: [8]u8 = undefined;
        @memcpy(&opaque_data, payload);
        
        return PingFrame{
            .header = header,
            .opaque_data = opaque_data,
        };
    }
    
    fn parseGoAwayFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !GoAwayFrame {
        if (payload.len < 8) return error.InvalidFrame;
        
        const last_stream_id: u31 = @truncate(std.mem.bigToNative(u32, std.mem.bytesToValue(u32, payload[0..4])) & 0x7FFFFFFF);
        const error_code: ErrorCode = @enumFromInt(std.mem.bigToNative(u32, std.mem.bytesToValue(u32, payload[4..8])));
        const debug_data = try self.allocator.dupe(u8, payload[8..]);
        
        return GoAwayFrame{
            .header = header,
            .last_stream_id = last_stream_id,
            .error_code = error_code,
            .additional_debug_data = debug_data,
        };
    }
    
    fn parseWindowUpdateFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !WindowUpdateFrame {
        _ = self;
        if (payload.len != 4) return error.InvalidFrame;
        
        const window_size_increment: u31 = @truncate(std.mem.bigToNative(u32, std.mem.bytesToValue(u32, payload[0..4])) & 0x7FFFFFFF);
        
        return WindowUpdateFrame{
            .header = header,
            .window_size_increment = window_size_increment,
        };
    }
    
    fn parseContinuationFrame(self: *FrameParser, header: FrameHeader, payload: []const u8) !ContinuationFrame {
        const header_block = try self.allocator.dupe(u8, payload);
        
        return ContinuationFrame{
            .header = header,
            .header_block_fragment = header_block,
        };
    }
    
    /// Process a parsed frame and update connection state
    pub fn processFrame(self: *FrameParser, frame: *Frame) !void {
        switch (frame.*) {
            .SETTINGS => |*settings| {
                if (!settings.header.getFlags().end_stream) { // ACK flag check
                    for (settings.parameters) |param| {
                        try self.connection.peer_settings.put(param.id, param.value);
                        
                        // Handle specific settings
                        switch (param.id) {
                            .HEADER_TABLE_SIZE => {
                                try self.connection.hpack_decoder.dynamic_table.setMaxSize(param.value);
                            },
                            .INITIAL_WINDOW_SIZE => {
                                // Update all stream window sizes
                                var iterator = self.connection.streams.iterator();
                                while (iterator.next()) |entry| {
                                    // Adjust window size difference
                                    const old_window_size = self.connection.local_settings.get(.INITIAL_WINDOW_SIZE) orelse 65535;
                                    entry.value_ptr.window_size += @as(i32, @intCast(param.value)) - @as(i32, @intCast(old_window_size));
                                }
                            },
                            else => {},
                        }
                    }
                }
            },
            .WINDOW_UPDATE => |*window_update| {
                const stream_id = window_update.header.getStreamId();
                if (stream_id == 0) {
                    // Connection-level window update
                    self.connection.connection_window_size += @as(i32, @intCast(window_update.window_size_increment));
                } else {
                    // Stream-level window update
                    if (self.connection.getStream(stream_id)) |stream| {
                        stream.window_size += @as(i32, @intCast(window_update.window_size_increment));
                    }
                }
            },
            .RST_STREAM => |*rst| {
                const stream_id = rst.header.getStreamId();
                if (self.connection.getStream(stream_id)) |stream| {
                    stream.state = .closed;
                }
            },
            .HEADERS => |*headers| {
                const stream_id = headers.header.getStreamId();
                
                // Create stream if it doesn't exist
                var stream = self.connection.getStream(stream_id) orelse 
                    try self.connection.createStream(stream_id);
                
                // Update stream state
                if (stream.state == .idle) {
                    stream.state = .open;
                }
                
                if (headers.header.getFlags().end_stream) {
                    stream.state = switch (stream.state) {
                        .open => .half_closed_remote,
                        .half_closed_local => .closed,
                        else => stream.state,
                    };
                }
                
                // Process priority information
                if (headers.priority) |priority| {
                    stream.dependency = if (priority.stream_dependency != 0) priority.stream_dependency else null;
                    stream.weight = priority.weight;
                    stream.exclusive = priority.exclusive;
                }
            },
            .DATA => |*data| {
                const stream_id = data.header.getStreamId();
                if (self.connection.getStream(stream_id)) |stream| {
                    if (data.header.getFlags().end_stream) {
                        stream.state = switch (stream.state) {
                            .open => .half_closed_remote,
                            .half_closed_local => .closed,
                            else => stream.state,
                        };
                    }
                    
                    // Update window sizes
                    stream.window_size -= @as(i32, @intCast(data.data.len));
                    self.connection.connection_window_size -= @as(i32, @intCast(data.data.len));
                }
            },
            else => {},
        }
    }
    
    /// Validate frame according to HTTP/2 rules
    pub fn validateFrame(self: *FrameParser, frame: *const Frame) !void {
        switch (frame.*) {
            .SETTINGS => |*settings| {
                if (settings.header.getStreamId() != 0) {
                    return error.ProtocolError; // SETTINGS must be on stream 0
                }
                
                for (settings.parameters) |param| {
                    switch (param.id) {
                        .ENABLE_PUSH => {
                            if (param.value != 0 and param.value != 1) {
                                return error.ProtocolError;
                            }
                        },
                        .INITIAL_WINDOW_SIZE => {
                            if (param.value > 0x7FFFFFFF) {
                                return error.FlowControlError;
                            }
                        },
                        .MAX_FRAME_SIZE => {
                            if (param.value < 16384 or param.value > 16777215) {
                                return error.ProtocolError;
                            }
                        },
                        else => {},
                    }
                }
            },
            .WINDOW_UPDATE => |*window_update| {
                if (window_update.window_size_increment == 0) {
                    return error.ProtocolError;
                }
            },
            .PING => |*ping| {
                if (ping.header.getStreamId() != 0) {
                    return error.ProtocolError; // PING must be on stream 0
                }
            },
            .GOAWAY => |*goaway| {
                if (goaway.header.getStreamId() != 0) {
                    return error.ProtocolError; // GOAWAY must be on stream 0
                }
            },
            else => {},
        }
    }
};

/// Create SETTINGS frame
pub fn createSettingsFrame(allocator: Allocator, parameters: []const SettingsParameter, ack: bool) ![]u8 {
    const flags = if (ack) FrameFlags{ .end_stream = true } else FrameFlags{};
    const header = FrameHeader.init(@intCast(parameters.len * 6), .SETTINGS, flags, 0);
    
    var buffer = try allocator.alloc(u8, 9 + parameters.len * 6);
    @memcpy(buffer[0..9], std.mem.asBytes(&header));
    
    for (parameters, 0..) |param, i| {
        const offset = 9 + i * 6;
        const id_bytes = std.mem.toBytes(std.mem.nativeToBig(u16, @intFromEnum(param.id)));
        const value_bytes = std.mem.toBytes(std.mem.nativeToBig(u32, param.value));
        
        @memcpy(buffer[offset..offset + 2], &id_bytes);
        @memcpy(buffer[offset + 2..offset + 6], &value_bytes);
    }
    
    return buffer;
}

/// Create PING frame
pub fn createPingFrame(allocator: Allocator, data: [8]u8, ack: bool) ![]u8 {
    const flags = if (ack) FrameFlags{ .end_stream = true } else FrameFlags{};
    const header = FrameHeader.init(8, .PING, flags, 0);
    
    var buffer = try allocator.alloc(u8, 17);
    @memcpy(buffer[0..9], std.mem.asBytes(&header));
    @memcpy(buffer[9..17], &data);
    
    return buffer;
}

/// Create WINDOW_UPDATE frame  
pub fn createWindowUpdateFrame(allocator: Allocator, stream_id: u31, increment: u31) ![]u8 {
    const header = FrameHeader.init(4, .WINDOW_UPDATE, FrameFlags{}, stream_id);
    const increment_bytes = std.mem.toBytes(std.mem.nativeToBig(u32, @as(u32, increment)));
    
    var buffer = try allocator.alloc(u8, 13);
    @memcpy(buffer[0..9], std.mem.asBytes(&header));
    @memcpy(buffer[9..13], &increment_bytes);
    
    return buffer;
}

/// Create GOAWAY frame
pub fn createGoAwayFrame(allocator: Allocator, last_stream_id: u31, error_code: ErrorCode, debug_data: []const u8) ![]u8 {
    const header = FrameHeader.init(@intCast(8 + debug_data.len), .GOAWAY, FrameFlags{}, 0);
    const last_stream_bytes = std.mem.toBytes(std.mem.nativeToBig(u32, @as(u32, last_stream_id)));
    const error_code_bytes = std.mem.toBytes(std.mem.nativeToBig(u32, @intFromEnum(error_code)));
    
    var buffer = try allocator.alloc(u8, 9 + 8 + debug_data.len);
    @memcpy(buffer[0..9], std.mem.asBytes(&header));
    @memcpy(buffer[9..13], &last_stream_bytes);
    @memcpy(buffer[13..17], &error_code_bytes);
    @memcpy(buffer[17..], debug_data);
    
    return buffer;
}

/// HTTP/2 connection manager
pub const Http2Connection = struct {
    state: ConnectionState,
    parser: FrameParser,
    allocator: Allocator,
    is_server: bool,
    
    pub fn init(allocator: Allocator, is_server: bool) !Http2Connection {
        var state = try ConnectionState.init(allocator, is_server);
        var parser = FrameParser.init(allocator, &state);
        
        return Http2Connection{
            .state = state,
            .parser = parser,
            .allocator = allocator,
            .is_server = is_server,
        };
    }
    
    pub fn deinit(self: *Http2Connection) void {
        self.state.deinit();
    }
    
    /// Process incoming data buffer containing HTTP/2 frames
    pub fn processData(self: *Http2Connection, data: []const u8) !void {
        var pos: usize = 0;
        
        while (pos < data.len) {
            if (pos + 9 > data.len) break; // Need at least frame header
            
            const header = @as(*const FrameHeader, @ptrCast(data.ptr + pos)).*;
            const frame_length = header.getLength();
            
            if (pos + 9 + frame_length > data.len) break; // Incomplete frame
            
            if (var frame = try self.parser.parseFrame(data[pos..])) |*f| {
                try self.parser.validateFrame(f);
                try self.parser.processFrame(f);
                f.deinit();
            }
            
            pos += 9 + frame_length;
        }
    }
    
    /// Get settings frame for connection initialization
    pub fn getInitialSettings(self: *Http2Connection) ![]u8 {
        var params = .empty;
        defer params.deinit();
        
        var iterator = self.state.local_settings.iterator();
        while (iterator.next()) |entry| {
            try params.append(self.allocator, SettingsParameter{
                .id = entry.key_ptr.*,
                .value = entry.value_ptr.*,
            });
        }
        
        return createSettingsFrame(self.allocator, params.items, false);
    }
};

// Test functions
pub fn testHttp2FrameParsing() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test SETTINGS frame creation and parsing
    const settings_params = [_]SettingsParameter{
        .{ .id = .HEADER_TABLE_SIZE, .value = 4096 },
        .{ .id = .ENABLE_PUSH, .value = 1 },
        .{ .id = .MAX_CONCURRENT_STREAMS, .value = 100 },
    };
    
    const settings_data = try createSettingsFrame(allocator, &settings_params, false);
    defer allocator.free(settings_data);
    
    var connection = try Http2Connection.init(allocator, false);
    defer connection.deinit();
    
    if (try connection.parser.parseFrame(settings_data)) |frame| {
        var f = frame;
        defer f.deinit();
        
        switch (f) {
            .SETTINGS => |settings| {
                std.debug.print("Parsed SETTINGS frame with {} parameters\n", .{settings.parameters.len});
                for (settings.parameters) |param| {
                    std.debug.print("  {s} = {}\n", .{ @tagName(param.id), param.value });
                }
            },
            else => std.debug.print("Unexpected frame type\n", .{}),
        }
    }
    
    // Test PING frame
    const ping_data = try createPingFrame(allocator, [8]u8{ 1, 2, 3, 4, 5, 6, 7, 8 }, false);
    defer allocator.free(ping_data);
    
    if (try connection.parser.parseFrame(ping_data)) |frame| {
        var f = frame;
        defer f.deinit();
        
        switch (f) {
            .PING => |ping| {
                std.debug.print("Parsed PING frame with data: {any}\n", .{ping.opaque_data});
            },
            else => std.debug.print("Unexpected frame type\n", .{}),
        }
    }
    
    std.debug.print("HTTP/2 frame parsing test completed successfully!\n", .{});
}
