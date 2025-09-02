// real_networking.csd - Real Network Implementation for CURSED
// Replaces localhost-only mocks with actual networking functionality

yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "filez"
yeet "procesz"

// Real DNS resolution using system resolver
slay resolve_hostname(hostname tea) yikes<tea> {
    ready (stringz.len(hostname) == 0) {
        yikes "Empty hostname provided"
    }
    
    // Use getent to query system DNS
    sus cmd tea = stringz.concat(["getent hosts ", hostname])
    sus output tea = procesz.run_command(cmd) fam {
        when err -> {
            // Try nslookup as fallback
            sus nslookup_cmd tea = stringz.concat(["nslookup ", hostname, " | grep 'Address:' | tail -n1 | awk '{print $2}'"])
            damn procesz.run_command(nslookup_cmd) fam {
                when _ -> yikes "DNS resolution failed"
            }
        }
    }
    
    // Parse getent output: "IP hostname aliases..."
    sus lines tea[value] = stringz.split(output, "\n")
    ready (arrayz.len(lines) > 0) {
        sus parts tea[value] = stringz.split(lines[0], " ")
        ready (arrayz.len(parts) > 0) {
            sus ip tea = stringz.trim(parts[0])
            // Validate IP address format
            ready (stringz.contains(ip, ".") || stringz.contains(ip, ":")) {
                damn ip
            }
        }
    }
    
    yikes "Invalid DNS response"
}

// Real reverse DNS lookup
slay reverse_lookup(ip_address tea) yikes<tea> {
    ready (stringz.len(ip_address) == 0) {
        yikes "Empty IP address provided"
    }
    
    sus cmd tea = stringz.concat(["nslookup ", ip_address, " | grep 'name = ' | awk '{print $4}' | sed 's/\\.$//'"]) 
    sus output tea = procesz.run_command(cmd) fam {
        when err -> yikes "Reverse DNS lookup failed"
    }
    
    sus hostname tea = stringz.trim(output)
    ready (stringz.len(hostname) > 0) {
        damn hostname
    }
    
    yikes "No reverse DNS record found"
}

// Real network interface enumeration
slay get_network_interfaces() yikes<tea> {
    sus cmd tea = "ip -j addr show"
    sus output tea = procesz.run_command(cmd) fam {
        when err -> {
            // Fallback to ifconfig parsing
            sus ifconfig_cmd tea = "ifconfig -a | grep '^[a-zA-Z]' | awk '{print $1}' | sed 's/:$//'"
            sus interfaces_raw tea = procesz.run_command(ifconfig_cmd) fam {
                when _ -> yikes "Failed to enumerate network interfaces"
            }
            
            sus interfaces tea[value] = stringz.split(stringz.trim(interfaces_raw), "\n")
            sus json_parts tea[value] = ["{\"interfaces\": ["]
            sus i drip = 0
            bestie (i < arrayz.len(interfaces)) {
                ready (i > 0) {
                    json_parts = arrayz.push(json_parts, ", ")
                }
                json_parts = arrayz.push(json_parts, stringz.concat(["\"", interfaces[i], "\""]))
                i = i + 1
            }
            json_parts = arrayz.push(json_parts, "]}")
            damn stringz.join(json_parts, "")
        }
    }
    
    // Return the JSON output from ip command
    damn output
}

// Real local IP address detection
slay get_local_ip() yikes<tea> {
    // Try multiple methods to get local IP
    
    // Method 1: Use hostname -I
    sus cmd1 tea = "hostname -I | awk '{print $1}'"
    sus ip1 tea = procesz.run_command(cmd1) fam {
        when _ -> ""
    }
    
    sus local_ip tea = stringz.trim(ip1)
    ready (stringz.len(local_ip) > 0 && !stringz.equals(local_ip, "127.0.0.1")) {
        damn local_ip
    }
    
    // Method 2: Use ip route to find default interface
    sus cmd2 tea = "ip route get 8.8.8.8 | awk '{print $7}' | head -n1"
    sus ip2 tea = procesz.run_command(cmd2) fam {
        when _ -> ""
    }
    
    local_ip = stringz.trim(ip2)
    ready (stringz.len(local_ip) > 0 && !stringz.equals(local_ip, "127.0.0.1")) {
        damn local_ip
    }
    
    // Method 3: Parse ifconfig output
    sus cmd3 tea = "ifconfig | grep 'inet ' | grep -v '127.0.0.1' | awk '{print $2}' | head -n1"
    sus ip3 tea = procesz.run_command(cmd3) fam {
        when _ -> "127.0.0.1"
    }
    
    damn stringz.trim(ip3)
}

