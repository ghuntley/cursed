fr fr Web application with authentication and middleware
fr fr Demonstrates session management, authentication, and advanced middleware

yeet "web_vibez"
yeet "vibez"
yeet "json_tea"
yeet "cryptz"
yeet "time_utils"

fr fr User session structure
squad Session {
    token: tea,
    user_id: numo,
    username: tea,
    created_at: numo,
    expires_at: numo
}

fr fr User structure
squad User {
    id: numo,
    username: tea,
    email: tea,
    password_hash: tea,
    role: tea,
    created_at: numo
}

fr fr Global storage (in production, use a real database)
sus users = []User{}
sus sessions = []Session{}
sus next_user_id = 1

fr fr Authentication middleware
slay auth_middleware() {
    damn slay(request) {
        fr fr Skip auth for login/register endpoints
        lowkey request.url == "/api/login" || request.url == "/api/register" || request.url == "/" {
            damn cap fr fr Continue
        }
        
        fr fr Check for Authorization header
        sus auth_header = request.headers.get("Authorization")
        lowkey auth_header == cap {
            damn web_vibez.Response{
                status: 401,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Missing Authorization header"}'
            }
        }
        
        fr fr Extract bearer token
        lowkey !auth_header.starts_with("Bearer ") {
            damn web_vibez.Response{
                status: 401,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Invalid Authorization header format"}'
            }
        }
        
        sus token = auth_header.substring(7) fr fr Remove "Bearer "
        
        fr fr Find session
        sus session = cap
        bestie s in sessions {
            lowkey s.token == token {
                fr fr Check if session is expired
                sus now = time_utils.unix_timestamp()
                lowkey s.expires_at > now {
                    session = s
                    break
                }
            }
        }
        
        lowkey session == cap {
            damn web_vibez.Response{
                status: 401,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Invalid or expired token"}'
            }
        }
        
        fr fr Add user info to request context (simplified)
        request.user_id = session.user_id
        request.username = session.username
        
        damn cap fr fr Continue
    }
}

fr fr Role-based access control middleware
slay rbac_middleware(required_role: tea) {
    damn slay(request) {
        fr fr Get user from request context
        lowkey request.user_id == cap {
            damn web_vibez.Response{
                status: 403,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Access denied: User not authenticated"}'
            }
        }
        
        fr fr Find user
        sus user = cap
        bestie u in users {
            lowkey u.id == request.user_id {
                user = u
                break
            }
        }
        
        lowkey user == cap || user.role != required_role {
            damn web_vibez.Response{
                status: 403,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Access denied: Insufficient permissions"}'
            }
        }
        
        damn cap fr fr Continue
    }
}

fr fr Rate limiting middleware
slay rate_limit_middleware(max_requests: numo, window_seconds: numo) {
    sus request_counts = {}
    
    damn slay(request) {
        sus client_ip = request.headers.get("X-Real-IP") || "127.0.0.1"
        sus now = time_utils.unix_timestamp()
        sus window_start = now - window_seconds
        
        fr fr Clean old entries
        bestie ip, timestamps in request_counts {
            request_counts[ip] = timestamps.filter(slay(t) { damn t > window_start })
        }
        
        fr fr Check current count
        lowkey request_counts[client_ip] == cap {
            request_counts[client_ip] = []
        }
        
        lowkey request_counts[client_ip].len() >= max_requests {
            damn web_vibez.Response{
                status: 429,
                headers: {
                    "Content-Type": "application/json",
                    "Retry-After": window_seconds.to_string()
                },
                body: '{"error": "Rate limit exceeded"}'
            }
        }
        
        fr fr Add current request
        request_counts[client_ip].append(now)
        
        damn cap fr fr Continue
    }
}

