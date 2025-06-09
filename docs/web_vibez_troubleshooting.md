# CURSED web_vibez Troubleshooting Guide

Having issues with your CURSED web server? No cap, we've got you covered! This guide helps you debug and solve common problems. 🔧

## Table of Contents

1. [Server Won't Start](#server-wont-start)
2. [Connection Issues](#connection-issues)
3. [Performance Problems](#performance-problems)
4. [Request/Response Issues](#requestresponse-issues)
5. [Middleware Problems](#middleware-problems)
6. [Client Request Errors](#client-request-errors)
7. [Memory and Resource Issues](#memory-and-resource-issues)
8. [Debugging Tips](#debugging-tips)
9. [Common Error Messages](#common-error-messages)
10. [Getting Help](#getting-help)

## Server Won't Start

### Problem: "Failed to bind to address"

```
Error: Failed to bind to 127.0.0.1:8080: Address already in use
```

**Solutions:**

1. **Check if port is already in use:**
   ```bash
   # On Linux/macOS
   lsof -i :8080
   netstat -tulpn | grep :8080
   
   # On Windows
   netstat -ano | findstr :8080
   ```

2. **Use a different port:**
   ```cursed
   sus config = web_vibez.ServerConfig{
       host: "127.0.0.1",
       port: 8081,  fr fr Try different port
       max_connections: 100,
       timeout: 30000
   }
   ```

3. **Let the OS choose a port:**
   ```cursed
   sus config = web_vibez.ServerConfig{
       host: "127.0.0.1",
       port: 0,  fr fr OS will assign available port
       max_connections: 100,
       timeout: 30000
   }
   ```

### Problem: "Permission denied"

```
Error: Failed to bind to 0.0.0.0:80: Permission denied
```

**Solutions:**

1. **Use a port > 1024 for non-root users:**
   ```cursed
   sus config = web_vibez.ServerConfig{
       host: "0.0.0.0",
       port: 8080,  fr fr Use unprivileged port
       max_connections: 100,
       timeout: 30000
   }
   ```

2. **Run with appropriate permissions (not recommended for development):**
   ```bash
   sudo cursed run server.csd
   ```

3. **Use a reverse proxy (recommended for production):**
   - Set up nginx or Apache to proxy to your CURSED server
   - Run CURSED server on high port (8080, 3000, etc.)

## Connection Issues

### Problem: "Connection refused"

**Symptoms:**
- Browser shows "This site can't be reached"
- Client requests fail with connection errors

**Debugging steps:**

1. **Verify server is actually running:**
   ```cursed
   slay main() {
       sus server = web_vibez.create_server(config)
       server.add_route("/health", slay(request) {
           yolo web_vibez.Response{
               status: 200,
               headers: {},
               body: "Server is running!"
           }
       })
       
       vibez.spill("Server starting on http://" + config.host + ":" + config.port.to_string())
       vibez.spill("Health check: http://" + config.host + ":" + config.port.to_string() + "/health")
       
       server.listen_and_serve()
   }
   ```

2. **Check firewall settings:**
   ```bash
   # Linux (ufw)
   sudo ufw allow 8080
   
   # Linux (iptables)
   sudo iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
   ```

3. **Test with curl:**
   ```bash
   curl -v http://127.0.0.1:8080/health
   ```

### Problem: "Connection timeout"

**Solutions:**

1. **Increase server timeout:**
   ```cursed
   sus config = web_vibez.ServerConfig{
       host: "127.0.0.1",
       port: 8080,
       max_connections: 100,
       timeout: 60000  fr fr Increase to 60 seconds
   }
   ```

2. **Increase client timeout:**
   ```cursed
   web_vibez.client_timeout(30000)  fr fr 30 seconds
   ```

3. **Check network connectivity:**
   ```bash
   ping 127.0.0.1
   telnet 127.0.0.1 8080
   ```

## Performance Problems

### Problem: Slow response times

**Debugging:**

1. **Add timing middleware:**
   ```cursed
   slay timing_middleware() {
       yolo slay(request) {
           sus start_time = time_utils.unix_timestamp_ms()
           
           fr fr Log request start
           vibez.spill("Request started: " + request.method + " " + request.url)
           
           yolo cap  fr fr Continue to handler
       }
   }
   
   server.add_middleware(timing_middleware())
   ```

2. **Profile your route handlers:**
   ```cursed
   server.add_route("/slow-endpoint", slay(request) {
       sus start = time_utils.unix_timestamp_ms()
       
       fr fr Your processing logic here
       sus result = process_data()
       
       sus duration = time_utils.unix_timestamp_ms() - start
       vibez.spill("Processing took: " + duration.to_string() + "ms")
       
       yolo web_vibez.Response{
           status: 200,
           headers: {"X-Processing-Time": duration.to_string()},
           body: result
       }
   })
   ```

3. **Check for blocking operations:**
   - Avoid synchronous I/O in handlers
   - Use async operations where possible
   - Consider caching expensive computations

### Problem: High memory usage

**Solutions:**

1. **Limit request body size:**
   ```cursed
   slay body_size_middleware(max_size: numo) {
       yolo slay(request) {
           lowkey request.body.len() > max_size {
               yolo web_vibez.Response{
                   status: 413,
                   headers: {},
                   body: "Request body too large"
               }
           }
           yolo cap
       }
   }
   
   server.add_middleware(body_size_middleware(1048576))  fr fr 1MB limit
   ```

2. **Implement connection limits:**
   ```cursed
   sus config = web_vibez.ServerConfig{
       host: "127.0.0.1",
       port: 8080,
       max_connections: 100,  fr fr Limit concurrent connections
       timeout: 30000
   }
   ```

3. **Clean up resources:**
   ```cursed
   server.add_route("/upload", slay(request) {
       yolo vibe_check {
           sus file_data = process_upload(request.body)
           sus result = save_file(file_data)
           
           fr fr Clean up temporary data
           cleanup_temp_data(file_data)
           
           yolo web_vibez.Response{
               status: 201,
               headers: {},
               body: "Upload successful"
           }
       } catch err {
           vibez.spill("Upload error: " + err.to_string())
           yolo web_vibez.Response{
               status: 500,
               headers: {},
               body: "Upload failed"
           }
       }
   })
   ```

## Request/Response Issues

### Problem: "404 Not Found" for valid routes

**Debugging:**

1. **Check route registration order:**
   ```cursed
   fr fr More specific routes should come before wildcards
   server.add_route("/api/users/new", new_user_handler)
   server.add_route("/api/users/*", user_handler)  fr fr This comes after
   ```

2. **Debug route matching:**
   ```cursed
   server.add_middleware(slay(request) {
       vibez.spill("Incoming request: " + request.method + " " + request.url)
       yolo cap
   })
   ```

3. **Verify exact URL matching:**
   ```cursed
   fr fr URLs are case-sensitive and must match exactly
   server.add_route("/API/Users", handler)     fr fr Won't match "/api/users"
   server.add_route("/api/users/", handler)    fr fr Won't match "/api/users"
   ```

### Problem: Request body is empty

**Solutions:**

1. **Check Content-Type header:**
   ```cursed
   server.add_route("/api/data", slay(request) {
       sus content_type = request.headers.get("Content-Type")
       lowkey content_type == cap {
           yolo web_vibez.Response{
               status: 400,
               headers: {},
               body: "Content-Type header required"
           }
       }
       
       lowkey request.body.is_empty() {
           yolo web_vibez.Response{
               status: 400,
               headers: {},
               body: "Request body cannot be empty"
           }
       }
       
       fr fr Process request body
       sus data = json_tea.decode(request.body)
       fr fr ...
   })
   ```

2. **Debug request parsing:**
   ```cursed
   server.add_middleware(slay(request) {
       vibez.spill("Request headers: " + json_tea.encode(request.headers))
       vibez.spill("Request body length: " + request.body.len().to_string())
       lowkey !request.body.is_empty() {
           vibez.spill("Request body preview: " + request.body.substring(0, 100))
       }
       yolo cap
   })
   ```

### Problem: JSON parsing errors

**Solutions:**

1. **Add error handling for JSON:**
   ```cursed
   server.add_route("/api/users", slay(request) {
       yolo vibe_check {
           sus user_data = json_tea.decode(request.body)
           fr fr Process valid JSON
           yolo process_user(user_data)
       } catch json_error {
           vibez.spill("JSON parse error: " + json_error.to_string())
           yolo web_vibez.Response{
               status: 400,
               headers: {"Content-Type": "application/json"},
               body: '{"error": "Invalid JSON in request body"}'
           }
       }
   })
   ```

2. **Validate JSON structure:**
   ```cursed
   slay validate_user_json(data) {
       lowkey data.name == cap {
           throw "Missing required field: name"
       }
       lowkey data.email == cap {
           throw "Missing required field: email"
       }
       lowkey !data.email.contains("@") {
           throw "Invalid email format"
       }
   }
   ```

## Middleware Problems

### Problem: Middleware not executing

**Debugging:**

1. **Check middleware order:**
   ```cursed
   fr fr Middleware executes in the order added
   server.add_middleware(logging_middleware())    fr fr Executes first
   server.add_middleware(auth_middleware())       fr fr Executes second
   server.add_middleware(cors_middleware())       fr fr Executes third
   ```

2. **Ensure middleware returns correctly:**
   ```cursed
   slay my_middleware() {
       yolo slay(request) {
           fr fr Do middleware logic
           vibez.spill("Middleware executed for: " + request.url)
           
           fr fr IMPORTANT: Return None to continue to next middleware/handler
           yolo cap
           
           fr fr Or return a Response to short-circuit
           fr fr yolo web_vibez.Response{...}
       }
   }
   ```

3. **Debug middleware execution:**
   ```cursed
   slay debug_middleware() {
       yolo slay(request) {
           vibez.spill("DEBUG: Middleware executing for " + request.url)
           yolo cap
       }
   }
   
   server.add_middleware(debug_middleware())
   ```

### Problem: CORS issues

**Solutions:**

1. **Ensure CORS middleware is first:**
   ```cursed
   server.add_middleware(web_vibez.cors_middleware())  fr fr Add this first
   server.add_middleware(other_middleware())
   ```

2. **Custom CORS configuration:**
   ```cursed
   slay custom_cors_middleware() {
       yolo slay(request) {
           fr fr Handle preflight requests
           lowkey request.method == "OPTIONS" {
               yolo web_vibez.Response{
                   status: 200,
                   headers: {
                       "Access-Control-Allow-Origin": "*",
                       "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
                       "Access-Control-Allow-Headers": "Content-Type, Authorization",
                       "Access-Control-Max-Age": "3600"
                   },
                   body: ""
               }
           }
           yolo cap
       }
   }
   ```

## Client Request Errors

### Problem: "Real HTTP requests not implemented yet"

This error occurs when trying to make actual HTTP requests without mock mode.

**Current solution:**
```cursed
fr fr Use mock mode for testing
sus response = web_vibez.get("https://example.com", facts)  fr fr Mock mode enabled

fr fr For real requests, this feature is coming soon
fr fr Track issue: https://github.com/cursed/cursed/issues/http-client
```

### Problem: Client timeout issues

**Solutions:**

1. **Adjust timeout for slow services:**
   ```cursed
   web_vibez.client_timeout(60000)  fr fr 60 seconds for slow APIs
   sus response = web_vibez.get("https://slow-api.com/data", facts)
   ```

2. **Check network connectivity:**
   ```cursed
   yolo vibe_check {
       sus response = web_vibez.get("https://httpbin.org/get", facts)
       vibez.spill("Test request successful")
   } catch network_error {
       vibez.spill("Network issue: " + network_error.to_string())
   }
   ```

## Memory and Resource Issues

### Problem: Memory leaks

**Debugging:**

1. **Monitor memory usage:**
   ```cursed
   slay memory_monitor_middleware() {
       yolo slay(request) {
           sus memory_before = system.memory_usage()
           
           fr fr Continue to handler
           yolo cap
           
           fr fr Note: In real implementation, you'd measure after response
           sus memory_after = system.memory_usage()
           sus memory_diff = memory_after - memory_before
           
           lowkey memory_diff > 1024 * 1024 {  fr fr > 1MB
               vibez.spill("WARNING: High memory usage for " + request.url + ": " + memory_diff.to_string() + " bytes")
           }
       }
   }
   ```

2. **Implement resource cleanup:**
   ```cursed
   server.add_route("/api/process", slay(request) {
       sus temp_files = []
       
       yolo vibe_check {
           sus result = process_request(request)
           yolo web_vibez.Response{
               status: 200,
               headers: {},
               body: result
           }
       } catch err {
           yolo web_vibez.Response{
               status: 500,
               headers: {},
               body: "Processing failed"
           }
       } finally {
           fr fr Always clean up resources
           bestie file in temp_files {
               file_utils.delete(file)
           }
       }
   })
   ```

### Problem: Too many open connections

**Solutions:**

1. **Implement connection pooling:**
   ```cursed
   sus config = web_vibez.ServerConfig{
       host: "127.0.0.1",
       port: 8080,
       max_connections: 50,  fr fr Limit concurrent connections
       timeout: 10000        fr fr Shorter timeout to free connections faster
   }
   ```

2. **Add connection limiting middleware:**
   ```cursed
   slay connection_limit_middleware(max_connections: numo) {
       sus current_connections = 0
       
       yolo slay(request) {
           lowkey current_connections >= max_connections {
               yolo web_vibez.Response{
                   status: 503,
                   headers: {"Retry-After": "10"},
                   body: "Server too busy, try again later"
               }
           }
           
           current_connections++
           yolo cap
           fr fr Note: In real implementation, decrement on response completion
       }
   }
   ```

## Debugging Tips

### Enable Detailed Logging

```cursed
slay detailed_logging_middleware() {
    yolo slay(request) {
        sus timestamp = time_utils.now().to_string()
        sus client_ip = request.headers.get("X-Real-IP") || "unknown"
        
        vibez.spill("=== REQUEST START ===")
        vibez.spill("Time: " + timestamp)
        vibez.spill("Client: " + client_ip)
        vibez.spill("Method: " + request.method)
        vibez.spill("URL: " + request.url)
        vibez.spill("Headers: " + json_tea.encode(request.headers))
        lowkey !request.body.is_empty() {
            vibez.spill("Body length: " + request.body.len().to_string())
            vibez.spill("Body preview: " + request.body.substring(0, 200))
        }
        vibez.spill("=== REQUEST END ===")
        
        yolo cap
    }
}

server.add_middleware(detailed_logging_middleware())
```

### Test with curl

```bash
# Basic GET request
curl -v http://127.0.0.1:8080/api/test

# POST with JSON data
curl -v -X POST \
  -H "Content-Type: application/json" \
  -d '{"name": "test", "value": 123}' \
  http://127.0.0.1:8080/api/data

# With custom headers
curl -v -H "Authorization: Bearer token123" \
  http://127.0.0.1:8080/api/protected

# Test CORS
curl -v -X OPTIONS \
  -H "Origin: http://localhost:3000" \
  -H "Access-Control-Request-Method: POST" \
  http://127.0.0.1:8080/api/test
```

### Health Check Endpoint

Always add a health check endpoint for debugging:

```cursed
server.add_route("/health", slay(request) {
    sus health_info = {
        "status": "healthy",
        "timestamp": time_utils.now().to_string(),
        "server": "CURSED web_vibez",
        "version": "1.0.0",
        "uptime_seconds": get_uptime(),
        "memory_usage_mb": get_memory_usage() / 1024 / 1024,
        "active_connections": get_active_connections()
    }
    
    yolo web_vibez.Response{
        status: 200,
        headers: {"Content-Type": "application/json"},
        body: json_tea.encode(health_info)
    }
})
```

## Common Error Messages

### "Invalid request line"
- **Cause:** Malformed HTTP request
- **Solution:** Check client request format, ensure proper HTTP method and URL

### "Method not allowed"
- **Cause:** Route doesn't support the HTTP method used
- **Solution:** Check your route handler supports the method (GET, POST, etc.)

### "Rate limit exceeded"
- **Cause:** Too many requests from same client
- **Solution:** Implement exponential backoff in client, or increase rate limits

### "Request body too large"
- **Cause:** Request exceeds size limits
- **Solution:** Increase limits or reduce request size

### "Internal server error"
- **Cause:** Unhandled exception in route handler
- **Solution:** Add proper error handling and logging

## Getting Help

### Community Resources

1. **GitHub Issues:** Report bugs and request features
   - https://github.com/cursed/cursed/issues

2. **Documentation:** Check the official docs
   - [API Reference](./web_vibez_api_reference.md)
   - [Examples](../examples/)

3. **Discord Community:** Get help from other developers
   - https://discord.gg/cursed-lang

### Debug Information to Include

When asking for help, please include:

1. **CURSED version:** `cursed --version`
2. **Operating system:** Windows, macOS, Linux
3. **Minimal code example:** Reproducing the issue
4. **Error messages:** Full error output
5. **Expected vs actual behavior:** What should happen vs what does happen

### Professional Support

For production deployments and enterprise support:
- Email: support@cursed-lang.org
- Enterprise support packages available

---

Remember: Every developer faces bugs and issues. The key is to debug systematically and don't be afraid to ask for help. Your CURSED web server will be slaying in no time! 🔥✨
