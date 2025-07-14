# GlowUpHTTP Module

Modern HTTP server and client library for CURSED with Gen Z enhanced APIs, WebSocket support, and comprehensive middleware system.

## Overview

GlowUpHTTP provides a complete HTTP/1.1 and HTTP/2 implementation with:
- 🚀 High-performance HTTP server with routing
- 💫 Fluent client API for HTTP requests
- 🔌 WebSocket support for real-time communication
- 🛡️ Comprehensive middleware system
- 🎯 Type-safe request/response handling
- 💅 Gen Z enhanced APIs with fluent interfaces
- 🔐 Built-in security features (CORS, CSRF, rate limiting)

## Quick Start

### HTTP Server

```cursed
yeet "glowup_http"

fr fr Create router with middleware
router := glowup_http.NewVibeRouter()
router.UseMiddleware(glowup_http.LoggingMiddleware)
router.UseMiddleware(glowup_http.CORSMiddleware)

fr fr Add routes
router.GET("/", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    w.JSON(map[tea]tea{"message": "Welcome to the vibe!"})
})

router.GET("/users/:id", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    id := r.PathParam("id")
    w.JSON(map[tea]tea{"user_id": id})
})

fr fr Start server
glowup_http.Serve(":8080", router)
```

### HTTP Client

```cursed
yeet "glowup_http"

fr fr Create client
client := glowup_http.NewVibeClient()

fr fr GET request
resp, err := client.Get("https://api.example.com/users")
lowkey err == "" {
    body, _ := resp.String()
    vibez.spill("Response: " + body)
}

fr fr POST with form data
formData := map[tea]tea{
    "name": "John Doe",
    "email": "john@example.com",
}
postResp, _ := client.PostForm("https://api.example.com/users", formData)
```

### WebSocket Support

```cursed
yeet "glowup_http"

fr fr WebSocket handler
router.GET("/ws", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    upgrader := glowup_http.NewWebSocketUpgrader()
    conn, err := upgrader.Upgrade(w, r)
    
    lowkey err == "" {
        fr fr Send welcome message
        conn.WriteMessage(glowup_http.WS_TEXT_MESSAGE, 
            stringz.to_bytes("Welcome to WebSocket!"))
        
        fr fr Read messages
        msgType, data, readErr := conn.ReadMessage()
        lowkey readErr == "" {
            vibez.spill("Received: " + stringz.from_bytes(data))
        }
        
        conn.Close()
    }
})
```

## Core Components

### VibeRouter

The main router for handling HTTP requests with pattern matching and middleware support.

```cursed
router := glowup_http.NewVibeRouter()

fr fr HTTP methods
router.GET("/path", handler)
router.POST("/path", handler)
router.PUT("/path", handler)
router.DELETE("/path", handler)
router.PATCH("/path", handler)
router.OPTIONS("/path", handler)
router.HEAD("/path", handler)

fr fr Middleware
router.UseMiddleware(middlewareFunc)

fr fr Generic route handling
router.HandleFunc("/path", handler)
router.Handle("/path", handlerInterface)
```

### VibeServer

Configurable HTTP server with advanced options.

```cursed
server := &glowup_http.VibeServer{
    Addr:         ":8080",
    Handler:      router,
    ReadTimeout:  30000,  fr fr 30 seconds
    WriteTimeout: 30000,
    IdleTimeout:  120000, fr fr 2 minutes
}

fr fr Start server
err := server.ListenAndServe()

fr fr TLS server
err := server.ListenAndServeTLS("cert.pem", "key.pem")

fr fr Graceful shutdown
err := server.Shutdown(context)
```

### VibeRequest

HTTP request object with enhanced functionality.

```cursed
fr fr In handler function
slay myHandler(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    fr fr Basic properties
    method := r.Method
    path := r.URL.Path
    host := r.Host
    
    fr fr Headers
    userAgent := r.Header["User-Agent"][0]
    
    fr fr Path parameters
    id := r.PathParam("id")
    
    fr fr Form handling
    r.ParseForm()
    name := r.FormValue("name")
    email := r.PostFormValue("email")
    
    fr fr Cookies
    sessionCookie, err := r.Cookie("session")
    cookies := r.Cookies()
    
    fr fr Basic authentication
    username, password, ok := r.BasicAuth()
    
    fr fr JSON parsing
    var data map[tea]interface{}
    err := r.GetJSON(&data)
}
```

### ResponderVibe

