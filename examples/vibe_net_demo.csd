fr fr VibeNet Networking Demo - Complete networking example
fr fr This example demonstrates the comprehensive networking capabilities of the CURSED VibeNet package

yeet "stdlib::vibe_net"
yeet "stdlib::oglogging"

slay runVibeNetDemo() {
    oglogging.spill("🌐 VibeNet Demo - Let's get connected!")
    
    // Test IP address handling
    testIPAddresses()
    
    // Test network address resolution  
    testAddressResolution()
    
    // Test TCP networking
    runTCPExample()
    
    // Test UDP networking
    runUDPExample()
    
    // Test DNS resolution
    testDNSResolution()
    
    // Test enhanced features
    testEnhancedFeatures()
    
    oglogging.spill("✨ VibeNet Demo completed successfully!")
}

slay testIPAddresses() {
    oglogging.spill("\n🔍 Testing IP Address Operations...")
    
    // Parse IPv4 address
    facts ipv4 = vibe_net.IPVibe.ParseIP("192.168.1.1")?
    oglogging.spillf("IPv4 address: %s", ipv4.String())
    oglogging.spillf("Is IPv4: %t", ipv4.IsIPv4())
    oglogging.spillf("Is private: %t", ipv4.IsPrivate())
    
    // Parse IPv6 address
    facts ipv6 = vibe_net.IPVibe.ParseIP("2001:db8::1")?
    oglogging.spillf("IPv6 address: %s", ipv6.String())
    oglogging.spillf("Is IPv6: %t", ipv6.IsIPv6())
    oglogging.spillf("Is global unicast: %t", ipv6.IsGlobalUnicast())
    
    // Test CIDR parsing
    facts (ip, net) = vibe_net.IPNetVibe.ParseCIDR("192.168.1.0/24")?
    oglogging.spillf("Network: %s", net.String())
    oglogging.spillf("Contains 192.168.1.100: %t", net.Contains(vibe_net.IPVibe.ParseIP("192.168.1.100")?))
    
    // Test IPv4 mask
    facts mask = vibe_net.IPMaskVibe.IPv4Mask(255, 255, 255, 0)
    oglogging.spillf("Subnet mask: %s", mask.String())
    
    oglogging.spill("✅ IP address operations completed")
}

slay testAddressResolution() {
    oglogging.spill("\n🎯 Testing Address Resolution...")
    
    // Resolve TCP address
    facts tcpAddr = vibe_net.ResolveTCPAddr("tcp", "localhost:8080")?
    oglogging.spillf("TCP address: %s", tcpAddr.String())
    oglogging.spillf("IP: %s, Port: %d", tcpAddr.IP().String(), tcpAddr.Port())
    
    // Resolve UDP address
    facts udpAddr = vibe_net.ResolveUDPAddr("udp", "127.0.0.1:9090")?
    oglogging.spillf("UDP address: %s", udpAddr.String())
    oglogging.spillf("Network: %s", udpAddr.Network())
    
    // Resolve Unix address
    facts unixAddr = vibe_net.ResolveUnixAddr("unix", "/tmp/demo.sock")?
    oglogging.spillf("Unix address: %s", unixAddr.String())
    oglogging.spillf("Path: %s", unixAddr.Name())
    
    oglogging.spill("✅ Address resolution completed")
}

slay runTCPExample() {
    oglogging.spill("\n🚀 Testing TCP Networking...")
    
    // Start TCP server in a goroutine
    stan startTCPServer()
    
    // Give server time to start
    time.Sleep(100 * time.Millisecond)
    
    // Connect as client
    runTCPClient()
    
    oglogging.spill("✅ TCP networking completed")
}

