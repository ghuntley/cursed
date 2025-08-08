# CURSED HTTP/2 Module (httpz_v2)

Modern HTTP/2 protocol implementation for CURSED with multiplexing, server push, and HPACK compression.

## Overview

The `httpz_v2` module provides a complete HTTP/2 implementation following RFC 7540 specifications. It includes client and server capabilities, frame processing, header compression, flow control, stream prioritization, and server push functionality.

## Features

### ✅ Core HTTP/2 Protocol
- **Binary Framing**: Complete HTTP/2 frame handling (DATA, HEADERS, SETTINGS, etc.)
- **Multiplexing**: Multiple concurrent streams over single connection
- **Flow Control**: Window-based flow control for streams and connections
- **Stream Prioritization**: Dependency-based stream prioritization
- **Server Push**: Proactive resource pushing to clients
- **Connection Management**: Connection lifecycle and state management

### ✅ HPACK Header Compression
- **Dynamic Table**: Header compression with dynamic table management
- **Static Table**: Predefined header index compression
- **Literal Headers**: New header encoding and decoding
- **Memory Efficient**: Configurable table size limits

### ✅ Advanced Features
- **Error Handling**: Comprehensive error codes and recovery
- **Ping/Pong**: Connection health monitoring
- **Settings Negotiation**: Connection parameter configuration
- **Connection Preface**: Proper HTTP/2 connection establishment
- **Protocol Upgrade**: HTTP/1.1 to HTTP/2 upgrade support

### ✅ Client & Server APIs
- **High-Level APIs**: Simple HTTP/2 client and server interfaces
- **Request/Response**: Complete HTTP semantics over HTTP/2
- **Streaming**: Support for streaming responses and server-sent events
- **Security**: Origin validation and security headers

## Quick Start

### HTTP/2 Client

```cursed
yeet "httpz_v2"

# Simple GET request
sus headers [2]tea
headers[0] = "accept: application/json"
headers[1] = "user-agent: CURSED-App/1.0"

sus response tea = http2_client_get("https://api.example.com/users", headers, 2)
vibez.spill("Response: " + response)

# POST request with body
sus post_headers [2]tea
post_headers[0] = "content-type: application/json"
post_headers[1] = "accept: application/json"

sus post_body tea = "{\"name\": \"John\", \"email\": \"john@example.com\"}"
sus post_response tea = http2_client_post("https://api.example.com/users", post_body, post_headers, 2)
vibez.spill("Created: " + post_response)
```

### HTTP/2 Server

```cursed
yeet "httpz_v2"

# Create HTTP/2 server
sus server HTTP2Connection = http2_server_create(8443)

# Handle requests
sus response tea = http2_server_handle_request(&server, "GET", "/api/users", "")
vibez.spill("Response: " + response)

# Handle POST with body
sus post_body tea = "{\"action\": \"create_user\"}"
sus post_response tea = http2_server_handle_request(&server, "POST", "/api/users", post_body)
vibez.spill("Created: " + post_response)
```

### Advanced Usage

```cursed
yeet "httpz_v2"

# Create connection with custom settings
sus conn HTTP2Connection = http2_connection_create()
conn.max_concurrent_streams = 200
conn.initial_window_size = 131072

# Send request with multiple streams
sus stream1 normie = http2_send_request(&conn, "GET", "/api/data1", headers, 2, "")
sus stream2 normie = http2_send_request(&conn, "GET", "/api/data2", headers, 2, "")
sus stream3 normie = http2_send_request(&conn, "GET", "/api/data3", headers, 2, "")

# Configure server push
sus push_headers [3]tea
push_headers[0] = ":path: /static/app.css"
push_headers[1] = ":method: GET"
push_headers[2] = "accept: text/css"

sus pushed_stream normie = http2_server_push(&conn, stream1, "/static/app.css", push_headers, 3)
vibez.spill("Pushed stream: " + stringz.int_to_string(pushed_stream))
```

## API Reference

### Connection Management

#### `http2_connection_create() -> HTTP2Connection`
Creates a new HTTP/2 connection with default settings.

#### `http2_connection_close(conn *HTTP2Connection) -> lit`
Closes an HTTP/2 connection and all associated streams.

#### `http2_stream_create(conn *HTTP2Connection, stream_id normie) -> HTTP2Stream`
Creates a new stream within an HTTP/2 connection.

### Frame Processing

#### `http2_frame_create(frame_type smol, flags smol, stream_id normie, payload tea) -> HTTP2Frame`
Creates an HTTP/2 frame with specified type, flags, stream ID, and payload.

#### `http2_frame_serialize(frame HTTP2Frame) -> tea`
Serializes an HTTP/2 frame to binary format for transmission.

#### `http2_frame_parse(data tea) -> HTTP2Frame`
Parses binary data into an HTTP/2 frame structure.

### HPACK Compression

#### `hpack_context_create() -> HPACKContext`
Creates a new HPACK compression context with default settings.

#### `hpack_encode_header(ctx *HPACKContext, name tea, value tea) -> tea`
Encodes a header name-value pair using HPACK compression.

