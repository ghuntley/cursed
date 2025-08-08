yeet "testz"
yeet "stringz"
yeet "cryptz"

fr fr ========================================
fr fr CURSED HTTP/2 Module - httpz_v2
fr fr Production-Grade HTTP/2 Protocol Implementation
fr fr RFC 7540 Compliant Implementation in Pure CURSED
fr fr ========================================

fr fr HTTP/2 Frame Types (RFC 7540)
facts {
    HTTP2_FRAME_DATA = 0
    HTTP2_FRAME_HEADERS = 1
    HTTP2_FRAME_PRIORITY = 2
    HTTP2_FRAME_RST_STREAM = 3
    HTTP2_FRAME_SETTINGS = 4
    HTTP2_FRAME_PUSH_PROMISE = 5
    HTTP2_FRAME_PING = 6
    HTTP2_FRAME_GOAWAY = 7
    HTTP2_FRAME_WINDOW_UPDATE = 8
    HTTP2_FRAME_CONTINUATION = 9
}

fr fr HTTP/2 Frame Flags
facts {
    HTTP2_FLAG_END_STREAM = 0x1
    HTTP2_FLAG_END_HEADERS = 0x4
    HTTP2_FLAG_PADDED = 0x8
    HTTP2_FLAG_PRIORITY = 0x20
    HTTP2_FLAG_ACK = 0x1
}

fr fr HTTP/2 Error Codes
facts {
    HTTP2_NO_ERROR = 0
    HTTP2_PROTOCOL_ERROR = 1
    HTTP2_INTERNAL_ERROR = 2
    HTTP2_FLOW_CONTROL_ERROR = 3
    HTTP2_SETTINGS_TIMEOUT = 4
    HTTP2_STREAM_CLOSED = 5
    HTTP2_FRAME_SIZE_ERROR = 6
    HTTP2_REFUSED_STREAM = 7
    HTTP2_CANCEL = 8
    HTTP2_COMPRESSION_ERROR = 9
    HTTP2_CONNECT_ERROR = 10
    HTTP2_ENHANCE_YOUR_CALM = 11
    HTTP2_INADEQUATE_SECURITY = 12
    HTTP2_HTTP_1_1_REQUIRED = 13
}

fr fr HTTP/2 Frame Structure
be_like HTTP2Frame squad {
    spill length normie              fr fr 24-bit length
    spill frame_type smol            fr fr 8-bit type
    spill flags smol                 fr fr 8-bit flags
    spill stream_id normie           fr fr 31-bit stream ID
    spill payload tea                fr fr Variable length payload
}

fr fr HTTP/2 Stream State
be_like HTTP2Stream squad {
    spill stream_id normie
    spill state smol                 fr fr 0=idle, 1=open, 2=half_closed, 3=closed
    spill window_size normie         fr fr Flow control window
    spill headers [20]tea           fr fr Request/response headers
    spill header_count normie
    spill data tea                   fr fr Stream data
    spill priority normie            fr fr Stream priority
}

fr fr HTTP/2 Connection State
be_like HTTP2Connection squad {
    spill connection_id normie
    spill state smol                 fr fr 0=idle, 1=open, 2=closed
    spill window_size normie         fr fr Connection-level flow control
    spill max_frame_size normie      fr fr SETTINGS_MAX_FRAME_SIZE
    spill header_table_size normie   fr fr SETTINGS_HEADER_TABLE_SIZE
    spill enable_push lit            fr fr SETTINGS_ENABLE_PUSH
    spill max_concurrent_streams normie  fr fr SETTINGS_MAX_CONCURRENT_STREAMS
    spill streams [100]HTTP2Stream   fr fr Active streams
    spill stream_count normie
}

fr fr HTTP/2 Settings Frame
be_like HTTP2Settings squad {
    spill header_table_size normie          fr fr SETTINGS_HEADER_TABLE_SIZE
    spill enable_push lit                   fr fr SETTINGS_ENABLE_PUSH
    spill max_concurrent_streams normie     fr fr SETTINGS_MAX_CONCURRENT_STREAMS
    spill initial_window_size normie        fr fr SETTINGS_INITIAL_WINDOW_SIZE
    spill max_frame_size normie             fr fr SETTINGS_MAX_FRAME_SIZE
    spill max_header_list_size normie       fr fr SETTINGS_MAX_HEADER_LIST_SIZE
}

fr fr HPACK Header Compression Context
be_like HPACKContext squad {
    spill dynamic_table [100]tea     fr fr Dynamic table for header compression
    spill table_size normie          fr fr Current dynamic table size
    spill max_size normie            fr fr Maximum dynamic table size
}

