# VibeNet (net package)

## Overview
VibeNet provides a portable interface for network I/O, including TCP/IP, UDP, domain name resolution, and socket programming with modern vibes. It's inspired by Go's net package but with enhanced usability, extended protocols, and performance optimizations.

## Core Types

### IP Addressing

```go
type IPVibe struct {}

// Constructors
func ParseIP(s string) IPVibe
func IPv4(a, b, c, d byte) IPVibe

// Methods
func (ip IPVibe) String() string
func (ip IPVibe) IsLoopback() bool
func (ip IPVibe) IsMulticast() bool
func (ip IPVibe) IsGlobalUnicast() bool
func (ip IPVibe) IsLinkLocalUnicast() bool
func (ip IPVibe) IsLinkLocalMulticast() bool
func (ip IPVibe) IsInterfaceLocalMulticast() bool
func (ip IPVibe) IsPrivate() bool
func (ip IPVibe) IsUnspecified() bool
func (ip IPVibe) To4() IPVibe
func (ip IPVibe) To16() IPVibe
func (ip IPVibe) Equal(x IPVibe) bool
func (ip IPVibe) IsIPv4() bool
func (ip IPVibe) IsIPv6() bool
func (ip IPVibe) MarshalText() ([]byte, error)
func (ip *IPVibe) UnmarshalText(text []byte) error

type IPNetVibe struct {
    IP        IPVibe
    Mask      IPMaskVibe
    PrefixLen int
}

// Constructor
func ParseCIDR(s string) (IPVibe, *IPNetVibe, error)

// Methods
func (n *IPNetVibe) Contains(ip IPVibe) bool
func (n *IPNetVibe) Network() string
func (n *IPNetVibe) String() string

type IPMaskVibe []byte

// Constructors
func IPv4Mask(a, b, c, d byte) IPMaskVibe
func CIDRMask(ones, bits int) IPMaskVibe

// Methods
func (m IPMaskVibe) String() string
func (m IPMaskVibe) Size() (ones, bits int)
```

### Network Address

```go
type AddrVibe interface {
    Network() string
    String() string
}

type TCPAddrVibe struct {}

// Constructors
func ResolveTCPAddr(network, address string) (*TCPAddrVibe, error)

// Methods
func (a *TCPAddrVibe) Network() string
func (a *TCPAddrVibe) String() string
func (a *TCPAddrVibe) IP() IPVibe
func (a *TCPAddrVibe) Port() int
func (a *TCPAddrVibe) Zone() string

type UDPAddrVibe struct {}

// Constructors
func ResolveUDPAddr(network, address string) (*UDPAddrVibe, error)

// Methods
func (a *UDPAddrVibe) Network() string
func (a *UDPAddrVibe) String() string
func (a *UDPAddrVibe) IP() IPVibe
func (a *UDPAddrVibe) Port() int
func (a *UDPAddrVibe) Zone() string

type UnixAddrVibe struct {}

// Constructors
func ResolveUnixAddr(network, address string) (*UnixAddrVibe, error)

// Methods
func (a *UnixAddrVibe) Network() string
func (a *UnixAddrVibe) String() string
func (a *UnixAddrVibe) Name() string
```

### Connections

