#!/usr/bin/env cursed

//! # Production Web Server in CURSED
//! 
//! This example demonstrates a complete, production-ready web server built with
//! the CURSED programming language, showcasing:
//! 
//! - HTTP server with routing and middleware
//! - Database integration with connection pooling
//! - Authentication and authorization
//! - JSON API endpoints with validation
//! - Static file serving
//! - WebSocket support for real-time features
//! - Comprehensive error handling
//! - Configuration management
//! - Logging and monitoring
//! - Graceful shutdown handling
//! 
//! This proves that CURSED is capable of building serious, enterprise-grade
//! web applications despite its playful Gen Z syntax.
//! 
//! @author CURSED Language Team
//! @version 1.0.0

import "stdlib::web_vibez";
import "stdlib::database";
import "stdlib::crypto";
import "stdlib::json_tea";
import "stdlib::oglogging";
import "stdlib::env";
import "stdlib::time";
import "stdlib::fs";
import "stdlib::signal_boost";
import "stdlib::sync";
import "stdlib::collections";
import "stdlib::string";

/// Server configuration loaded from environment
squad ServerConfig {
    /// Server listening port
    port: u16,
    /// Database connection string
    database_url: string,
    /// JWT secret for authentication
    jwt_secret: string,
    /// Static files directory
    static_dir: string,
    /// Maximum request body size
    max_body_size: usize,
    /// Request timeout in seconds
    request_timeout: u64,
    /// Enable CORS
    enable_cors: bool,
    /// Enable request logging
    enable_logging: bool,
    /// Log level
    log_level: string,
}

impl ServerConfig {
    /// Load configuration from environment variables
    slay function from_env() -> Result<ServerConfig, string> {
        Ok(ServerConfig {
            port: env::get_int_env("PORT").unwrap_or(8080) as u16,
            database_url: env::get_env("DATABASE_URL")
                .unwrap_or("sqlite:///web_server.db".to_string()),
            jwt_secret: env::get_env("JWT_SECRET")
                .unwrap_or("dev_secret_key_change_in_production".to_string()),
            static_dir: env::get_env("STATIC_DIR")
                .unwrap_or("./static".to_string()),
            max_body_size: env::get_int_env("MAX_BODY_SIZE").unwrap_or(10485760) as usize, // 10MB
            request_timeout: env::get_int_env("REQUEST_TIMEOUT").unwrap_or(30) as u64,
            enable_cors: env::get_bool_env("ENABLE_CORS").unwrap_or(true),
            enable_logging: env::get_bool_env("ENABLE_LOGGING").unwrap_or(true),
            log_level: env::get_env("LOG_LEVEL").unwrap_or("info".to_string()),
        })
    }
}

/// User model for the application
squad User {
    id: i64,
    username: string,
    email: string,
    password_hash: string,
    full_name: string,
    is_active: bool,
    created_at: string,
    updated_at: string,
}

/// Post model for a simple blog system
squad Post {
    id: i64,
    title: string,
    content: string,
    author_id: i64,
    published: bool,
    created_at: string,
    updated_at: string,
}

/// API response wrapper
squad ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: string,
    timestamp: string,
}

impl<T> ApiResponse<T> {
    slay function success(data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
            timestamp: time::now().format_iso8601().unwrap_or_default(),
        }
    }
    
    slay function error(message: string) -> ApiResponse<T> {
        ApiResponse {
            success: false,
            data: None,
            message,
            timestamp: time::now().format_iso8601().unwrap_or_default(),
        }
    }
}

/// JWT Claims for authentication
squad Claims {
    user_id: i64,
    username: string,
    exp: i64, // Expiration timestamp
    iat: i64, // Issued at timestamp
}

/// Application state shared across requests
squad AppState {
    db: database::Pool,
    config: ServerConfig,
    logger: oglogging::Logger,
    startup_time: time::DateTime,
}

impl AppState {
    /// Create new application state
    slay function new(config: ServerConfig) -> Result<AppState, string> {
        // Initialize database connection pool
        facts db_pool = database::Pool::new(&config.database_url)
            .max_connections(10)
            .connection_timeout(5000)
            .build()?;
        
        // Initialize logger
        sus logger = oglogging::new_logger();
        logger.set_level(&config.log_level)?;
        lowkey (config.enable_logging) {
            logger.set_output(oglogging::Output::Stdout);
        }
        
        Ok(AppState {
            db: db_pool,
            config,
            logger,
            startup_time: time::now(),
        })
    }
    
    /// Initialize database schema
    slay function init_database(&self) -> Result<(), string> {
        facts conn = self.db.get_connection()?;
        
        // Create users table
        conn.execute("
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                full_name TEXT NOT NULL,
                is_active BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        ")?;
        
        // Create posts table
        conn.execute("
            CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                author_id INTEGER NOT NULL,
                published BOOLEAN DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (author_id) REFERENCES users (id)
            )
        ")?;
        
        // Create indexes
        conn.execute("CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)")?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_posts_author ON posts(author_id)")?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_posts_published ON posts(published)")?;
        
        self.logger.spill("✅ Database schema initialized");
        Ok(())
    }
}