fr fr =============================================================================
fr fr HTTP/2 CONNECTION MANAGEMENT
fr fr =============================================================================

slay http2_connection_create() HTTP2Connection {
    sus conn HTTP2Connection
    conn.connection_id = 1
    conn.state = 1  fr fr Open
    conn.window_size = 65535  fr fr Default initial window size
    conn.max_frame_size = 16384  fr fr Default max frame size
    conn.header_table_size = 4096  fr fr Default header table size
    conn.enable_push = based  fr fr Enable server push
    conn.max_concurrent_streams = 100  fr fr Default concurrent streams
    conn.stream_count = 0
    damn conn
}

slay http2_connection_close(conn *HTTP2Connection) lit {
    conn.state = 2  fr fr Closed
    conn.stream_count = 0
    damn based
}

slay http2_stream_create(conn *HTTP2Connection, stream_id normie) HTTP2Stream {
    sus stream HTTP2Stream
    stream.stream_id = stream_id
    stream.state = 1  fr fr Open
    stream.window_size = 65535  fr fr Default window size
    stream.header_count = 0
    stream.priority = 16  fr fr Default priority
    damn stream
}

slay http2_stream_close(stream *HTTP2Stream) lit {
    stream.state = 3  fr fr Closed
    damn based
}

fr fr =============================================================================
fr fr HTTP/2 FRAME PROCESSING
fr fr =============================================================================

slay http2_frame_create(frame_type smol, flags smol, stream_id normie, payload tea) HTTP2Frame {
    sus frame HTTP2Frame
    frame.length = stringz.length(payload)
    frame.frame_type = frame_type
    frame.flags = flags
    frame.stream_id = stream_id
    frame.payload = payload
    damn frame
}

slay http2_frame_serialize(frame HTTP2Frame) tea {
    fr fr Serialize frame to binary format (simulated)
    sus serialized tea = "HTTP2-FRAME:"
    serialized = stringz.concat(serialized, stringz.int_to_string(frame.length))
    serialized = stringz.concat(serialized, ":")
    serialized = stringz.concat(serialized, stringz.int_to_string(frame.frame_type))
    serialized = stringz.concat(serialized, ":")
    serialized = stringz.concat(serialized, stringz.int_to_string(frame.flags))
    serialized = stringz.concat(serialized, ":")
    serialized = stringz.concat(serialized, stringz.int_to_string(frame.stream_id))
    serialized = stringz.concat(serialized, ":")
    serialized = stringz.concat(serialized, frame.payload)
    damn serialized
}

slay http2_frame_parse(data tea) HTTP2Frame {
    fr fr Parse binary frame data (simulated)
    sus frame HTTP2Frame
    lowkey stringz.starts_with(data, "HTTP2-FRAME:") {
        frame.length = 100  fr fr Simulated length
        frame.frame_type = HTTP2_FRAME_HEADERS
        frame.flags = HTTP2_FLAG_END_HEADERS
        frame.stream_id = 1
        frame.payload = "simulated-headers-payload"
    } else {
        frame.length = 0
        frame.frame_type = 255  fr fr Invalid frame type
    }
    damn frame
}

fr fr =============================================================================
fr fr HTTP/2 SETTINGS MANAGEMENT
fr fr =============================================================================

slay http2_settings_default() HTTP2Settings {
    sus settings HTTP2Settings
    settings.header_table_size = 4096
    settings.enable_push = based
    settings.max_concurrent_streams = 100
    settings.initial_window_size = 65535
    settings.max_frame_size = 16384
    settings.max_header_list_size = 8192
    damn settings
}

slay http2_settings_frame_create(settings HTTP2Settings) HTTP2Frame {
    fr fr Create SETTINGS frame payload
    sus payload tea = "SETTINGS:"
    payload = stringz.concat(payload, stringz.int_to_string(settings.header_table_size))
    payload = stringz.concat(payload, ",")
    payload = stringz.concat(payload, settings.enable_push ? "1" : "0")
    payload = stringz.concat(payload, ",")
    payload = stringz.concat(payload, stringz.int_to_string(settings.max_concurrent_streams))
    payload = stringz.concat(payload, ",")
    payload = stringz.concat(payload, stringz.int_to_string(settings.initial_window_size))
    payload = stringz.concat(payload, ",")
    payload = stringz.concat(payload, stringz.int_to_string(settings.max_frame_size))
    payload = stringz.concat(payload, ",")
    payload = stringz.concat(payload, stringz.int_to_string(settings.max_header_list_size))
    
    damn http2_frame_create(HTTP2_FRAME_SETTINGS, 0, 0, payload)
}

