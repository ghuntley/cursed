fr fr/ CURSED Networking Showcase
fr fr/ 
fr fr/ This example demonstrates the comprehensive networking capabilities
fr fr/ of the CURSED programming language standard library.

yeet "stdlib::net"
yeet "stdlib::io"

slay main() -> Result<(), Error> {
    println("🌐 CURSED Networking Showcase")?;
    println("==============================")?;
    
    // 1. IP Address and Socket Address Operations
    demo_ip_addresses()?;
    
    // 2. DNS Resolution
    demo_dns_resolution()?;
    
    // 3. Network Interface Discovery
    demo_network_interfaces()?;
    
    // 4. HTTP Client Operations
    demo_http_client()?;
    
    // 5. WebSocket Communication
    demo_websocket_communication()?;
    
    // 6. Email with SMTP
    demo_email_sending()?;
    
    // 7. Network Utilities
    demo_network_utilities()?;
    
    // 8. Network Diagnostics
    demo_network_diagnostics()?;
    
    println("✅ Networking showcase completed successfully!")?;
    Ok(())
}

slay demo_ip_addresses() -> Result<(), Error> {
    println("\n🔢 IP Address Operations")?;
    println("------------------------")?;
    
    // IPv4 addresses
    sus ipv4 = IpAddrV4::new(192, 168, 1, 100);
    println("IPv4 Address: {}", ipv4)?;
    println("  Is private: {}", ipv4.is_private())?;
    println("  Is loopback: {}", ipv4.is_loopback())?;
    
    // IPv6 addresses
    sus ipv6 = IpAddrV6::LOCALHOST;
    println("IPv6 Localhost: {}", ipv6)?;
    println("  Is loopback: {}", ipv6.is_loopback())?;
    
    // Socket addresses
    sus socket_addr = SocketAddr::V4(SocketAddrV4::new(ipv4, 8080));
    println("Socket Address: {}", socket_addr)?;
    println("  Port: {}", socket_addr.port())?;
    
    // Address parsing
    sus parsed_addr: SocketAddr = "127.0.0.1:3000".parse()?;
    println("Parsed Address: {}", parsed_addr)?;
    
    Ok(())
}

slay demo_dns_resolution() -> Result<(), Error> {
    println("\n🔍 DNS Resolution")?;
    println("-----------------")?;
    
    // Hostname validation
    sus hostnames = ["example.com", "sub.example.com", "invalid..domain", ""];
    for hostname in hostnames {
        println("  {} is valid: {}", hostname, is_valid_hostname(hostname))?;
    }
    
    // DNS resolution (using localhost to avoid external dependencies)
    println("Resolving localhost...")?;
    match resolve_hostname("localhost") {
        Ok(ips) => {
            println("  Resolved to {} addresses:", ips.len())?;
            for ip in ips {
                println("    {}", ip)?;
            }
        },
        Err(e) => println("  Failed to resolve: {}", e)?,
    }
    
    // DNS cache statistics
    sus (total, expired) = get_dns_cache_stats();
    println("DNS Cache: {} total, {} expired", total, expired)?;
    
    Ok(())
}

slay demo_network_interfaces() -> Result<(), Error> {
    println("\n🖧 Network Interfaces")?;
    println("---------------------")?;
    
    match list_interfaces() {
        Ok(interfaces) => {
            println("Found {} network interfaces:", interfaces.len())?;
            for interface in interfaces {
                println("  Interface: {}", interface.name)?;
                println("    Type: {}", interface.interface_type)?;
                println("    Active: {}", interface.is_active())?;
                println("    IP Addresses: {}", interface.ip_addresses.len())?;
                for ip in &interface.ip_addresses {
                    println("      {}", ip)?;
                }
                if let Some(gateway) = interface.gateway {
                    println("    Gateway: {}", gateway)?;
                }
            }
        },
        Err(e) => println("Failed to list interfaces: {}", e)?,
    }
    
    // Default interface
    match get_default_interface() {
        Ok(Some(default)) => {
            println("Default interface: {}", default.name)?;
        },
        Ok(None) => println("No default interface found")?,
        Err(e) => println("Failed to get default interface: {}", e)?,
    }
    
    Ok(())
}