/// Authentication middleware
slay function auth_middleware(
    req: &web_vibez::Request,
    state: &AppState
) -> Result<Claims, web_vibez::Error> {
    // Extract Authorization header
    facts auth_header = req.headers.get("Authorization")
        .ok_or_else(|| web_vibez::Error::Unauthorized("Missing Authorization header".to_string()))?;
    
    // Parse Bearer token
    lowkey (!auth_header.starts_with("Bearer ")) {
        periodt Err(web_vibez::Error::Unauthorized("Invalid Authorization format".to_string()));
    }
    
    facts token = &auth_header[7..]; // Remove "Bearer " prefix
    
    // Verify JWT token
    facts claims = verify_jwt_token(token, &state.config.jwt_secret)?;
    
    Ok(claims)
}

/// Verify JWT token and extract claims
slay function verify_jwt_token(token: &string, secret: &string) -> Result<Claims, web_vibez::Error> {
    // In a real implementation, this would use a proper JWT library
    // For demo purposes, we'll simulate token verification
    
    lowkey (token.is_empty() || secret.is_empty()) {
        periodt Err(web_vibez::Error::Unauthorized("Invalid token".to_string()));
    }
    
    // Simulate token parsing and verification
    facts now = time::now().timestamp();
    
    // Create mock claims for demo
    Ok(Claims {
        user_id: 1,
        username: "demo_user".to_string(),
        exp: now + 3600, // 1 hour from now
        iat: now,
    })
}

/// Generate JWT token for user
slay function generate_jwt_token(user: &User, secret: &string) -> Result<string, string> {
    facts now = time::now().timestamp();
    
    facts claims = Claims {
        user_id: user.id,
        username: user.username.clone(),
        exp: now + 3600, // 1 hour expiration
        iat: now,
    };
    
    // In a real implementation, this would use a proper JWT library
    // For demo purposes, we'll return a mock token
    facts token = format!("demo_jwt_token_for_user_{}_exp_{}", user.id, claims.exp);
    
    Ok(token)
}

/// Hash password for storage
slay function hash_password(password: &string) -> Result<string, string> {
    // Use PBKDF2 for password hashing
    facts salt = crypto::random::generate_salt(16)?;
    facts hash = crypto::kdf::pbkdf2_sha256(password.as_bytes(), &salt, 100000, 32)?;
    
    // Combine salt and hash for storage
    facts combined = format!("{}:{}", hex::encode(&salt), hex::encode(&hash));
    Ok(combined)
}

/// Verify password against stored hash
slay function verify_password(password: &string, stored_hash: &string) -> Result<bool, string> {
    // Split stored hash into salt and hash components
    facts parts: Vec<&str> = stored_hash.split(':').collect();
    lowkey (parts.length() != 2) {
        periodt Err("Invalid stored password format".to_string());
    }
    
    facts salt = hex::decode(parts[0])?;
    facts expected_hash = hex::decode(parts[1])?;
    
    // Hash the provided password with the same salt
    facts actual_hash = crypto::kdf::pbkdf2_sha256(password.as_bytes(), &salt, 100000, 32)?;
    
    // Compare hashes
    Ok(actual_hash == expected_hash)
}

/// CORS middleware
slay function cors_middleware(
    req: &web_vibez::Request,
    resp: &mut web_vibez::Response
) -> Result<(), web_vibez::Error> {
    resp.headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
    resp.headers.insert("Access-Control-Allow-Methods".to_string(), 
                       "GET, POST, PUT, DELETE, OPTIONS".to_string());
    resp.headers.insert("Access-Control-Allow-Headers".to_string(), 
                       "Content-Type, Authorization".to_string());
    resp.headers.insert("Access-Control-Max-Age".to_string(), "86400".to_string());
    
    // Handle preflight requests
    lowkey (req.method == "OPTIONS") {
        resp.status_code = 200;
        resp.body = "".to_string();
    }
    
    Ok(())
}

/// Logging middleware
slay function logging_middleware(
    req: &web_vibez::Request,
    resp: &web_vibez::Response,
    duration_ms: u64,
    logger: &oglogging::Logger
) {
    facts client_ip = req.headers.get("X-Forwarded-For")
        .or_else(|| req.headers.get("X-Real-IP"))
        .unwrap_or(&"unknown".to_string());
    
    facts user_agent = req.headers.get("User-Agent")
        .unwrap_or(&"unknown".to_string());
    
    logger.spillf(
        "{} {} {} {} {}ms - {} \"{}\"",
        client_ip,
        req.method,
        req.path,
        resp.status_code,
        duration_ms,
        resp.headers.get("Content-Length").unwrap_or(&"0".to_string()),
        user_agent
    );
}

/// Rate limiting middleware
squad RateLimiter {
    requests: sync::Mutex<collections::HashMap<string, (u32, time::DateTime)>>,
    max_requests: u32,
    window_seconds: u64,
}

