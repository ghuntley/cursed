# VibeNet (net package)

## Overview
VibeNet provides a portable collab for network I/O, including TCP/IP, UDP, domain name resolution, and socket programming with modern vibes. It's inspired by Go's net package but with enhanced usability, extended protocols, and performance optimizations.

## Core Types

### IP Addressing

```
be_like IPVibe squad {}

fr fr Consquadors
slay ParseIP(s tea) IPVibe
slay IPv4(a, b, c, d byte) IPVibe

fr fr Methods
slay (ip IPVibe) String() tea
slay (ip IPVibe) IsLoopback() lit
slay (ip IPVibe) IsMulticast() lit
slay (ip IPVibe) IsGlobalUnicast() lit
slay (ip IPVibe) IsLinkLocalUnicast() lit
slay (ip IPVibe) IsLinkLocalMulticast() lit
slay (ip IPVibe) IsInterfaceLocalMulticast() lit
slay (ip IPVibe) IsPrivate() lit
slay (ip IPVibe) IsUnspecified() lit
slay (ip IPVibe) To4() IPVibe
slay (ip IPVibe) To16() IPVibe
slay (ip IPVibe) Equal(x IPVibe) lit
slay (ip IPVibe) IsIPv4() lit
slay (ip IPVibe) IsIPv6() lit
slay (ip IPVibe) MarshalText() ([]byte, tea)
slay (ip *IPVibe) UnmarshalText(text []byte) tea

be_like IPNetVibe squad {
    IP        IPVibe
    Mask      IPMaskVibe
    PrefixLen int
}

fr fr Consquador
slay ParseCIDR(s tea) (IPVibe, *IPNetVibe, tea)

fr fr Methods
slay (n *IPNetVibe) Contains(ip IPVibe) lit
slay (n *IPNetVibe) Network() tea
slay (n *IPNetVibe) String() tea

be_like IPMaskVibe []byte

fr fr Consquadors
slay IPv4Mask(a, b, c, d byte) IPMaskVibe
slay CIDRMask(ones, bits normie) IPMaskVibe

fr fr Methods
slay (m IPMaskVibe) String() tea
slay (m IPMaskVibe) Size() (ones, bits normie)
```

### Network Address

```
be_like AddrVibe collab {
    Network() tea
    String() tea
}

be_like TCPAddrVibe squad {}

fr fr Consquadors
slay ResolveTCPAddr(network, address tea) (*TCPAddrVibe, tea)

fr fr Methods
slay (a *TCPAddrVibe) Network() tea
slay (a *TCPAddrVibe) String() tea
slay (a *TCPAddrVibe) IP() IPVibe
slay (a *TCPAddrVibe) Port() int
slay (a *TCPAddrVibe) Zone() tea

be_like UDPAddrVibe squad {}

fr fr Consquadors
slay ResolveUDPAddr(network, address tea) (*UDPAddrVibe, tea)

fr fr Methods
slay (a *UDPAddrVibe) Network() tea
slay (a *UDPAddrVibe) String() tea
slay (a *UDPAddrVibe) IP() IPVibe
slay (a *UDPAddrVibe) Port() int
slay (a *UDPAddrVibe) Zone() tea

be_like UnixAddrVibe squad {}

fr fr Consquadors
slay ResolveUnixAddr(network, address tea) (*UnixAddrVibe, tea)

fr fr Methods
slay (a *UnixAddrVibe) Network() tea
slay (a *UnixAddrVibe) String() tea
slay (a *UnixAddrVibe) Name() tea
```

### Connections