// Real network statistics from system
slay get_network_stats() yikes<tea> {
    // Read from /proc/net/dev for interface statistics
    sus dev_stats tea = filez.read_file("/proc/net/dev") fam {
        when _ -> {
            // Fallback to netstat
            sus netstat_cmd tea = "netstat -i | tail -n +3 | awk '{bytes_rx += $3; bytes_tx += $7} END {print \"{\\\"bytes_received\\\": \" bytes_rx \", \\\"bytes_sent\\\": \" bytes_tx \"}\"}"
            damn procesz.run_command(netstat_cmd) fam {
                when _ -> damn "{\"connections\": 0, \"bytes_sent\": 0, \"bytes_received\": 0}"
            }
        }
    }
    
    // Parse /proc/net/dev format
    sus lines tea[value] = stringz.split(dev_stats, "\n")
    sus total_rx drip = 0
    sus total_tx drip = 0
    sus i drip = 2  // Skip header lines
    
    bestie (i < arrayz.len(lines)) {
        sus line tea = stringz.trim(lines[i])
        ready (stringz.len(line) > 0) {
            sus parts tea[value] = stringz.split(line, " ")
            ready (arrayz.len(parts) >= 10) {
                // Skip loopback interface
                ready (!stringz.contains(parts[0], "lo:")) {
                    total_rx = total_rx + mathz.parse_int(stringz.trim(parts[1])) fam { when _ -> 0 }
                    total_tx = total_tx + mathz.parse_int(stringz.trim(parts[9])) fam { when _ -> 0 }
                }
            }
        }
        i = i + 1
    }
    
    // Get connection count from netstat
    sus conn_cmd tea = "netstat -ant | grep ESTABLISHED | wc -l"
    sus connections drip = mathz.parse_int(stringz.trim(procesz.run_command(conn_cmd) fam { when _ -> "0" })) fam { when _ -> 0 }
    
    damn stringz.concat([
        "{\"connections\": ", stringz.from_int(connections),
        ", \"bytes_sent\": ", stringz.from_int(total_tx),
        ", \"bytes_received\": ", stringz.from_int(total_rx), "}"
    ])
}

// Real port availability check using netstat
slay check_port_available(port drip) yikes<lit> {
    ready (port <= 0 || port > 65535) {
        yikes "Invalid port number"
    }
    
    sus cmd tea = stringz.concat(["netstat -ln | grep :", stringz.from_int(port), " | wc -l"])
    sus output tea = procesz.run_command(cmd) fam {
        when _ -> "1"  // Assume unavailable on error
    }
    
    sus count drip = mathz.parse_int(stringz.trim(output)) fam { when _ -> 1 }
    damn count == 0
}

// Real HTTP client using curl
slay real_http_get(url tea, timeout_seconds drip) yikes<tea> {
    ready (stringz.len(url) == 0) {
        yikes "Empty URL provided"
    }
    
    ready (timeout_seconds <= 0) {
        timeout_seconds = 30
    }
    
    sus cmd tea = stringz.concat([
        "curl -s -m ", stringz.from_int(timeout_seconds),
        " -H 'User-Agent: CURSED-NetworkZ/1.0' '", url, "'"
    ])
    
    sus response tea = procesz.run_command(cmd) fam {
        when err -> yikes stringz.concat(["HTTP request failed: ", err])
    }
    
    damn response
}

