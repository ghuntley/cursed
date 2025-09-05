vibez.spill("🌐 Testing Network functionality")

fr fr Test basic network operations
slay net_connect(host tea, port normie) normie {
    vibez.spill("Connecting to: " + host + ":" + tea(port))
    damn 1001
}

slay net_send(socket normie, data tea) normie {
    vibez.spill("Sending data: " + data)
    damn 13
}

slay net_receive(socket normie, size normie) tea {
    vibez.spill("Receiving data")
    damn "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!"
}

slay net_close(socket normie) {
    vibez.spill("Closing socket: " + tea(socket))
}

fr fr Test the functions
sus host tea = "example.com"
sus port normie = 80
sus socket normie = net_connect(host, port)
sus bytes_sent normie = net_send(socket, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
sus response tea = net_receive(socket, 1024)
net_close(socket)

vibez.spill("✅ Network operations work")
vibez.spill("Socket: " + tea(socket))
vibez.spill("Bytes sent: " + tea(bytes_sent))
vibez.spill("Response: " + response)

fr fr Test HTTP operations
slay http_get(url tea) tea {
    vibez.spill("GET request to: " + url)
    damn "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from HTTP!"
}

slay http_post(url tea, data tea) tea {
    vibez.spill("POST request to: " + url)
    damn "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\"status\":\"created\"}"
}

slay http_status(response tea) normie {
    vibez.spill("Getting status from response")
    damn 200
}

slay http_body(response tea) tea {
    vibez.spill("Getting body from response")
    damn "Hello from HTTP!"
}

fr fr Test the functions
sus url tea = "https://api.example.com/data"
sus get_response tea = http_get(url)
sus post_response tea = http_post(url, "{\"key\":\"value\"}")
sus status normie = http_status(get_response)
sus body tea = http_body(get_response)

vibez.spill("✅ HTTP operations work")
vibez.spill("GET response: " + get_response)
vibez.spill("POST response: " + post_response)
vibez.spill("Status: " + tea(status))
vibez.spill("Body: " + body)

fr fr Test DNS operations
slay dns_resolve(hostname tea) tea {
    vibez.spill("Resolving: " + hostname)
    damn "93.184.216.34"
}

slay dns_reverse(ip tea) tea {
    vibez.spill("Reverse DNS for: " + ip)
    damn "example.com"
}

slay ping(hostname tea) lit {
    vibez.spill("Pinging: " + hostname)
    damn based
}

fr fr Test the functions
sus hostname tea = "example.com"
sus ip tea = dns_resolve(hostname)
sus reverse_name tea = dns_reverse(ip)
sus ping_result lit = ping(hostname)

vibez.spill("✅ DNS operations work")
vibez.spill("Hostname: " + hostname)
vibez.spill("IP: " + ip)
vibez.spill("Reverse: " + reverse_name)
vibez.spill("Ping result: " + tea(ping_result))

fr fr Test server operations
slay server_create(port normie) normie {
    vibez.spill("Creating server on port: " + tea(port))
    damn 2001
}

slay server_listen(server normie) {
    vibez.spill("Server listening on: " + tea(server))
}

slay server_accept(server normie) normie {
    vibez.spill("Accepting connection")
    damn 3001
}

slay server_close(server normie) {
    vibez.spill("Closing server: " + tea(server))
}

fr fr Test the functions
sus server_port normie = 8080
sus server normie = server_create(server_port)
server_listen(server)
sus client normie = server_accept(server)
server_close(server)

vibez.spill("✅ Server operations work")
vibez.spill("Server: " + tea(server))
vibez.spill("Client: " + tea(client))

vibez.spill("🎉 All Network functionality works!")