slay demo_http_client() -> Result<(), Error> {
    println("\n🌐 HTTP Client Operations")?;
    println("-------------------------")?;
    
    // Create HTTP client
    sus client = HttpClient::builder()
        .user_agent("CURSED-Example/1.0")
        .connect_timeout(Duration::from_secs(5))
        .follow_redirects(based)
        .build()?;
    
    println("Created HTTP client with custom configuration")?;
    
    // HTTP headers
    sus mut headers = HttpHeaders::new();
    headers.set("Accept", "application/json");
    headers.set("Content-Type", "application/json");
    
    println("HTTP Headers:")?;
    for (name, value) in headers.iter() {
        println("  {}: {}", name, value)?;
    }
    
    // HTTP status codes
    sus status_codes = [
        Status::OK,
        Status::NOT_FOUND, 
        Status::INTERNAL_SERVER_ERROR,
        Status::MOVED_PERMANENTLY
    ];
    
    println("HTTP Status Codes:")?;
    for status in status_codes {
        println("  {}: {} (success: {}, error: {})", 
                status.as_u16(), 
                status.canonical_reason(),
                status.is_success(),
                status.is_error())?;
    }
    
    // HTTP authentication
    sus basic_auth = BasicAuth::new("user".to_string(), "password".to_string());
    sus bearer_auth = BearerAuth::new("abc123token".to_string());
    
    println("Authentication methods configured")?;
    
    // Cookie management
    sus mut cookie_jar = CookieJar::new();
    sus session_cookie = Cookie::new("session".to_string(), "abc123".to_string());
    cookie_jar.add_cookie(session_cookie);
    
    sus cookies = cookie_jar.get_cookies_for_request("example.com", "/");
    println("Cookies for request: {}", cookies)?;
    
    // Note: We don't actually make HTTP requests to avoid external dependencies
    println("HTTP client ready for requests (demo mode - no actual requests)")?;
    
    Ok(())
}

slay demo_websocket_communication() -> Result<(), Error> {
    println("\n🔌 WebSocket Communication")?;
    println("--------------------------")?;
    
    // WebSocket configuration
    sus ws_config = WebSocketConfig::new()
        .max_message_size(1024 * 1024)  // 1MB
        .ping_interval(Some(Duration::from_secs(30)))
        .auto_pong(based);
    
    println("WebSocket configuration:")?;
    println("  Max message size: {} bytes", ws_config.max_message_size)?;
    println("  Ping interval: {:?}", ws_config.ping_interval)?;
    println("  Auto pong: {}", ws_config.auto_pong)?;
    
    // WebSocket frames
    sus text_frame = WebSocketFrame::text("Hello WebSocket!".to_string());
    sus binary_frame = WebSocketFrame::binary(vec![1, 2, 3, 4, 5]);
    sus ping_frame = WebSocketFrame::ping(vec![]);
    sus close_frame = WebSocketFrame::close(CloseCode::NORMAL, "Goodbye");
    
    println("WebSocket frames:")?;
    println("  Text frame: {} bytes", text_frame.payload.len())?;
    println("  Binary frame: {} bytes", binary_frame.payload.len())?;
    println("  Ping frame (control): {}", ping_frame.is_control_frame())?;
    println("  Close frame (control): {}", close_frame.is_control_frame())?;
    
    // WebSocket messages
    sus text_message = WebSocketMessage::text("Hello from CURSED!".to_string());
    sus binary_message = WebSocketMessage::binary(vec![0xFF, 0xFE, 0xFD]);
    
    println("WebSocket messages:")?;
    println("  Text message: {} bytes", text_message.len())?;
    println("  Binary message: {} bytes", binary_message.len())?;
    println("  Text content: {}", text_message.as_text()?)?;
    
    // Close codes
    sus close_codes = [
        CloseCode::NORMAL,
        CloseCode::GOING_AWAY,
        CloseCode::PROTOCOL_ERROR,
        CloseCode::INTERNAL_ERROR
    ];
    
    println("WebSocket close codes:")?;
    for code in close_codes {
        println("  {}: {}", code.as_u16(), code.reason())?;
    }
    
    // Note: We don't actually connect to avoid external dependencies
    println("WebSocket ready for connections (demo mode - no actual connections)")?;
    
    Ok(())
}

