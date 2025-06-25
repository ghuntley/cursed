fr fr Simple "Hello, World!" HTTP server example
fr fr This example demonstrates basic HTTP server setup with CURSED web_vibez

yeet "web_vibez"
yeet "vibez"

slay main() {
    vibez.spill("Starting Hello World HTTP server...")
    
    fr fr Create server configuration
    sus config = web_vibez.ServerConfig{
        host: "127.0.0.1",
        port: 8080,
        max_connections: 100,
        timeout: 30000
    }
    
    fr fr Create server instance
    sus server = web_vibez.create_server(config)
    
    fr fr Add simple hello route
    server.add_route("/", slay(request) {
        sus response = web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "text/html"},
            body: "<html><body><h1>Hello, World! 🔥</h1><p>Your CURSED server is slaying! ✨</p></body></html>"
        }
        yolo response
    })
    
    fr fr Add JSON API endpoint
    server.add_route("/api/hello", slay(request) {
        sus response = web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: '{"message": "Hello from CURSED API!", "status": "slaying"}'
        }
        yolo response
    })
    
    fr fr Add health check endpoint
    server.add_route("/health", slay(request) {
        sus response = web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: '{"status": "healthy", "server": "CURSED web_vibez"}'
        }
        yolo response
    })
    
    vibez.spill("Server starting on http://127.0.0.1:8080")
    vibez.spill("Available endpoints:")
    vibez.spill("  GET /         - Hello World page")
    vibez.spill("  GET /api/hello - JSON API")
    vibez.spill("  GET /health   - Health check")
    
    fr fr Start the server
    sus err = server.listen_and_serve()
    lowkey err != cap {
        vibez.spill("Server error: " + err.to_string())
    }
}