Response writer with fluent API for building HTTP responses.

```cursed
fr fr Basic response
w.WriteHeader(glowup_http.HTTP_OK)
w.Write(stringz.to_bytes("Hello World"))

fr fr Fluent API
w.Status(glowup_http.HTTP_OK).JSON(map[tea]tea{"message": "Success"})
w.Status(glowup_http.HTTP_CREATED).Text("Resource created")
w.Status(glowup_http.HTTP_OK).HTML("<h1>Hello HTML</h1>")
w.Status(glowup_http.HTTP_OK).File("/path/to/file.txt")

fr fr Headers and cookies
w.Header()["Content-Type"] = []tea{"application/json"}
w.SetCookie(cookie)
w.Redirect("https://example.com", glowup_http.HTTP_FOUND)

fr fr JSON response
w.WriteJSON(map[tea]tea{"data": "value"})

fr fr Template rendering
w.WriteTemplate("template.html", templateData)
```

### VibeClient

HTTP client for making requests with automatic handling of common scenarios.

```cursed
client := glowup_http.NewVibeClient()
client.Timeout = 60000  fr fr 60 seconds
client.UserAgent = "MyApp/1.0"

fr fr Custom headers
client.Headers["Authorization"] = []tea{"Bearer token123"}

fr fr HTTP methods
resp, err := client.Get(url)
resp, err := client.Post(url, contentType, body)
resp, err := client.PostForm(url, formData)
resp, err := client.Head(url)

fr fr Custom request
req := &glowup_http.VibeRequest{
    Method: "PATCH",
    URL:    glowup_http.parseURL(url),
    Header: customHeaders,
    Body:   requestBody,
}
resp, err := client.Do(req)
```

### VibeResponse

HTTP response object with parsing utilities.

```cursed
fr fr Response properties
status := resp.Status
statusCode := resp.StatusCode
headers := resp.Header

fr fr Body parsing
body, err := resp.String()
bytes, err := resp.Bytes()
err := resp.ParseJSON(&dataStruct)

fr fr Cookies
cookies := resp.Cookies()
```

## Middleware System

### Built-in Middleware

```cursed
fr fr Logging middleware
router.UseMiddleware(glowup_http.LoggingMiddleware)

fr fr Security headers
router.UseMiddleware(glowup_http.UnbotheredMiddleware)

fr fr CORS support
router.UseMiddleware(glowup_http.CORSMiddleware)

fr fr Rate limiting
router.UseMiddleware(glowup_http.RateLimitMiddleware(100)) fr fr 100 requests per second

fr fr JWT authentication
router.UseMiddleware(glowup_http.JWTAuthMiddleware("secret-key"))

fr fr Compression
router.UseMiddleware(glowup_http.CompressionMiddleware)
```

### Custom Middleware

```cursed
fr fr Define middleware function
customMiddleware := slay(next glowup_http.HandlerFunc) glowup_http.HandlerFunc {
    damn slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        fr fr Pre-processing
        vibez.spill("Before handler")
        
        fr fr Call next handler
        next(w, r)
        
        fr fr Post-processing
        vibez.spill("After handler")
    }
}

router.UseMiddleware(customMiddleware)
```

## WebSocket Support

### WebSocket Server

```cursed
fr fr Create upgrader
upgrader := glowup_http.NewWebSocketUpgrader()

fr fr Custom origin checker
upgrader.CheckOrigin = slay(r *glowup_http.VibeRequest) lit {
    origin := r.Header["Origin"][0]
    damn stringz.contains(origin, "trusted-domain.com")
}

fr fr WebSocket route
router.GET("/ws", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    conn, err := upgrader.Upgrade(w, r)
    lowkey err != "" {
        vibez.spill("WebSocket upgrade failed: " + err)
        damn
    }
    
    fr fr Handle connection
    handleWebSocket(conn)
})

slay handleWebSocket(conn *glowup_http.WebSocketConn) {
    fr fr Set handlers
    conn.SetCloseHandler(slay(code normie, text tea) tea {
        vibez.spill("Connection closed: " + stringz.itoa(code))
        damn ""
    })
    
    conn.SetPongHandler(slay(appData tea) tea {
        vibez.spill("Pong received: " + appData)
        damn ""
    })
    
    fr fr Message loop
    bestie {
        msgType, data, err := conn.ReadMessage()
        lowkey err != "" {
            ghosted
        }
        
        lowkey msgType == glowup_http.WS_TEXT_MESSAGE {
            message := stringz.from_bytes(data)
            vibez.spill("Received: " + message)
            
            fr fr Echo message back
            response := "Echo: " + message
            conn.WriteMessage(glowup_http.WS_TEXT_MESSAGE, stringz.to_bytes(response))
        }
    }
    
    conn.Close()
}
```

