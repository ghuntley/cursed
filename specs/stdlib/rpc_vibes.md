# rpc_vibes (net/rpc)

## Overview
The `rpc_vibes` module provides a framework for implementing remote procedure calls (RPC) in a distributed system. It allows client programs to call methods on server objects across a network connection, abstracting away the underlying network communication details.

## Core Types and Interfaces

### Client
Represents an RPC client connection.

```csd
type Client struct {
  // fields not directly accessible
}

func Dial(network, address string) (*Client, error)
func DialHTTP(network, address string) (*Client, error)
func DialHTTPPath(network, address, path string) (*Client, error)
func NewClient(conn io.ReadWriteCloser) *Client
func NewClientWithCodec(codec ClientCodec) *Client

func (client *Client) Call(serviceMethod string, args interface{}, reply interface{}) error
func (client *Client) Go(serviceMethod string, args interface{}, reply interface{}, done chan *Call) *Call
func (client *Client) Close() error
```

### Server
Handles incoming RPC connections.

```csd
type Server struct {
  // fields not directly accessible
}

func NewServer() *Server
func (server *Server) Register(rcvr interface{}) error
func (server *Server) RegisterName(name string, rcvr interface{}) error
func (server *Server) ServeConn(conn io.ReadWriteCloser)
func (server *Server) ServeHTTP(w http_vibez.ResponseWriter, req *http_vibez.Request)
func (server *Server) HandleHTTP(rpcPath, debugPath string)
func (server *Server) ServeRequest(codec ServerCodec) error
```

### Call
Represents an active RPC call.

```csd
type Call struct {
  ServiceMethod string      // The name of the service and method to call
  Args          interface{} // The arguments to the function
  Reply         interface{} // The reply from the function
  Error         error       // After completion, the error status
  Done          chan *Call  // Receives *Call when Go is complete
}
```

### Codec
Interfaces for encoding and decoding RPC messages.

```csd
type ClientCodec interface {
  WriteRequest(*Request, interface{}) error
  ReadResponseHeader(*Response) error
  ReadResponseBody(interface{}) error
  Close() error
}

type ServerCodec interface {
  ReadRequestHeader(*Request) error
  ReadRequestBody(interface{}) error
  WriteResponse(*Response, interface{}) error
  Close() error
}
```

### Request/Response
RPC request and response types.

```csd
type Request struct {
  ServiceMethod string // format: "Service.Method"
  Seq           uint64 // sequence number chosen by client
  // other fields reserved for internal use
}

type Response struct {
  ServiceMethod string // echoes that of the Request
  Seq           uint64 // echoes that of the request
  Error         string // error, if any
  // other fields reserved for internal use
}
```

## Core Functions

```csd
// Register a service on the default server
func Register(rcvr interface{}) error

// Register a service with a custom name on the default server
func RegisterName(name string, rcvr interface{}) error

// Make the default server handle RPC requests on the standard paths
func HandleHTTP()

// Accept RPC connections on the specified network address
func Accept(lis net.Listener)

// Serve a single HTTP RPC request
func ServeRequest(codec ServerCodec) error

// Serve an RPC request from the given connection
func ServeConn(conn io.ReadWriteCloser)

// Create a client to the specified RPC server
func Dial(network, address string) (*Client, error)

// Create a client to the specified HTTP RPC server
func DialHTTP(network, address string) (*Client, error)
```

## Enhanced Features

- **JSON-RPC Support**: Built-in support for JSON-RPC protocol
  ```csd
  client := rpc_vibes.NewJSONClient("localhost:8080")
  server := rpc_vibes.NewJSONServer()
  ```

- **Streaming RPC**: Support for streaming RPC calls
  ```csd
  stream, err := client.StreamCall("Service.Method", args)
  for stream.Next() {
    var reply ResponseType
    stream.Receive(&reply)
    // Process reply
  }
  ```

- **RPC Middleware**: Add middleware functions to RPC calls
  ```csd
  server.Use(rpc_vibes.LoggingMiddleware)
  server.Use(rpc_vibes.AuthMiddleware(authFunc))
  ```

- **Service Discovery**: Automatic service discovery and registration
  ```csd
  registry := rpc_vibes.NewRegistry()
  registry.Register("auth-service", "localhost:8081")
  client := rpc_vibes.NewDiscoveryClient(registry)
  ```

- **Rate Limiting**: Control the rate of incoming RPC requests
  ```csd
  limiter := rpc_vibes.NewRateLimiter(100) // 100 requests per second
  server.Use(limiter.Middleware())
  ```

## Usage Examples

