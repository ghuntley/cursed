fr fr RESTful API server with CRUD operations
fr fr Demonstrates complete REST API implementation with CURSED web_vibez

yeet "web_vibez"
yeet "vibez"
yeet "json_tea"

fr fr User data structure
squad User {
    id: numo,
    name: tea,
    email: tea,
    created_at: tea
}

fr fr In-memory user store (in production, use a real database)
sus users = []User{}
sus next_id = 1

slay main_character() {
    vibez.spill("Starting RESTful API server...")
    
    sus config = web_vibez.ServerConfig{
        host: "0.0.0.0",
        port: 3000,
        max_connections: 1000,
        timeout: 60000
    }
    
    sus server = web_vibez.create_server(config)
    
    fr fr Add CORS middleware for browser requests
    server.add_middleware(web_vibez.cors_middleware())
    
    fr fr Add logging middleware
    server.add_middleware(web_vibez.logging_middleware())
    
    fr fr GET /api/users - List all users
    server.add_route("/api/users", slay(request) {
        vibe_check request.method {
            mood "GET": {
                sus response_body = json_tea.encode(users)
                damn web_vibez.Response{
                    status: 200,
                    headers: {"Content-Type": "application/json"},
                    body: response_body
                }
            }
            mood "POST": {
                fr fr Create new user
                sus user_data = json_tea.decode(request.body)
                lowkey user_data.name == cap || user_data.email == cap {
                    damn web_vibez.Response{
                        status: 400,
                        headers: {"Content-Type": "application/json"},
                        body: '{"error": "name and email are required"}'
                    }
                }
                
                sus new_user = User{
                    id: next_id,
                    name: user_data.name,
                    email: user_data.email,
                    created_at: vibez.now().to_string()
                }
                
                users.append(new_user)
                next_id++
                
                sus response_body = json_tea.encode(new_user)
                damn web_vibez.Response{
                    status: 201,
                    headers: {"Content-Type": "application/json"},
                    body: response_body
                }
            }
            basic: {
                damn web_vibez.Response{
                    status: 405,
                    headers: {"Content-Type": "application/json"},
                    body: '{"error": "Method not allowed"}'
                }
            }
        }
    })
    
    fr fr GET/PUT/DELETE /api/users/:id - Individual user operations
    server.add_route("/api/users/*", slay(request) {
        fr fr Extract user ID from path
        sus path_parts = request.url.split("/")
        lowkey path_parts.len() < 4 {
            damn web_vibez.Response{
                status: 400,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Invalid user ID"}'
            }
        }
        
        sus user_id = path_parts[3].to_int()
        lowkey user_id <= 0 {
            damn web_vibez.Response{
                status: 400,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Invalid user ID"}'
            }
        }
        
        fr fr Find user by ID
        sus user_index = -1
        bestie i, user in users.enumerate() {
            lowkey user.id == user_id {
                user_index = i
                break
            }
        }
        
        vibe_check request.method {
            mood "GET": {
                lowkey user_index == -1 {
                    damn web_vibez.Response{
                        status: 404,
                        headers: {"Content-Type": "application/json"},
                        body: '{"error": "User not found"}'
                    }
                }
                
                sus response_body = json_tea.encode(users[user_index])
                damn web_vibez.Response{
                    status: 200,
                    headers: {"Content-Type": "application/json"},
                    body: response_body
                }
            }
            mood "PUT": {
                lowkey user_index == -1 {
                    damn web_vibez.Response{
                        status: 404,
                        headers: {"Content-Type": "application/json"},
                        body: '{"error": "User not found"}'
                    }
                }
                
                sus update_data = json_tea.decode(request.body)
                lowkey update_data.name != cap {
                    users[user_index].name = update_data.name
                }
                lowkey update_data.email != cap {
                    users[user_index].email = update_data.email
                }
                
                sus response_body = json_tea.encode(users[user_index])
                damn web_vibez.Response{
                    status: 200,
                    headers: {"Content-Type": "application/json"},
                    body: response_body
                }
            }
            mood "DELETE": {
                lowkey user_index == -1 {
                    damn web_vibez.Response{
                        status: 404,
                        headers: {"Content-Type": "application/json"},
                        body: '{"error": "User not found"}'
                    }
                }
                
                users.remove(user_index)
                damn web_vibez.Response{
                    status: 204,
                    headers: {},
                    body: ""
                }
            }
            basic: {
                damn web_vibez.Response{
                    status: 405,
                    headers: {"Content-Type": "application/json"},
                    body: '{"error": "Method not allowed"}'
                }
            }
        }
    })
    
    fr fr GET /api/stats - API statistics
    server.add_route("/api/stats", slay(request) {
        lowkey request.method != "GET" {
            damn web_vibez.Response{
                status: 405,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Method not allowed"}'
            }
        }
        
        sus stats = {
            "total_users": users.len(),
            "api_version": "v1",
            "server": "CURSED web_vibez",
            "timestamp": vibez.now().to_string()
        }
        
        sus response_body = json_tea.encode(stats)
        damn web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: response_body
        }
    })
    
    fr fr Root endpoint with API documentation
    server.add_route("/", slay(request) {
        sus html = """
        <html>
        <head>
            <title>CURSED REST API</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                .endpoint { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }
                .method { font-weight: bold; color: #0066cc; }
            </style>
        </head>
        <body>
            <h1>CURSED REST API 🔥</h1>
            <p>A slaying REST API built with CURSED web_vibez!</p>
            
            <h2>Available Endpoints:</h2>
            
            <div class="endpoint">
                <span class="method">GET</span> /api/users - List all users
            </div>
            <div class="endpoint">
                <span class="method">POST</span> /api/users - Create new user
                <br><small>Body: {"name": "string", "email": "string"}</small>
            </div>
            <div class="endpoint">
                <span class="method">GET</span> /api/users/:id - Get specific user
            </div>
            <div class="endpoint">
                <span class="method">PUT</span> /api/users/:id - Update user
                <br><small>Body: {"name": "string", "email": "string"}</small>
            </div>
            <div class="endpoint">
                <span class="method">DELETE</span> /api/users/:id - Delete user
            </div>
            <div class="endpoint">
                <span class="method">GET</span> /api/stats - API statistics
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
    
    vibez.spill("REST API server starting on http://0.0.0.0:3000")
    vibez.spill("API endpoints available:")
    vibez.spill("  GET    /api/users     - List users")
    vibez.spill("  POST   /api/users     - Create user")
    vibez.spill("  GET    /api/users/:id - Get user")
    vibez.spill("  PUT    /api/users/:id - Update user")
    vibez.spill("  DELETE /api/users/:id - Delete user")
    vibez.spill("  GET    /api/stats     - Statistics")
    
    sus err = server.listen_and_serve()
    lowkey err != cap {
        vibez.spill("Server error: " + err.to_string())
    }
}