```go
type ConnVibe interface {
    Read(b []byte) (n int, err error)
    Write(b []byte) (n int, err error)
    Close() error
    LocalAddr() AddrVibe
    RemoteAddr() AddrVibe
    SetDeadline(t time.Time) error
    SetReadDeadline(t time.Time) error
    SetWriteDeadline(t time.Time) error
}

type TCPConnVibe struct {}

// Methods
func (c *TCPConnVibe) Read(b []byte) (n int, err error)
func (c *TCPConnVibe) Write(b []byte) (n int, err error)
func (c *TCPConnVibe) Close() error
func (c *TCPConnVibe) LocalAddr() AddrVibe
func (c *TCPConnVibe) RemoteAddr() AddrVibe
func (c *TCPConnVibe) SetDeadline(t time.Time) error
func (c *TCPConnVibe) SetReadDeadline(t time.Time) error
func (c *TCPConnVibe) SetWriteDeadline(t time.Time) error
func (c *TCPConnVibe) SetKeepAlive(keepalive bool) error
func (c *TCPConnVibe) SetKeepAlivePeriod(d time.Duration) error
func (c *TCPConnVibe) SetLinger(sec int) error
func (c *TCPConnVibe) SetNoDelay(noDelay bool) error
func (c *TCPConnVibe) SetReadBuffer(bytes int) error
func (c *TCPConnVibe) SetWriteBuffer(bytes int) error

type UDPConnVibe struct {}

// Methods
func (c *UDPConnVibe) Read(b []byte) (n int, err error)
func (c *UDPConnVibe) Write(b []byte) (n int, err error)
func (c *UDPConnVibe) Close() error
func (c *UDPConnVibe) LocalAddr() AddrVibe
func (c *UDPConnVibe) RemoteAddr() AddrVibe
func (c *UDPConnVibe) SetDeadline(t time.Time) error
func (c *UDPConnVibe) SetReadDeadline(t time.Time) error
func (c *UDPConnVibe) SetWriteDeadline(t time.Time) error
func (c *UDPConnVibe) ReadFromUDP(b []byte) (n int, addr *UDPAddrVibe, err error)
func (c *UDPConnVibe) WriteToUDP(b []byte, addr *UDPAddrVibe) (n int, err error)
func (c *UDPConnVibe) ReadFrom(b []byte) (int, AddrVibe, error)
func (c *UDPConnVibe) WriteTo(b []byte, addr AddrVibe) (n int, err error)
```

### Listeners

```go
type ListenerVibe interface {
    Accept() (ConnVibe, error)
    Close() error
    Addr() AddrVibe
}

type TCPListenerVibe struct {}

// Constructors
func ListenTCP(network string, laddr *TCPAddrVibe) (*TCPListenerVibe, error)

// Methods
func (l *TCPListenerVibe) Accept() (ConnVibe, error)
func (l *TCPListenerVibe) AcceptTCP() (*TCPConnVibe, error)
func (l *TCPListenerVibe) Close() error
func (l *TCPListenerVibe) Addr() AddrVibe
func (l *TCPListenerVibe) SetDeadline(t time.Time) error

type UnixListenerVibe struct {}

// Constructors
func ListenUnix(network string, laddr *UnixAddrVibe) (*UnixListenerVibe, error)

// Methods
func (l *UnixListenerVibe) Accept() (ConnVibe, error)
func (l *UnixListenerVibe) AcceptUnix() (*UnixConnVibe, error)
func (l *UnixListenerVibe) Close() error
func (l *UnixListenerVibe) Addr() AddrVibe
func (l *UnixListenerVibe) SetDeadline(t time.Time) error
```

### DNS Resolution

```go
type DNSResolverVibe struct {}

// Constructor
func NewDNSResolver() *DNSResolverVibe

// Methods
func LookupHost(host string) (addrs []string, err error)
func LookupIP(host string) ([]IPVibe, error)
func LookupPort(network, service string) (port int, err error)
func LookupCNAME(host string) (cname string, err error)
func LookupSRV(service, proto, name string) (cname string, addrs []*SRVVibe, err error)
func LookupMX(name string) ([]*MXVibe, error)
func LookupNS(name string) ([]*NSVibe, error)
func LookupTXT(name string) ([]string, error)
func LookupAddr(addr string) (names []string, err error)

type MXVibe struct {
    Host string
    Pref uint16
}

type NSVibe struct {
    Host string
}

type SRVVibe struct {
    Target   string
    Port     uint16
    Priority uint16
    Weight   uint16
}
```

### Dialer