slay startTCPServer() {
    oglogging.spill("🖥️  Starting TCP server...")
    
    // Create TCP listener
    facts listener = vibe_net.Listen("tcp", ":8081")?
    defer listener.Close()
    
    oglogging.spillf("Server listening on %s", listener.Addr()?.String())
    
    // Accept one connection for demo
    facts conn = listener.Accept()?
    defer conn.Close()
    
    oglogging.spillf("Accepted connection from %s", conn.RemoteAddr()?.String())
    
    // Read message
    sus buffer = make([]byte, 1024)
    facts n = conn.Read(buffer)?
    oglogging.spillf("Received: %s", tea(buffer[:n]))
    
    // Send response
    facts response = "Hello from VibeNet TCP server! 🌐"
    conn.Write([]byte(response))?
    
    oglogging.spill("TCP server handled connection")
}

slay runTCPClient() {
    oglogging.spill("📱 Connecting as TCP client...")
    
    // Create dialer with configuration
    sus dialer = &vibe_net.DialerVibe{
        Timeout:   5 * time.Second,
        KeepAlive: 30 * time.Second,
    }
    
    // Connect to server
    facts conn = dialer.Dial("tcp", "localhost:8081")?
    defer conn.Close()
    
    oglogging.spillf("Connected to server at %s", conn.RemoteAddr()?.String())
    
    // Send message
    facts message = "Hello from VibeNet TCP client! 📱"
    conn.Write([]byte(message))?
    
    // Read response
    sus buffer = make([]byte, 1024)
    facts n = conn.Read(buffer)?
    oglogging.spillf("Server response: %s", tea(buffer[:n]))
    
    oglogging.spill("TCP client completed")
}

slay runUDPExample() {
    oglogging.spill("\n📡 Testing UDP Networking...")
    
    // Start UDP server in a goroutine
    stan startUDPServer()
    
    // Give server time to start
    time.Sleep(100 * time.Millisecond)
    
    // Run UDP client
    runUDPClient()
    
    oglogging.spill("✅ UDP networking completed")
}

slay startUDPServer() {
    oglogging.spill("📻 Starting UDP server...")
    
    // Create UDP listener
    facts conn = vibe_net.ListenPacket("udp", ":8082")?
    defer conn.Close()
    
    oglogging.spillf("UDP server listening on %s", conn.LocalAddr()?.String())
    
    // Read packet
    sus buffer = make([]byte, 1024)
    facts (n, addr) = conn.ReadFrom(buffer)?
    oglogging.spillf("Received from %s: %s", addr.String(), tea(buffer[:n]))
    
    // Send response
    facts response = "Hello from VibeNet UDP server! 📡"
    conn.WriteTo([]byte(response), addr)?
    
    oglogging.spill("UDP server handled packet")
}

slay runUDPClient() {
    oglogging.spill("📲 Running UDP client...")
    
    // Connect to UDP server
    facts conn = vibe_net.Dial("udp", "localhost:8082")?
    defer conn.Close()
    
    // Send message
    facts message = "Hello from VibeNet UDP client! 📲"
    conn.Write([]byte(message))?
    
    // Read response
    sus buffer = make([]byte, 1024)
    facts n = conn.Read(buffer)?
    oglogging.spillf("Server response: %s", tea(buffer[:n]))
    
    oglogging.spill("UDP client completed")
}

slay testDNSResolution() {
    oglogging.spill("\n🔍 Testing DNS Resolution...")
    
    // Create DNS resolver
    facts resolver = vibe_net.DNSResolverVibe.New()
    
    // Look up IP addresses
    facts ips = vibe_net.LookupIP("localhost")?
    oglogging.spillf("IP addresses for localhost:")
    lowkey (sus ip in ips) {
        oglogging.spillf("  - %s (IPv4: %t, IPv6: %t)", ip.String(), ip.IsIPv4(), ip.IsIPv6())
    }
    
    // Look up service port
    facts port = vibe_net.LookupPort("tcp", "http")?
    oglogging.spillf("HTTP port: %d", port)
    
    // Look up host names
    facts hostnames = vibe_net.LookupHost("localhost")?
    oglogging.spillf("Hostnames for localhost:")
    lowkey (sus hostname in hostnames) {
        oglogging.spillf("  - %s", hostname)
    }
    
    oglogging.spill("✅ DNS resolution completed")
}

