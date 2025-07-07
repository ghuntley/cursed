// Test networking compilation

slay main() {
    vibez.spill("Testing networking module compilation...")
    
    // Test simple networking creation functions (non-blocking)
    sus tcp_handle normie = net_tcp_create()
    sus udp_handle normie = net_udp_create()
    
    vibez.spill("TCP handle created: ")
    vibez.spill("UDP handle created: ")
    
    // Test some utility functions
    sus local_ip tea = net_get_local_ip()
    vibez.spill("Local IP: ")
    
    vibez.spill("Networking compilation test completed!")
}

// Network runtime function declarations
slay net_tcp_create() normie {}
slay net_udp_create() normie {}
slay net_get_local_ip() tea {}