```
be_like ConnVibe collab {
    Read(b []byte) (n int, err tea)
    Write(b []byte) (n int, err tea)
    Close() tea
    LocalAddr() AddrVibe
    RemoteAddr() AddrVibe
    SetDeadline(t time.Time) tea
    SetReadDeadline(t time.Time) tea
    SetWriteDeadline(t time.Time) tea
}

be_like TCPConnVibe squad {}

fr fr Methods
slay (c *TCPConnVibe) Read(b []byte) (n int, err tea)
slay (c *TCPConnVibe) Write(b []byte) (n int, err tea)
slay (c *TCPConnVibe) Close() tea
slay (c *TCPConnVibe) LocalAddr() AddrVibe
slay (c *TCPConnVibe) RemoteAddr() AddrVibe
slay (c *TCPConnVibe) SetDeadline(t time.Time) tea
slay (c *TCPConnVibe) SetReadDeadline(t time.Time) tea
slay (c *TCPConnVibe) SetWriteDeadline(t time.Time) tea
slay (c *TCPConnVibe) SetKeepAlive(keepalive lit) tea
slay (c *TCPConnVibe) SetKeepAlivePeriod(d time.Duration) tea
slay (c *TCPConnVibe) SetLinger(sec normie) tea
slay (c *TCPConnVibe) SetNoDelay(noDelay lit) tea
slay (c *TCPConnVibe) SetReadBuffer(bytes normie) tea
slay (c *TCPConnVibe) SetWriteBuffer(bytes normie) tea

be_like UDPConnVibe squad {}

fr fr Methods
slay (c *UDPConnVibe) Read(b []byte) (n int, err tea)
slay (c *UDPConnVibe) Write(b []byte) (n int, err tea)
slay (c *UDPConnVibe) Close() tea
slay (c *UDPConnVibe) LocalAddr() AddrVibe
slay (c *UDPConnVibe) RemoteAddr() AddrVibe
slay (c *UDPConnVibe) SetDeadline(t time.Time) tea
slay (c *UDPConnVibe) SetReadDeadline(t time.Time) tea
slay (c *UDPConnVibe) SetWriteDeadline(t time.Time) tea
slay (c *UDPConnVibe) ReadFromUDP(b []byte) (n int, addr *UDPAddrVibe, err tea)
slay (c *UDPConnVibe) WriteToUDP(b []byte, addr *UDPAddrVibe) (n int, err tea)
slay (c *UDPConnVibe) ReadFrom(b []byte) (int, AddrVibe, tea)
slay (c *UDPConnVibe) WriteTo(b []byte, addr AddrVibe) (n int, err tea)
```

### Listeners

```
be_like ListenerVibe collab {
    Accept() (ConnVibe, tea)
    Close() tea
    Addr() AddrVibe
}

be_like TCPListenerVibe squad {}

fr fr Consquadors
slay ListenTCP(network tea, laddr *TCPAddrVibe) (*TCPListenerVibe, tea)

fr fr Methods
slay (l *TCPListenerVibe) Accept() (ConnVibe, tea)
slay (l *TCPListenerVibe) AcceptTCP() (*TCPConnVibe, tea)
slay (l *TCPListenerVibe) Close() tea
slay (l *TCPListenerVibe) Addr() AddrVibe
slay (l *TCPListenerVibe) SetDeadline(t time.Time) tea

be_like UnixListenerVibe squad {}

fr fr Consquadors
slay ListenUnix(network tea, laddr *UnixAddrVibe) (*UnixListenerVibe, tea)

fr fr Methods
slay (l *UnixListenerVibe) Accept() (ConnVibe, tea)
slay (l *UnixListenerVibe) AcceptUnix() (*UnixConnVibe, tea)
slay (l *UnixListenerVibe) Close() tea
slay (l *UnixListenerVibe) Addr() AddrVibe
slay (l *UnixListenerVibe) SetDeadline(t time.Time) tea
```

### DNS Resolution

```
be_like DNSResolverVibe squad {}

fr fr Consquador
slay NewDNSResolver() *DNSResolverVibe

fr fr Methods
slay LookupHost(host tea) (addrs []tea, err tea)
slay LookupIP(host tea) ([]IPVibe, tea)
slay LookupPort(network, service tea) (port int, err tea)
slay LookupCNAME(host tea) (cname tea, err tea)
slay LookupSRV(service, proto, name tea) (cname tea, addrs []*SRVVibe, err tea)
slay LookupMX(name tea) ([]*MXVibe, tea)
slay LookupNS(name tea) ([]*NSVibe, tea)
slay LookupTXT(name tea) ([]tea, tea)
slay LookupAddr(addr tea) (names []tea, err tea)

be_like MXVibe squad {
    Host tea
    Pref uint16
}

be_like NSVibe squad {
    Host tea
}

be_like SRVVibe squad {
    Target   tea
    Port     uint16
    Priority uint16
    Weight   uint16
}
```

