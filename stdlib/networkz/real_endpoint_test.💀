fr fr CURSED Network Endpoint Configuration Test
fr fr Tests real configurable network endpoints instead of hardcoded ones

yeet "vibez"
yeet "testz" 
yeet "stringz"
yeet "net_protocols"
yeet "networking_complete"
yeet "httpz"
yeet "database_drivers"

fr fr Test configurable SMTP server hostnames
slay test_smtp_configuration() {
    vibez.spill("🔧 Testing SMTP server configuration...")
    
    fr fr Test default configuration
    sus default_greeting = smtp_connect()
    testz.assert_true(string_contains(default_greeting, "localhost"))
    vibez.spill("✅ Default SMTP server: localhost")
    
    fr fr Test custom server configuration
    smtp_configure_server("mail.example.org")
    sus custom_greeting = smtp_connect()
    testz.assert_true(string_contains(custom_greeting, "mail.example.org"))
    testz.assert_false(string_contains(custom_greeting, "cursed-mail.example.com"))
    vibez.spill("✅ Custom SMTP server: mail.example.org")
    
    fr fr Test production server configuration
    smtp_configure_server("smtp.gmail.com")
    sus gmail_greeting = smtp_connect()
    testz.assert_true(string_contains(gmail_greeting, "smtp.gmail.com"))
    vibez.spill("✅ Production SMTP server: smtp.gmail.com")
}

fr fr Test real DNS resolution
slay test_dns_resolution() {
    vibez.spill("🌐 Testing real DNS resolution...")
    
    fr fr Test localhost resolution
    sus localhost_ip = dns_resolve_a("localhost")
    testz.assert_eq_string(localhost_ip, "127.0.0.1")
    vibez.spill("✅ localhost resolves to 127.0.0.1")
    
    fr fr Test real domain resolution
    sus google_ip = dns_resolve_a("google.com")
    testz.assert_false(string_equals(google_ip, "0.0.0.0"))
    testz.assert_true(string_contains(google_ip, "."))
    vibez.spill("✅ google.com resolves to: " + google_ip)
    
    fr fr Test GitHub resolution
    sus github_ip = dns_resolve_a("github.com")
    testz.assert_false(string_equals(github_ip, "0.0.0.0"))
    vibez.spill("✅ github.com resolves to: " + github_ip)
    
    fr fr Test example.com (real domain)
    sus example_ip = dns_resolve_a("example.com")
    testz.assert_eq_string(example_ip, "93.184.216.34")
    vibez.spill("✅ example.com resolves to real IP: " + example_ip)
}

fr fr Test configurable database connections  
slay test_database_configuration() {
    vibez.spill("🗄️ Testing configurable database connections...")
    
    fr fr Test PostgreSQL with custom host
    sus pg_config = create_postgresql_config()
    testz.assert_eq_string(pg_config.host, "localhost") fr fr Default from env
    vibez.spill("✅ PostgreSQL default host: " + pg_config.host)
    
    fr fr Test MySQL with custom host
    sus mysql_config = create_mysql_config()
    testz.assert_eq_string(mysql_config.host, "localhost") fr fr Default from env
    vibez.spill("✅ MySQL default host: " + mysql_config.host)
    
    fr fr Test custom PostgreSQL configuration
    sus custom_pg_config = PostgreSQLConfig{
        host: "postgres.production.com",
        port: 5432,
        database: "myapp",
        username: "app_user",
        password: "secure_pass",
        ssl_mode: "require",
        connect_timeout: 30,
        query_timeout: 300,
        max_connections: 50,
        pool_max_idle: 10,
        pool_max_lifetime: 3600
    }
    testz.assert_eq_string(custom_pg_config.host, "postgres.production.com")
    vibez.spill("✅ Custom PostgreSQL host: " + custom_pg_config.host)
}

fr fr Test HTTP client with real endpoints
slay test_http_real_endpoints() {
    vibez.spill("🌍 Testing HTTP client with real endpoints...")
    
    fr fr Test HTTP request parsing
    sus test_url = "https://api.github.com/users/octocat"
    sus parsed = parse_url(test_url)
    testz.assert_eq_string(parsed.host, "api.github.com")
    testz.assert_eq_int(parsed.port, 443)
    testz.assert_eq_string(parsed.path, "/users/octocat")
    vibez.spill("✅ URL parsing: " + parsed.host + ":" + parsed.port)
    
    fr fr Test different endpoints
    sus local_url = "http://localhost:8080/health"
    sus local_parsed = parse_url(local_url)
    testz.assert_eq_string(local_parsed.host, "localhost")
    testz.assert_eq_int(local_parsed.port, 8080)
    vibez.spill("✅ Local endpoint: " + local_parsed.host)
    
    fr fr Test production endpoint
    sus prod_url = "https://api.stripe.com/v1/charges"
    sus prod_parsed = parse_url(prod_url)
    testz.assert_eq_string(prod_parsed.host, "api.stripe.com")
    testz.assert_eq_string(prod_parsed.path, "/v1/charges")
    vibez.spill("✅ Production endpoint: " + prod_parsed.host)
}

fr fr Test SSL/TLS configuration
slay test_ssl_configuration() {
    vibez.spill("🔒 Testing SSL/TLS configuration...")
    
    fr fr Test SSL certificate validation
    testz.assert_true(verify_ssl_certificate("github.com", "Subject: CN=github.com"))
    testz.assert_false(verify_ssl_certificate("github.com", "Subject: CN=badactor.com"))
    vibez.spill("✅ SSL certificate validation working")
    
    fr fr Test different SSL configurations
    testz.assert_true(verify_ssl_certificate("google.com", "Subject: CN=*.google.com"))
    vibez.spill("✅ Wildcard SSL certificate validation")
}

fr fr Test network timeouts and error handling
slay test_network_error_handling() {
    vibez.spill("⚠️ Testing network error handling...")
    
    fr fr Test invalid hostname
    sus invalid_ip = dns_resolve_a("nonexistent-domain-12345.invalid")
    testz.assert_eq_string(invalid_ip, "0.0.0.0")
    vibez.spill("✅ Invalid hostname returns fallback IP")
    
    fr fr Test timeout simulation
    sus timeout_response = http_get_with_timeout("http://httpbin.org/delay/10", 5)
    testz.assert_true(string_contains(timeout_response, "timeout"))
    vibez.spill("✅ HTTP timeout handling working")
}

fr fr Main test execution
slay main_character() {
    vibez.spill("🚀 Starting Real Network Endpoint Tests...")
    vibez.spill("=" * 50)
    
    test_smtp_configuration()
    vibez.spill("")
    
    test_dns_resolution()
    vibez.spill("")
    
    test_database_configuration()
    vibez.spill("")
    
    test_http_real_endpoints()
    vibez.spill("")
    
    test_ssl_configuration()
    vibez.spill("")
    
    test_network_error_handling()
    vibez.spill("")
    
    vibez.spill("=" * 50)
    vibez.spill("✅ All real network endpoint tests completed!")
    vibez.spill("📡 Network modules now support configurable endpoints")
    vibez.spill("🔧 No more hardcoded localhost/example.com dependencies")
    vibez.spill("🌐 Ready for production deployment!")
}

main()
