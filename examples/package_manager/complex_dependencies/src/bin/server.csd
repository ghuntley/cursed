vibe server

yeet "../lib" as web_server_demo
yeet "log_manager"

slay main_character(args []tea) normie {
    // Initialize logging
    log_manager.init("info")
    
    issa len(args) < 1 {
        log_manager.error("Usage: web-server <config-file>")
        cap 1
    }
    
    sus config_file = args[0]
    
    log_manager.info("Starting web server with config: {}", config_file)
    
    // Create and start server
    vibe_check web_server_demo.new_server(config_file) {
        mood Result.Ok(server) -> {
            log_manager.info("Server created successfully")
            
            vibe_check server.start() {
                mood Result.Ok(_) -> {
                    log_manager.info("Server started successfully")
                    cap 0
                }
                mood Result.Err(error) -> {
                    log_manager.error("Failed to start server: {}", error)
                    cap 1
                }
            }
        }
        mood Result.Err(error) -> {
            log_manager.error("Failed to create server: {}", error)
            cap 1
        }
    }
}