slay testEnhancedFeatures() {
    oglogging.spill("\n⚡ Testing Enhanced Features...")
    
    // Test connection pool
    testConnectionPool()
    
    // Test circuit breaker
    testCircuitBreaker()
    
    // Test rate limiter
    testRateLimiter()
    
    // Test network interfaces
    testNetworkInterfaces()
    
    oglogging.spill("✅ Enhanced features completed")
}

slay testConnectionPool() {
    oglogging.spill("🏊 Testing Connection Pool...")
    
    // Create connection pool
    facts pool = vibe_net.NewConnPool("tcp", "localhost:80", 10)
    defer pool.Close()
    
    // Get pool statistics
    facts stats = pool.Stats()
    oglogging.spillf("Pool stats - Max: %d, Active: %d, Idle: %d", 
        stats.MaxConns, stats.ActiveConns, stats.IdleConns)
    
    oglogging.spill("Connection pool test completed")
}

slay testCircuitBreaker() {
    oglogging.spill("⚡ Testing Circuit Breaker...")
    
    // Create circuit breaker
    sus cb = vibe_net.NewCircuitBreaker(3, 5*time.Second)
    
    oglogging.spillf("Circuit breaker state: %v", cb.State())
    
    // Test execution
    facts err = cb.Execute(func() tea {
        oglogging.spill("Circuit breaker executed function")
        damn cap
    })
    
    lowkey (err != cap) {
        oglogging.spillf("Circuit breaker error: %s", err.Error())
    }
    
    oglogging.spill("Circuit breaker test completed")
}

slay testRateLimiter() {
    oglogging.spill("🚦 Testing Rate Limiter...")
    
    // Create rate limiter (10 requests per second)
    sus limiter = vibe_net.NewRateLimiter(10, time.Second)
    
    // Test rate limiting
    lowkey (sus i = 0; i < 5; i++) {
        lowkey (limiter.Allow()) {
            oglogging.spillf("Request %d allowed", i+1)
        } bestie {
            oglogging.spillf("Request %d rate limited", i+1)
        }
    }
    
    oglogging.spill("Rate limiter test completed")
}

slay testNetworkInterfaces() {
    oglogging.spill("🔌 Testing Network Interfaces...")
    
    // List all network interfaces
    facts interfaces = vibe_net.Interfaces()?
    oglogging.spillf("Found %d network interfaces:", len(interfaces))
    
    lowkey (sus intf in interfaces) {
        oglogging.spillf("  - %s (Index: %d, MTU: %d)", 
            intf.Name, intf.Index, intf.MTU)
        
        // Get addresses for this interface
        facts addrs = intf.Addrs()?
        lowkey (sus addr in addrs) {
            oglogging.spillf("    Address: %s", addr.String())
        }
    }
    
    oglogging.spill("Network interfaces test completed")
}

slay demonstrateProtocolAdapters() {
    oglogging.spill("\n🔌 Testing Protocol Adapters...")
    
    // Note: These would require actual implementations
    // This demonstrates the API structure
    
    oglogging.spill("Protocol adapters available:")
    oglogging.spill("  - WebSocket support")
    oglogging.spill("  - MQTT support") 
    oglogging.spill("  - HTTP/2 support")
    
    oglogging.spill("Protocol adapters demonstration completed")
}

slay main() {
    oglogging.spill("🎉 Welcome to the VibeNet Networking Demo!")
    oglogging.spill("This demo showcases the comprehensive networking capabilities of CURSED VibeNet")
    
    runVibeNetDemo()
    
    oglogging.spill("\n🎊 Demo completed! VibeNet is ready for your networking needs!")
}