```go
type DialerVibe struct {
    Timeout         time.Duration
    Deadline        time.Time
    LocalAddr       AddrVibe
    DualStack       bool
    FallbackDelay   time.Duration
    KeepAlive       time.Duration
    Resolver        *DNSResolverVibe
    Cancel          <-chan struct{}
    Control         func(network, address string, c syscall.RawConn) error
}

// Methods
func (d *DialerVibe) Dial(network, address string) (ConnVibe, error)
func (d *DialerVibe) DialContext(ctx VibeContext, network, address string) (ConnVibe, error)
```

## High-Level Functions

```go
// Dial connects to the address on the named network
func Dial(network, address string) (ConnVibe, error)

// DialTimeout connects to the address with a timeout
func DialTimeout(network, address string, timeout time.Duration) (ConnVibe, error)

// Listen announces on the local network address
func Listen(network, address string) (ListenerVibe, error)

// ListenPacket listens for packets
func ListenPacket(network, address string) (PacketConnVibe, error)

// ResolveTCPAddr resolves a TCP address
func ResolveTCPAddr(network, address string) (*TCPAddrVibe, error)

// ResolveUDPAddr resolves a UDP address
func ResolveUDPAddr(network, address string) (*UDPAddrVibe, error)

// ResolveUnixAddr resolves a Unix address
func ResolveUnixAddr(network, address string) (*UnixAddrVibe, error)

// DialTCP connects to the TCP address
func DialTCP(network string, laddr, raddr *TCPAddrVibe) (*TCPConnVibe, error)

// DialUDP connects to the UDP address
func DialUDP(network string, laddr, raddr *UDPAddrVibe) (*UDPConnVibe, error)

// DialUnix connects to the Unix address
func DialUnix(network string, laddr, raddr *UnixAddrVibe) (*UnixConnVibe, error)
```

## Enhanced Features

### Connection Pool

```go
type ConnPoolVibe struct {}

// Constructor
func NewConnPool(network, address string, maxConns int) *ConnPoolVibe

// Methods
func (p *ConnPoolVibe) Get() (ConnVibe, error)
func (p *ConnPoolVibe) Put(conn ConnVibe)
func (p *ConnPoolVibe) Close() error
func (p *ConnPoolVibe) Len() int
func (p *ConnPoolVibe) Stats() ConnPoolStats

type ConnPoolStats struct {
    MaxConns      int
    ActiveConns   int
    IdleConns     int
    TotalAcquired int64
    TotalReleased int64
    TotalErrors   int64
}
```

### Circuit Breaker

```go
type CircuitBreakerVibe struct {}

// Constructor
func NewCircuitBreaker(maxFailures int, resetTimeout time.Duration) *CircuitBreakerVibe

// Methods
func (cb *CircuitBreakerVibe) Execute(fn func() error) error
func (cb *CircuitBreakerVibe) State() CircuitBreakerState
func (cb *CircuitBreakerVibe) Reset()
func (cb *CircuitBreakerVibe) Trip()

type CircuitBreakerState int

const (
    CircuitClosed CircuitBreakerState = iota
    CircuitHalfOpen
    CircuitOpen
)
```

### Rate Limiter

```go
type RateLimiterVibe struct {}

// Constructor
func NewRateLimiter(rate int, perDuration time.Duration) *RateLimiterVibe

// Methods
func (rl *RateLimiterVibe) Allow() bool
func (rl *RateLimiterVibe) Wait(ctx VibeContext) error
func (rl *RateLimiterVibe) Reserve() *Reservation
func (rl *RateLimiterVibe) SetRate(rate int, perDuration time.Duration)

type Reservation struct{}

// Methods
func (r *Reservation) Cancel()
func (r *Reservation) Delay() time.Duration
func (r *Reservation) OK() bool
```

### Protocol Adapters