### WebSocket Message Types

```cursed
fr fr Available message types
glowup_http.WS_TEXT_MESSAGE    fr fr 1 - Text message
glowup_http.WS_BINARY_MESSAGE  fr fr 2 - Binary message
glowup_http.WS_CLOSE_MESSAGE   fr fr 8 - Close message
glowup_http.WS_PING_MESSAGE    fr fr 9 - Ping message
glowup_http.WS_PONG_MESSAGE    fr fr 10 - Pong message
```

## Route Patterns

### Path Parameters

```cursed
fr fr Single parameter
router.GET("/users/:id", handler)           fr fr Matches /users/123
router.GET("/posts/:postId/comments/:id", handler)  fr fr Matches /posts/456/comments/789

fr fr Extract parameters in handler
slay handler(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    id := r.PathParam("id")
    postId := r.PathParam("postId")
}
```

### Wildcard Routes

```cursed
fr fr Catch-all route (handle this in route matching logic)
router.GET("/static/*", staticFileHandler)
```

## Security Features

### CORS (Cross-Origin Resource Sharing)

```cursed
fr fr Enable CORS for all routes
router.UseMiddleware(glowup_http.CORSMiddleware)

fr fr Custom CORS middleware
corsMiddleware := slay(next glowup_http.HandlerFunc) glowup_http.HandlerFunc {
    damn slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        w.Header()["Access-Control-Allow-Origin"] = []tea{"https://trusted-domain.com"}
        w.Header()["Access-Control-Allow-Methods"] = []tea{"GET, POST, PUT, DELETE"}
        w.Header()["Access-Control-Allow-Headers"] = []tea{"Content-Type, Authorization"}
        
        lowkey r.Method == "OPTIONS" {
            w.WriteHeader(glowup_http.HTTP_OK)
            damn
        }
        
        next(w, r)
    }
}
```

### Security Headers

```cursed
fr fr Built-in security middleware
router.UseMiddleware(glowup_http.UnbotheredMiddleware)

fr fr Adds these headers automatically:
fr fr X-Content-Type-Options: nosniff
fr fr X-Frame-Options: DENY
fr fr X-XSS-Protection: 1; mode=block
fr fr Strict-Transport-Security: max-age=31536000; includeSubDomains
```

### Rate Limiting

```cursed
fr fr Rate limiting middleware
router.UseMiddleware(glowup_http.RateLimitMiddleware(100))  fr fr 100 RPS

fr fr Custom rate limiting
rateLimitMiddleware := slay(limit normie) glowup_http.MiddlewareFunc {
    damn slay(next glowup_http.HandlerFunc) glowup_http.HandlerFunc {
        damn slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
            fr fr Check rate limit logic
            lowkey exceedsRateLimit(r.RemoteAddr) {
                w.Status(429).JSON(map[tea]tea{"error": "Rate limit exceeded"})
                damn
            }
            
            next(w, r)
        }
    }
}
```

### JWT Authentication

```cursed
fr fr JWT middleware
router.UseMiddleware(glowup_http.JWTAuthMiddleware("your-secret-key"))

fr fr Protected routes
router.GET("/protected", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    fr fr JWT validated automatically by middleware
    w.JSON(map[tea]tea{"message": "Access granted"})
})
```

## HTTP Status Codes

```cursed
fr fr Success codes
glowup_http.HTTP_OK                    fr fr 200
glowup_http.HTTP_CREATED              fr fr 201

fr fr Client error codes
glowup_http.HTTP_BAD_REQUEST          fr fr 400
glowup_http.HTTP_UNAUTHORIZED         fr fr 401
glowup_http.HTTP_FORBIDDEN            fr fr 403
glowup_http.HTTP_NOT_FOUND            fr fr 404

fr fr Server error codes
glowup_http.HTTP_INTERNAL_ERROR       fr fr 500
glowup_http.HTTP_BAD_GATEWAY          fr fr 502
glowup_http.HTTP_SERVICE_UNAVAILABLE  fr fr 503
```

## Cookie Handling

### Setting Cookies