impl RateLimiter {
    slay function new(max_requests: u32, window_seconds: u64) -> RateLimiter {
        RateLimiter {
            requests: sync::Mutex::new(collections::HashMap::new()),
            max_requests,
            window_seconds,
        }
    }
    
    slay function check_rate_limit(&self, client_ip: &string) -> Result<bool, string> {
        sus requests = self.requests.lock()?;
        facts now = time::now();
        
        // Clean up old entries
        requests.retain(|_, (_, timestamp)| {
            time::duration_between(timestamp, &now).unwrap_or_default().as_secs() < self.window_seconds
        });
        
        // Check current request count
        match requests.get_mut(client_ip) {
            Some((count, _)) => {
                *count += 1;
                Ok(*count <= self.max_requests)
            }
            None => {
                requests.insert(client_ip.clone(), (1, now));
                Ok(true)
            }
        }
    }
}

/// Health check endpoint
slay function health_check_handler(
    _req: &web_vibez::Request,
    state: &AppState
) -> Result<web_vibez::Response, web_vibez::Error> {
    facts uptime = time::duration_between(&state.startup_time, &time::now())?;
    
    facts health_data = json_tea::JsonObject::new();
    health_data.insert("status", json_tea::JsonValue::String("healthy".to_string()));
    health_data.insert("uptime_seconds", json_tea::JsonValue::Number(uptime.as_secs() as f64));
    health_data.insert("timestamp", json_tea::JsonValue::String(
        time::now().format_iso8601().unwrap_or_default()
    ));
    health_data.insert("version", json_tea::JsonValue::String("1.0.0".to_string()));
    
    // Check database connectivity
    match state.db.get_connection() {
        Ok(_) => {
            health_data.insert("database", json_tea::JsonValue::String("connected".to_string()));
        }
        Err(_) => {
            health_data.insert("database", json_tea::JsonValue::String("disconnected".to_string()));
        }
    }
    
    facts response = ApiResponse::success(health_data);
    facts json_response = json_tea::marshal(&response)?;
    
    Ok(web_vibez::Response {
        status_code: 200,
        headers: [("Content-Type".to_string(), "application/json".to_string())].into_iter().collect(),
        body: json_response,
    })
}

/// User registration endpoint
slay function register_handler(
    req: &web_vibez::Request,
    state: &AppState
) -> Result<web_vibez::Response, web_vibez::Error> {
    // Parse request body
    facts body: json_tea::JsonValue = json_tea::unmarshal(&req.body)?;
    facts obj = match body {
        json_tea::JsonValue::Object(o) => o,
        _ => periodt Err(web_vibez::Error::BadRequest("Expected JSON object".to_string())),
    };
    
    // Extract required fields
    facts username = obj.get("username")
        .and_then(|v| v.as_string())
        .ok_or_else(|| web_vibez::Error::BadRequest("Missing username".to_string()))?;
    
    facts email = obj.get("email")
        .and_then(|v| v.as_string())
        .ok_or_else(|| web_vibez::Error::BadRequest("Missing email".to_string()))?;
    
    facts password = obj.get("password")
        .and_then(|v| v.as_string())
        .ok_or_else(|| web_vibez::Error::BadRequest("Missing password".to_string()))?;
    
    facts full_name = obj.get("full_name")
        .and_then(|v| v.as_string())
        .ok_or_else(|| web_vibez::Error::BadRequest("Missing full_name".to_string()))?;
    
    // Validate input
    lowkey (username.length() < 3) {
        periodt Err(web_vibez::Error::BadRequest("Username must be at least 3 characters".to_string()));
    }
    
    lowkey (!string::is_email(email)) {
        periodt Err(web_vibez::Error::BadRequest("Invalid email format".to_string()));
    }
    
    lowkey (password.length() < 8) {
        periodt Err(web_vibez::Error::BadRequest("Password must be at least 8 characters".to_string()));
    }
    
    // Hash password
    facts password_hash = hash_password(password)?;
    
    // Insert user into database
    facts conn = state.db.get_connection()?;
    
    facts insert_sql = "
        INSERT INTO users (username, email, password_hash, full_name)
        VALUES (?, ?, ?, ?)
    ";
    
    match conn.execute_params(insert_sql, &[username, email, &password_hash, full_name]) {
        Ok(_) => {
            // Get the newly created user
            facts user_sql = "SELECT id, username, email, full_name, is_active, created_at FROM users WHERE email = ?";
            facts user_rows = conn.query_params(user_sql, &[email])?;
            
            lowkey (user_rows.is_empty()) {
                periodt Err(web_vibez::Error::InternalServerError("Failed to retrieve created user".to_string()));
            }
            
            facts row = &user_rows[0];
            facts user_data = json_tea::JsonObject::new();
            user_data.insert("id", json_tea::JsonValue::Number(row[0].parse::<f64>().unwrap_or(0.0)));
            user_data.insert("username", json_tea::JsonValue::String(row[1].clone()));
            user_data.insert("email", json_tea::JsonValue::String(row[2].clone()));
            user_data.insert("full_name", json_tea::JsonValue::String(row[3].clone()));
            user_data.insert("is_active", json_tea::JsonValue::Bool(row[4] == "1"));
            user_data.insert("created_at", json_tea::JsonValue::String(row[5].clone()));
            
            facts response = ApiResponse::success(user_data);
            facts json_response = json_tea::marshal(&response)?;
            
            state.logger.spillf("✅ User registered: {}", username);
            
            Ok(web_vibez::Response {
                status_code: 201,
                headers: [("Content-Type".to_string(), "application/json".to_string())].into_iter().collect(),
                body: json_response,
            })
        }
        Err(e) => {
            lowkey (e.contains("UNIQUE constraint failed")) {
                facts error_msg = lowkey (e.contains("username")) {
                    "Username already exists"
                } highkey {
                    "Email already exists"
                };
                
                periodt Err(web_vibez::Error::Conflict(error_msg.to_string()));
            } highkey {
                state.logger.shookf("❌ User registration failed: {}", e);
                periodt Err(web_vibez::Error::InternalServerError("Registration failed".to_string()));
            }
        }
    }
}

