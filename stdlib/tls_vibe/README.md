# TLS Vibe Module

Enterprise-grade TLS/SSL functionality for secure communications in CURSED. This module provides comprehensive support for TLS connections, certificate management, and secure data transmission.

## Features

- **TLS 1.2/1.3 Support**: Modern TLS protocol versions with strong security
- **Client/Server Operations**: Full client and server-side TLS functionality
- **Certificate Management**: Certificate validation and peer verification
- **Secure Data Transfer**: Encrypted read/write operations
- **Hostname Verification**: Automatic hostname validation against certificates
- **Session Management**: Session key generation and connection state tracking
- **Error Handling**: Comprehensive error handling and graceful degradation

## Usage Examples

### Basic TLS Client

```cursed
yeet "tls_vibe"

// Create TLS configuration
sus config := tls_config_new()
config = tls_config_set_cert(config, "client.pem")
config = tls_config_set_key(config, "client.key")
config = tls_config_set_ca(config, "ca.pem")

// Create and connect client
sus client := tls_client_new(config)
sus connected := tls_connect(client, "secure.example.com")

when connected {
    // Send secure data
    sus message tea = "Hello, secure world!"
    sus bytes_sent := tls_write(client.connection, message)
    
    // Receive secure response
    sus response tea = ""
    sus bytes_received := tls_read(client.connection, response)
    
    // Verify peer certificate
    sus peer_cert := tls_get_peer_cert(client.connection)
    sus hostname_valid := tls_verify_hostname(client.connection, "secure.example.com")
    
    // Close connection
    tls_close(client.connection)
}
```

### TLS Server

```cursed
yeet "tls_vibe"

// Create server configuration
sus config := tls_config_new()
config = tls_config_set_cert(config, "server.pem")
config = tls_config_set_key(config, "server.key")

// Create and start server
sus server := tls_server_new(config)
server.port = 8443

// Accept client connections
sus client_conn := tls_accept(server)
sus handshake_success := tls_handshake(client_conn)

when handshake_success {
    // Handle secure communication
    sus request tea = ""
    tls_read(client_conn, request)
    
    sus response tea = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nSecure response"
    tls_write(client_conn, response)
    
    tls_close(client_conn)
}
```

### Certificate Validation

```cursed
yeet "tls_vibe"

// Validate certificate chain
sus config := tls_config_new()
sus client := tls_client_new(config)
tls_connect(client, "trusted.example.com")

// Check connection security
sus is_secure := tls_is_secure(client.connection)
sus cert_valid := tls_validate_cert_chain(client.connection)
sus hostname_match := tls_verify_hostname(client.connection, "trusted.example.com")

when is_secure && cert_valid && hostname_match {
    vibez.spill("Connection is fully secure and verified")
}
```

## API Reference

### Configuration Functions

#### `tls_config_new() -> TLSConfig`
Creates a new TLS configuration with default settings.

**Returns**: TLSConfig with default values
- `verify_peer`: true
- `min_version`: TLS 1.2
- `max_version`: TLS 1.3
- `cipher_suites`: "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256"

#### `tls_config_set_cert(config TLSConfig, cert tea) -> TLSConfig`
Sets the certificate file path in the configuration.

**Parameters**:
- `config`: TLS configuration to modify
- `cert`: Path to certificate file

**Returns**: Updated TLS configuration

#### `tls_config_set_key(config TLSConfig, key tea) -> TLSConfig`
Sets the private key file path in the configuration.

**Parameters**:
- `config`: TLS configuration to modify
- `key`: Path to private key file

**Returns**: Updated TLS configuration

#### `tls_config_set_ca(config TLSConfig, ca tea) -> TLSConfig`
Sets the CA certificates file path in the configuration.

**Parameters**:
- `config`: TLS configuration to modify
- `ca`: Path to CA certificates file

**Returns**: Updated TLS configuration

### Client Functions

#### `tls_client_new(config TLSConfig) -> TLSClient`
Creates a new TLS client with the specified configuration.

**Parameters**:
- `config`: TLS configuration to use

**Returns**: New TLS client instance

#### `tls_connect(client TLSClient, hostname tea) -> lit`
Establishes a TLS connection to the specified hostname.

**Parameters**:
- `client`: TLS client instance
- `hostname`: Target hostname to connect to

**Returns**: `based` if connection successful, `cap` otherwise

### Server Functions

#### `tls_server_new(config TLSConfig) -> TLSServer`
Creates a new TLS server with the specified configuration.

**Parameters**:
- `config`: TLS configuration to use

**Returns**: New TLS server instance

#### `tls_accept(server TLSServer) -> TLSConnection`
Accepts a new client connection on the server.

**Parameters**:
- `server`: TLS server instance

**Returns**: New TLS connection for the client

### Connection Functions

#### `tls_handshake(conn TLSConnection) -> lit`
Performs the TLS handshake on a connection.

**Parameters**:
- `conn`: TLS connection to handshake

**Returns**: `based` if handshake successful, `cap` otherwise

#### `tls_read(conn TLSConnection, buffer tea) -> normie`
Reads encrypted data from a TLS connection.

