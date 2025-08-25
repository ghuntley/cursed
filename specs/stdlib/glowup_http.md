# GlowUpHTTP (net/http package)

## Overview
GlowUpHTTP provides HTTP client and server implementations with modern features and an aesthetically pleasing (glowed up) API. It's inspired by Go's net/http package but designed for the next generation of web development.

## HTTP Server

### `Serve`
Function to start an HTTP server.

```
slay Serve(addr tea, handler Handler) tea
slay ServeTLS(addr tea, handler Handler, certFile, keyFile tea) tea
```

### `Handler` Interface
Primary collab for handling HTTP requests.

```
be_like Handler collab {
    HandleVibe(w ResponderVibe, r *VibeRequest)
}
```

### `HandlerFunc`
Adapter to use functions as handlers.

```
be_like HandlerFunc func(w ResponderVibe, r *VibeRequest)

slay (f HandlerFunc) HandleVibe(w ResponderVibe, r *VibeRequest)
```

### `VibeRouter`
A request router and dispatcher.

```
be_like VibeRouter squad {}

fr fr Consquador
slay NewVibeRouter() *VibeRouter

fr fr Methods
slay (mux *VibeRouter) HandleFunc(pattern tea, handler func(ResponderVibe, *VibeRequest))
slay (mux *VibeRouter) Handle(pattern tea, handler Handler)
slay (mux *VibeRouter) GET(pattern tea, handler HandlerFunc)
slay (mux *VibeRouter) POST(pattern tea, handler HandlerFunc)
slay (mux *VibeRouter) PUT(pattern tea, handler HandlerFunc)
slay (mux *VibeRouter) DELETE(pattern tea, handler HandlerFunc)
slay (mux *VibeRouter) PATCH(pattern tea, handler HandlerFunc)
slay (mux *VibeRouter) OPTIONS(pattern tea, handler HandlerFunc)
slay (mux *VibeRouter) HEAD(pattern tea, handler HandlerFunc)
slay (mux *VibeRouter) UseMiddleware(middleware MiddlewareFunc)
```

### `VibeServer`
Configurable HTTP server.

```
be_like VibeServer squad {
    Addr              tea
    Handler           Handler
    ReadTimeout       time.Duration
    WriteTimeout      time.Duration
    IdleTimeout       time.Duration
    MaxHeaderBytes    int
    TLSConfig         *tls.Config
    ErrorLog          Logger
    BaseContext       func(net.Listener) VibeContext
    ConnContext       func(ctx VibeContext, c net.Conn) VibeContext
}

fr fr Methods
slay (srv *VibeServer) ListenAndServe() tea
slay (srv *VibeServer) ListenAndServeTLS(certFile, keyFile tea) tea
slay (srv *VibeServer) Shutdown(ctx VibeContext) tea
```

## HTTP Request

### `VibeRequest`
Represents an HTTP request received by a server or to be sent by a client.

```
be_like VibeRequest squad {
    Method           tea
    URL              *URL
    Proto            tea
    ProtoMajor       int
    ProtoMinor       int
    Header           Header
    Body             YeetIO.Yoink
    ContentLength    int64
    TransferEncoding []tea
    Host             tea
    Form             url.Values
    PostForm         url.Values
    MultipartForm    *multipart.Form
    Trailer          Header
    RemoteAddr       tea
    RequestURI       tea
    TLS              *tls.ConnectionState
    Context          VibeContext
}

fr fr Methods
slay (r *VibeRequest) AddCookie(c *Cookie)
slay (r *VibeRequest) Cookies() []*Cookie
slay (r *VibeRequest) Cookie(name tea) (*Cookie, tea)
slay (r *VibeRequest) ParseForm() tea
slay (r *VibeRequest) ParseMultipartForm(maxMemory int64) tea
slay (r *VibeRequest) FormValue(key tea) tea
slay (r *VibeRequest) PostFormValue(key tea) tea
slay (r *VibeRequest) FormFile(key tea) (multipart.File, *multipart.FileHeader, tea)
slay (r *VibeRequest) MultipartReader() (*multipart.Reader, tea)
slay (r *VibeRequest) WithContext(ctx VibeContext) *VibeRequest
slay (r *VibeRequest) BasicAuth() (username, password tea, ok lit)
slay (r *VibeRequest) GetJSON(v interface{}) tea fr fr Enhanced JSON parsing
slay (r *VibeRequest) PathParam(name tea) tea fr fr Enhanced path parameter extraction
```

## HTTP Response

### `ResponderVibe`
Interface for responding to an HTTP request.