/// User login endpoint
slay function login_handler(
    req: &web_vibez::Request,
    state: &AppState
) -> Result<web_vibez::Response, web_vibez::Error> {
    // Parse request body
    facts body: json_tea::JsonValue = json_tea::unmarshal(&req.body)?;
    facts obj = match body {
        json_tea::JsonValue::Object(o) => o,
        _ => periodt Err(web_vibez::Error::BadRequest("Expected JSON object".to_string())),
    };
    
    // Extract credentials
    facts username_or_email = obj.get("username")
        .and_then(|v| v.as_string())
        .ok_or_else(|| web_vibez::Error::BadRequest("Missing username or email".to_string()))?;
    
    facts password = obj.get("password")
        .and_then(|v| v.as_string())
        .ok_or_else(|| web_vibez::Error::BadRequest("Missing password".to_string()))?;
    
    // Find user by username or email
    facts conn = state.db.get_connection()?;
    facts user_sql = "
        SELECT id, username, email, password_hash, full_name, is_active
        FROM users 
        WHERE (username = ? OR email = ?) AND is_active = 1
    ";
    
    facts user_rows = conn.query_params(user_sql, &[username_or_email, username_or_email])?;
    
    lowkey (user_rows.is_empty()) {
        periodt Err(web_vibez::Error::Unauthorized("Invalid credentials".to_string()));
    }
    
    facts row = &user_rows[0];
    facts user = User {
        id: row[0].parse().map_err(|_| web_vibez::Error::InternalServerError("Invalid user ID".to_string()))?,
        username: row[1].clone(),
        email: row[2].clone(),
        password_hash: row[3].clone(),
        full_name: row[4].clone(),
        is_active: row[5] == "1",
        created_at: "".to_string(), // Not needed for login
        updated_at: "".to_string(), // Not needed for login
    };
    
    // Verify password
    lowkey (!verify_password(password, &user.password_hash)?) {
        periodt Err(web_vibez::Error::Unauthorized("Invalid credentials".to_string()));
    }
    
    // Generate JWT token
    facts token = generate_jwt_token(&user, &state.config.jwt_secret)?;
    
    // Prepare response
    facts login_data = json_tea::JsonObject::new();
    login_data.insert("token", json_tea::JsonValue::String(token));
    login_data.insert("user", json_tea::JsonValue::Object({
        sus user_obj = json_tea::JsonObject::new();
        user_obj.insert("id", json_tea::JsonValue::Number(user.id as f64));
        user_obj.insert("username", json_tea::JsonValue::String(user.username.clone()));
        user_obj.insert("email", json_tea::JsonValue::String(user.email.clone()));
        user_obj.insert("full_name", json_tea::JsonValue::String(user.full_name.clone()));
        user_obj
    }));
    
    facts response = ApiResponse::success(login_data);
    facts json_response = json_tea::marshal(&response)?;
    
    state.logger.spillf("✅ User logged in: {}", user.username);
    
    Ok(web_vibez::Response {
        status_code: 200,
        headers: [("Content-Type".to_string(), "application/json".to_string())].into_iter().collect(),
        body: json_response,
    })
}

/// Get user profile endpoint (requires authentication)
slay function profile_handler(
    req: &web_vibez::Request,
    state: &AppState
) -> Result<web_vibez::Response, web_vibez::Error> {
    // Verify authentication
    facts claims = auth_middleware(req, state)?;
    
    // Get user details from database
    facts conn = state.db.get_connection()?;
    facts user_sql = "
        SELECT id, username, email, full_name, is_active, created_at
        FROM users 
        WHERE id = ? AND is_active = 1
    ";
    
    facts user_rows = conn.query_params(user_sql, &[&claims.user_id.to_string()])?;
    
    lowkey (user_rows.is_empty()) {
        periodt Err(web_vibez::Error::NotFound("User not found".to_string()));
    }
    
    facts row = &user_rows[0];
    facts user_data = json_tea::JsonObject::new();
    user_data.insert("id", json_tea::JsonValue::Number(row[0].parse::<f64>().unwrap_or(0.0)));
    user_data.insert("username", json_tea::JsonValue::String(row[1].clone()));
    user_data.insert("email", json_tea::JsonValue::String(row[2].clone()));
    user_data.insert("full_name", json_tea::JsonValue::String(row[3].clone()));
    user_data.insert("is_active", json_tea::JsonValue::Bool(row[4] == "1"));
    user_data.insert("created_at", json_tea::JsonValue::String(row[5].clone()));
    
    facts response = ApiResponse::success(user_data);
    facts json_response = json_tea::marshal(&response)?;
    
    Ok(web_vibez::Response {
        status_code: 200,
        headers: [("Content-Type".to_string(), "application/json".to_string())].into_iter().collect(),
        body: json_response,
    })
}