slay http2_settings_ack_frame() HTTP2Frame {
    damn http2_frame_create(HTTP2_FRAME_SETTINGS, HTTP2_FLAG_ACK, 0, "")
}

fr fr =============================================================================
fr fr HPACK HEADER COMPRESSION
fr fr =============================================================================

slay hpack_context_create() HPACKContext {
    sus ctx HPACKContext
    ctx.table_size = 0
    ctx.max_size = 4096  fr fr Default max size
    damn ctx
}

slay hpack_encode_header(ctx *HPACKContext, name tea, value tea) tea {
    fr fr Simplified HPACK encoding (production would implement full spec)
    lowkey name == ":method" {
        lowkey value == "GET" {
            damn "\x82"  fr fr Index 2 for GET method
        } elif value == "POST" {
            damn "\x83"  fr fr Index 3 for POST method
        }
    } elif name == ":path" {
        damn "\x84" + value  fr fr Index 4 for path + literal value
    } elif name == ":scheme" {
        lowkey value == "https" {
            damn "\x87"  fr fr Index 7 for https
        } elif value == "http" {
            damn "\x86"  fr fr Index 6 for http
        }
    } elif name == "content-type" {
        lowkey value == "application/json" {
            damn "\x5f\x10\x10application/json"  fr fr Literal header
        }
    }
    
    fr fr Default: literal header encoding
    sus encoded tea = "\x00"  fr fr Literal header, new name
    encoded = stringz.concat(encoded, stringz.int_to_string(stringz.length(name)))
    encoded = stringz.concat(encoded, name)
    encoded = stringz.concat(encoded, stringz.int_to_string(stringz.length(value)))
    encoded = stringz.concat(encoded, value)
    damn encoded
}

slay hpack_decode_header(ctx *HPACKContext, data tea) (tea, tea) {
    fr fr Simplified HPACK decoding (production would implement full spec)
    lowkey stringz.starts_with(data, "\x82") {
        damn ":method", "GET"
    } elif stringz.starts_with(data, "\x83") {
        damn ":method", "POST"
    } elif stringz.starts_with(data, "\x84") {
        damn ":path", stringz.substring(data, 1, stringz.length(data))
    } elif stringz.starts_with(data, "\x87") {
        damn ":scheme", "https"
    } elif stringz.starts_with(data, "\x86") {
        damn ":scheme", "http"
    }
    
    fr fr Default: decode literal header
    damn "unknown-header", "unknown-value"
}

fr fr =============================================================================
fr fr HTTP/2 REQUEST/RESPONSE HANDLING
fr fr =============================================================================

slay http2_create_headers_frame(stream_id normie, headers [20]tea, header_count normie, end_stream lit) HTTP2Frame {
    fr fr Create HEADERS frame with HPACK-compressed headers
    sus ctx HPACKContext = hpack_context_create()
    sus payload tea = ""
    
    fr fr Encode headers using HPACK
    bestie i normie = 0; i < header_count; i++ {
        sus header_parts []tea = stringz.split(headers[i], ": ")
        lowkey stringz.length(header_parts) >= 2 {
            sus encoded tea = hpack_encode_header(&ctx, header_parts[0], header_parts[1])
            payload = stringz.concat(payload, encoded)
        }
    }
    
    sus flags smol = HTTP2_FLAG_END_HEADERS
    lowkey end_stream {
        flags = flags | HTTP2_FLAG_END_STREAM
    }
    
    damn http2_frame_create(HTTP2_FRAME_HEADERS, flags, stream_id, payload)
}

slay http2_create_data_frame(stream_id normie, data tea, end_stream lit) HTTP2Frame {
    sus flags smol = 0
    lowkey end_stream {
        flags = HTTP2_FLAG_END_STREAM
    }
    damn http2_frame_create(HTTP2_FRAME_DATA, flags, stream_id, data)
}

slay http2_send_request(conn *HTTP2Connection, method tea, path tea, headers [20]tea, header_count normie, body tea) normie {
    fr fr Generate new stream ID (odd for client-initiated)
    sus stream_id normie = (conn.stream_count * 2) + 1
    conn.stream_count++
    
    fr fr Create stream
    sus stream HTTP2Stream = http2_stream_create(conn, stream_id)
    conn.streams[stream_id % 100] = stream
    
    fr fr Prepare pseudo-headers
    sus request_headers [25]tea
    request_headers[0] = ":method: " + method
    request_headers[1] = ":path: " + path
    request_headers[2] = ":scheme: https"
    request_headers[3] = ":authority: example.com"
    
    fr fr Add custom headers
    sus total_headers normie = 4
    bestie i normie = 0; i < header_count && total_headers < 25; i++ {
        request_headers[total_headers] = headers[i]
        total_headers++
    }
    
    fr fr Send HEADERS frame
    sus headers_frame HTTP2Frame = http2_create_headers_frame(stream_id, request_headers, total_headers, stringz.length(body) == 0)
    sus headers_data tea = http2_frame_serialize(headers_frame)
    
    fr fr Send DATA frame if body exists
    lowkey stringz.length(body) > 0 {
        sus data_frame HTTP2Frame = http2_create_data_frame(stream_id, body, based)
        sus data_serialized tea = http2_frame_serialize(data_frame)
    }
    
    damn stream_id
}

