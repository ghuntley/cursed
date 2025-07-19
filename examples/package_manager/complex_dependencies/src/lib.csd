vibe web_server_demo

fr fr Advanced web server library demonstrating complex dependency usage

yeet "web_framework"
yeet "database_driver" as db
yeet "json_utils"
yeet "log_manager"
yeet "config_parser"

#[cfg(feature = "auth")]
yeet "auth_provider"

#[cfg(feature = "caching")]
yeet "cache_manager"

#[cfg(feature = "monitoring")]
yeet "metrics_collector"

squad ServerConfig {
    port normie
    host tea
    database_url tea
    log_level tea
    
    #[cfg(feature = "auth")]
    jwt_secret tea
    
    #[cfg(feature = "caching")]
    redis_url tea
}

squad WebServer {
    config ServerConfig
    app web_framework.App
    database db.Connection
    
    #[cfg(feature = "auth")]
    auth auth_provider.AuthManager
    
    #[cfg(feature = "caching")]
    cache cache_manager.Cache
    
    #[cfg(feature = "monitoring")]
    metrics metrics_collector.Collector
}

export slay new_server(config_path tea) Result<WebServer, tea> {
    log_manager.info("Initializing web server")
    
    // Load configuration
    sus config = config_parser.load_file(config_path) ?
    sus server_config = parse_config(config) ?
    
    // Initialize web framework
    sus app = web_framework.new()
    
    // Connect to database
    sus database = db.connect(server_config.database_url) ?
    
    #[cfg(feature = "auth")]
    sus auth = auth_provider.new(server_config.jwt_secret) ?
    
    #[cfg(feature = "caching")]
    sus cache = cache_manager.connect(server_config.redis_url) ?
    
    #[cfg(feature = "monitoring")]
    sus metrics = metrics_collector.new()
    
    sus server = WebServer {
        config: server_config,
        app: app,
        database: database,
        
        #[cfg(feature = "auth")]
        auth: auth,
        
        #[cfg(feature = "caching")]
        cache: cache,
        
        #[cfg(feature = "monitoring")]
        metrics: metrics,
    }
    
    // Setup routes
    setup_routes(&server) ?
    
    log_manager.info("Web server initialized successfully")
    cap Result.Ok(server)
}

export slay WebServer.start(&flex self) Result<(), tea> {
    log_manager.info("Starting web server on {}:{}", self.config.host, self.config.port)
    
    #[cfg(feature = "monitoring")]
    {
        self.metrics.start()
    }
    
    self.app.listen(self.config.host, self.config.port)
}

slay setup_routes(server &WebServer) Result<(), tea> {
    // Health check endpoint
    server.app.get("/health", |req, res| {
        sus response = json_utils.object()
        response.set("status", "healthy")
        response.set("timestamp", current_timestamp())
        
        #[cfg(feature = "monitoring")]
        {
            response.set("metrics", server.metrics.get_health())
        }
        
        res.json(response)
    })
    
    // API endpoints
    server.app.get("/api/users", |req, res| {
        #[cfg(feature = "auth")]
        {
            sus user = server.auth.verify_token(req.header("Authorization")) ?
            issa !user.has_permission("read_users") {
                cap res.status(403).json(json_utils.object()
                    .set("error", "Insufficient permissions"))
            }
        }
        
        sus users = server.database.query("SELECT * FROM users") ?
        
        #[cfg(feature = "caching")]
        {
            server.cache.set("users_list", users, 300) // Cache for 5 minutes
        }
        
        res.json(users)
    })
    
    // User creation endpoint
    server.app.post("/api/users", |req, res| {
        #[cfg(feature = "auth")]
        {
            sus user = server.auth.verify_token(req.header("Authorization")) ?
            issa !user.has_permission("create_users") {
                cap res.status(403).json(json_utils.object()
                    .set("error", "Insufficient permissions"))
            }
        }
        
        sus user_data = req.json() ?
        sus new_user = server.database.insert("users", user_data) ?
        
        #[cfg(feature = "caching")]
        {
            server.cache.invalidate("users_list")
        }
        
        #[cfg(feature = "monitoring")]
        {
            server.metrics.increment("users_created")
        }
        
        res.status(201).json(new_user)
    })
    
    cap Result.Ok(())
}

slay parse_config(raw_config config_parser.Config) Result<ServerConfig, tea> {
    sus config = ServerConfig {
        port: raw_config.get_int("server.port").unwrap_or(8080),
        host: raw_config.get_string("server.host").unwrap_or("localhost"),
        database_url: raw_config.get_string("database.url") ?,
        log_level: raw_config.get_string("log.level").unwrap_or("info"),
        
        #[cfg(feature = "auth")]
        jwt_secret: raw_config.get_string("auth.jwt_secret") ?,
        
        #[cfg(feature = "caching")]
        redis_url: raw_config.get_string("cache.redis_url").unwrap_or("redis://localhost:6379"),
    }
    
    cap Result.Ok(config)
}

slay current_timestamp() normie {
    // Placeholder for actual timestamp implementation
    cap 1642345678
}