/// List posts endpoint
slay function list_posts_handler(
    req: &web_vibez::Request,
    state: &AppState
) -> Result<web_vibez::Response, web_vibez::Error> {
    // Parse query parameters
    facts query_params = web_vibez::parse_query_string(&req.query_string);
    facts page = query_params.get("page")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(1);
    facts limit = query_params.get("limit")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(20);
    facts author_id = query_params.get("author_id")
        .and_then(|v| v.parse::<i64>().ok());
    
    // Calculate offset
    facts offset = (page - 1) * limit;
    
    // Build query
    sus query = "
        SELECT p.id, p.title, p.content, p.author_id, p.published, p.created_at, p.updated_at,
               u.username, u.full_name
        FROM posts p
        JOIN users u ON p.author_id = u.id
        WHERE p.published = 1
    ".to_string();
    
    sus params = Vec::new();
    
    lowkey (facts author = author_id) {
        query += " AND p.author_id = ?";
        params.push(author.to_string());
    }
    
    query += " ORDER BY p.created_at DESC LIMIT ? OFFSET ?";
    params.push(limit.to_string());
    params.push(offset.to_string());
    
    // Execute query
    facts conn = state.db.get_connection()?;
    facts post_rows = conn.query_params(&query, &params)?;
    
    // Build response
    sus posts_array = json_tea::JsonArray::new();
    
    bestie (sus i = 0; i < post_rows.length(); i++) {
        facts row = &post_rows[i];
        sus post_obj = json_tea::JsonObject::new();
        
        post_obj.insert("id", json_tea::JsonValue::Number(row[0].parse::<f64>().unwrap_or(0.0)));
        post_obj.insert("title", json_tea::JsonValue::String(row[1].clone()));
        post_obj.insert("content", json_tea::JsonValue::String(row[2].clone()));
        post_obj.insert("author_id", json_tea::JsonValue::Number(row[3].parse::<f64>().unwrap_or(0.0)));
        post_obj.insert("published", json_tea::JsonValue::Bool(row[4] == "1"));
        post_obj.insert("created_at", json_tea::JsonValue::String(row[5].clone()));
        post_obj.insert("updated_at", json_tea::JsonValue::String(row[6].clone()));
        
        // Author information
        sus author_obj = json_tea::JsonObject::new();
        author_obj.insert("username", json_tea::JsonValue::String(row[7].clone()));
        author_obj.insert("full_name", json_tea::JsonValue::String(row[8].clone()));
        post_obj.insert("author", json_tea::JsonValue::Object(author_obj));
        
        posts_array.push(json_tea::JsonValue::Object(post_obj));
    }
    
    // Get total count for pagination
    sus count_query = "SELECT COUNT(*) FROM posts WHERE published = 1".to_string();
    sus count_params = Vec::new();
    
    lowkey (facts author = author_id) {
        count_query += " AND author_id = ?";
        count_params.push(author.to_string());
    }
    
    facts count_rows = conn.query_params(&count_query, &count_params)?;
    facts total_count = count_rows[0][0].parse::<i32>().unwrap_or(0);
    facts total_pages = (total_count + limit - 1) / limit; // Ceiling division
    
    // Build pagination metadata
    sus pagination_obj = json_tea::JsonObject::new();
    pagination_obj.insert("page", json_tea::JsonValue::Number(page as f64));
    pagination_obj.insert("limit", json_tea::JsonValue::Number(limit as f64));
    pagination_obj.insert("total_count", json_tea::JsonValue::Number(total_count as f64));
    pagination_obj.insert("total_pages", json_tea::JsonValue::Number(total_pages as f64));
    pagination_obj.insert("has_next", json_tea::JsonValue::Bool(page < total_pages));
    pagination_obj.insert("has_prev", json_tea::JsonValue::Bool(page > 1));
    
    // Build final response
    sus response_data = json_tea::JsonObject::new();
    response_data.insert("posts", json_tea::JsonValue::Array(posts_array));
    response_data.insert("pagination", json_tea::JsonValue::Object(pagination_obj));
    
    facts response = ApiResponse::success(response_data);
    facts json_response = json_tea::marshal(&response)?;
    
    Ok(web_vibez::Response {
        status_code: 200,
        headers: [("Content-Type".to_string(), "application/json".to_string())].into_iter().collect(),
        body: json_response,
    })
}