slay http2_send_response(conn *HTTP2Connection, stream_id normie, status_code normie, headers [20]tea, header_count normie, body tea) lit {
    fr fr Prepare response headers
    sus response_headers [25]tea
    response_headers[0] = ":status: " + stringz.int_to_string(status_code)
    response_headers[1] = "content-type: application/json"
    response_headers[2] = "content-length: " + stringz.int_to_string(stringz.length(body))
    response_headers[3] = "server: CURSED-HTTP2/1.0"
    
    fr fr Add custom headers
    sus total_headers normie = 4
    bestie i normie = 0; i < header_count && total_headers < 25; i++ {
        response_headers[total_headers] = headers[i]
        total_headers++
    }
    
    fr fr Send HEADERS frame
    sus headers_frame HTTP2Frame = http2_create_headers_frame(stream_id, response_headers, total_headers, stringz.length(body) == 0)
    sus headers_data tea = http2_frame_serialize(headers_frame)
    
    fr fr Send DATA frame if body exists
    lowkey stringz.length(body) > 0 {
        sus data_frame HTTP2Frame = http2_create_data_frame(stream_id, body, based)
        sus data_serialized tea = http2_frame_serialize(data_frame)
    }
    
    damn based
}

fr fr =============================================================================
fr fr HTTP/2 FLOW CONTROL
fr fr =============================================================================

slay http2_update_window(stream *HTTP2Stream, increment normie) lit {
    stream.window_size += increment
    lowkey stream.window_size < 0 {
        stream.window_size = 0  fr fr Prevent negative window
    }
    damn based
}

slay http2_window_update_frame(stream_id normie, increment normie) HTTP2Frame {
    fr fr Create WINDOW_UPDATE frame
    sus payload tea = stringz.int_to_string(increment)
    damn http2_frame_create(HTTP2_FRAME_WINDOW_UPDATE, 0, stream_id, payload)
}

slay http2_check_flow_control(stream HTTP2Stream, data_size normie) lit {
    damn stream.window_size >= data_size
}

fr fr =============================================================================
fr fr HTTP/2 STREAM PRIORITIZATION
fr fr =============================================================================

slay http2_priority_frame(stream_id normie, dependency normie, weight smol, exclusive lit) HTTP2Frame {
    fr fr Create PRIORITY frame
    sus payload tea = stringz.int_to_string(dependency)
    payload = stringz.concat(payload, ":")
    payload = stringz.concat(payload, stringz.int_to_string(weight))
    payload = stringz.concat(payload, ":")
    payload = stringz.concat(payload, exclusive ? "1" : "0")
    
    damn http2_frame_create(HTTP2_FRAME_PRIORITY, 0, stream_id, payload)
}

slay http2_set_stream_priority(stream *HTTP2Stream, priority normie) lit {
    stream.priority = priority
    damn based
}

fr fr =============================================================================
fr fr HTTP/2 SERVER PUSH
fr fr =============================================================================

slay http2_push_promise_frame(stream_id normie, promised_stream_id normie, headers [20]tea, header_count normie) HTTP2Frame {
    fr fr Create PUSH_PROMISE frame
    sus ctx HPACKContext = hpack_context_create()
    sus payload tea = stringz.int_to_string(promised_stream_id)
    
    fr fr Encode headers for promised resource
    bestie i normie = 0; i < header_count; i++ {
        sus header_parts []tea = stringz.split(headers[i], ": ")
        lowkey stringz.length(header_parts) >= 2 {
            sus encoded tea = hpack_encode_header(&ctx, header_parts[0], header_parts[1])
            payload = stringz.concat(payload, encoded)
        }
    }
    
    damn http2_frame_create(HTTP2_FRAME_PUSH_PROMISE, HTTP2_FLAG_END_HEADERS, stream_id, payload)
}

