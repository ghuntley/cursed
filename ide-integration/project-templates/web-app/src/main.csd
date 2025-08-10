// {{PROJECT_NAME}} - Modern Web Application
// Built with CURSED for performance and developer experience

yeet "httpz"
yeet "jsonz"
yeet "routerz"
yeet "authz"
yeet "dbz"
yeet "logz"
yeet "configz"
yeet "vibez"

// Application configuration
squad Config {
    port drip,
    host tea,
    database_url tea,
    redis_url tea,
    jwt_secret tea,
    log_level tea,
    cors_enabled lit,
    rate_limit drip,
}

// Load configuration from environment and files
slay load_config() Config {
    sus config Config = Config{
        port: configz.get_int("PORT", 8080),
        host: configz.get_string("HOST", "localhost"),
        database_url: configz.get_string("DATABASE_URL", "postgresql://localhost:5432/{{PROJECT_NAME}}"),
        redis_url: configz.get_string("REDIS_URL", "redis://localhost:6379"),
        jwt_secret: configz.get_string("JWT_SECRET", "your-secret-key-here"),
        log_level: configz.get_string("LOG_LEVEL", "info"),
        cors_enabled: configz.get_bool("CORS_ENABLED", based),
        rate_limit: configz.get_int("RATE_LIMIT", 100),
    }
    
    damn config
}

// Database connection
slay setup_database(config Config) yikes<tea> {
    sus db_pool = dbz.connect(config.database_url) fam {
        when _ -> yikes "Failed to connect to database"
    }
    
    // Run migrations
    dbz.migrate(db_pool, "migrations/") fam {
        when _ -> yikes "Failed to run database migrations"
    }
    
    logz.info("Database connected and migrated successfully")
    damn nocap
}

// Redis connection
slay setup_redis(config Config) yikes<tea> {
    sus redis_client = redisz.connect(config.redis_url) fam {
        when _ -> yikes "Failed to connect to Redis"
    }
    
    logz.info("Redis connected successfully")
    damn nocap
}

// Authentication middleware
slay auth_middleware(req httpz.Request, res httpz.Response, next httpz.NextFunction) {
    sus auth_header tea = req.get_header("Authorization") fam {
        when _ -> {
            res.status(401).json(jsonz.object({
                "error": "Authorization header required"
            }))
            damn
        }
    }
    
    ready (!stringz.starts_with(auth_header, "Bearer ")) {
        res.status(401).json(jsonz.object({
            "error": "Invalid authorization format"
        }))
        damn
    }
    
    sus token tea = stringz.substring(auth_header, 7)
    
    sus claims = jwtiz.verify(token, config.jwt_secret) fam {
        when _ -> {
            res.status(401).json(jsonz.object({
                "error": "Invalid or expired token"
            }))
            damn
        }
    }
    
    // Add user info to request context
    req.set_context("user", claims)
    next()
}

// Rate limiting middleware
slay rate_limit_middleware(config Config) httpz.Middleware {
    sus limiter = httpz.rate_limiter({
        max: config.rate_limit,
        window: 60 * 1000, // 1 minute
        message: "Too many requests, please try again later"
    })
    
    damn limiter
}

// CORS middleware
slay cors_middleware(config Config) httpz.Middleware {
    ready (!config.cors_enabled) {
        damn httpz.no_op_middleware()
    }
    
    damn httpz.cors({
        origin: "*",
        methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
        headers: ["Content-Type", "Authorization"],
        credentials: based
    })
}

// Health check endpoint
slay health_check(req httpz.Request, res httpz.Response) {
    sus health_data = jsonz.object({
        "status": "healthy",
        "timestamp": timez.now().iso_string(),
        "version": "{{VERSION}}",
        "uptime": process.uptime(),
        "memory": process.memory_usage(),
        "database": "connected",
        "redis": "connected"
    })
    
    res.json(health_data)
}