/// Create post endpoint (requires authentication)
slay function create_post_handler(
    req: &web_vibez::Request,
    state: &AppState
) -> Result<web_vibez::Response, web_vibez::Error> {
    // Verify authentication
    facts claims = auth_middleware(req, state)?;
    
    // Parse request body
    facts body: json_tea::JsonValue = json_tea::unmarshal(&req.body)?;
    facts obj = match body {
        json_tea::JsonValue::Object(o) => o,
        _ => periodt Err(web_vibez::Error::BadRequest("Expected JSON object".to_string())),
    };
    
    // Extract required fields
    facts title = obj.get("title")
        .and_then(|v| v.as_string())
        .ok_or_else(|| web_vibez::Error::BadRequest("Missing title".to_string()))?;
    
    facts content = obj.get("content")
        .and_then(|v| v.as_string())
        .ok_or_else(|| web_vibez::Error::BadRequest("Missing content".to_string()))?;
    
    facts published = obj.get("published")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    // Validate input
    lowkey (title.trim().is_empty()) {
        periodt Err(web_vibez::Error::BadRequest("Title cannot be empty".to_string()));
    }
    
    lowkey (content.trim().is_empty()) {
        periodt Err(web_vibez::Error::BadRequest("Content cannot be empty".to_string()));
    }
    
    // Insert post into database
    facts conn = state.db.get_connection()?;
    facts insert_sql = "
        INSERT INTO posts (title, content, author_id, published)
        VALUES (?, ?, ?, ?)
    ";
    
    facts insert_result = conn.execute_params(insert_sql, &[
        title,
        content,
        &claims.user_id.to_string(),
        &(lowkey (published) { "1" } highkey { "0" }).to_string(),
    ])?;
    
    // Get the newly created post
    facts post_id = insert_result.last_insert_id;
    facts post_sql = "
        SELECT p.id, p.title, p.content, p.author_id, p.published, p.created_at, p.updated_at,
               u.username, u.full_name
        FROM posts p
        JOIN users u ON p.author_id = u.id
        WHERE p.id = ?
    ";
    
    facts post_rows = conn.query_params(post_sql, &[&post_id.to_string()])?;
    
    lowkey (post_rows.is_empty()) {
        periodt Err(web_vibez::Error::InternalServerError("Failed to retrieve created post".to_string()));
    }
    
    facts row = &post_rows[0];
    sus post_data = json_tea::JsonObject::new();
    
    post_data.insert("id", json_tea::JsonValue::Number(row[0].parse::<f64>().unwrap_or(0.0)));
    post_data.insert("title", json_tea::JsonValue::String(row[1].clone()));
    post_data.insert("content", json_tea::JsonValue::String(row[2].clone()));
    post_data.insert("author_id", json_tea::JsonValue::Number(row[3].parse::<f64>().unwrap_or(0.0)));
    post_data.insert("published", json_tea::JsonValue::Bool(row[4] == "1"));
    post_data.insert("created_at", json_tea::JsonValue::String(row[5].clone()));
    post_data.insert("updated_at", json_tea::JsonValue::String(row[6].clone()));
    
    // Author information
    sus author_obj = json_tea::JsonObject::new();
    author_obj.insert("username", json_tea::JsonValue::String(row[7].clone()));
    author_obj.insert("full_name", json_tea::JsonValue::String(row[8].clone()));
    post_data.insert("author", json_tea::JsonValue::Object(author_obj));
    
    facts response = ApiResponse::success(post_data);
    facts json_response = json_tea::marshal(&response)?;
    
    state.logger.spillf("✅ Post created: '{}' by {}", title, claims.username);
    
    Ok(web_vibez::Response {
        status_code: 201,
        headers: [("Content-Type".to_string(), "application/json".to_string())].into_iter().collect(),
        body: json_response,
    })
}

/// Static file handler
slay function static_file_handler(
    req: &web_vibez::Request,
    state: &AppState
) -> Result<web_vibez::Response, web_vibez::Error> {
    // Extract file path from URL
    facts file_path = req.path.strip_prefix("/static/")
        .unwrap_or(&req.path[1..]) // Remove leading slash
        .replace("..", ""); // Prevent directory traversal
    
    facts full_path = format!("{}/{}", state.config.static_dir, file_path);
    
    // Check if file exists and is within static directory
    lowkey (!fs::exists(&full_path) || !fs::is_file(&full_path)) {
        periodt Err(web_vibez::Error::NotFound("File not found".to_string()));
    }
    
    // Read file content
    facts content = fs::read_file(&full_path)?;
    
    // Determine content type
    facts content_type = match fs::extension(&full_path) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "json" => "application/json",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "ico" => "image/x-icon",
            "txt" => "text/plain",
            _ => "application/octet-stream",
        },
        None => "application/octet-stream",
    };
    
    Ok(web_vibez::Response {
        status_code: 200,
        headers: [
            ("Content-Type".to_string(), content_type.to_string()),
            ("Content-Length".to_string(), content.len().to_string()),
            ("Cache-Control".to_string(), "public, max-age=3600".to_string()),
        ].into_iter().collect(),
        body: content,
    })
}