```go
// WebSocket adapter
func WebSocketConn(conn ConnVibe) (*WebSocketConnVibe, error)
type WebSocketConnVibe struct{}

// Methods
func (ws *WebSocketConnVibe) ReadMessage() (messageType int, p []byte, err error)
func (ws *WebSocketConnVibe) WriteMessage(messageType int, data []byte) error
func (ws *WebSocketConnVibe) Close() error

// MQTT adapter
func MQTTConn(conn ConnVibe) (*MQTTConnVibe, error)
type MQTTConnVibe struct{}

// Methods
func (mqtt *MQTTConnVibe) Subscribe(topic string, qos byte) error
func (mqtt *MQTTConnVibe) Publish(topic string, qos byte, retain bool, payload []byte) error
func (mqtt *MQTTConnVibe) Close() error

// HTTP/2 adapter
func HTTP2Conn(conn ConnVibe) (*HTTP2ConnVibe, error)
type HTTP2ConnVibe struct{}

// Methods
func (h2 *HTTP2ConnVibe) CreateStream() (*HTTP2StreamVibe, error)
func (h2 *HTTP2ConnVibe) Close() error
```

### Enhanced IPv6 Support

```go
func IsIPv6Enabled() bool
func PreferIPv6() bool
func SetPreferIPv6(prefer bool)
func IPv6InterfaceAddrs() ([]IPVibe, error)
```

### Network Interface

```go
type InterfaceVibe struct {
    Index        int
    MTU          int
    Name         string
    HardwareAddr HardwareAddrVibe
    Flags        InterfaceFlags
}

// Constructors
func InterfaceByIndex(index int) (*InterfaceVibe, error)
func InterfaceByName(name string) (*InterfaceVibe, error)

// Methods
func (ifi *InterfaceVibe) Addrs() ([]AddrVibe, error)
func (ifi *InterfaceVibe) MulticastAddrs() ([]AddrVibe, error)

// Functions
func Interfaces() ([]InterfaceVibe, error)
```

## Usage Example