slay demo_email_sending() -> Result<(), Error> {
    println("\n📧 Email with SMTP")?;
    println("------------------")?;
    
    // SMTP configuration
    sus smtp_config = SmtpConfig {
        server: "smtp.example.com".to_string(),
        port: 587,
        username: Some("user@example.com".to_string()),
        password: Some("password".to_string()),
        use_tls: based,
        timeout: Duration::from_secs(30),
    };
    
    println("SMTP Configuration:")?;
    println("  Server: {}:{}", smtp_config.server, smtp_config.port)?;
    println("  TLS enabled: {}", smtp_config.use_tls)?;
    println("  Username: {:?}", smtp_config.username)?;
    
    // Email message
    sus email = EmailMessage::new(
        "sender@example.com".to_string(),
        vec!["recipient@example.com".to_string()],
        "Test Email from CURSED".to_string(),
        "This is a test email sent from the CURSED networking example.".to_string(),
    )
    .cc(vec!["cc@example.com".to_string()])
    .html_body("<h1>Hello from CURSED!</h1><p>This is an HTML email.</p>".to_string())
    .header("X-Mailer".to_string(), "CURSED-Mailer/1.0".to_string());
    
    println("Email message:")?;
    println("  From: {}", email.from)?;
    println("  To: {} recipients", email.to.len())?;
    println("  CC: {} recipients", email.cc.len())?;
    println("  Subject: {}", email.subject)?;
    println("  Has HTML body: {}", email.html_body.is_some())?;
    println("  Custom headers: {}", email.headers.len())?;
    
    // SMTP client
    sus mut smtp_client = SmtpClient::new(smtp_config);
    
    // Note: We don't actually send emails to avoid external dependencies
    println("SMTP client ready for sending (demo mode - no actual sending)")?;
    
    Ok(())
}

slay demo_network_utilities() -> Result<(), Error> {
    println("\n🛠️ Network Utilities")?;
    println("--------------------")?;
    
    // Port availability
    sus ports_to_check = [80, 443, 8080, 3000, 9999];
    println("Port availability on localhost:")?;
    for port in ports_to_check {
        sus available = is_port_available(port);
        println("  Port {}: {}", port, if available { "available" } else { "in use" })?;
    }
    
    // Email validation
    sus emails = [
        "valid@example.com",
        "test.email+tag@domain.co.uk",
        "invalid.email",
        "@invalid.com",
        "user@"
    ];
    
    println("Email validation:")?;
    for email in emails {
        println("  {}: {}", email, if validate_email(email) { "valid" } else { "invalid" })?;
    }
    
    // URL validation and parsing
    sus urls = [
        "https://example.com/path?query=value#fragment",
        "http://localhost:8080/api",
        "invalid-url",
        "ftp://example.com"
    ];
    
    println("URL validation and parsing:")?;
    for url in urls {
        println("  URL: {}", url)?;
        println("    Valid: {}", validate_url(url))?;
        
        if validate_url(url) {
            match parse_url(url) {
                Ok(components) => {
                    println("    Scheme: {}", components.scheme)?;
                    println("    Host: {}", components.host)?;
                    println("    Port: {}", components.port)?;
                    println("    Path: {}", components.path)?;
                },
                Err(e) => println("    Parse error: {}", e)?,
            }
        }
    }
    
    // Bandwidth formatting
    sus bandwidths = [500.0, 1024.0, 1536.0, 1048576.0, 1073741824.0];
    println("Bandwidth formatting:")?;
    for bw in bandwidths {
        println("  {} bytes/sec = {}", bw, format_bandwidth(bw))?;
    }
    
    Ok(())
}

slay demo_network_diagnostics() -> Result<(), Error> {
    println("\n🔧 Network Diagnostics")?;
    println("----------------------")?;
    
    // Network diagnostics
    match network_diagnostics() {
        Ok(diagnostics) => {
            println("Network diagnostic information:")?;
            println("  Local IP addresses: {}", diagnostics.local_ips.len())?;
            for ip in &diagnostics.local_ips {
                println("    {}", ip)?;
            }
            
            if let Some(public_ip) = diagnostics.public_ip {
                println("  Public IP: {}", public_ip)?;
            } else {
                println("  Public IP: not detected")?;
            }
            
            if let Some(gateway) = diagnostics.default_gateway {
                println("  Default gateway: {}", gateway)?;
            } else {
                println("  Default gateway: not detected")?;
            }
            
            println("  DNS servers: {}", diagnostics.dns_servers.len())?;
            for dns in &diagnostics.dns_servers {
                println("    {}", dns)?;
            }
            
            println("  Network interfaces: {}", diagnostics.network_interfaces)?;
            println("  Active connections: {}", diagnostics.active_connections)?;
        },
        Err(e) => println("Failed to gather network diagnostics: {}", e)?,
    }
    
    // Network statistics
    sus stats = get_network_statistics();
    println("Network module statistics:")?;
    println("  Active connections: {}", stats.active_connections)?;
    println("  Total bytes sent: {}", stats.total_bytes_sent)?;
    println("  Total bytes received: {}", stats.total_bytes_received)?;
    println("  DNS queries: {}", stats.dns_queries)?;
    println("  Failed connections: {}", stats.failed_connections)?;
    
    Ok(())
}