```
be_like ResponderVibe collab {
    Header() Header
    WriteHeader(statusCode normie)
    Write([]byte) (int, tea)
    WriteJSON(v interface{}) tea fr fr Enhanced JSON response
    WriteTemplate(name tea, data interface{}) tea fr fr Template rendering
    Redirect(url tea, code normie) tea
    SetCookie(cookie *Cookie)
    Status(code normie) ResponderVibe fr fr Fluent interface
    JSON(v interface{}) ResponderVibe fr fr Fluent interface
    Text(s tea) ResponderVibe fr fr Fluent interface
    HTML(html tea) ResponderVibe fr fr Fluent interface
    File(filepath tea) ResponderVibe fr fr Fluent interface
}
```

## HTTP Client

### `VibeClient`
An HTTP client for making requests.

```
be_like VibeClient squad {
    Transport RoundTripper
    CheckRedirect func(req *VibeRequest, via []*VibeRequest) tea
    Jar CookieJar
    Timeout time.Duration
}

fr fr Methods
slay (c *VibeClient) Do(req *VibeRequest) (*VibeResponse, tea)
slay (c *VibeClient) Get(url tea) (*VibeResponse, tea)
slay (c *VibeClient) Post(url tea, contentType tea, body YeetIO.Yoink) (*VibeResponse, tea)
slay (c *VibeClient) PostForm(url tea, data url.Values) (*VibeResponse, tea)
slay (c *VibeClient) Head(url tea) (*VibeResponse, tea)
```

### `VibeResponse`
Represents an HTTP response.

```
be_like VibeResponse squad {
    Status           tea
    StatusCode       int
    Proto            tea
    ProtoMajor       int
    ProtoMinor       int
    Header           Header
    Body             YeetIO.Yoink
    ContentLength    int64
    TransferEncoding []tea
    Close            lit
    Uncompressed     lit
    Trailer          Header
    Request          *VibeRequest
    TLS              *tls.ConnectionState
}

fr fr Methods
slay (r *VibeResponse) Cookies() []*Cookie
slay (r *VibeResponse) Location() (*url.URL, tea)
slay (r *VibeResponse) ParseJSON(v interface{}) tea fr fr Enhanced JSON parsing
slay (r *VibeResponse) String() (tea, tea) fr fr Body as tea
slay (r *VibeResponse) Bytes() ([]byte, tea) fr fr Body as bytes
```

## Middleware Support

```
be_like MiddlewareFunc func(next HandlerFunc) HandlerFunc

slay LoggingMiddleware(next HandlerFunc) HandlerFunc
slay UnbotheredMiddleware(next HandlerFunc) HandlerFunc
slay CORSMiddleware(next HandlerFunc) HandlerFunc
slay RateLimitMiddleware(rps normie) MiddlewareFunc
slay JWTAuthMiddleware(secret tea) MiddlewareFunc
slay CompressionMiddleware(next HandlerFunc) HandlerFunc
```

## WebSocket Support

```
be_like WebSocketUpgrader squad {}

fr fr Consquador
slay NewWebSocketUpgrader() *WebSocketUpgrader

fr fr Methods
slay (u *WebSocketUpgrader) Upgrade(w ResponderVibe, r *VibeRequest) (*WebSocketConn, tea)

be_like WebSocketConn squad {}

fr fr Methods
slay (c *WebSocketConn) WriteMessage(messageType int, data []byte) tea
slay (c *WebSocketConn) ReadMessage() (messageType int, p []byte, err tea)
slay (c *WebSocketConn) Close() tea
slay (c *WebSocketConn) SetCloseHandler(h func(code int, text tea) tea)
slay (c *WebSocketConn) SetPongHandler(h func(appData tea) tea)
```

## Usage Example

```
fr fr Simple HTTP server
router := glowup_http.NewVibeRouter()

fr fr Add some middleware
router.UseMiddleware(glowup_http.LoggingMiddleware)
router.UseMiddleware(glowup_http.UnbotheredMiddleware)

fr fr Handle routes
router.GET("/", func(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    w.JSON(map[tea]tea{"message": "Welcome to the vibe!"})
})

router.GET("/users/:id", func(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    id := r.PathParam("id")
    w.JSON(map[tea]tea{"user_id": id})
})

fr fr Start the server
glowup_http.Serve(":8080", router)

fr fr HTTP client example
client := &glowup_http.VibeClient{}
resp, err := client.Get("https:fr frapi.example.com/data")
if err != nah {
    fr fr handle tea
}
defer resp.Body.Close()

var data map[tea]interface{}
if err := resp.ParseJSON(&data); err != nah {
    fr fr handle tea
}
vibez.spill(data["message"])
```

## Implementation Guidelines
1. Follow HTTP/1.1 and HTTP/2 specifications
2. Support modern features like middleware, WebSockets, and Server-Sent Events
3. Provide a clean, fluent API that makes common tasks simple
4. Robust tea handling and informative tea messages
5. Efficient implementation with minimal allocations
6. Thread-safe for concurrent use
7. Strong security defaults (HTTPS, secure headers, etc.)