**Parameters**:
- `conn`: TLS connection to read from
- `buffer`: Buffer to store decrypted data

**Returns**: Number of bytes read, or -1 on error

#### `tls_write(conn TLSConnection, data tea) -> normie`
Writes encrypted data to a TLS connection.

**Parameters**:
- `conn`: TLS connection to write to
- `data`: Data to encrypt and send

**Returns**: Number of bytes written, or -1 on error

#### `tls_close(conn TLSConnection) -> lit`
Closes a TLS connection.

**Parameters**:
- `conn`: TLS connection to close

**Returns**: `based` if closed successfully, `cap` otherwise

### Certificate Functions

#### `tls_get_peer_cert(conn TLSConnection) -> tea`
Retrieves the peer's certificate information.

**Parameters**:
- `conn`: TLS connection to get certificate from

**Returns**: Peer certificate data, or empty string if not available

#### `tls_verify_hostname(conn TLSConnection, hostname tea) -> lit`
Verifies that the peer's certificate matches the specified hostname.

**Parameters**:
- `conn`: TLS connection to verify
- `hostname`: Hostname to verify against certificate

**Returns**: `based` if hostname matches, `cap` otherwise

#### `tls_validate_cert_chain(conn TLSConnection) -> lit`
Validates the peer's certificate chain.

**Parameters**:
- `conn`: TLS connection to validate

**Returns**: `based` if certificate chain is valid, `cap` otherwise

### Utility Functions

#### `tls_get_cipher_suite(conn TLSConnection) -> tea`
Gets the negotiated cipher suite for a connection.

**Parameters**:
- `conn`: TLS connection to query

**Returns**: Cipher suite name, or empty string if not connected

#### `tls_get_state(conn TLSConnection) -> normie`
Gets the current state of a TLS connection.

**Parameters**:
- `conn`: TLS connection to query

**Returns**: Connection state (0=init, 1=handshake, 2=connected, 3=closed)

#### `tls_is_secure(conn TLSConnection) -> lit`
Checks if a TLS connection is secure.

**Parameters**:
- `conn`: TLS connection to check

**Returns**: `based` if connection is secure, `cap` otherwise

#### `tls_generate_session_key(conn TLSConnection) -> tea`
Generates a session key for the TLS connection.

**Parameters**:
- `conn`: TLS connection to generate key for

**Returns**: Session key data, or empty string if not connected

## Data Structures

### TLSConfig
Configuration structure for TLS operations.

```cursed
struct TLSConfig {
    cert_path tea        // Path to certificate file
    key_path tea         // Path to private key file
    ca_path tea          // Path to CA certificates file
    verify_peer lit      // Whether to verify peer certificates
    min_version normie   // Minimum TLS version
    max_version normie   // Maximum TLS version
    cipher_suites tea    // Allowed cipher suites
}
```

### TLSConnection
Represents an active TLS connection.

```cursed
struct TLSConnection {
    socket normie              // Socket file descriptor
    state normie               // Connection state
    peer_cert tea              // Peer certificate data
    cipher_suite tea           // Negotiated cipher suite
    is_server lit              // Whether this is a server connection
    handshake_complete lit     // Whether handshake is complete
    buffer tea                 // Internal buffer for data
}
```

### TLSClient
TLS client instance.

```cursed
struct TLSClient {
    config TLSConfig          // TLS configuration
    connection TLSConnection  // Active connection
    hostname tea              // Target hostname
}
```

### TLSServer
TLS server instance.

```cursed
struct TLSServer {
    config TLSConfig     // TLS configuration
    socket normie        // Server socket
    port normie          // Server port
    active_connections normie  // Number of active connections
}
```

## Security Features

### Supported Cipher Suites
- TLS_AES_256_GCM_SHA384 (TLS 1.3)
- TLS_CHACHA20_POLY1305_SHA256 (TLS 1.3)

### Protocol Versions
- TLS 1.2 (minimum supported)
- TLS 1.3 (preferred)

### Certificate Validation
- Peer certificate verification
- Hostname validation
- Certificate chain validation
- CA certificate validation

## Error Handling

All functions return appropriate error codes:
- `lit` functions return `cap` on error
- `normie` functions return -1 on error
- `tea` functions return empty string on error

Common error conditions:
- Connection not established
- Handshake not complete
- Invalid certificate
- Hostname mismatch
- Network errors

## Dependencies

- `hash_drip`: For cryptographic hashing operations
- `big_mood`: For string formatting and manipulation

## Testing

Run the comprehensive test suite:

```bash
# Test in interpretation mode
cargo run --bin cursed stdlib/tls_vibe/test_tls_vibe.csd

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/tls_vibe/test_tls_vibe.csd
./test_tls_vibe
```

The test suite includes:
- Configuration management tests
- Client/server connection tests
- Handshake and encryption tests
- Certificate validation tests
- Error handling tests
- Multiple connection tests

## Implementation Notes

This is a pure CURSED implementation that simulates TLS operations using cryptographic primitives from the `hash_drip` module. In a production environment, this would interface with a real TLS library like OpenSSL or rustls.

The implementation focuses on providing a complete API surface that matches real TLS libraries while maintaining the CURSED language's design principles and security best practices.