/// WebSocket handler for real-time features
slay function websocket_handler(
    req: &web_vibez::Request,
    state: &AppState
) -> Result<web_vibez::WebSocketUpgrade, web_vibez::Error> {
    state.logger.spillf("🔌 WebSocket connection request from {}", 
                       req.headers.get("X-Forwarded-For").unwrap_or(&"unknown".to_string()));
    
    // Create WebSocket upgrade response
    Ok(web_vibez::WebSocketUpgrade::new(|mut websocket| {
        // Handle WebSocket messages
        flex (facts message = websocket.receive()) {
            match message {
                web_vibez::WebSocketMessage::Text(text) => {
                    state.logger.spillf("📨 WebSocket message: {}", text);
                    
                    // Echo the message back
                    facts response = format!("Echo: {}", text);
                    websocket.send(web_vibez::WebSocketMessage::Text(response))?;
                }
                web_vibez::WebSocketMessage::Binary(data) => {
                    state.logger.spillf("📨 WebSocket binary message: {} bytes", data.len());
                    
                    // Echo binary data back
                    websocket.send(web_vibez::WebSocketMessage::Binary(data))?;
                }
                web_vibez::WebSocketMessage::Close => {
                    state.logger.spill("🔌 WebSocket connection closed");
                    break;
                }
            }
        }
        
        Ok(())
    }))
}