#### `hpack_decode_header(ctx *HPACKContext, data tea) -> (tea, tea)`
Decodes HPACK-compressed header data into name-value pair.

### Client API

#### `http2_client_get(url tea, headers [20]tea, header_count normie) -> tea`
Performs an HTTP/2 GET request to the specified URL with custom headers.

#### `http2_client_post(url tea, body tea, headers [20]tea, header_count normie) -> tea`
Performs an HTTP/2 POST request with request body and custom headers.

### Server API

#### `http2_server_create(port normie) -> HTTP2Connection`
Creates an HTTP/2 server listening on the specified port.

#### `http2_server_handle_request(conn *HTTP2Connection, method tea, path tea, body tea) -> tea`
Handles an incoming HTTP/2 request and returns the response.

#### `http2_server_push(conn *HTTP2Connection, parent_stream_id normie, push_path tea, push_headers [20]tea, header_count normie) -> normie`
Initiates server push for a resource to the client.

### Flow Control

#### `http2_update_window(stream *HTTP2Stream, increment normie) -> lit`
Updates the flow control window for a stream.

#### `http2_window_update_frame(stream_id normie, increment normie) -> HTTP2Frame`
Creates a WINDOW_UPDATE frame for flow control.

#### `http2_check_flow_control(stream HTTP2Stream, data_size normie) -> lit`
Checks if sufficient flow control window is available for data transmission.

### Settings Management

#### `http2_settings_default() -> HTTP2Settings`
Creates default HTTP/2 settings with standard values.

#### `http2_settings_frame_create(settings HTTP2Settings) -> HTTP2Frame`
Creates a SETTINGS frame from HTTP/2 settings.

#### `http2_settings_ack_frame() -> HTTP2Frame`
Creates a SETTINGS ACK frame to acknowledge received settings.

### Error Handling

#### `http2_rst_stream_frame(stream_id normie, error_code normie) -> HTTP2Frame`
Creates a RST_STREAM frame to reset a stream with an error code.

#### `http2_goaway_frame(last_stream_id normie, error_code normie, debug_data tea) -> HTTP2Frame`
Creates a GOAWAY frame to gracefully close a connection.

#### `http2_handle_error(conn *HTTP2Connection, stream_id normie, error_code normie) -> lit`
Handles HTTP/2 errors by sending appropriate frames.

### Utility Functions

#### `http2_get_frame_type_name(frame_type smol) -> tea`
Returns human-readable name for HTTP/2 frame type.

#### `http2_get_error_name(error_code normie) -> tea`
Returns human-readable name for HTTP/2 error code.

#### `http2_is_valid_frame_type(frame_type smol) -> lit`
Validates if frame type is within valid HTTP/2 range.

## HTTP/2 Frame Types

| Frame Type | Value | Description |
|------------|-------|-------------|
| DATA | 0 | Stream data payload |
| HEADERS | 1 | Header information |
| PRIORITY | 2 | Stream priority |
| RST_STREAM | 3 | Stream reset |
| SETTINGS | 4 | Connection settings |
| PUSH_PROMISE | 5 | Server push promise |
| PING | 6 | Connection ping |
| GOAWAY | 7 | Connection termination |
| WINDOW_UPDATE | 8 | Flow control window update |
| CONTINUATION | 9 | Header continuation |

## HTTP/2 Error Codes

| Error Code | Value | Description |
|------------|-------|-------------|
| NO_ERROR | 0 | No error |
| PROTOCOL_ERROR | 1 | Protocol violation |
| INTERNAL_ERROR | 2 | Internal server error |
| FLOW_CONTROL_ERROR | 3 | Flow control violation |
| SETTINGS_TIMEOUT | 4 | Settings acknowledgment timeout |
| STREAM_CLOSED | 5 | Stream already closed |
| FRAME_SIZE_ERROR | 6 | Invalid frame size |
| REFUSED_STREAM | 7 | Stream refused |
| CANCEL | 8 | Stream cancelled |
| COMPRESSION_ERROR | 9 | Header compression error |
| CONNECT_ERROR | 10 | Connection error |
| ENHANCE_YOUR_CALM | 11 | Rate limiting |
| INADEQUATE_SECURITY | 12 | Insufficient security |
| HTTP_1_1_REQUIRED | 13 | HTTP/1.1 fallback required |

## Examples

### Multiple Concurrent Requests

```cursed
yeet "httpz_v2"

# Create connection
sus conn HTTP2Connection = http2_connection_create()

# Send multiple concurrent requests (HTTP/2 multiplexing)
sus headers [2]tea
headers[0] = "accept: application/json"
headers[1] = "user-agent: CURSED-Multiplex/1.0"

sus stream1 normie = http2_send_request(&conn, "GET", "/api/users", headers, 2, "")
sus stream2 normie = http2_send_request(&conn, "GET", "/api/posts", headers, 2, "")
sus stream3 normie = http2_send_request(&conn, "GET", "/api/comments", headers, 2, "")

vibez.spill("Sent 3 concurrent requests:")
vibez.spill("Stream 1: " + stringz.int_to_string(stream1))
vibez.spill("Stream 2: " + stringz.int_to_string(stream2))
vibez.spill("Stream 3: " + stringz.int_to_string(stream3))
```

