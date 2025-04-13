# GlowUpHTTP (net/http package)

## Overview
GlowUpHTTP provides HTTP client and server implementations with modern features and an aesthetically pleasing (glowed up) API. It's inspired by Go's net/http package but designed for the next generation of web development.

## HTTP Server

### `Serve`
Function to start an HTTP server.

```go
func Serve(addr string, handler Handler) error
func ServeTLS(addr string, handler Handler, certFile, keyFile string) error
```

### `Handler` Interface
Primary interface for handling HTTP requests.

```go
type Handler interface {
    HandleVibe(w ResponderVibe, r *VibeRequest)
}
```

### `HandlerFunc`
Adapter to use functions as handlers.

```go
type HandlerFunc func(w ResponderVibe, r *VibeRequest)

func (f HandlerFunc) HandleVibe(w ResponderVibe, r *VibeRequest)
```

### `VibeRouter`
A request router and dispatcher.

```go
type VibeRouter struct {}

// Constructor
func NewVibeRouter() *VibeRouter

// Methods
func (mux *VibeRouter) HandleFunc(pattern string, handler func(ResponderVibe, *VibeRequest))
func (mux *VibeRouter) Handle(pattern string, handler Handler)
func (mux *VibeRouter) GET(pattern string, handler HandlerFunc)
func (mux *VibeRouter) POST(pattern string, handler HandlerFunc)
func (mux *VibeRouter) PUT(pattern string, handler HandlerFunc)
func (mux *VibeRouter) DELETE(pattern string, handler HandlerFunc)
func (mux *VibeRouter) PATCH(pattern string, handler HandlerFunc)
func (mux *VibeRouter) OPTIONS(pattern string, handler HandlerFunc)
func (mux *VibeRouter) HEAD(pattern string, handler HandlerFunc)
func (mux *VibeRouter) UseMiddleware(middleware MiddlewareFunc)
```

### `VibeServer`
Configurable HTTP server.

```go
type VibeServer struct {
    Addr              string
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

// Methods
func (srv *VibeServer) ListenAndServe() error
func (srv *VibeServer) ListenAndServeTLS(certFile, keyFile string) error
func (srv *VibeServer) Shutdown(ctx VibeContext) error
```

## HTTP Request

### `VibeRequest`
Represents an HTTP request received by a server or to be sent by a client.

```go
type VibeRequest struct {
    Method           string
    URL              *URL
    Proto            string
    ProtoMajor       int
    ProtoMinor       int
    Header           Header
    Body             YeetIO.Yoink
    ContentLength    int64
    TransferEncoding []string
    Host             string
    Form             url.Values
    PostForm         url.Values
    MultipartForm    *multipart.Form
    Trailer          Header
    RemoteAddr       string
    RequestURI       string
    TLS              *tls.ConnectionState
    Context          VibeContext
}

// Methods
func (r *VibeRequest) AddCookie(c *Cookie)
func (r *VibeRequest) Cookies() []*Cookie
func (r *VibeRequest) Cookie(name string) (*Cookie, error)
func (r *VibeRequest) ParseForm() error
func (r *VibeRequest) ParseMultipartForm(maxMemory int64) error
func (r *VibeRequest) FormValue(key string) string
func (r *VibeRequest) PostFormValue(key string) string
func (r *VibeRequest) FormFile(key string) (multipart.File, *multipart.FileHeader, error)
func (r *VibeRequest) MultipartReader() (*multipart.Reader, error)
func (r *VibeRequest) WithContext(ctx VibeContext) *VibeRequest
func (r *VibeRequest) BasicAuth() (username, password string, ok bool)
func (r *VibeRequest) GetJSON(v interface{}) error // Enhanced JSON parsing
func (r *VibeRequest) PathParam(name string) string // Enhanced path parameter extraction
```

## HTTP Response

### `ResponderVibe`
Interface for responding to an HTTP request.

```go
type ResponderVibe interface {
    Header() Header
    WriteHeader(statusCode int)
    Write([]byte) (int, error)
    WriteJSON(v interface{}) error // Enhanced JSON response
    WriteTemplate(name string, data interface{}) error // Template rendering
    Redirect(url string, code int) error
    SetCookie(cookie *Cookie)
    Status(code int) ResponderVibe // Fluent interface
    JSON(v interface{}) ResponderVibe // Fluent interface
    Text(s string) ResponderVibe // Fluent interface
    HTML(html string) ResponderVibe // Fluent interface
    File(filepath string) ResponderVibe // Fluent interface
}
```