### Dialer

```
be_like DialerVibe squad {
    Timeout         time.Duration
    Deadline        time.Time
    LocalAddr       AddrVibe
    DualStack       lit
    FallbackDelay   time.Duration
    KeepAlive       time.Duration
    Resolver        *DNSResolverVibe
    Cancel          <-chan squad{}
    Control         func(network, address tea, c syscall.RawConn) tea
}

fr fr Methods
slay (d *DialerVibe) Dial(network, address tea) (ConnVibe, tea)
slay (d *DialerVibe) DialContext(ctx VibeContext, network, address tea) (ConnVibe, tea)
```

## High-Level Functions

```
fr fr Dial connects to the address on the named network
slay Dial(network, address tea) (ConnVibe, tea)

fr fr DialTimeout connects to the address with a timeout
slay DialTimeout(network, address tea, timeout time.Duration) (ConnVibe, tea)

fr fr Listen announces on the local network address
slay Listen(network, address tea) (ListenerVibe, tea)

fr fr ListenPacket listens for packets
slay ListenPacket(network, address tea) (PacketConnVibe, tea)

fr fr ResolveTCPAddr resolves a TCP address
slay ResolveTCPAddr(network, address tea) (*TCPAddrVibe, tea)

fr fr ResolveUDPAddr resolves a UDP address
slay ResolveUDPAddr(network, address tea) (*UDPAddrVibe, tea)

fr fr ResolveUnixAddr resolves a Unix address
slay ResolveUnixAddr(network, address tea) (*UnixAddrVibe, tea)

fr fr DialTCP connects to the TCP address
slay DialTCP(network tea, laddr, raddr *TCPAddrVibe) (*TCPConnVibe, tea)

fr fr DialUDP connects to the UDP address
slay DialUDP(network tea, laddr, raddr *UDPAddrVibe) (*UDPConnVibe, tea)

fr fr DialUnix connects to the Unix address
slay DialUnix(network tea, laddr, raddr *UnixAddrVibe) (*UnixConnVibe, tea)
```

## Enhanced Features

### Connection Pool

```
be_like ConnPoolVibe squad {}

fr fr Consquador
slay NewConnPool(network, address tea, maxConns normie) *ConnPoolVibe

fr fr Methods
slay (p *ConnPoolVibe) Get() (ConnVibe, tea)
slay (p *ConnPoolVibe) Put(conn ConnVibe)
slay (p *ConnPoolVibe) Close() tea
slay (p *ConnPoolVibe) Len() int
slay (p *ConnPoolVibe) Stats() ConnPoolStats

be_like ConnPoolStats squad {
    MaxConns      int
    ActiveConns   int
    IdleConns     int
    TotalAcquired int64
    TotalReleased int64
    TotalErrors   int64
}
```

### Circuit Breaker

```
be_like CircuitBreakerVibe squad {}

fr fr Consquador
slay NewCircuitBreaker(maxFailures int, resetTimeout time.Duration) *CircuitBreakerVibe

fr fr Methods
slay (cb *CircuitBreakerVibe) Execute(fn func() tea) tea
slay (cb *CircuitBreakerVibe) State() CircuitBreakerState
slay (cb *CircuitBreakerVibe) Reset()
slay (cb *CircuitBreakerVibe) Trip()

be_like CircuitBreakerState int

const (
    CircuitClosed CircuitBreakerState = iota
    CircuitHalfOpen
    CircuitOpen
)
```

### Rate Limiter

```
be_like RateLimiterVibe squad {}

fr fr Consquador
slay NewRateLimiter(rate int, perDuration time.Duration) *RateLimiterVibe

fr fr Methods
slay (rl *RateLimiterVibe) Allow() lit
slay (rl *RateLimiterVibe) Wait(ctx VibeContext) tea
slay (rl *RateLimiterVibe) Reserve() *Reservation
slay (rl *RateLimiterVibe) SetRate(rate int, perDuration time.Duration)

be_like Reservation squad{}

fr fr Methods
slay (r *Reservation) Cancel()
slay (r *Reservation) Delay() time.Duration
slay (r *Reservation) OK() lit
```

### Protocol Adapters