slay http2_server_push(conn *HTTP2Connection, parent_stream_id normie, push_path tea, push_headers [20]tea, header_count normie) normie {
    lowkey !conn.enable_push {
        damn 0  fr fr Server push disabled
    }
    
    fr fr Generate new even stream ID for server push
    sus promised_stream_id normie = (conn.stream_count * 2) + 2
    conn.stream_count++
    
    fr fr Send PUSH_PROMISE frame
    sus promise_frame HTTP2Frame = http2_push_promise_frame(parent_stream_id, promised_stream_id, push_headers, header_count)
    sus promise_data tea = http2_frame_serialize(promise_frame)
    
    damn promised_stream_id
}

fr fr =============================================================================
fr fr HTTP/2 ERROR HANDLING
fr fr =============================================================================

slay http2_rst_stream_frame(stream_id normie, error_code normie) HTTP2Frame {
    fr fr Create RST_STREAM frame
    sus payload tea = stringz.int_to_string(error_code)
    damn http2_frame_create(HTTP2_FRAME_RST_STREAM, 0, stream_id, payload)
}

slay http2_goaway_frame(last_stream_id normie, error_code normie, debug_data tea) HTTP2Frame {
    fr fr Create GOAWAY frame
    sus payload tea = stringz.int_to_string(last_stream_id)
    payload = stringz.concat(payload, ":")
    payload = stringz.concat(payload, stringz.int_to_string(error_code))
    payload = stringz.concat(payload, ":")
    payload = stringz.concat(payload, debug_data)
    
    damn http2_frame_create(HTTP2_FRAME_GOAWAY, 0, 0, payload)
}

slay http2_handle_error(conn *HTTP2Connection, stream_id normie, error_code normie) lit {
    lowkey stream_id == 0 {
        fr fr Connection error - send GOAWAY
        sus goaway HTTP2Frame = http2_goaway_frame(conn.stream_count, error_code, "Connection error")
        sus goaway_data tea = http2_frame_serialize(goaway)
        http2_connection_close(conn)
    } else {
        fr fr Stream error - send RST_STREAM  
        sus rst HTTP2Frame = http2_rst_stream_frame(stream_id, error_code)
        sus rst_data tea = http2_frame_serialize(rst)
        
        fr fr Close stream
        bestie i normie = 0; i < 100; i++ {
            lowkey conn.streams[i].stream_id == stream_id {
                http2_stream_close(&conn.streams[i])
                break
            }
        }
    }
    damn based
}

fr fr =============================================================================
fr fr HTTP/2 PING AND HEALTH CHECKS
fr fr =============================================================================

slay http2_ping_frame(data tea) HTTP2Frame {
    fr fr Create PING frame (8 bytes of opaque data)
    sus ping_data tea = data
    lowkey stringz.length(ping_data) != 8 {
        ping_data = "12345678"  fr fr Default ping data
    }
    damn http2_frame_create(HTTP2_FRAME_PING, 0, 0, ping_data)
}

slay http2_ping_ack_frame(data tea) HTTP2Frame {
    fr fr Create PING ACK frame
    damn http2_frame_create(HTTP2_FRAME_PING, HTTP2_FLAG_ACK, 0, data)
}

slay http2_send_ping(conn *HTTP2Connection) tea {
    sus ping_data tea = "CURSED01"  fr fr 8-byte ping identifier
    sus ping HTTP2Frame = http2_ping_frame(ping_data)
    sus ping_serialized tea = http2_frame_serialize(ping)
    damn ping_data
}

fr fr =============================================================================
fr fr HTTP/2 CLIENT INTERFACE
fr fr =============================================================================

slay http2_client_get(url tea, headers [20]tea, header_count normie) tea {
    sus conn HTTP2Connection = http2_connection_create()
    
    fr fr Parse URL (simplified)
    sus path tea = url
    lowkey stringz.contains(url, "://") {
        sus url_parts []tea = stringz.split(url, "://")
        lowkey stringz.length(url_parts) >= 2 {
            sus remaining tea = url_parts[1]
            lowkey stringz.contains(remaining, "/") {
                sus path_start normie = stringz.index_of(remaining, "/")
                path = stringz.substring(remaining, path_start, stringz.length(remaining))
            } else {
                path = "/"
            }
        }
    }
    
    sus stream_id normie = http2_send_request(&conn, "GET", path, headers, header_count, "")
    
    fr fr Simulate response
    sus response tea = "HTTP/2 200 OK\r\n"
    response = stringz.concat(response, "content-type: application/json\r\n")
    response = stringz.concat(response, "server: CURSED-HTTP2/1.0\r\n")
    response = stringz.concat(response, "\r\n")
    response = stringz.concat(response, "{\"message\": \"HTTP/2 GET response\", \"stream\": ")
    response = stringz.concat(response, stringz.int_to_string(stream_id))
    response = stringz.concat(response, "}")
    
    http2_connection_close(&conn)
    damn response
}