// User registration endpoint
slay register_user(req httpz.Request, res httpz.Response) {
    sus body = req.json() fam {
        when _ -> {
            res.status(400).json(jsonz.object({
                "error": "Invalid JSON body"
            }))
            damn
        }
    }
    
    // Validate required fields
    sus email tea = body.get_string("email") fam {
        when _ -> {
            res.status(400).json(jsonz.object({
                "error": "Email is required"
            }))
            damn
        }
    }
    
    sus password tea = body.get_string("password") fam {
        when _ -> {
            res.status(400).json(jsonz.object({
                "error": "Password is required"
            }))
            damn
        }
    }
    
    // Hash password
    sus password_hash tea = cryptz.hash_password(password) fam {
        when _ -> {
            res.status(500).json(jsonz.object({
                "error": "Failed to hash password"
            }))
            damn
        }
    }
    
    // Create user in database
    sus user_id drip = dbz.query_one(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
        [email, password_hash]
    ) fam {
        when "duplicate_key" -> {
            res.status(409).json(jsonz.object({
                "error": "User already exists"
            }))
            damn
        }
        when _ -> {
            res.status(500).json(jsonz.object({
                "error": "Failed to create user"
            }))
            damn
        }
    }
    
    // Generate JWT token
    sus token tea = jwtiz.sign({
        "user_id": user_id,
        "email": email,
        "exp": timez.now().add_hours(24).timestamp()
    }, config.jwt_secret) fam {
        when _ -> {
            res.status(500).json(jsonz.object({
                "error": "Failed to generate token"
            }))
            damn
        }
    }
    
    res.status(201).json(jsonz.object({
        "message": "User created successfully",
        "token": token,
        "user": jsonz.object({
            "id": user_id,
            "email": email
        })
    }))
}

// User login endpoint
slay login_user(req httpz.Request, res httpz.Response) {
    sus body = req.json() fam {
        when _ -> {
            res.status(400).json(jsonz.object({
                "error": "Invalid JSON body"
            }))
            damn
        }
    }
    
    sus email tea = body.get_string("email") fam {
        when _ -> {
            res.status(400).json(jsonz.object({
                "error": "Email is required"
            }))
            damn
        }
    }
    
    sus password tea = body.get_string("password") fam {
        when _ -> {
            res.status(400).json(jsonz.object({
                "error": "Password is required"
            }))
            damn
        }
    }
    
    // Find user in database
    sus user = dbz.query_one(
        "SELECT id, email, password_hash FROM users WHERE email = $1",
        [email]
    ) fam {
        when "not_found" -> {
            res.status(401).json(jsonz.object({
                "error": "Invalid credentials"
            }))
            damn
        }
        when _ -> {
            res.status(500).json(jsonz.object({
                "error": "Database error"
            }))
            damn
        }
    }
    
    // Verify password
    sus password_valid lit = cryptz.verify_password(password, user.get_string("password_hash")) fam {
        when _ -> {
            res.status(500).json(jsonz.object({
                "error": "Password verification failed"
            }))
            damn
        }
    }
    
    ready (!password_valid) {
        res.status(401).json(jsonz.object({
            "error": "Invalid credentials"
        }))
        damn
    }
    
    // Generate JWT token
    sus token tea = jwtiz.sign({
        "user_id": user.get_int("id"),
        "email": user.get_string("email"),
        "exp": timez.now().add_hours(24).timestamp()
    }, config.jwt_secret) fam {
        when _ -> {
            res.status(500).json(jsonz.object({
                "error": "Failed to generate token"
            }))
            damn
        }
    }
    
    res.json(jsonz.object({
        "message": "Login successful",
        "token": token,
        "user": jsonz.object({
            "id": user.get_int("id"),
            "email": user.get_string("email")
        })
    }))
}