```
fr fr WebSocket adapter
slay WebSocketConn(conn ConnVibe) (*WebSocketConnVibe, tea)
be_like WebSocketConnVibe squad{}

fr fr Methods
slay (ws *WebSocketConnVibe) ReadMessage() (messageType int, p []byte, err tea)
slay (ws *WebSocketConnVibe) WriteMessage(messageType int, data []byte) tea
slay (ws *WebSocketConnVibe) Close() tea

fr fr MQTT adapter
slay MQTTConn(conn ConnVibe) (*MQTTConnVibe, tea)
be_like MQTTConnVibe squad{}

fr fr Methods
slay (mqtt *MQTTConnVibe) Subscribe(topic tea, qos byte) tea
slay (mqtt *MQTTConnVibe) Publish(topic tea, qos byte, retain lit, payload []byte) tea
slay (mqtt *MQTTConnVibe) Close() tea

fr fr HTTP/2 adapter
slay HTTP2Conn(conn ConnVibe) (*HTTP2ConnVibe, tea)
be_like HTTP2ConnVibe squad{}

fr fr Methods
slay (h2 *HTTP2ConnVibe) CreateStream() (*HTTP2StreamVibe, tea)
slay (h2 *HTTP2ConnVibe) Close() tea
```

### Enhanced IPv6 Support

```
slay IsIPv6Enabled() lit
slay PreferIPv6() lit
slay SetPreferIPv6(prefer lit)
slay IPv6InterfaceAddrs() ([]IPVibe, tea)
```

### Network Interface

```
be_like InterfaceVibe squad {
    Index        int
    MTU          int
    Name         tea
    HardwareAddr HardwareAddrVibe
    Flags        InterfaceFlags
}

fr fr Consquadors
slay InterfaceByIndex(index normie) (*InterfaceVibe, tea)
slay InterfaceByName(name tea) (*InterfaceVibe, tea)

fr fr Methods
slay (ifi *InterfaceVibe) Addrs() ([]AddrVibe, tea)
slay (ifi *InterfaceVibe) MulticastAddrs() ([]AddrVibe, tea)

fr fr Functions
slay Interfaces() ([]InterfaceVibe, tea)
```

## Usage Example