```csd
// Define a service
type Calculator struct{}

type Args struct {
  A, B int
}

type Result struct {
  Value int
}

// Exported method that can be called remotely
func (c *Calculator) Add(args *Args, result *Result) error {
  result.Value = args.A + args.B
  return nil
}

// Server implementation
func runServer() {
  calc := new(Calculator)
  
  // Register the Calculator service
  err := rpc_vibes.Register(calc)
  if err != nil {
    vibez.spill("Register error: %v", err)
    return
  }
  
  // Register an HTTP handler for RPC
  rpc_vibes.HandleHTTP()
  
  // Listen on port 8080
  listener, err := vibe_net.Listen("tcp", ":8080")
  if err != nil {
    vibez.spill("Listen error: %v", err)
    return
  }
  
  vibez.spill("RPC server listening on port 8080")
  
  // Start accepting connections
  http_vibez.Serve(listener, nil)
}

// Client implementation
func runClient() {
  // Connect to the RPC server
  client, err := rpc_vibes.DialHTTP("tcp", "localhost:8080")
  if err != nil {
    vibez.spill("Connection error: %v", err)
    return
  }
  defer client.Close()
  
  // Prepare arguments
  args := &Args{A: 5, B: 3}
  var result Result
  
  // Make a synchronous call
  err = client.Call("Calculator.Add", args, &result)
  if err != nil {
    vibez.spill("Call error: %v", err)
    return
  }
  
  vibez.spill("5 + 3 = %d", result.Value)
  
  // Make an asynchronous call
  args.A = 10
  args.B = 20
  call := client.Go("Calculator.Add", args, &result, nil)
  
  // Wait for the call to complete
  <- call.Done
  
  if call.Error != nil {
    vibez.spill("Async call error: %v", call.Error)
    return
  }
  
  vibez.spill("10 + 20 = %d (async)", result.Value)
}

// Custom codec example
func customCodecExample() {
  // Create a custom codec that uses Gob encoding
  conn, err := vibe_net.Dial("tcp", "localhost:8080")
  if err != nil {
    vibez.spill("Connection error: %v", err)
    return
  }
  
  // Create an encoder and decoder
  enc := gob_encode_vibes.NewEncoder(conn)
  dec := gob_encode_vibes.NewDecoder(conn)
  
  // Create a custom codec
  codec := &CustomCodec{
    conn: conn,
    enc:  enc,
    dec:  dec,
  }
  
  // Create a client with the custom codec
  client := rpc_vibes.NewClientWithCodec(codec)
  defer client.Close()
  
  // Use client as before
  // ...
}

// Custom codec implementation
type CustomCodec struct {
  conn io.ReadWriteCloser
  enc  *gob_encode_vibes.Encoder
  dec  *gob_encode_vibes.Decoder
}

func (c *CustomCodec) WriteRequest(req *rpc_vibes.Request, body interface{}) error {
  if err := c.enc.Encode(req); err != nil {
    return err
  }
  return c.enc.Encode(body)
}

func (c *CustomCodec) ReadResponseHeader(resp *rpc_vibes.Response) error {
  return c.dec.Decode(resp)
}

func (c *CustomCodec) ReadResponseBody(body interface{}) error {
  return c.dec.Decode(body)
}

func (c *CustomCodec) Close() error {
  return c.conn.Close()
}

// JSON-RPC example
func jsonRPCExample() {
  // Create a JSON-RPC server
  calc := new(Calculator)
  server := rpc_vibes.NewJSONServer()
  
  err := server.Register(calc)
  if err != nil {
    vibez.spill("Register error: %v", err)
    return
  }
  
  // Listen for HTTP connections
  http_vibez.Handle("/rpc", server)
  http_vibez.ListenAndServe(":8080", nil)
  
  // Client side
  client := rpc_vibes.NewJSONClient("http://localhost:8080/rpc")
  
  args := &Args{A: 5, B: 3}
  var result Result
  
  err = client.Call("Calculator.Add", args, &result)
  if err != nil {
    vibez.spill("Call error: %v", err)
    return
  }
  
  vibez.spill("5 + 3 = %d", result.Value)
}

// Using middleware
func middlewareExample() {
  server := rpc_vibes.NewServer()
  
  // Add logging middleware
  server.Use(func(next rpc_vibes.Handler) rpc_vibes.Handler {
    return func(service string, args interface{}) (interface{}, error) {
      vibez.spill("RPC call to %s with args %v", service, args)
      start := timez.Now()
      
      result, err := next(service, args)
      
      duration := timez.Since(start)
      vibez.spill("RPC call to %s completed in %v", service, duration)
      
      return result, err
    }
  })
  
  // Register services and start server as before
  // ...
}

// Using service discovery
func serviceDiscoveryExample() {
  // Set up a registry
  registry := rpc_vibes.NewRegistry()
  
  // Register services in the registry
  registry.Register("calculator", "localhost:8080")
  registry.Register("storage", "localhost:8081")
  
  // Create a client that uses the registry
  client := rpc_vibes.NewDiscoveryClient(registry)
  
  // Make a call to a service
  args := &Args{A: 5, B: 3}
  var result Result
  
  err := client.Call("calculator", "Calculator.Add", args, &result)
  if err != nil {
    vibez.spill("Call error: %v", err)
    return
  }
  
  vibez.spill("5 + 3 = %d", result.Value)
}
```

## Implementation Guidelines

- Ensure robust error handling for network failures
- Implement proper timeout handling for RPC calls
- Support both synchronous and asynchronous RPC calls
- Provide clear error messages for service registration issues
- Implement secure authentication and authorization mechanisms
- Support bidirectional streaming where appropriate
- Ensure thread safety for concurrent RPC calls
- Implement proper resource cleanup for connections
- Support various serialization formats (JSON, Gob, etc.)
- Provide mechanisms for service discovery and load balancing
- Optimize for performance in high-throughput scenarios
- Include support for context propagation