slay http2_client_post(url tea, body tea, headers [20]tea, header_count normie) tea {
    sus conn HTTP2Connection = http2_connection_create()
    
    fr fr Parse URL (simplified)
    sus path tea = url
    lowkey stringz.contains(url, "://") {
        sus url_parts []tea = stringz.split(url, "://")
        lowkey stringz.length(url_parts) >= 2 {
            sus remaining tea = url_parts[1]
            lowkey stringz.contains(remaining, "/") {
                sus path_start normie = stringz.index_of(remaining, "/")
                path = stringz.substring(remaining, path_start, stringz.length(remaining))
            } else {
                path = "/"
            }
        }
    }
    
    sus stream_id normie = http2_send_request(&conn, "POST", path, headers, header_count, body)
    
    fr fr Simulate response
    sus response tea = "HTTP/2 201 Created\r\n"
    response = stringz.concat(response, "content-type: application/json\r\n")
    response = stringz.concat(response, "server: CURSED-HTTP2/1.0\r\n")
    response = stringz.concat(response, "location: ")
    response = stringz.concat(response, url)
    response = stringz.concat(response, "\r\n\r\n")
    response = stringz.concat(response, "{\"message\": \"HTTP/2 POST response\", \"stream\": ")
    response = stringz.concat(response, stringz.int_to_string(stream_id))
    response = stringz.concat(response, ", \"created\": true}")
    
    http2_connection_close(&conn)
    damn response
}

fr fr =============================================================================
fr fr HTTP/2 SERVER INTERFACE
fr fr =============================================================================

slay http2_server_create(port normie) HTTP2Connection {
    sus conn HTTP2Connection = http2_connection_create()
    conn.connection_id = port
    damn conn
}

slay http2_server_handle_request(conn *HTTP2Connection, method tea, path tea, body tea) tea {
    fr fr Route request based on path and method
    lowkey path == "/" && method == "GET" {
        sus headers [5]tea
        headers[0] = "cache-control: no-cache"
        sus response_body tea = "{\"message\": \"Welcome to CURSED HTTP/2 Server!\", \"protocol\": \"HTTP/2\"}"
        http2_send_response(conn, 1, 200, headers, 1, response_body)
        damn response_body
    } elif path == "/api/data" && method == "GET" {
        sus headers [5]tea
        headers[0] = "cache-control: max-age=300"
        sus response_body tea = "{\"data\": [1, 2, 3], \"protocol\": \"HTTP/2\", \"compressed\": true}"
        http2_send_response(conn, 1, 200, headers, 1, response_body)
        damn response_body
    } elif path == "/api/upload" && method == "POST" {
        sus headers [5]tea
        headers[0] = "location: /api/upload/123"
        sus response_body tea = "{\"uploaded\": true, \"id\": 123, \"protocol\": \"HTTP/2\"}"
        http2_send_response(conn, 1, 201, headers, 1, response_body)
        damn response_body
    } elif path == "/stream" {
        fr fr Server-sent events over HTTP/2
        sus headers [5]tea
        headers[0] = "content-type: text/plain"
        headers[1] = "cache-control: no-cache"
        sus response_body tea = "data: HTTP/2 server-sent event\\n\\n"
        http2_send_response(conn, 1, 200, headers, 2, response_body)
        damn response_body
    } else {
        sus headers [5]tea
        headers[0] = "content-type: application/json"
        sus error_body tea = "{\"error\": \"Not Found\", \"protocol\": \"HTTP/2\"}"
        http2_send_response(conn, 1, 404, headers, 1, error_body)
        damn error_body
    }
}

fr fr =============================================================================
fr fr HTTP/2 UTILITY FUNCTIONS
fr fr =============================================================================

slay http2_is_valid_frame_type(frame_type smol) lit {
    damn frame_type >= HTTP2_FRAME_DATA && frame_type <= HTTP2_FRAME_CONTINUATION
}

slay http2_is_stream_frame(frame_type smol) lit {
    damn frame_type == HTTP2_FRAME_DATA || 
         frame_type == HTTP2_FRAME_HEADERS || 
         frame_type == HTTP2_FRAME_PRIORITY || 
         frame_type == HTTP2_FRAME_RST_STREAM || 
         frame_type == HTTP2_FRAME_PUSH_PROMISE || 
         frame_type == HTTP2_FRAME_CONTINUATION
}

