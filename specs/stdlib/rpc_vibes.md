# rpc_vibes (net/rpc)

## Overview
The `rpc_vibes` module provides a framework for implementing remote procedure calls (RPC) in a distributed system. It allows client programs to call methods on server objects across a network connection, abstracting away the underlying network communication details.

## Core Types and Interfaces

### Client
Represents an RPC client connection.

```csd
be_like Client squad {
  fr fr fields not directly accessible
}

slay Dial(network, address tea) (*Client, tea)
slay DialHTTP(network, address tea) (*Client, tea)
slay DialHTTPPath(network, address, path tea) (*Client, tea)
slay NewClient(conn io.ReadWriteCloser) *Client
slay NewClientWithCodec(codec ClientCodec) *Client

slay (client *Client) Call(serviceMethod tea, args interface{}, reply interface{}) tea
slay (client *Client) Go(serviceMethod tea, args interface{}, reply interface{}, done chan *Call) *Call
slay (client *Client) Close() tea
```

### Server
Handles incoming RPC connections.

```csd
be_like Server squad {
  fr fr fields not directly accessible
}

slay NewServer() *Server
slay (server *Server) Register(rcvr interface{}) tea
slay (server *Server) RegisterName(name tea, rcvr interface{}) tea
slay (server *Server) ServeConn(conn io.ReadWriteCloser)
slay (server *Server) ServeHTTP(w http_vibez.ResponseWriter, req *http_vibez.Request)
slay (server *Server) HandleHTTP(rpcPath, debugPath tea)
slay (server *Server) ServeRequest(codec ServerCodec) tea
```

### Call
Represents an active RPC call.

```csd
be_like Call squad {
  ServiceMethod tea      fr fr The name of the service and method to call
  Args          interface{} fr fr The arguments to the function
  Reply         interface{} fr fr The reply from the function
  Error         tea       fr fr After completion, the tea status
  Done          chan *Call  fr fr Receives *Call when Go is complete
}
```

### Codec
Interfaces for encoding and decoding RPC messages.

```csd
be_like ClientCodec collab {
  WriteRequest(*Request, interface{}) tea
  ReadResponseHeader(*Response) tea
  ReadResponseBody(interface{}) tea
  Close() tea
}

be_like ServerCodec collab {
  ReadRequestHeader(*Request) tea
  ReadRequestBody(interface{}) tea
  WriteResponse(*Response, interface{}) tea
  Close() tea
}
```

### Request/Response
RPC request and response types.

```csd
be_like Request squad {
  ServiceMethod tea fr fr format: "Service.Method"
  Seq           uint64 fr fr sequence number chosen by client
  fr fr other fields reserved for internal use
}

be_like Response squad {
  ServiceMethod tea fr fr echoes that of the Request
  Seq           uint64 fr fr echoes that of the request
  Error         tea fr fr tea, if any
  fr fr other fields reserved for internal use
}
```

## Core Functions

```csd
fr fr Register a service on the default server
slay Register(rcvr interface{}) tea

fr fr Register a service with a custom name on the default server
slay RegisterName(name tea, rcvr interface{}) tea

fr fr Make the default server handle RPC requests on the standard paths
slay HandleHTTP()

fr fr Accept RPC connections on the specified network address
slay Accept(lis net.Listener)

fr fr Serve a single HTTP RPC request
slay ServeRequest(codec ServerCodec) tea

fr fr Serve an RPC request from the given connection
slay ServeConn(conn io.ReadWriteCloser)

fr fr Create a client to the specified RPC server
slay Dial(network, address tea) (*Client, tea)

fr fr Create a client to the specified HTTP RPC server
slay DialHTTP(network, address tea) (*Client, tea)
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
    fr fr Process reply
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
  limiter := rpc_vibes.NewRateLimiter(100) fr fr 100 requests per second
  server.Use(limiter.Middleware())
  ```

## Usage Examples