```go
// TCP Server Example
func runTCPServer() {
    // Create a TCP address for the server
    addr, err := vibe_net.ResolveTCPAddr("tcp", ":8080")
    if err != nil {
        vibez.spill("Error resolving address:", err)
        return
    }
    
    // Create a TCP listener
    listener, err := vibe_net.ListenTCP("tcp", addr)
    if err != nil {
        vibez.spill("Error listening:", err)
        return
    }
    defer listener.Close()
    
    vibez.spill("Server listening on", listener.Addr())
    
    for {
        // Accept a connection
        conn, err := listener.Accept()
        if err != nil {
            vibez.spill("Error accepting connection:", err)
            continue
        }
        
        // Handle the connection in a new goroutine
        go handleConnection(conn)
    }
}

func handleConnection(conn vibe_net.ConnVibe) {
    defer conn.Close()
    
    // Set a deadline for the connection
    conn.SetDeadline(time.Now().Add(10 * time.Second))
    
    // Create a buffer for reading
    buffer := make([]byte, 1024)
    
    // Read from the connection
    n, err := conn.Read(buffer)
    if err != nil {
        vibez.spill("Error reading:", err)
        return
    }
    
    vibez.spill("Received:", string(buffer[:n]))
    
    // Write a response
    response := "Hello from server!"
    _, err = conn.Write([]byte(response))
    if err != nil {
        vibez.spill("Error writing:", err)
        return
    }
}

// TCP Client Example
func runTCPClient() {
    // Create a dialer with options
    dialer := &vibe_net.DialerVibe{
        Timeout:   5 * time.Second,
        KeepAlive: 30 * time.Second,
    }
    
    // Connect to the server
    conn, err := dialer.Dial("tcp", "localhost:8080")
    if err != nil {
        vibez.spill("Error connecting:", err)
        return
    }
    defer conn.Close()
    
    // Send a message
    message := "Hello from client!"
    _, err = conn.Write([]byte(message))
    if err != nil {
        vibez.spill("Error sending message:", err)
        return
    }
    
    // Read the response
    buffer := make([]byte, 1024)
    n, err := conn.Read(buffer)
    if err != nil {
        vibez.spill("Error reading response:", err)
        return
    }
    
    vibez.spill("Server response:", string(buffer[:n]))
}

// UDP Example
func runUDPExample() {
    // Create a UDP address
    addr, err := vibe_net.ResolveUDPAddr("udp", ":8081")
    if err != nil {
        vibez.spill("Error resolving address:", err)
        return
    }
    
    // Create a UDP connection
    conn, err := vibe_net.ListenUDP("udp", addr)
    if err != nil {
        vibez.spill("Error listening:", err)
        return
    }
    defer conn.Close()
    
    // Create a buffer for reading
    buffer := make([]byte, 1024)
    
    // Read from the connection
    n, remoteAddr, err := conn.ReadFromUDP(buffer)
    if err != nil {
        vibez.spill("Error reading:", err)
        return
    }
    
    vibez.spill("Received from", remoteAddr, ":", string(buffer[:n]))
    
    // Send a response
    response := "Hello from UDP server!"
    _, err = conn.WriteToUDP([]byte(response), remoteAddr)
    if err != nil {
        vibez.spill("Error sending response:", err)
        return
    }
}

// DNS Resolution Example
func dnsExample() {
    // Lookup host names
    ips, err := vibe_net.LookupIP("example.com")
    if err != nil {
        vibez.spill("Error looking up IP:", err)
        return
    }
    
    for _, ip := range ips {
        vibez.spill("IP address:", ip)
        vibez.spill("  Is IPv4:", ip.IsIPv4())
        vibez.spill("  Is IPv6:", ip.IsIPv6())
    }
    
    // Lookup MX records
    mxRecords, err := vibe_net.LookupMX("gmail.com")
    if err != nil {
        vibez.spill("Error looking up MX records:", err)
        return
    }
    
    for _, mx := range mxRecords {
        vibez.spill("MX record:", mx.Host, "(Priority:", mx.Pref, ")")
    }
}

// Network Interface Example
func interfaceExample() {
    // Get all network interfaces
    interfaces, err := vibe_net.Interfaces()
    if err != nil {
        vibez.spill("Error getting interfaces:", err)
        return
    }
    
    for _, intf := range interfaces {
        vibez.spill("Interface:", intf.Name)
        vibez.spill("  Index:", intf.Index)
        vibez.spill("  MTU:", intf.MTU)
        vibez.spill("  Hardware address:", intf.HardwareAddr)
        
        // Get addresses for this interface
        addrs, err := intf.Addrs()
        if err != nil {
            vibez.spill("  Error getting addresses:", err)
            continue
        }
        
        for _, addr := range addrs {
            vibez.spill("  Address:", addr)
        }
    }
}

// Connection Pool Example
func connectionPoolExample() {
    // Create a connection pool
    pool := vibe_net.NewConnPool("tcp", "example.com:80", 10)
    defer pool.Close()
    
    // Get a connection from the pool
    conn, err := pool.Get()
    if err != nil {
        vibez.spill("Error getting connection:", err)
        return
    }
    
    // Use the connection
    // ...
    
    // Return the connection to the pool
    pool.Put(conn)
    
    // Get pool statistics
    stats := pool.Stats()
    vibez.spill("Pool stats:")
    vibez.spill("  Active connections:", stats.ActiveConns)
    vibez.spill("  Idle connections:", stats.IdleConns)
    vibez.spill("  Total acquired:", stats.TotalAcquired)
}
```

## Implementation Guidelines
1. Ensure consistent error handling across all network operations
2. Provide backward compatibility with Go's net package
3. Optimize for performance, especially in high-concurrency scenarios
4. Implement proper resource cleanup to prevent leaks
5. Support both IPv4 and IPv6 with seamless dual-stack operation
6. Make timeouts and deadlines consistent and intuitive
7. Provide detailed documentation for all network functions
8. Include helper functions for common network tasks