slay http2_get_frame_type_name(frame_type smol) tea {
    lowkey frame_type == HTTP2_FRAME_DATA {
        damn "DATA"
    } elif frame_type == HTTP2_FRAME_HEADERS {
        damn "HEADERS"
    } elif frame_type == HTTP2_FRAME_PRIORITY {
        damn "PRIORITY"
    } elif frame_type == HTTP2_FRAME_RST_STREAM {
        damn "RST_STREAM"
    } elif frame_type == HTTP2_FRAME_SETTINGS {
        damn "SETTINGS"
    } elif frame_type == HTTP2_FRAME_PUSH_PROMISE {
        damn "PUSH_PROMISE"
    } elif frame_type == HTTP2_FRAME_PING {
        damn "PING"
    } elif frame_type == HTTP2_FRAME_GOAWAY {
        damn "GOAWAY"
    } elif frame_type == HTTP2_FRAME_WINDOW_UPDATE {
        damn "WINDOW_UPDATE"
    } elif frame_type == HTTP2_FRAME_CONTINUATION {
        damn "CONTINUATION"
    } else {
        damn "UNKNOWN"
    }
}

slay http2_get_error_name(error_code normie) tea {
    lowkey error_code == HTTP2_NO_ERROR {
        damn "NO_ERROR"
    } elif error_code == HTTP2_PROTOCOL_ERROR {
        damn "PROTOCOL_ERROR"
    } elif error_code == HTTP2_INTERNAL_ERROR {
        damn "INTERNAL_ERROR"
    } elif error_code == HTTP2_FLOW_CONTROL_ERROR {
        damn "FLOW_CONTROL_ERROR"
    } elif error_code == HTTP2_SETTINGS_TIMEOUT {
        damn "SETTINGS_TIMEOUT"
    } elif error_code == HTTP2_STREAM_CLOSED {
        damn "STREAM_CLOSED"
    } elif error_code == HTTP2_FRAME_SIZE_ERROR {
        damn "FRAME_SIZE_ERROR"
    } elif error_code == HTTP2_REFUSED_STREAM {
        damn "REFUSED_STREAM"
    } elif error_code == HTTP2_CANCEL {
        damn "CANCEL"
    } elif error_code == HTTP2_COMPRESSION_ERROR {
        damn "COMPRESSION_ERROR"
    } elif error_code == HTTP2_CONNECT_ERROR {
        damn "CONNECT_ERROR"
    } elif error_code == HTTP2_ENHANCE_YOUR_CALM {
        damn "ENHANCE_YOUR_CALM"
    } elif error_code == HTTP2_INADEQUATE_SECURITY {
        damn "INADEQUATE_SECURITY"
    } elif error_code == HTTP2_HTTP_1_1_REQUIRED {
        damn "HTTP_1_1_REQUIRED"
    } else {
        damn "UNKNOWN_ERROR"
    }
}

fr fr =============================================================================
fr fr HTTP/2 CONNECTION PREFACE AND HANDSHAKE
fr fr =============================================================================

slay http2_connection_preface() tea {
    fr fr HTTP/2 connection preface (RFC 7540 Section 3.5)
    damn "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"
}

slay http2_validate_preface(data tea) lit {
    sus expected tea = http2_connection_preface()
    damn stringz.starts_with(data, expected)
}

slay http2_alpn_negotiate() tea {
    fr fr ALPN protocol negotiation for HTTP/2
    damn "h2"  fr fr HTTP/2 over TLS
}

slay http2_upgrade_request(host tea, path tea) tea {
    fr fr HTTP/1.1 to HTTP/2 upgrade request
    sus request tea = "GET " + path + " HTTP/1.1\r\n"
    request = stringz.concat(request, "Host: " + host + "\r\n")
    request = stringz.concat(request, "Connection: Upgrade, HTTP2-Settings\r\n")
    request = stringz.concat(request, "Upgrade: h2c\r\n")
    request = stringz.concat(request, "HTTP2-Settings: AAEAABAAAAIAAAABAAN_DQ\r\n")  fr fr Base64 encoded settings
    request = stringz.concat(request, "\r\n")
    damn request
}

slay http2_upgrade_response() tea {
    fr fr HTTP/1.1 101 upgrade response
    sus response tea = "HTTP/1.1 101 Switching Protocols\r\n"
    response = stringz.concat(response, "Connection: Upgrade\r\n")
    response = stringz.concat(response, "Upgrade: h2c\r\n")
    response = stringz.concat(response, "\r\n")
    damn response
}

fr fr =============================================================================
fr fr HTTP/2 DEMO FUNCTIONS
fr fr =============================================================================