### Server Push Example

```cursed
yeet "httpz_v2"

# Create server
sus server HTTP2Connection = http2_server_create(8443)

# When serving HTML, push CSS and JS resources
sus css_headers [3]tea
css_headers[0] = ":path: /static/app.css"
css_headers[1] = ":method: GET"
css_headers[2] = "accept: text/css"

sus js_headers [3]tea
js_headers[0] = ":path: /static/app.js"
js_headers[1] = ":method: GET"
js_headers[2] = "accept: application/javascript"

# Push resources proactively
sus css_stream normie = http2_server_push(&server, 1, "/static/app.css", css_headers, 3)
sus js_stream normie = http2_server_push(&server, 1, "/static/app.js", js_headers, 3)

vibez.spill("Pushed CSS on stream: " + stringz.int_to_string(css_stream))
vibez.spill("Pushed JS on stream: " + stringz.int_to_string(js_stream))
```

### Flow Control Management

```cursed
yeet "httpz_v2"

# Create stream
sus conn HTTP2Connection = http2_connection_create()
sus stream HTTP2Stream = http2_stream_create(&conn, 1)

# Check flow control before sending large data
sus large_data tea = "very_large_response_data_here"
sus data_size normie = stringz.length(large_data)

lowkey http2_check_flow_control(stream, data_size) {
    # Send data
    sus data_frame HTTP2Frame = http2_create_data_frame(1, large_data, based)
    vibez.spill("Sent data frame")
} else {
    # Update window and retry
    sus window_frame HTTP2Frame = http2_window_update_frame(1, data_size)
    http2_update_window(&stream, data_size)
    vibez.spill("Updated flow control window")
}
```

### HPACK Header Compression

```cursed
yeet "httpz_v2"

# Create HPACK context
sus ctx HPACKContext = hpack_context_create()

# Encode common headers
sus method_encoded tea = hpack_encode_header(&ctx, ":method", "GET")
sus path_encoded tea = hpack_encode_header(&ctx, ":path", "/api/data")
sus scheme_encoded tea = hpack_encode_header(&ctx, ":scheme", "https")

vibez.spill("Encoded :method: " + method_encoded)
vibez.spill("Encoded :path: " + path_encoded)
vibez.spill("Encoded :scheme: " + scheme_encoded)

# Decode headers
sus (name1, value1) = hpack_decode_header(&ctx, method_encoded)
sus (name2, value2) = hpack_decode_header(&ctx, path_encoded)

vibez.spill("Decoded: " + name1 + " = " + value1)
vibez.spill("Decoded: " + name2 + " = " + value2)
```

## Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed stdlib/httpz_v2/test_httpz_v2.csd
```

The test suite covers:
- Frame creation, serialization, and parsing
- Connection and stream management
- HPACK header compression/decompression
- Flow control and window management
- Settings negotiation and acknowledgment
- Stream prioritization
- Server push functionality
- Error handling and recovery
- Client and server APIs
- Integration scenarios
- Performance testing

## Performance Considerations

- **Multiplexing**: HTTP/2 allows up to 100 concurrent streams by default
- **Header Compression**: HPACK reduces header overhead by 85-90%
- **Server Push**: Eliminates round trips for critical resources
- **Flow Control**: Prevents buffer overflow and ensures fair resource usage
- **Binary Protocol**: More efficient parsing than HTTP/1.1 text protocol

## Security Features

- **Origin Validation**: Server validates request origins
- **TLS Requirements**: HTTPS strongly recommended for HTTP/2
- **Flow Control**: Prevents denial-of-service attacks
- **Error Handling**: Graceful degradation and error recovery
- **Settings Validation**: Parameter bounds checking

## Implementation Notes

- **Pure CURSED**: No external dependencies, implemented entirely in CURSED
- **RFC 7540 Compliant**: Follows HTTP/2 specification requirements
- **Production Ready**: Comprehensive error handling and edge case coverage
- **Memory Efficient**: Optimized frame processing and header compression
- **Cross-Platform**: Works on all platforms supporting CURSED

## Browser Compatibility

HTTP/2 is supported by all modern browsers:
- Chrome 40+
- Firefox 36+
- Safari 9+
- Edge 12+
- Opera 27+

## Migration from HTTP/1.1

1. **Replace imports**: Change `yeet "httpz"` to `yeet "httpz_v2"`
2. **Update APIs**: Use new HTTP/2 client/server functions
3. **Configure settings**: Tune HTTP/2 parameters for your use case
4. **Enable server push**: Take advantage of proactive resource pushing
5. **Test thoroughly**: Validate multiplexing and flow control behavior

The httpz_v2 module provides a complete, production-ready HTTP/2 implementation that offers significant performance improvements over HTTP/1.1 through multiplexing, header compression, and server push capabilities.
