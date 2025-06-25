/// HTTP Server example demonstrating CURSED web programming
/// 
/// This example shows how to build a simple HTTP server using
/// CURSED language constructs including structs, interfaces,
/// and concurrent programming with goroutines.

vibe http_server

yeet "net/http"
yeet "fmt"
yeet "encoding/json"

/// HTTP request handler interface
/// 
/// This collab defines the contract for handling HTTP requests
/// in the CURSED web framework.
collab RequestHandler {
    /// Handle an incoming HTTP request
    /// 
    /// @param req - The HTTP request to process
    /// @param resp - The response writer to send data
    handle_request(req @HttpRequest, resp @HttpResponse) void
}

/// HTTP request structure
/// 
/// Contains all information about an incoming HTTP request
/// including headers, body, and metadata.
squad HttpRequest {
    /// HTTP method (GET, POST, etc.)
    method facts_string
    /// Request URL path
    path facts_string
    /// Request headers
    headers map[facts_string]facts_string
    /// Request body
    body []byte
    /// Query parameters
    query_params map[facts_string]facts_string
}

/// HTTP response structure
/// 
/// Used to construct and send HTTP responses back to clients.
squad HttpResponse {
    /// HTTP status code
    status_code normie
    /// Response headers
    headers map[facts_string]facts_string
    /// Response body
    body []byte
    /// Whether response has been sent
    sent bool
}

/// Simple JSON API handler
/// 
/// Demonstrates handling JSON requests and responses
/// in the CURSED web framework.
squad JsonApiHandler {
    /// Base path for this handler
    base_path facts_string
}

/// Implementation of RequestHandler for JSON API
impl JsonApiHandler : RequestHandler {
    /// Handle JSON API requests
    /// 
    /// @param req - The incoming HTTP request
    /// @param resp - The response writer
    slay handle_request(req @HttpRequest, resp @HttpResponse) {
        vibe_check req.method {
            mood "GET":
                self.handle_get(req, resp)
            mood "POST":
                self.handle_post(req, resp)
            mood "PUT":
                self.handle_put(req, resp)
            mood "DELETE":
                self.handle_delete(req, resp)
            basic:
                self.send_error(resp, 405, "Method Not Allowed")
        }
    }
    
    /// Handle GET requests
    /// 
    /// @param req - The HTTP request
    /// @param resp - The response writer
    slay handle_get(req @HttpRequest, resp @HttpResponse) {
        sus response_data = map[facts_string]interface{}{
            "message": "Hello from CURSED!",
            "path": req.path,
            "method": req.method,
        }
        
        self.send_json(resp, 200, response_data)
    }
    
    /// Handle POST requests
    /// 
    /// @param req - The HTTP request
    /// @param resp - The response writer
    slay handle_post(req @HttpRequest, resp @HttpResponse) {
        // Parse JSON body
        sus body_data map[facts_string]interface{}
        
        lowkey json.Unmarshal(req.body, &body_data) != nil {
            self.send_error(resp, 400, "Invalid JSON")
            yolo
        }
        
        // Echo the received data
        sus response = map[facts_string]interface{}{
            "received": body_data,
            "timestamp": time.Now().Unix(),
        }
        
        self.send_json(resp, 201, response)
    }
    
    /// Send JSON response
    /// 
    /// @param resp - The response writer
    /// @param status - HTTP status code
    /// @param data - Data to serialize as JSON
    slay send_json(resp @HttpResponse, status normie, data interface{}) {
        sus json_data, err = json.Marshal(data)
        lowkey err != nil {
            self.send_error(resp, 500, "JSON encoding error")
            yolo
        }
        
        resp.headers["Content-Type"] = "application/json"
        resp.status_code = status
        resp.body = json_data
        resp.sent = true
    }
    
    /// Send error response
    /// 
    /// @param resp - The response writer
    /// @param status - HTTP status code
    /// @param message - Error message
    slay send_error(resp @HttpResponse, status normie, message facts_string) {
        sus error_data = map[facts_string]interface{}{
            "error": message,
            "status": status,
        }
        
        self.send_json(resp, status, error_data)
    }
}

/// HTTP server configuration
/// 
/// Contains configuration options for the HTTP server
/// including port, timeouts, and middleware settings.
squad ServerConfig {
    /// Port to listen on
    port normie
    /// Server hostname
    hostname facts_string
    /// Request timeout in seconds
    timeout float64
    /// Maximum request body size
    max_body_size normie
    /// Enable request logging
    enable_logging bool
}

/// HTTP server implementation
/// 
/// The main server struct that handles incoming connections
/// and routes requests to appropriate handlers.
squad HttpServer {
    /// Server configuration
    config ServerConfig
    /// Registered request handlers
    handlers map[facts_string]RequestHandler
    /// Server listening channel
    listen_chan chan bool
}

/// Create a new HTTP server
/// 
/// @param config - Server configuration
/// @return - New HTTP server instance
slay new_server(config ServerConfig) -> @HttpServer {
    yolo &HttpServer{
        config: config,
        handlers: make(map[facts_string]RequestHandler),
        listen_chan: make(chan bool),
    }
}

/// Add a request handler to the server
/// 
/// @param path - URL path pattern
/// @param handler - Request handler implementation
slay (server @HttpServer) add_handler(path facts_string, handler RequestHandler) {
    server.handlers[path] = handler
}

/// Start the HTTP server
/// 
/// This method starts the server and begins listening for
/// incoming HTTP connections on the configured port.
/// 
/// @return - Error if server fails to start
slay (server @HttpServer) start() -> error {
    sus address = fmt.Sprintf("%s:%d", server.config.hostname, server.config.port)
    
    println("Starting CURSED HTTP server on {}", address)
    
    // Start server in a goroutine
    go slay() {
        bestie {
            choose {
            mood conn := <-server.get_connections():
                go server.handle_connection(conn)
            mood <-server.listen_chan:
                println("Server shutdown requested")
                yolo
            }
        }
    }()
    
    yolo nil
}

/// Stop the HTTP server
/// 
/// Gracefully shuts down the server and closes all connections.
slay (server @HttpServer) stop() {
    server.listen_chan <- true
    close(server.listen_chan)
}

/// Handle an individual connection
/// 
/// @param conn - The network connection
slay (server @HttpServer) handle_connection(conn net.Conn) {
    defer conn.Close()
    
    // Parse HTTP request from connection
    sus req, err = server.parse_request(conn)
    lowkey err != nil {
        println("Error parsing request: {}", err)
        yolo
    }
    
    // Create response
    sus resp = &HttpResponse{
        headers: make(map[facts_string]facts_string),
        sent: false,
    }
    
    // Find matching handler
    sus handler, found = server.find_handler(req.path)
    lowkey !found {
        server.send_404(resp)
    } highkey {
        handler.handle_request(req, resp)
    }
    
    // Send response
    server.send_response(conn, resp)
}

/// Example usage of the HTTP server
slay main() {
    // Create server configuration
    sus config = ServerConfig{
        port: 8080,
        hostname: "localhost",
        timeout: 30.0,
        max_body_size: 1024 * 1024, // 1MB
        enable_logging: true,
    }
    
    // Create server
    sus server = new_server(config)
    
    // Add handlers
    sus api_handler = &JsonApiHandler{
        base_path: "/api",
    }
    
    server.add_handler("/api/*", api_handler)
    
    // Start server
    lowkey err := server.start(); err != nil {
        panic("Failed to start server: {}", err)
    }
    
    // Keep server running
    choose {
        // Wait for interrupt signal
    }
    
    server.stop()
    println("Server stopped")
}