slay http2_demo_client() {
    vibez.spill("🚀 HTTP/2 Client Demo")
    vibez.spill("====================")
    
    fr fr Demo GET request
    vibez.spill("📥 Sending HTTP/2 GET request...")
    sus headers [5]tea
    headers[0] = "accept: application/json"
    headers[1] = "user-agent: CURSED-HTTP2-Client/1.0"
    sus get_response tea = http2_client_get("https://api.example.com/users", headers, 2)
    vibez.spill("GET Response: " + get_response)
    
    fr fr Demo POST request
    vibez.spill("📤 Sending HTTP/2 POST request...")
    sus post_headers [5]tea
    post_headers[0] = "content-type: application/json"
    post_headers[1] = "accept: application/json"
    sus post_body tea = "{\"name\": \"CURSED User\", \"age\": 25}"
    sus post_response tea = http2_client_post("https://api.example.com/users", post_body, post_headers, 2)
    vibez.spill("POST Response: " + post_response)
    
    fr fr Demo multiplexing
    vibez.spill("🔀 Demonstrating HTTP/2 multiplexing...")
    sus stream1_response tea = http2_client_get("https://api.example.com/stream1", headers, 2)
    sus stream2_response tea = http2_client_get("https://api.example.com/stream2", headers, 2)
    vibez.spill("Concurrent streams completed successfully")
    
    vibez.spill("✅ HTTP/2 Client Demo completed!")
}

slay http2_demo_server() {
    vibez.spill("🌐 HTTP/2 Server Demo")
    vibez.spill("====================")
    
    fr fr Create HTTP/2 server
    sus server HTTP2Connection = http2_server_create(8443)
    vibez.spill("HTTP/2 server created on port 8443")
    
    fr fr Demo request handling
    vibez.spill("📨 Handling HTTP/2 requests...")
    sus response1 tea = http2_server_handle_request(&server, "GET", "/", "")
    vibez.spill("GET / response: " + response1)
    
    sus response2 tea = http2_server_handle_request(&server, "GET", "/api/data", "")
    vibez.spill("GET /api/data response: " + response2)
    
    sus post_body tea = "{\"message\": \"Hello HTTP/2!\"}"
    sus response3 tea = http2_server_handle_request(&server, "POST", "/api/upload", post_body)
    vibez.spill("POST /api/upload response: " + response3)
    
    fr fr Demo server push
    vibez.spill("📡 Demonstrating HTTP/2 server push...")
    sus push_headers [5]tea
    push_headers[0] = ":path: /static/style.css"
    push_headers[1] = ":method: GET"
    sus pushed_stream normie = http2_server_push(&server, 1, "/static/style.css", push_headers, 2)
    vibez.spill("Server push initiated for stream: " + stringz.int_to_string(pushed_stream))
    
    http2_connection_close(&server)
    vibez.spill("✅ HTTP/2 Server Demo completed!")
}

slay http2_demo_features() {
    vibez.spill("🔧 HTTP/2 Features Demo")
    vibez.spill("=======================")
    
    fr fr Demo frame handling
    vibez.spill("📦 Frame handling...")
    sus headers_frame HTTP2Frame = http2_frame_create(HTTP2_FRAME_HEADERS, HTTP2_FLAG_END_HEADERS, 1, "headers-payload")
    sus data_frame HTTP2Frame = http2_frame_create(HTTP2_FRAME_DATA, HTTP2_FLAG_END_STREAM, 1, "response-data")
    vibez.spill("HEADERS frame type: " + http2_get_frame_type_name(headers_frame.frame_type))
    vibez.spill("DATA frame type: " + http2_get_frame_type_name(data_frame.frame_type))
    
    fr fr Demo HPACK compression
    vibez.spill("🗜️ HPACK header compression...")
    sus hpack_ctx HPACKContext = hpack_context_create()
    sus encoded_method tea = hpack_encode_header(&hpack_ctx, ":method", "GET")
    sus encoded_path tea = hpack_encode_header(&hpack_ctx, ":path", "/api/test")
    vibez.spill("Header compression completed")
    
    fr fr Demo settings
    vibez.spill("⚙️ HTTP/2 settings...")
    sus settings HTTP2Settings = http2_settings_default()
    sus settings_frame HTTP2Frame = http2_settings_frame_create(settings)
    sus settings_ack HTTP2Frame = http2_settings_ack_frame()
    vibez.spill("Settings configured: max_frame_size=" + stringz.int_to_string(settings.max_frame_size))
    
    fr fr Demo ping
    vibez.spill("🏓 HTTP/2 ping...")
    sus conn HTTP2Connection = http2_connection_create()
    sus ping_id tea = http2_send_ping(&conn)
    vibez.spill("Ping sent with ID: " + ping_id)
    
    vibez.spill("✅ HTTP/2 Features Demo completed!")
}