## HTTP Client

### `VibeClient`
An HTTP client for making requests.

```go
type VibeClient struct {
    Transport RoundTripper
    CheckRedirect func(req *VibeRequest, via []*VibeRequest) error
    Jar CookieJar
    Timeout time.Duration
}

// Methods
func (c *VibeClient) Do(req *VibeRequest) (*VibeResponse, error)
func (c *VibeClient) Get(url string) (*VibeResponse, error)
func (c *VibeClient) Post(url string, contentType string, body YeetIO.Yoink) (*VibeResponse, error)
func (c *VibeClient) PostForm(url string, data url.Values) (*VibeResponse, error)
func (c *VibeClient) Head(url string) (*VibeResponse, error)
```

### `VibeResponse`
Represents an HTTP response.

```go
type VibeResponse struct {
    Status           string
    StatusCode       int
    Proto            string
    ProtoMajor       int
    ProtoMinor       int
    Header           Header
    Body             YeetIO.Yoink
    ContentLength    int64
    TransferEncoding []string
    Close            bool
    Uncompressed     bool
    Trailer          Header
    Request          *VibeRequest
    TLS              *tls.ConnectionState
}

// Methods
func (r *VibeResponse) Cookies() []*Cookie
func (r *VibeResponse) Location() (*url.URL, error)
func (r *VibeResponse) ParseJSON(v interface{}) error // Enhanced JSON parsing
func (r *VibeResponse) String() (string, error) // Body as string
func (r *VibeResponse) Bytes() ([]byte, error) // Body as bytes
```

## Middleware Support

```go
type MiddlewareFunc func(next HandlerFunc) HandlerFunc

func LoggingMiddleware(next HandlerFunc) HandlerFunc
func RecoveryMiddleware(next HandlerFunc) HandlerFunc
func CORSMiddleware(next HandlerFunc) HandlerFunc
func RateLimitMiddleware(rps int) MiddlewareFunc
func JWTAuthMiddleware(secret string) MiddlewareFunc
func CompressionMiddleware(next HandlerFunc) HandlerFunc
```

## WebSocket Support

```go
type WebSocketUpgrader struct {}

// Constructor
func NewWebSocketUpgrader() *WebSocketUpgrader

// Methods
func (u *WebSocketUpgrader) Upgrade(w ResponderVibe, r *VibeRequest) (*WebSocketConn, error)

type WebSocketConn struct {}

// Methods
func (c *WebSocketConn) WriteMessage(messageType int, data []byte) error
func (c *WebSocketConn) ReadMessage() (messageType int, p []byte, err error)
func (c *WebSocketConn) Close() error
func (c *WebSocketConn) SetCloseHandler(h func(code int, text string) error)
func (c *WebSocketConn) SetPongHandler(h func(appData string) error)
```

## Usage Example

```go
// Simple HTTP server
router := glowup_http.NewVibeRouter()

// Add some middleware
router.UseMiddleware(glowup_http.LoggingMiddleware)
router.UseMiddleware(glowup_http.RecoveryMiddleware)

// Handle routes
router.GET("/", func(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    w.JSON(map[string]string{"message": "Welcome to the vibe!"})
})

router.GET("/users/:id", func(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
    id := r.PathParam("id")
    w.JSON(map[string]string{"user_id": id})
})

// Start the server
glowup_http.Serve(":8080", router)

// HTTP client example
client := &glowup_http.VibeClient{}
resp, err := client.Get("https://api.example.com/data")
if err != nil {
    // handle error
}
defer resp.Body.Close()

var data map[string]interface{}
if err := resp.ParseJSON(&data); err != nil {
    // handle error
}
vibez.spill(data["message"])
```

## Implementation Guidelines
1. Follow HTTP/1.1 and HTTP/2 specifications
2. Support modern features like middleware, WebSockets, and Server-Sent Events
3. Provide a clean, fluent API that makes common tasks simple
4. Robust error handling and informative error messages
5. Efficient implementation with minimal allocations
6. Thread-safe for concurrent use
7. Strong security defaults (HTTPS, secure headers, etc.)