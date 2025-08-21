# Simple HTTP Web Server in CURSED
#
# This example demonstrates:
# - HTTP server creation
# - Request routing
# - JSON responses
# - Concurrent request handling
# - Error handling

yeet "vibez"
yeet "networkz"
yeet "jsonz"
yeet "stringz"
yeet "concurrenz"

# Define our response structure
squad ApiResponse {
    message tea,
    status drip,
    data map<tea, any>
}

# Simple request handler
slay handle_root() ApiResponse {
    damn ApiResponse{
        message: "Welcome to CURSED Web Server! 🔥",
        status: 200,
        data: {
            "version": "1.0.0",
            "language": "CURSED",
            "endpoints": ["/", "/api/status", "/api/time"]
        }
    }
}

slay handle_status() ApiResponse {
    damn ApiResponse{
        message: "Server is running perfectly!",
        status: 200,
        data: {
            "uptime": "5 minutes",
            "memory_usage": "12MB",
            "active_connections": 3
        }
    }
}

slay handle_time() ApiResponse {
    yeet "timez"
    sus current_time tea = timez.format_iso(timez.now())
    
    damn ApiResponse{
        message: "Current server time",
        status: 200,
        data: {
            "timestamp": current_time,
            "timezone": "UTC",
            "format": "ISO 8601"
        }
    }
}

# Route dispatcher
slay route_request(path tea) ApiResponse {
    ready (path == "/") {
        damn handle_root()
    } otherwise ready (path == "/api/status") {
        damn handle_status()
    } otherwise ready (path == "/api/time") {
        damn handle_time()
    } otherwise {
        damn ApiResponse{
            message: "Endpoint not found",
            status: 404,
            data: {
                "requested_path": path,
                "available_endpoints": ["/", "/api/status", "/api/time"]
            }
        }
    }
}

# Process incoming HTTP requests
slay handle_connection(conn Connection) {
    # Read the HTTP request
    sus request_data tea = networkz.read_http_request(conn) fam {
        when err -> {
            vibez.spill("Error reading request:", err)
            networkz.close(conn)
            damn
        }
    }
    
    # Parse the request to extract the path
    sus lines []tea = stringz.split(request_data, "\n")
    sus request_line []tea = stringz.split(lines[0], " ")
    sus method tea = request_line[0]
    sus path tea = request_line[1]
    
    vibez.spill("Handling", method, "request to", path)
    
    # Route the request
    sus response ApiResponse = route_request(path)
    
    # Convert response to JSON
    sus json_response tea = jsonz.stringify(response, indent: 2)
    
    # Create HTTP response
    sus http_response tea = "HTTP/1.1 " + string(response.status) + " OK\r\n" +
                           "Content-Type: application/json\r\n" +
                           "Content-Length: " + string(len(json_response)) + "\r\n" +
                           "Access-Control-Allow-Origin: *\r\n" +
                           "\r\n" +
                           json_response
    
    # Send response
    networkz.write(conn, http_response) fam {
        when err -> vibez.spill("Error sending response:", err)
    }
    
    # Close connection
    networkz.close(conn)
}

# Main server function
slay start_server(port drip) yikes<tea> {
    vibez.spill("🔥 Starting CURSED Web Server on port", port)
    
    # Create a TCP listener
    sus listener = networkz.listen("localhost", port) fam {
        when err -> yikes "Failed to start server on port " + string(port) + ": " + err
    }
    
    vibez.spill("✅ Server running at http://localhost:" + string(port))
    vibez.spill("📡 Try these endpoints:")
    vibez.spill("   - http://localhost:" + string(port) + "/")
    vibez.spill("   - http://localhost:" + string(port) + "/api/status")
    vibez.spill("   - http://localhost:" + string(port) + "/api/time")
    vibez.spill("Press Ctrl+C to stop the server")
    
    # Accept connections in a loop
    bestie (based) {
        # Accept a new connection
        sus conn = networkz.accept(listener) fam {
            when err -> {
                vibez.spill("Error accepting connection:", err)
                continue
            }
        }
        
        # Handle each connection concurrently
        go {
            handle_connection(conn)
        }
    }
}

# Application entry point
slay main() {
    start_server(8080) fam {
        when err -> {
            vibez.spill("❌ Server error:", err)
            vibez.spill("Make sure port 8080 is not already in use")
        }
    }
}

# Run the server
main()

# To test this server:
# 1. Run: cursed-zig web-server.csd
# 2. Open browser to http://localhost:8080
# 3. Try different endpoints:
#    - http://localhost:8080/
#    - http://localhost:8080/api/status
#    - http://localhost:8080/api/time
#    - http://localhost:8080/nonexistent (for 404 demo)
#
# Features demonstrated:
# ✅ HTTP server with routing
# ✅ JSON API responses
# ✅ Concurrent request handling
# ✅ Error handling and validation
# ✅ Multiple endpoint support
# ✅ Proper HTTP headers