slay main_character() {
    vibez.spill("Starting authenticated API server...")
    
    sus config = web_vibez.ServerConfig{
        host: "0.0.0.0",
        port: 4000,
        max_connections: 500,
        timeout: 60000
    }
    
    sus server = web_vibez.create_server(config)
    
    fr fr Add global middleware
    server.add_middleware(web_vibez.cors_middleware())
    server.add_middleware(web_vibez.logging_middleware())
    server.add_middleware(rate_limit_middleware(100, 3600)) fr fr 100 requests per hour
    
    fr fr Public endpoints (no auth required)
    
    fr fr Root endpoint with API documentation
    server.add_route("/", slay(request) {
        sus html = """
        <html>
        <head>
            <title>CURSED Auth API</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }
                .container { max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
                .endpoint { background: #f8f9fa; padding: 15px; margin: 10px 0; border-radius: 5px; border-left: 4px solid #007bff; }
                .method { font-weight: bold; color: #007bff; }
                .auth-required { border-left-color: #dc3545; }
                .auth-required .method { color: #dc3545; }
                .admin-only { border-left-color: #ffc107; }
                .admin-only .method { color: #e8a317; }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>🔐 CURSED Authentication API</h1>
                <p>A full-featured authentication API with JWT tokens, role-based access control, and rate limiting!</p>
                
                <h2>Public Endpoints:</h2>
                <div class="endpoint">
                    <span class="method">POST</span> /api/register - Register new user
                    <br><small>Body: {"username": "string", "email": "string", "password": "string"}</small>
                </div>
                <div class="endpoint">
                    <span class="method">POST</span> /api/login - Login user
                    <br><small>Body: {"username": "string", "password": "string"}</small>
                </div>
                
                <h2>Protected Endpoints (Auth Required):</h2>
                <div class="endpoint auth-required">
                    <span class="method">GET</span> /api/profile - Get user profile
                    <br><small>Header: Authorization: Bearer {token}</small>
                </div>
                <div class="endpoint auth-required">
                    <span class="method">PUT</span> /api/profile - Update user profile
                    <br><small>Header: Authorization: Bearer {token}</small>
                </div>
                <div class="endpoint auth-required">
                    <span class="method">POST</span> /api/logout - Logout user
                    <br><small>Header: Authorization: Bearer {token}</small>
                </div>
                
                <h2>Admin Only Endpoints:</h2>
                <div class="endpoint admin-only">
                    <span class="method">GET</span> /api/admin/users - List all users
                    <br><small>Header: Authorization: Bearer {admin_token}</small>
                </div>
                <div class="endpoint admin-only">
                    <span class="method">DELETE</span> /api/admin/users/:id - Delete user
                    <br><small>Header: Authorization: Bearer {admin_token}</small>
                </div>
                
                <h2>Features:</h2>
                <ul>
                    <li>JWT-based authentication</li>
                    <li>Password hashing with bcrypt</li>
                    <li>Role-based access control (user/admin)</li>
                    <li>Rate limiting (100 requests/hour)</li>
                    <li>Session management</li>
                    <li>CORS support</li>
                    <li>Request logging</li>
                </ul>
            </div>
        </body>
        </html>
        """
        
        damn web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "text/html"},
            body: html
        }
    })
    
    fr fr User registration
    server.add_route("/api/register", slay(request) {
        lowkey request.method != "POST" {
            damn web_vibez.Response{
                status: 405,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Method not allowed"}'
            }
        }
        
        sus user_data = json_tea.decode(request.body)
        lowkey user_data.username == cap || user_data.email == cap || user_data.password == cap {
            damn web_vibez.Response{
                status: 400,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "username, email, and password are required"}'
            }
        }
        
        fr fr Check if username already exists
        bestie existing_user in users {
            lowkey existing_user.username == user_data.username {
                damn web_vibez.Response{
                    status: 409,
                    headers: {"Content-Type": "application/json"},
                    body: '{"error": "Username already exists"}'
                }
            }
        }
        
        fr fr Hash password
        sus password_hash = cryptz.hash_password(user_data.password)
        
        fr fr Create new user
        sus new_user = User{
            id: next_user_id,
            username: user_data.username,
            email: user_data.email,
            password_hash: password_hash,
            role: "user", fr fr Default role
            created_at: time_utils.unix_timestamp()
        }
        
        users.append(new_user)
        next_user_id++
        
        sus response = {
            "message": "User registered successfully",
            "user": {
                "id": new_user.id,
                "username": new_user.username,
                "email": new_user.email,
                "role": new_user.role
            }
        }
        
        damn web_vibez.Response{
            status: 201,
            headers: {"Content-Type": "application/json"},
            body: json_tea.encode(response)
        }
    })
    
    fr fr User login
    server.add_route("/api/login", slay(request) {
        lowkey request.method != "POST" {
            damn web_vibez.Response{
                status: 405,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Method not allowed"}'
            }
        }
        
        sus login_data = json_tea.decode(request.body)
        lowkey login_data.username == cap || login_data.password == cap {
            damn web_vibez.Response{
                status: 400,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "username and password are required"}'
            }
        }
        
        fr fr Find user
        sus user = cap
        bestie u in users {
            lowkey u.username == login_data.username {
                user = u
                break
            }
        }
        
        lowkey user == cap || !cryptz.verify_password(login_data.password, user.password_hash) {
            damn web_vibez.Response{
                status: 401,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Invalid username or password"}'
            }
        }
        
        fr fr Create session
        sus token = cryptz.generate_jwt_token(user.id, user.username, user.role)
        sus expires_at = time_utils.unix_timestamp() + 86400 fr fr 24 hours
        
        sus session = Session{
            token: token,
            user_id: user.id,
            username: user.username,
            created_at: time_utils.unix_timestamp(),
            expires_at: expires_at
        }
        
        sessions.append(session)
        
        sus response = {
            "message": "Login successful",
            "token": token,
            "expires_at": expires_at,
            "user": {
                "id": user.id,
                "username": user.username,
                "email": user.email,
                "role": user.role
            }
        }
        
        damn web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: json_tea.encode(response)
        }
    })
    
    fr fr Add authentication middleware for protected routes
    server.add_middleware(auth_middleware())
    
    fr fr Protected endpoints
    
    fr fr Get user profile
    server.add_route("/api/profile", slay(request) {
        lowkey request.method != "GET" {
            damn web_vibez.Response{
                status: 405,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Method not allowed"}'
            }
        }
        
        fr fr Find user
        sus user = cap
        bestie u in users {
            lowkey u.id == request.user_id {
                user = u
                break
            }
        }
        
        sus response = {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "role": user.role,
            "created_at": user.created_at
        }
        
        damn web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: json_tea.encode(response)
        }
    })
    
    fr fr Logout user
    server.add_route("/api/logout", slay(request) {
        lowkey request.method != "POST" {
            damn web_vibez.Response{
                status: 405,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Method not allowed"}'
            }
        }
        
        fr fr Remove session
        sus auth_header = request.headers.get("Authorization")
        sus token = auth_header.substring(7)
        
        bestie i, session in sessions.enumerate() {
            lowkey session.token == token {
                sessions.remove(i)
                break
            }
        }
        
        damn web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: '{"message": "Logged out successfully"}'
        }
    })
    
    fr fr Admin-only endpoints
    server.add_middleware(rbac_middleware("admin"))
    
    fr fr List all users (admin only)
    server.add_route("/api/admin/users", slay(request) {
        lowkey request.method != "GET" {
            damn web_vibez.Response{
                status: 405,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Method not allowed"}'
            }
        }
        
        sus user_list = []
        bestie user in users {
            user_list.append({
                "id": user.id,
                "username": user.username,
                "email": user.email,
                "role": user.role,
                "created_at": user.created_at
            })
        }
        
        damn web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: json_tea.encode(user_list)
        }
    })
    
    fr fr Create default admin user
    sus admin_password_hash = cryptz.hash_password("admin123")
    sus admin_user = User{
        id: next_user_id,
        username: "admin",
        email: "admin@example.com",
        password_hash: admin_password_hash,
        role: "admin",
        created_at: time_utils.unix_timestamp()
    }
    users.append(admin_user)
    next_user_id++
    
    vibez.spill("Authentication API server starting on http://0.0.0.0:4000")
    vibez.spill("Default admin user created:")
    vibez.spill("  Username: admin")
    vibez.spill("  Password: admin123")
    vibez.spill("")
    vibez.spill("API endpoints:")
    vibez.spill("  POST /api/register    - Register user")
    vibez.spill("  POST /api/login       - Login user")
    vibez.spill("  GET  /api/profile     - Get profile (auth required)")
    vibez.spill("  POST /api/logout      - Logout (auth required)")
    vibez.spill("  GET  /api/admin/users - List users (admin only)")
    
    sus err = server.listen_and_serve()
    lowkey err != cap {
        vibez.spill("Server error: " + err.to_string())
    }
}