// Protected user profile endpoint
slay get_user_profile(req httpz.Request, res httpz.Response) {
    sus user_claims = req.get_context("user") fam {
        when _ -> {
            res.status(401).json(jsonz.object({
                "error": "Authentication required"
            }))
            damn
        }
    }
    
    sus user_id drip = user_claims.get_int("user_id")
    
    sus user = dbz.query_one(
        "SELECT id, email, created_at, updated_at FROM users WHERE id = $1",
        [user_id]
    ) fam {
        when "not_found" -> {
            res.status(404).json(jsonz.object({
                "error": "User not found"
            }))
            damn
        }
        when _ -> {
            res.status(500).json(jsonz.object({
                "error": "Database error"
            }))
            damn
        }
    }
    
    res.json(jsonz.object({
        "user": user
    }))
}

// Setup routes
slay setup_routes(app httpz.App, config Config) {
    // Public routes
    app.get("/", slay(req, res) {
        res.json(jsonz.object({
            "message": "Welcome to {{PROJECT_NAME}} API",
            "version": "{{VERSION}}",
            "documentation": "/docs"
        }))
    })
    
    app.get("/health", health_check)
    app.post("/auth/register", register_user)
    app.post("/auth/login", login_user)
    
    // Protected routes
    sus protected_routes = app.group("/api/v1")
    protected_routes.use(auth_middleware)
    
    protected_routes.get("/profile", get_user_profile)
    protected_routes.put("/profile", update_user_profile)
    protected_routes.delete("/profile", delete_user_profile)
    
    // API documentation
    app.get("/docs", slay(req, res) {
        res.redirect("https://docs.{{PROJECT_NAME}}.example.com")
    })
    
    // Catch-all for 404
    app.use(slay(req, res) {
        res.status(404).json(jsonz.object({
            "error": "Endpoint not found",
            "path": req.path(),
            "method": req.method()
        }))
    })
}

// Graceful shutdown
slay setup_graceful_shutdown(server httpz.Server) {
    process.on_signal("SIGINT", slay() {
        logz.info("Received SIGINT, shutting down gracefully...")
        
        server.close() fam {
            when _ -> {
                logz.error("Error during server shutdown")
                process.exit(1)
            }
        }
        
        dbz.close_all_connections()
        redisz.close_all_connections()
        
        logz.info("Server shut down successfully")
        process.exit(0)
    })
    
    process.on_signal("SIGTERM", slay() {
        logz.info("Received SIGTERM, shutting down gracefully...")
        server.close()
        process.exit(0)
    })
}

// Main application function
slay main() yikes<tea> {
    // Initialize logging
    logz.init({
        level: "info",
        format: "json",
        output: "stdout"
    })
    
    logz.info("Starting {{PROJECT_NAME}} web application...")
    
    // Load configuration
    sus config Config = load_config()
    logz.info("Configuration loaded", jsonz.object({
        "port": config.port,
        "host": config.host,
        "cors_enabled": config.cors_enabled,
        "rate_limit": config.rate_limit
    }))
    
    // Setup database
    setup_database(config) fam {
        when error -> {
            logz.error("Database setup failed: " + error)
            yikes error
        }
    }
    
    // Setup Redis
    setup_redis(config) fam {
        when error -> {
            logz.error("Redis setup failed: " + error)
            yikes error
        }
    }
    
    // Create HTTP application
    sus app = httpz.create()
    
    // Apply middleware
    app.use(httpz.logger())
    app.use(cors_middleware(config))
    app.use(rate_limit_middleware(config))
    app.use(httpz.json_parser())
    app.use(httpz.url_encoded_parser())
    
    // Setup routes
    setup_routes(app, config)
    
    // Start server
    sus server = app.listen(config.port, config.host) fam {
        when error -> {
            logz.error("Failed to start server: " + error)
            yikes error
        }
    }
    
    logz.info("Server started successfully", jsonz.object({
        "url": "http://" + config.host + ":" + config.port,
        "environment": process.env("NODE_ENV", "development")
    }))
    
    // Setup graceful shutdown
    setup_graceful_shutdown(server)
    
    damn nocap
}

// Error handling for main function
ready (main() fam { when error -> {
    logz.error("Application failed to start: " + error)
    process.exit(1)
}}) {
    // Success case handled by main function
}
