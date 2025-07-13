yeet "vibe_net"

# Simple verification test for vibe_net module
vibez.spill("Testing vibe_net networking module...")

# Test TCP socket creation
sus tcp_socket normie = tcp_create_socket()
vibez.spill("TCP socket created: " + tcp_socket.(tea))

# Test DNS resolution
sus google_ip tea = dns_resolve("google.com")
vibez.spill("Google.com resolves to: " + google_ip)

# Test WebSocket creation
sus ws_id normie = websocket_create()
vibez.spill("WebSocket ID: " + ws_id.(tea))

# Test HTTP GET
sus response tea = http_get("http://example.com")
vibez.spill("HTTP response received")

# Test network utilities
sus local_ip tea = get_local_ip()
vibez.spill("Local IP: " + local_ip)

# Test port validation
sus valid_port lit = is_valid_port(8080)
vibez.spill("Port 8080 is valid: " + valid_port.(tea))

vibez.spill("vibe_net module test completed successfully!")