```cursed
cookie := &glowup_http.Cookie{
    Name:     "session_id",
    Value:    "abc123xyz",
    Path:     "/",
    Domain:   ".example.com",
    MaxAge:   3600,        fr fr 1 hour
    Secure:   based,       fr fr HTTPS only
    HttpOnly: based,       fr fr No JavaScript access
    SameSite: "Strict",    fr fr CSRF protection
}

w.SetCookie(cookie)
```

### Reading Cookies

```cursed
fr fr Get specific cookie
sessionCookie, err := r.Cookie("session_id")
lowkey err == "" {
    sessionId := sessionCookie.Value
}

fr fr Get all cookies
cookies := r.Cookies()
bestie _, cookie := range cookies {
    vibez.spill(cookie.Name + "=" + cookie.Value)
}
```

## Form Data Processing

### URL-Encoded Forms

```cursed
fr fr Parse form data
err := r.ParseForm()

fr fr Get form values
name := r.FormValue("name")        fr fr From URL query or POST body
email := r.PostFormValue("email")  fr fr Only from POST body

fr fr Get multiple values
tags := r.Form["tags"]  fr fr []tea
```

### JSON Request Bodies

```cursed
fr fr Parse JSON request
var requestData map[tea]interface{}
err := r.GetJSON(&requestData)

lowkey err == "" {
    name := requestData["name"].(tea)
    age := requestData["age"].(normie)
}
```

## Error Handling

### Custom Error Pages

```cursed
fr fr Custom 404 handler
router.notFound = slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    w.Status(glowup_http.HTTP_NOT_FOUND).JSON(map[tea]tea{
        "error": "Page not found",
        "path": r.URL.Path,
        "suggestion": "Check the URL and try again",
    })
}
```

### Error Middleware

```cursed
errorMiddleware := slay(next glowup_http.HandlerFunc) glowup_http.HandlerFunc {
    damn slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        fr fr Panic recovery
        defer slay() {
            lowkey recover() != cap {
                w.Status(glowup_http.HTTP_INTERNAL_ERROR).JSON(map[tea]tea{
                    "error": "Internal server error",
                    "timestamp": timez.now_iso(),
                })
            }
        }()
        
        next(w, r)
    }
}
```

## Performance Tips

1. **Use Middleware Efficiently**: Order middleware by frequency of execution
2. **Connection Pooling**: Reuse HTTP client instances
3. **Streaming**: Use streaming for large request/response bodies
4. **Compression**: Enable compression middleware for text responses
5. **Caching**: Implement appropriate caching headers

## Gen Z API Features

The GlowUpHTTP module includes Gen Z enhanced APIs that make HTTP development more intuitive and enjoyable:

### Fluent Response Building

```cursed
fr fr Chain operations for cleaner code
w.Status(glowup_http.HTTP_OK).
  JSON(map[tea]tea{"vibe": "immaculate"})

w.Status(glowup_http.HTTP_CREATED).
  Text("User created - that's fire! 🔥")
```

### Intuitive Method Names

- `ResponderVibe` instead of `ResponseWriter`
- `VibeRequest` instead of `Request` 
- `VibeRouter` instead of `ServeMux`
- `VibeServer` instead of `Server`
- `VibeClient` instead of `Client`

### Modern Error Messages

Error responses include helpful, modern language:

```cursed
w.Status(glowup_http.HTTP_NOT_FOUND).JSON(map[tea]tea{
    "error": "Route not found",
    "vibe": "This endpoint doesn't exist",
    "suggestion": "Check your URL - no cap!"
})
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/glowup_http/test_glowup_http.csd
```

The test suite covers:
- ✅ HTTP server and client functionality
- ✅ WebSocket connections and messaging
- ✅ Middleware chain execution
- ✅ Route matching and parameter extraction
- ✅ Cookie and form data handling
- ✅ Security features (CORS, rate limiting, JWT)
- ✅ Error handling and status codes
- ✅ Gen Z enhanced APIs and fluent interfaces

## Examples

Check out the comprehensive examples in the test file and the demo functions:

- `glowup_http.demo_http_server()` - Complete HTTP server setup
- `glowup_http.demo_http_client()` - HTTP client usage examples  
- `glowup_http.demo_websocket()` - WebSocket implementation example

## License

This module is part of the CURSED programming language standard library.

---

**GlowUpHTTP** - Making HTTP development absolutely iconic! 💯🔥✨
