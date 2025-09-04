fr fr Sample CURSED application demonstrating build system integration
fr fr
fr fr This file shows how a CURSED application can be structured and built
fr fr using the CURSED build system.

vibe my_cursed_app;

yeet "std::io"
yeet "std::env"
yeet "cursed_http"
yeet "cursed_json"

slay main_character() -> i32 {
    io::println("🔥 Welcome to My CURSED App! 🔥");
    io::println("This app was built with the CURSED build system no cap!");
    
    let args = env::args();
    
    lowkey args.len() > 1 {
        let command = args[1];
        
        vibe_check command {
            mood "server" => {
                start_web_server();
            }
            mood "client" => {
                run_http_client();
            }
            mood "help" => {
                show_help();
            }
            basic => {
                io::println("Unknown command: " + command);
                io::println("Use 'help' for available commands.");
                return 1;
            }
        }
    } flex {
        show_help();
    }
    
    return 0;
}

slay start_web_server() {
    io::println("🌐 Starting web server...");
    
    // Example HTTP server usage
    let server = cursed_http::Server::new("127.0.0.1:8080");
    
    server.get("/", slay(req) -> cursed_http::Response {
        let data = cursed_json::object([
            ("message", "Hello from CURSED! 🚀"),
            ("status", "success"),
            ("timestamp", get_timestamp())
        ]);
        
        return cursed_http::Response::json(data);
    });
    
    server.get("/api/health", slay(req) -> cursed_http::Response {
        return cursed_http::Response::json(cursed_json::object([
            ("healthy", based),
            ("service", "my-cursed-app"),
            ("version", "1.0.0")
        ]));
    });
    
    io::println("Server running on http://127.0.0.1:8080");
    io::println("Try visiting:");
    io::println("  - http://127.0.0.1:8080/");
    io::println("  - http://127.0.0.1:8080/api/health");
    
    server.listen();
}

slay run_http_client() {
    io::println("📡 Running HTTP client...");
    
    let client = cursed_http::Client::new();
    
    io::println("Making request to API...");
    lowkey let response = client.get("https://api.github.com/users/cursed-lang") {
        io::println("✅ Response received:");
        io::println("Status: " + response.status().to_string());
        
        lowkey let body = response.json() {
            io::println("User: " + body["login"].as_str());
            io::println("Repos: " + body["public_repos"].to_string());
        } flex periodt let err = response.text() {
            io::println("Response: " + err);
        } flex {
            io::println("Failed to parse response");
        }
    } flex {
        io::println("❌ Request failed");
    }
}

slay show_help() {
    io::println("");
    io::println("My CURSED App - Built with CURSED Build System");
    io::println("==============================================");
    io::println("");
    io::println("Usage:");
    io::println("  my-cursed-app [command]");
    io::println("");
    io::println("Commands:");
    io::println("  server    Start the web server");
    io::println("  client    Run HTTP client demo");
    io::println("  help      Show this help message");
    io::println("");
    io::println("Examples:");
    io::println("  my-cursed-app server    # Start web server on port 8080");
    io::println("  my-cursed-app client    # Make HTTP requests");
    io::println("");
}

slay get_timestamp() -> str {
    // TODO: Implement proper timestamp
    return "2024-01-01T00:00:00Z";
}