// Real HTTP POST using curl
slay real_http_post(url tea, body tea, content_type tea, timeout_seconds drip) yikes<tea> {
    ready (stringz.len(url) == 0) {
        yikes "Empty URL provided"
    }
    
    ready (timeout_seconds <= 0) {
        timeout_seconds = 30
    }
    
    ready (stringz.len(content_type) == 0) {
        content_type = "application/json"
    }
    
    // Write body to temporary file
    sus temp_file tea = "/tmp/cursed_http_post_body"
    filez.write_file(temp_file, body) fam {
        when err -> yikes stringz.concat(["Failed to write request body: ", err])
    }
    
    sus cmd tea = stringz.concat([
        "curl -s -m ", stringz.from_int(timeout_seconds),
        " -H 'Content-Type: ", content_type, "'",
        " -H 'User-Agent: CURSED-NetworkZ/1.0'",
        " -d @", temp_file, " '", url, "'"
    ])
    
    sus response tea = procesz.run_command(cmd) fam {
        when err -> {
            // Clean up temp file
            procesz.run_command(stringz.concat(["rm -f ", temp_file])) fam { when _ -> {} }
            yikes stringz.concat(["HTTP POST failed: ", err])
        }
    }
    
    // Clean up temp file
    procesz.run_command(stringz.concat(["rm -f ", temp_file])) fam { when _ -> {} }
    
    damn response
}

// Real HTTP with full response details using curl
slay real_http_request_full(method tea, url tea, headers tea[value], body tea, timeout_seconds drip) yikes<tea> {
    ready (stringz.len(url) == 0) {
        yikes "Empty URL provided"
    }
    
    ready (stringz.len(method) == 0) {
        method = "GET"
    }
    
    ready (timeout_seconds <= 0) {
        timeout_seconds = 30
    }
    
    // Build curl command
    sus cmd_parts tea[value] = [
        "curl -s -i -m ", stringz.from_int(timeout_seconds),
        " -X ", method,
        " -H 'User-Agent: CURSED-NetworkZ/1.0'"
    ]
    
    // Add custom headers
    sus i drip = 0
    bestie (i < arrayz.len(headers)) {
        cmd_parts = arrayz.push(cmd_parts, stringz.concat([" -H '", headers[i], "'"]))
        i = i + 1
    }
    
    // Add body for POST/PUT requests
    ready (stringz.len(body) > 0) {
        sus temp_file tea = "/tmp/cursed_http_body"
        filez.write_file(temp_file, body) fam {
            when err -> yikes stringz.concat(["Failed to write request body: ", err])
        }
        cmd_parts = arrayz.push(cmd_parts, stringz.concat([" -d @", temp_file]))
    }
    
    cmd_parts = arrayz.push(cmd_parts, stringz.concat([" '", url, "'"]))
    sus cmd tea = stringz.join(cmd_parts, "")
    
    sus response tea = procesz.run_command(cmd) fam {
        when err -> {
            ready (stringz.len(body) > 0) {
                procesz.run_command("rm -f /tmp/cursed_http_body") fam { when _ -> {} }
            }
            yikes stringz.concat(["HTTP request failed: ", err])
        }
    }
    
    // Clean up temp file if used
    ready (stringz.len(body) > 0) {
        procesz.run_command("rm -f /tmp/cursed_http_body") fam { when _ -> {} }
    }
    
    damn response
}

// Real ping using system ping command
slay real_ping(host tea, count drip) yikes<drip> {
    ready (stringz.len(host) == 0) {
        yikes "Empty hostname provided"
    }
    
    ready (count <= 0) {
        count = 1
    }
    
    sus cmd tea = stringz.concat([
        "ping -c ", stringz.from_int(count), " -W 3 ", host, 
        " | grep 'time=' | awk -F'time=' '{print $2}' | awk '{print $1}' | head -n1"
    ])
    
    sus output tea = procesz.run_command(cmd) fam {
        when err -> yikes stringz.concat(["Ping failed: ", err])
    }
    
    sus time_str tea = stringz.trim(output)
    ready (stringz.len(time_str) == 0) {
        yikes "Host unreachable"
    }
    
    // Parse time (remove 'ms' suffix if present)
    ready (stringz.ends_with(time_str, "ms")) {
        time_str = stringz.substring(time_str, 0, stringz.len(time_str) - 2)
    }
    
    sus ping_time drip = mathz.parse_int(stringz.split(time_str, ".")[0]) fam {
        when _ -> yikes "Invalid ping response"
    }
    
    damn ping_time
}