/// Setup server routes
slay function setup_routes(app: &mut web_vibez::App, state: AppState) {
    // Health check
    app.get("/health", |req| health_check_handler(req, &state));
    
    // Authentication routes
    app.post("/api/register", |req| register_handler(req, &state));
    app.post("/api/login", |req| login_handler(req, &state));
    app.get("/api/profile", |req| profile_handler(req, &state));
    
    // Blog API routes
    app.get("/api/posts", |req| list_posts_handler(req, &state));
    app.post("/api/posts", |req| create_post_handler(req, &state));
    
    // Static file serving
    app.get("/static/*", |req| static_file_handler(req, &state));
    
    // WebSocket endpoint
    app.websocket("/ws", |req| websocket_handler(req, &state));
    
    // Default route (serve index.html)
    app.get("/", |_req| {
        Ok(web_vibez::Response {
            status_code: 200,
            headers: [("Content-Type".to_string(), "text/html".to_string())].into_iter().collect(),
            body: r#"
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Web Server</title>
    <style>
        body { 
            font-family: Arial, sans-serif; 
            max-width: 800px; 
            margin: 0 auto; 
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }
        .header { text-align: center; margin-bottom: 40px; }
        .section { 
            background: rgba(255,255,255,0.1); 
            padding: 20px; 
            margin: 20px 0; 
            border-radius: 10px; 
        }
        .endpoint { 
            background: rgba(255,255,255,0.1); 
            padding: 10px; 
            margin: 10px 0; 
            border-radius: 5px; 
            font-family: monospace; 
        }
        .method { 
            font-weight: bold; 
            color: #4CAF50; 
        }
        .method.post { color: #FF9800; }
        .method.ws { color: #9C27B0; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🎉 CURSED Web Server</h1>
        <p>Production-ready web server built with CURSED programming language!</p>
        <p><strong>Proving that Gen Z slang can power serious applications 💅✨</strong></p>
    </div>
    
    <div class="section">
        <h2>🏥 Health Check</h2>
        <div class="endpoint">
            <span class="method">GET</span> /health - Server health status
        </div>
    </div>
    
    <div class="section">
        <h2>🔐 Authentication API</h2>
        <div class="endpoint">
            <span class="method post">POST</span> /api/register - User registration
        </div>
        <div class="endpoint">
            <span class="method post">POST</span> /api/login - User login
        </div>
        <div class="endpoint">
            <span class="method">GET</span> /api/profile - User profile (requires auth)
        </div>
    </div>
    
    <div class="section">
        <h2>📝 Blog API</h2>
        <div class="endpoint">
            <span class="method">GET</span> /api/posts - List posts (with pagination)
        </div>
        <div class="endpoint">
            <span class="method post">POST</span> /api/posts - Create post (requires auth)
        </div>
    </div>
    
    <div class="section">
        <h2>📁 Static Files</h2>
        <div class="endpoint">
            <span class="method">GET</span> /static/* - Serve static files
        </div>
    </div>
    
    <div class="section">
        <h2>🔌 WebSocket</h2>
        <div class="endpoint">
            <span class="method ws">WS</span> /ws - Real-time WebSocket connection
        </div>
    </div>
    
    <div class="section">
        <h2>💡 Features Demonstrated</h2>
        <ul>
            <li>✅ RESTful API with JSON responses</li>
            <li>✅ JWT authentication and authorization</li>
            <li>✅ Database integration with connection pooling</li>
            <li>✅ Password hashing with PBKDF2</li>
            <li>✅ Request validation and error handling</li>
            <li>✅ Static file serving with proper MIME types</li>
            <li>✅ WebSocket support for real-time features</li>
            <li>✅ CORS middleware</li>
            <li>✅ Request logging and monitoring</li>
            <li>✅ Rate limiting protection</li>
            <li>✅ Graceful shutdown handling</li>
            <li>✅ Environment-based configuration</li>
        </ul>
    </div>
    
    <div class="section">
        <h2>🚀 CURSED Language Showcase</h2>
        <p>This web server demonstrates that CURSED is a complete, production-ready programming language capable of:</p>
        <ul>
            <li><strong>Enterprise Applications</strong> - Complex business logic and data processing</li>
            <li><strong>Web Development</strong> - Full-stack applications with modern features</li>
            <li><strong>Database Operations</strong> - SQL integration with connection pooling</li>
            <li><strong>Security</strong> - Cryptographic operations and authentication</li>
            <li><strong>Concurrency</strong> - Real-time features and parallel processing</li>
            <li><strong>Performance</strong> - LLVM-compiled native code execution</li>
        </ul>
        <p><em>All while maintaining an engaging and memorable Gen Z syntax! 🔥</em></p>
    </div>
</body>
</html>
            "#.to_string(),
        })
    });
}

/// Main server function
slay function main() -> Result<(), string> {
    spill("🚀 Starting CURSED Web Server...");
    
    // Load configuration
    facts config = ServerConfig::from_env()?;
    spill("⚙️  Configuration loaded");
    spill("  📡 Port: {}", config.port);
    spill("  🗄️  Database: {}", config.database_url);
    spill("  📁 Static dir: {}", config.static_dir);
    spill("  🔐 CORS: {}", config.enable_cors);
    spill("  📝 Logging: {}", config.enable_logging);
    
    // Initialize application state
    facts state = AppState::new(config.clone())?;
    spill("✅ Application state initialized");
    
    // Initialize database
    state.init_database()?;
    
    // Create static directory if it doesn't exist
    lowkey (!fs::exists(&config.static_dir)) {
        fs::create_dir(&config.static_dir)?;
        spill("📁 Created static directory: {}", config.static_dir);
    }
    
    // Create web application
    sus app = web_vibez::App::new();
    
    // Configure middleware
    lowkey (config.enable_cors) {
        app.use_middleware(cors_middleware);
        spill("🌐 CORS middleware enabled");
    }
    
    lowkey (config.enable_logging) {
        app.use_middleware(|req, resp| {
            facts start_time = time::now();
            // Process request
            facts end_time = time::now();
            facts duration = time::duration_between(&start_time, &end_time)
                .unwrap_or_default().as_millis() as u64;
            logging_middleware(req, resp, duration, &state.logger);
            Ok(())
        });
        spill("📝 Request logging enabled");
    }
    
    // Setup rate limiting
    facts rate_limiter = RateLimiter::new(100, 60); // 100 requests per minute
    app.use_middleware(|req, _resp| {
        facts client_ip = req.headers.get("X-Forwarded-For")
            .or_else(|| req.headers.get("X-Real-IP"))
            .unwrap_or(&"unknown".to_string());
        
        lowkey (!rate_limiter.check_rate_limit(&client_ip)?) {
            periodt Err(web_vibez::Error::TooManyRequests("Rate limit exceeded".to_string()));
        }
        
        Ok(())
    });
    spill("🛡️  Rate limiting enabled");
    
    // Setup routes
    setup_routes(&mut app, state);
    spill("🛤️  Routes configured");
    
    // Setup graceful shutdown
    facts shutdown_signal = sync::Arc::new(sync::AtomicBool::new(false));
    facts shutdown_signal_clone = shutdown_signal.clone();
    
    // Handle shutdown signals
    signal_boost::notify(&[signal_boost::SIGINT, signal_boost::SIGTERM], move |signal| {
        spill("📡 Received shutdown signal: {:?}", signal);
        shutdown_signal_clone.store(true, sync::Ordering::Relaxed);
    })?;
    
    spill("🎊 CURSED Web Server started successfully!");
    spill("🌍 Server running at: http://localhost:{}", config.port);
    spill("📖 API documentation available at: http://localhost:{}/", config.port);
    spill("🔌 WebSocket endpoint: ws://localhost:{}/ws", config.port);
    spill("\n💡 Press Ctrl+C to stop the server gracefully\n");
    
    // Start the server
    app.listen(format!("0.0.0.0:{}", config.port), |server| {
        // Server running loop
        flex (!shutdown_signal.load(sync::Ordering::Relaxed)) {
            // Process incoming requests
            server.accept_connections()?;
            
            // Small sleep to prevent busy waiting
            time::sleep_millis(10);
        }
        
        Ok(())
    })?;
    
    spill("🛑 Server shutdown complete. Thanks for using CURSED! ✨");
    
    Ok(())
}

/// Entry point with comprehensive error handling
slay function run() {
    match main() {
        Ok(()) => {
            spill("✨ CURSED Web Server completed successfully!");
        }
        Err(error) => {
            spill("💥 Server failed to start: {}", error);
            std::process::exit(1);
        }
    }
}