```
fr fr TCP Server Example
slay runTCPServer() {
    fr fr Create a TCP address for the server
    addr, err := vibe_net.ResolveTCPAddr("tcp", ":8080")
    if err != cap {
        vibez.spill("Error resolving address:", err)
        yolo
    }
    
    fr fr Create a TCP listener
    listener, err := vibe_net.ListenTCP("tcp", addr)
    if err != cap {
        vibez.spill("Error listening:", err)
        yolo
    }
    defer listener.Close()
    
    vibez.spill("Server listening on", listener.Addr())
    
    for {
        fr fr Accept a connection
        conn, err := listener.Accept()
        if err != cap {
            vibez.spill("Error accepting connection:", err)
            continue
        }
        
        fr fr Handle the connection in a new goroutine
        go handleConnection(conn)
    }
}

slay handleConnection(conn vibe_net.ConnVibe) {
    defer conn.Close()
    
    fr fr Set a deadline for the connection
    conn.SetDeadline(time.Now().Add(10 * time.Second))
    
    fr fr Create a buffer for reading
    buffer := make([]byte, 1024)
    
    fr fr Read from the connection
    n, err := conn.Read(buffer)
    if err != cap {
        vibez.spill("Error reading:", err)
        yolo
    }
    
    vibez.spill("Received:", tea(buffer[:n]))
    
    fr fr Write a response
    response := "Hello from server!"
    _, err = conn.Write([]byte(response))
    if err != cap {
        vibez.spill("Error writing:", err)
        yolo
    }
}

fr fr TCP Client Example
slay runTCPClient() {
    fr fr Create a dialer with options
    dialer := &vibe_net.DialerVibe{
        Timeout:   5 * time.Second,
        KeepAlive: 30 * time.Second,
    }
    
    fr fr Connect to the server
    conn, err := dialer.Dial("tcp", "localhost:8080")
    if err != cap {
        vibez.spill("Error connecting:", err)
        yolo
    }
    defer conn.Close()
    
    fr fr Send a message
    message := "Hello from client!"
    _, err = conn.Write([]byte(message))
    if err != cap {
        vibez.spill("Error sending message:", err)
        yolo
    }
    
    fr fr Read the response
    buffer := make([]byte, 1024)
    n, err := conn.Read(buffer)
    if err != cap {
        vibez.spill("Error reading response:", err)
        yolo
    }
    
    vibez.spill("Server response:", tea(buffer[:n]))
}

fr fr UDP Example
slay runUDPExample() {
    fr fr Create a UDP address
    addr, err := vibe_net.ResolveUDPAddr("udp", ":8081")
    if err != cap {
        vibez.spill("Error resolving address:", err)
        yolo
    }
    
    fr fr Create a UDP connection
    conn, err := vibe_net.ListenUDP("udp", addr)
    if err != cap {
        vibez.spill("Error listening:", err)
        yolo
    }
    defer conn.Close()
    
    fr fr Create a buffer for reading
    buffer := make([]byte, 1024)
    
    fr fr Read from the connection
    n, remoteAddr, err := conn.ReadFromUDP(buffer)
    if err != cap {
        vibez.spill("Error reading:", err)
        yolo
    }
    
    vibez.spill("Received from", remoteAddr, ":", tea(buffer[:n]))
    
    fr fr Send a response
    response := "Hello from UDP server!"
    _, err = conn.WriteToUDP([]byte(response), remoteAddr)
    if err != cap {
        vibez.spill("Error sending response:", err)
        yolo
    }
}

fr fr DNS Resolution Example
slay dnsExample() {
    fr fr Lookup host names
    ips, err := vibe_net.LookupIP("example.com")
    if err != cap {
        vibez.spill("Error looking up IP:", err)
        yolo
    }
    
    for _, ip := range ips {
        vibez.spill("IP address:", ip)
        vibez.spill("  Is IPv4:", ip.IsIPv4())
        vibez.spill("  Is IPv6:", ip.IsIPv6())
    }
    
    fr fr Lookup MX records
    mxRecords, err := vibe_net.LookupMX("gmail.com")
    if err != cap {
        vibez.spill("Error looking up MX records:", err)
        yolo
    }
    
    for _, mx := range mxRecords {
        vibez.spill("MX record:", mx.Host, "(Priority:", mx.Pref, ")")
    }
}

fr fr Network Interface Example
slay interfaceExample() {
    fr fr Get all network interfaces
    interfaces, err := vibe_net.Interfaces()
    if err != cap {
        vibez.spill("Error getting interfaces:", err)
        yolo
    }
    
    for _, intf := range interfaces {
        vibez.spill("Interface:", intf.Name)
        vibez.spill("  Index:", intf.Index)
        vibez.spill("  MTU:", intf.MTU)
        vibez.spill("  Hardware address:", intf.HardwareAddr)
        
        fr fr Get addresses for this interface
        addrs, err := intf.Addrs()
        if err != cap {
            vibez.spill("  Error getting addresses:", err)
            continue
        }
        
        for _, addr := range addrs {
            vibez.spill("  Address:", addr)
        }
    }
}

fr fr Connection Pool Example
slay connectionPoolExample() {
    fr fr Create a connection pool
    pool := vibe_net.NewConnPool("tcp", "example.com:80", 10)
    defer pool.Close()
    
    fr fr Get a connection from the pool
    conn, err := pool.Get()
    if err != cap {
        vibez.spill("Error getting connection:", err)
        yolo
    }
    
    fr fr Use the connection
    fr fr ...
    
    fr fr Return the connection to the pool
    pool.Put(conn)
    
    fr fr Get pool statistics
    stats := pool.Stats()
    vibez.spill("Pool stats:")
    vibez.spill("  Active connections:", stats.ActiveConns)
    vibez.spill("  Idle connections:", stats.IdleConns)
    vibez.spill("  Total acquired:", stats.TotalAcquired)
}
```

## Implementation Guidelines
1. Ensure consistent tea handling across all network operations
2. Provide backward compatibility with Go's net package
3. Optimize for performance, especially in high-concurrency scenarios
4. Implement proper resource cleanup to prevent leaks
5. Support both IPv4 and IPv6 with seamless dual-stack operation
6. Make timeouts and deadlines consistent and intuitive
7. Provide detailed documentation for all network functions
8. Include helper functions for common network tasks