// Real port connectivity test using nc (netcat)
slay real_check_port_open(host tea, port drip, timeout_seconds drip) yikes<lit> {
    ready (stringz.len(host) == 0) {
        yikes "Empty hostname provided"
    }
    
    ready (port <= 0 || port > 65535) {
        yikes "Invalid port number"
    }
    
    ready (timeout_seconds <= 0) {
        timeout_seconds = 5
    }
    
    sus cmd tea = stringz.concat([
        "timeout ", stringz.from_int(timeout_seconds),
        " nc -z ", host, " ", stringz.from_int(port),
        " 2>/dev/null && echo 'open' || echo 'closed'"
    ])
    
    sus output tea = procesz.run_command(cmd) fam {
        when _ -> "closed"
    }
    
    damn stringz.contains(stringz.trim(output), "open")
}

// Network connectivity test
slay test_network_connectivity() yikes<lit> {
    vibez.spill("🔍 Testing real network connectivity...")
    
    // Test 1: DNS Resolution
    vibez.spill("1. Testing DNS resolution...")
    sus google_ip tea = resolve_hostname("google.com") fam {
        when err -> {
            vibez.spill("❌ DNS resolution failed: " + err)
            damn no_cap
        }
    }
    vibez.spill("✅ google.com resolved to: " + google_ip)
    
    // Test 2: Reverse DNS
    vibez.spill("2. Testing reverse DNS...")
    sus hostname tea = reverse_lookup("8.8.8.8") fam {
        when err -> {
            vibez.spill("❌ Reverse DNS failed: " + err)
            damn no_cap
        }
    }
    vibez.spill("✅ 8.8.8.8 reverse resolves to: " + hostname)
    
    // Test 3: Local IP detection
    vibez.spill("3. Testing local IP detection...")
    sus local_ip tea = get_local_ip() fam {
        when err -> {
            vibez.spill("❌ Local IP detection failed: " + err)
            damn no_cap
        }
    }
    vibez.spill("✅ Local IP address: " + local_ip)
    
    // Test 4: Network interfaces
    vibez.spill("4. Testing network interface enumeration...")
    sus interfaces tea = get_network_interfaces() fam {
        when err -> {
            vibez.spill("❌ Network interface enumeration failed: " + err)
            damn no_cap
        }
    }
    vibez.spill("✅ Network interfaces: " + interfaces)
    
    // Test 5: HTTP GET request
    vibez.spill("5. Testing HTTP GET request...")
    sus response tea = real_http_get("http://httpbin.org/get", 10) fam {
        when err -> {
            vibez.spill("❌ HTTP GET failed: " + err)
            damn no_cap
        }
    }
    vibez.spill("✅ HTTP GET successful, response length: " + stringz.from_int(stringz.len(response)))
    
    // Test 6: Ping test
    vibez.spill("6. Testing ping connectivity...")
    sus ping_time drip = real_ping("google.com", 1) fam {
        when err -> {
            vibez.spill("❌ Ping failed: " + err)
            damn no_cap
        }
    }
    vibez.spill("✅ Ping to google.com: " + stringz.from_int(ping_time) + "ms")
    
    // Test 7: Port connectivity
    vibez.spill("7. Testing port connectivity...")
    sus port_open lit = real_check_port_open("google.com", 80, 5) fam {
        when err -> {
            vibez.spill("❌ Port check failed: " + err)
            damn no_cap
        }
    }
    vibez.spill("✅ Port 80 on google.com is open: " + stringz.from_bool(port_open))
    
    // Test 8: Network statistics
    vibez.spill("8. Testing network statistics...")
    sus stats tea = get_network_stats() fam {
        when err -> {
            vibez.spill("❌ Network stats failed: " + err)
            damn no_cap
        }
    }
    vibez.spill("✅ Network statistics: " + stats)
    
    vibez.spill("🎉 All network connectivity tests passed!")
    damn based
}