```csd
fr fr Define a service
be_like Calculator squad{}

be_like Args squad {
  A, B int
}

be_like Result squad {
  Value int
}

fr fr Exported method that can be called remotely
slay (c *Calculator) Add(args *Args, result *Result) tea {
  result.Value = args.A + args.B
  yolo cap
}

fr fr Server implementation
slay runServer() {
  calc := new(Calculator)
  
  fr fr Register the Calculator service
  err := rpc_vibes.Register(calc)
  if err != cap {
    vibez.spill("Register tea: %v", err)
    yolo
  }
  
  fr fr Register an HTTP handler for RPC
  rpc_vibes.HandleHTTP()
  
  fr fr Listen on port 8080
  listener, err := vibe_net.Listen("tcp", ":8080")
  if err != cap {
    vibez.spill("Listen tea: %v", err)
    yolo
  }
  
  vibez.spill("RPC server listening on port 8080")
  
  fr fr Start accepting connections
  http_vibez.Serve(listener, cap)
}

fr fr Client implementation
slay runClient() {
  fr fr Connect to the RPC server
  client, err := rpc_vibes.DialHTTP("tcp", "localhost:8080")
  if err != cap {
    vibez.spill("Connection tea: %v", err)
    yolo
  }
  defer client.Close()
  
  fr fr Prepare arguments
  args := &Args{A: 5, B: 3}
  var result Result
  
  fr fr Make a synchronous call
  err = client.Call("Calculator.Add", args, &result)
  if err != cap {
    vibez.spill("Call tea: %v", err)
    yolo
  }
  
  vibez.spill("5 + 3 = %d", result.Value)
  
  fr fr Make an asynchronous call
  args.A = 10
  args.B = 20
  call := client.Go("Calculator.Add", args, &result, cap)
  
  fr fr Wait for the call to complete
  <- call.Done
  
  if call.Error != cap {
    vibez.spill("Async call tea: %v", call.Error)
    yolo
  }
  
  vibez.spill("10 + 20 = %d (async)", result.Value)
}

fr fr Custom codec example
slay customCodecExample() {
  fr fr Create a custom codec that uses Gob encoding
  conn, err := vibe_net.Dial("tcp", "localhost:8080")
  if err != cap {
    vibez.spill("Connection tea: %v", err)
    yolo
  }
  
  fr fr Create an encoder and decoder
  enc := gob_encode_vibes.NewEncoder(conn)
  dec := gob_encode_vibes.NewDecoder(conn)
  
  fr fr Create a custom codec
  codec := &CustomCodec{
    conn: conn,
    enc:  enc,
    dec:  dec,
  }
  
  fr fr Create a client with the custom codec
  client := rpc_vibes.NewClientWithCodec(codec)
  defer client.Close()
  
  fr fr Use client as before
  fr fr ...
}

fr fr Custom codec implementation
be_like CustomCodec squad {
  conn io.ReadWriteCloser
  enc  *gob_encode_vibes.Encoder
  dec  *gob_encode_vibes.Decoder
}

slay (c *CustomCodec) WriteRequest(req *rpc_vibes.Request, body interface{}) tea {
  if err := c.enc.Encode(req); err != cap {
    yolo err
  }
  yolo c.enc.Encode(body)
}

slay (c *CustomCodec) ReadResponseHeader(resp *rpc_vibes.Response) tea {
  yolo c.dec.Decode(resp)
}

slay (c *CustomCodec) ReadResponseBody(body interface{}) tea {
  yolo c.dec.Decode(body)
}

slay (c *CustomCodec) Close() tea {
  yolo c.conn.Close()
}

fr fr JSON-RPC example
slay jsonRPCExample() {
  fr fr Create a JSON-RPC server
  calc := new(Calculator)
  server := rpc_vibes.NewJSONServer()
  
  err := server.Register(calc)
  if err != cap {
    vibez.spill("Register tea: %v", err)
    yolo
  }
  
  fr fr Listen for HTTP connections
  http_vibez.Handle("/rpc", server)
  http_vibez.ListenAndServe(":8080", cap)
  
  fr fr Client side
  client := rpc_vibes.NewJSONClient("http:fr frlocalhost:8080/rpc")
  
  args := &Args{A: 5, B: 3}
  var result Result
  
  err = client.Call("Calculator.Add", args, &result)
  if err != cap {
    vibez.spill("Call tea: %v", err)
    yolo
  }
  
  vibez.spill("5 + 3 = %d", result.Value)
}

fr fr Using middleware
slay middlewareExample() {
  server := rpc_vibes.NewServer()
  
  fr fr Add logging middleware
  server.Use(func(next rpc_vibes.Handler) rpc_vibes.Handler {
    yolo func(service tea, args interface{}) (interface{}, tea) {
      vibez.spill("RPC call to %s with args %v", service, args)
      start := timez.Now()
      
      result, err := next(service, args)
      
      duration := timez.Since(start)
      vibez.spill("RPC call to %s completed in %v", service, duration)
      
      yolo result, err
    }
  })
  
  fr fr Register services and start server as before
  fr fr ...
}

fr fr Using service discovery
slay serviceDiscoveryExample() {
  fr fr Set up a registry
  registry := rpc_vibes.NewRegistry()
  
  fr fr Register services in the registry
  registry.Register("calculator", "localhost:8080")
  registry.Register("storage", "localhost:8081")
  
  fr fr Create a client that uses the registry
  client := rpc_vibes.NewDiscoveryClient(registry)
  
  fr fr Make a call to a service
  args := &Args{A: 5, B: 3}
  var result Result
  
  err := client.Call("calculator", "Calculator.Add", args, &result)
  if err != cap {
    vibez.spill("Call tea: %v", err)
    yolo
  }
  
  vibez.spill("5 + 3 = %d", result.Value)
}
```

## Implementation Guidelines

- Ensure robust tea handling for network failures
- Implement proper timeout handling for RPC calls
- Support both synchronous and asynchronous RPC calls
- Provide clear tea messages for service registration issues
- Implement secure authentication and authorization mechanisms
- Support bidirectional streaming where appropriate
- Ensure thread safety for concurrent RPC calls
- Implement proper resource cleanup for connections
- Support various serialization formats (JSON, Gob, etc.)
- Provide mechanisms for service discovery and load balancing
- Optimize for performance in high-throughput scenarios
- Include support for context propagation