// CURSED Package Templates
// Pre-configured templates for different types of CURSED projects

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub const TemplateType = enum {
    library,
    binary,
    webapp,
    api_server,
    cli_tool,
    testing_framework,
    
    pub fn fromString(str: []const u8) ?TemplateType {
        const template_map = std.StaticStringMap(TemplateType).initComptime(.{
            .{ "lib", .library },
            .{ "library", .library },
            .{ "bin", .binary },
            .{ "binary", .binary },
            .{ "webapp", .webapp },
            .{ "web", .webapp },
            .{ "api", .api_server },
            .{ "server", .api_server },
            .{ "cli", .cli_tool },
            .{ "tool", .cli_tool },
            .{ "test", .testing_framework },
            .{ "framework", .testing_framework },
        });
        return template_map.get(str);
    }
};

pub const Template = struct {
    name: []const u8,
    description: []const u8,
    template_type: TemplateType,
    files: []const FileTemplate,
    dependencies: []const []const u8,
    dev_dependencies: []const []const u8,
    
    pub const FileTemplate = struct {
        path: []const u8,
        content: []const u8,
    };
};

pub const templates = struct {
    pub const library = Template{
        .name = "library",
        .description = "A reusable CURSED library",
        .template_type = .library,
        .files = &[_]Template.FileTemplate{
            .{ .path = "src/lib.csd", .content = library_main },
            .{ .path = "tests/lib_test.csd", .content = library_test },
            .{ .path = "examples/usage.csd", .content = library_example },
            .{ .path = "docs/api.md", .content = library_docs },
        },
        .dependencies = &[_][]const u8{},
        .dev_dependencies = &[_][]const u8{"testz"},
    };
    
    pub const binary = Template{
        .name = "binary",
        .description = "A standalone CURSED application",
        .template_type = .binary,
        .files = &[_]Template.FileTemplate{
            .{ .path = "src/main.csd", .content = binary_main },
            .{ .path = "src/cli.csd", .content = binary_cli },
            .{ .path = "tests/main_test.csd", .content = binary_test },
            .{ .path = "README.md", .content = binary_readme },
        },
        .dependencies = &[_][]const u8{"stringz"},
        .dev_dependencies = &[_][]const u8{"testz"},
    };
    
    pub const webapp = Template{
        .name = "webapp",
        .description = "A web application with HTTP server",
        .template_type = .webapp,
        .files = &[_]Template.FileTemplate{
            .{ .path = "src/main.csd", .content = webapp_main },
            .{ .path = "src/routes.csd", .content = webapp_routes },
            .{ .path = "src/middleware.csd", .content = webapp_middleware },
            .{ .path = "static/index.html", .content = webapp_html },
            .{ .path = "static/style.css", .content = webapp_css },
            .{ .path = "tests/server_test.csd", .content = webapp_test },
        },
        .dependencies = &[_][]const u8{ "http", "json", "stringz" },
        .dev_dependencies = &[_][]const u8{"testz"},
    };
    
    pub const api_server = Template{
        .name = "api_server",
        .description = "A REST API server",
        .template_type = .api_server,
        .files = &[_]Template.FileTemplate{
            .{ .path = "src/main.csd", .content = api_main },
            .{ .path = "src/handlers.csd", .content = api_handlers },
            .{ .path = "src/models.csd", .content = api_models },
            .{ .path = "src/auth.csd", .content = api_auth },
            .{ .path = "tests/api_test.csd", .content = api_test },
            .{ .path = "docs/openapi.yaml", .content = api_docs },
        },
        .dependencies = &[_][]const u8{ "http", "json", "crypto", "database" },
        .dev_dependencies = &[_][]const u8{"testz"},
    };
    
    pub const cli_tool = Template{
        .name = "cli_tool",
        .description = "A command-line interface tool",
        .template_type = .cli_tool,
        .files = &[_]Template.FileTemplate{
            .{ .path = "src/main.csd", .content = cli_main },
            .{ .path = "src/commands.csd", .content = cli_commands },
            .{ .path = "src/config.csd", .content = cli_config },
            .{ .path = "tests/cli_test.csd", .content = cli_test },
            .{ .path = "README.md", .content = cli_readme },
        },
        .dependencies = &[_][]const u8{ "stringz", "config" },
        .dev_dependencies = &[_][]const u8{"testz"},
    };
};

// ===== TEMPLATE CONTENT =====

const library_main = 
\\yeet "testz"
\\
\\// Library public API
\\slay add(a normie, b normie) normie {
\\    damn a + b
\\}
\\
\\slay multiply(a normie, b normie) normie {
\\    damn a * b
\\}
\\
\\slay format_message(name tea) tea {
\\    damn "Hello, " + name + "!"
\\}
\\
\\// Library configuration
\\squad LibConfig {
\\    spill debug lit = cap
\\    spill version tea = "1.0.0"
\\}
\\
\\sus lib_config LibConfig = LibConfig{}
\\
\\// Internal utilities
\\slay internal_helper(data tea) lit {
\\    damn data.len > 0
\\}
\\
\\test "library basic functionality" {
\\    test_start("library tests")
\\    
\\    assert_eq_int(add(2, 3), 5)
\\    assert_eq_int(multiply(4, 5), 20)
\\    assert_eq_string(format_message("World"), "Hello, World!")
\\    assert_true(internal_helper("test"))
\\    assert_false(internal_helper(""))
\\    
\\    print_test_summary()
\\}
;

const library_test = 
\\yeet "testz"
\\yeet "lib"
\\
\\test "comprehensive library tests" {
\\    test_start("comprehensive library test suite")
\\    
\\    // Test mathematical operations
\\    assert_eq_int(add(0, 0), 0)
\\    assert_eq_int(add(-1, 1), 0)
\\    assert_eq_int(add(100, 200), 300)
\\    
\\    assert_eq_int(multiply(0, 5), 0)
\\    assert_eq_int(multiply(1, 1), 1)
\\    assert_eq_int(multiply(-2, 3), -6)
\\    
\\    // Test string formatting
\\    assert_eq_string(format_message(""), "Hello, !")
\\    assert_eq_string(format_message("CURSED"), "Hello, CURSED!")
\\    
\\    // Test configuration
\\    assert_eq_string(lib_config.version, "1.0.0")
\\    assert_false(lib_config.debug)
\\    
\\    print_test_summary()
\\}
;

const library_example = 
\\yeet "lib"
\\
\\slay main() {
\\    vibez.spill("Library Usage Example")
\\    vibez.spill("===================")
\\    
\\    sus result normie = add(10, 15)
\\    vibez.spillf("10 + 15 = {}", result)
\\    
\\    sus product normie = multiply(6, 7)
\\    vibez.spillf("6 * 7 = {}", product)
\\    
\\    sus greeting tea = format_message("Library User")
\\    vibez.spill(greeting)
\\    
\\    vibez.spillf("Library version: {}", lib_config.version)
\\}
;

const library_docs = 
\\# Library API Documentation
\\
\\## Functions
\\
\\### `add(a: normie, b: normie) -> normie`
\\Adds two integers and returns the result.
\\
\\### `multiply(a: normie, b: normie) -> normie`
\\Multiplies two integers and returns the result.
\\
\\### `format_message(name: tea) -> tea`
\\Creates a greeting message with the given name.
\\
\\## Configuration
\\
\\### `lib_config: LibConfig`
\\Global library configuration with version and debug settings.
\\
\\## Example Usage
\\
\\```cursed
\\yeet "your-library"
\\
\\slay main() {
\\    sus result normie = add(5, 10)
\\    vibez.spillf("Result: {}", result)
\\}
\\```
;

const binary_main = 
\\yeet "cli"
\\yeet "stringz"
\\
\\slay main() {
\\    vibez.spill("CURSED Application")
\\    vibez.spill("==================")
\\    
\\    // Parse command line arguments
\\    sus args []tea = get_args()
\\    
\\    bestie (args.len == 0) {
\\        show_help()
\\        damn
\\    }
\\    
\\    sus command tea = args[0]
\\    
\\    ready command drip {
\\        "help" => show_help()
\\        "version" => show_version()
\\        "run" => {
\\            bestie (args.len > 1) {
\\                run_command(args[1])
\\            } else {
\\                vibez.spill("Error: filename required for run command")
\\            }
\\        }
\\        _ => {
\\            vibez.spillf("Unknown command: {}", command)
\\            show_help()
\\        }
\\    }
\\}
\\
\\slay show_help() {
\\    vibez.spill("Usage: app <command> [args]")
\\    vibez.spill("")
\\    vibez.spill("Commands:")
\\    vibez.spill("  help     Show this help message")
\\    vibez.spill("  version  Show version information")
\\    vibez.spill("  run <file>  Run the specified file")
\\}
\\
\\slay show_version() {
\\    vibez.spill("CURSED Application v1.0.0")
\\}
\\
\\slay run_command(filename tea) {
\\    vibez.spillf("Running: {}", filename)
\\    // Application logic here
\\}
\\
\\slay get_args() []tea {
\\    // Mock command line arguments
\\    damn ["help"]
\\}
;

const binary_cli = 
\\yeet "stringz"
\\
\\// Command line interface utilities
\\
\\squad CliConfig {
\\    spill verbose lit = cap
\\    spill output_format tea = "text"
\\    spill max_workers normie = 4
\\}
\\
\\slay parse_flags(args []tea) CliConfig {
\\    sus config CliConfig = CliConfig{}
\\    
\\    sos i drip = 0; i < args.len; i = i + 1 {
\\        sus arg tea = args[i]
\\        
\\        ready arg drip {
\\            "--verbose" => config.verbose = based
\\            "--format=json" => config.output_format = "json"
\\            "--format=yaml" => config.output_format = "yaml"
\\            _ => {
\\                bestie (stringz.starts_with(arg, "--workers=")) {
\\                    // Parse worker count
\\                    config.max_workers = 8
\\                }
\\            }
\\        }
\\    }
\\    
\\    damn config
\\}
\\
\\slay print_config(config CliConfig) {
\\    bestie (config.verbose) {
\\        vibez.spill("Configuration:")
\\        vibez.spillf("  Verbose: {}", config.verbose)
\\        vibez.spillf("  Format: {}", config.output_format)
\\        vibez.spillf("  Workers: {}", config.max_workers)
\\    }
\\}
;

const binary_test = 
\\yeet "testz"
\\yeet "main"
\\yeet "cli"
\\
\\test "application functionality" {
\\    test_start("application tests")
\\    
\\    // Test CLI configuration parsing
\\    sus args []tea = ["--verbose", "--format=json", "--workers=8"]
\\    sus config CliConfig = parse_flags(args)
\\    
\\    assert_true(config.verbose)
\\    assert_eq_string(config.output_format, "json")
\\    assert_eq_int(config.max_workers, 8)
\\    
\\    print_test_summary()
\\}
;

const binary_readme = 
\\# CURSED Application
\\
\\A standalone CURSED application with command-line interface.
\\
\\## Installation
\\
\\```bash
\\cursed-pkg install
\\cursed-pkg build
\\```
\\
\\## Usage
\\
\\```bash
\\./app help
\\./app version
\\./app run myfile.csd
\\```
\\
\\## Options
\\
\\- `--verbose` - Enable verbose output
\\- `--format=json|yaml|text` - Set output format
\\- `--workers=N` - Set number of worker threads
\\
\\## Development
\\
\\```bash
\\cursed-pkg test
\\cursed-pkg build --release
\\```
;

const webapp_main = 
\\yeet "http"
\\yeet "json"
\\yeet "routes"
\\yeet "middleware"
\\
\\slay main() {
\\    vibez.spill("Starting CURSED Web Application...")
\\    
\\    // Create HTTP server
\\    sus server HttpServer = http.create_server()
\\    
\\    // Add middleware
\\    server.use(middleware.logging())
\\    server.use(middleware.cors())
\\    server.use(middleware.json_parser())
\\    
\\    // Setup routes
\\    setup_routes(server)
\\    
\\    // Serve static files
\\    server.static("/", "static/")
\\    
\\    // Start server
\\    sus port normie = 8080
\\    vibez.spillf("Server starting on port {}", port)
\\    server.listen(port)
\\}
\\
\\slay setup_routes(server HttpServer) {
\\    server.get("/", routes.home)
\\    server.get("/api/status", routes.status)
\\    server.get("/api/users", routes.get_users)
\\    server.post("/api/users", routes.create_user)
\\    server.put("/api/users/:id", routes.update_user)
\\    server.delete("/api/users/:id", routes.delete_user)
\\}
;

const webapp_routes = 
\\yeet "http"
\\yeet "json"
\\
\\// Home page
\\slay home(req HttpRequest, res HttpResponse) {
\\    res.send_file("static/index.html")
\\}
\\
\\// API status endpoint
\\slay status(req HttpRequest, res HttpResponse) {
\\    sus status_data = {
\\        "status": "ok",
\\        "timestamp": get_timestamp(),
\\        "version": "1.0.0"
\\    }
\\    
\\    res.json(status_data)
\\}
\\
\\// User management endpoints
\\slay get_users(req HttpRequest, res HttpResponse) {
\\    // Mock user data
\\    sus users = [
\\        {"id": 1, "name": "Alice", "email": "alice@example.com"},
\\        {"id": 2, "name": "Bob", "email": "bob@example.com"}
\\    ]
\\    
\\    res.json(users)
\\}
\\
\\slay create_user(req HttpRequest, res HttpResponse) {
\\    sus user_data = req.body
\\    
\\    // Validate required fields
\\    bestie (!user_data.has("name") || !user_data.has("email")) {
\\        res.status(400).json({"error": "Name and email required"})
\\        damn
\\    }
\\    
\\    // Create new user
\\    sus new_user = {
\\        "id": get_next_user_id(),
\\        "name": user_data.get("name"),
\\        "email": user_data.get("email"),
\\        "created_at": get_timestamp()
\\    }
\\    
\\    res.status(201).json(new_user)
\\}
\\
\\slay update_user(req HttpRequest, res HttpResponse) {
\\    sus user_id normie = req.params.get("id")
\\    sus update_data = req.body
\\    
\\    // Find and update user
\\    sus updated_user = {
\\        "id": user_id,
\\        "name": update_data.get("name"),
\\        "email": update_data.get("email"),
\\        "updated_at": get_timestamp()
\\    }
\\    
\\    res.json(updated_user)
\\}
\\
\\slay delete_user(req HttpRequest, res HttpResponse) {
\\    sus user_id normie = req.params.get("id")
\\    
\\    // Delete user logic here
\\    res.status(204).send("")
\\}
\\
\\// Utility functions
\\slay get_timestamp() tea {
\\    damn "2025-01-15T10:30:00Z"
\\}
\\
\\slay get_next_user_id() normie {
\\    damn 3
\\}
;

const webapp_middleware = 
\\yeet "http"
\\
\\// Logging middleware
\\slay logging() Middleware {
\\    damn slay(req HttpRequest, res HttpResponse, next NextFunction) {
\\        sus timestamp tea = get_timestamp()
\\        vibez.spillf("[{}] {} {}", timestamp, req.method, req.url)
\\        next()
\\    }
\\}
\\
\\// CORS middleware
\\slay cors() Middleware {
\\    damn slay(req HttpRequest, res HttpResponse, next NextFunction) {
\\        res.header("Access-Control-Allow-Origin", "*")
\\        res.header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
\\        res.header("Access-Control-Allow-Headers", "Content-Type, Authorization")
\\        
\\        bestie (req.method == "OPTIONS") {
\\            res.status(200).send("")
\\            damn
\\        }
\\        
\\        next()
\\    }
\\}
\\
\\// JSON body parser middleware
\\slay json_parser() Middleware {
\\    damn slay(req HttpRequest, res HttpResponse, next NextFunction) {
\\        bestie (req.headers.get("content-type") == "application/json") {
\\            req.body = parse_json(req.raw_body)
\\        }
\\        next()
\\    }
\\}
\\
\\// Authentication middleware
\\slay auth() Middleware {
\\    damn slay(req HttpRequest, res HttpResponse, next NextFunction) {
\\        sus auth_header tea = req.headers.get("authorization")
\\        
\\        bestie (!auth_header || !is_valid_token(auth_header)) {
\\            res.status(401).json({"error": "Unauthorized"})
\\            damn
\\        }
\\        
\\        req.user = get_user_from_token(auth_header)
\\        next()
\\    }
\\}
\\
\\slay is_valid_token(token tea) lit {
\\    // Mock token validation
\\    damn token.len > 0
\\}
\\
\\slay get_user_from_token(token tea) User {
\\    // Mock user extraction
\\    damn User{id: 1, name: "Mock User"}
\\}
;

const webapp_html = 
\\<!DOCTYPE html>
\\<html lang="en">
\\<head>
\\    <meta charset="UTF-8">
\\    <meta name="viewport" content="width=device-width, initial-scale=1.0">
\\    <title>CURSED Web App</title>
\\    <link rel="stylesheet" href="style.css">
\\</head>
\\<body>
\\    <header>
\\        <h1>CURSED Web Application</h1>
\\        <nav>
\\            <a href="/">Home</a>
\\            <a href="/api/status">Status</a>
\\            <a href="/api/users">Users</a>
\\        </nav>
\\    </header>
\\    
\\    <main>
\\        <section>
\\            <h2>Welcome to CURSED</h2>
\\            <p>This is a web application built with the CURSED programming language.</p>
\\            
\\            <div class="api-demo">
\\                <h3>API Demo</h3>
\\                <button onclick="checkStatus()">Check Status</button>
\\                <button onclick="loadUsers()">Load Users</button>
\\                <div id="output"></div>
\\            </div>
\\        </section>
\\    </main>
\\    
\\    <script>
\\        async function checkStatus() {
\\            try {
\\                const response = await fetch('/api/status');
\\                const data = await response.json();
\\                document.getElementById('output').innerHTML = 
\\                    '<pre>' + JSON.stringify(data, null, 2) + '</pre>';
\\            } catch (error) {
\\                console.error('Error:', error);
\\            }
\\        }
\\        
\\        async function loadUsers() {
\\            try {
\\                const response = await fetch('/api/users');
\\                const data = await response.json();
\\                document.getElementById('output').innerHTML = 
\\                    '<pre>' + JSON.stringify(data, null, 2) + '</pre>';
\\            } catch (error) {
\\                console.error('Error:', error);
\\            }
\\        }
\\    </script>
\\</body>
\\</html>
;

const webapp_css = 
\\/* CURSED Web App Styles */
\\
\\* {
\\    margin: 0;
\\    padding: 0;
\\    box-sizing: border-box;
\\}
\\
\\body {
\\    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
\\    line-height: 1.6;
\\    color: #333;
\\    background-color: #f4f4f4;
\\}
\\
\\header {
\\    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
\\    color: white;
\\    padding: 1rem 0;
\\    box-shadow: 0 2px 5px rgba(0,0,0,0.1);
\\}
\\
\\header h1 {
\\    text-align: center;
\\    margin-bottom: 1rem;
\\}
\\
\\nav {
\\    text-align: center;
\\}
\\
\\nav a {
\\    color: white;
\\    text-decoration: none;
\\    margin: 0 1rem;
\\    padding: 0.5rem 1rem;
\\    border-radius: 5px;
\\    transition: background-color 0.3s;
\\}
\\
\\nav a:hover {
\\    background-color: rgba(255,255,255,0.2);
\\}
\\
\\main {
\\    max-width: 800px;
\\    margin: 2rem auto;
\\    padding: 0 1rem;
\\}
\\
\\section {
\\    background: white;
\\    padding: 2rem;
\\    border-radius: 10px;
\\    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
\\}
\\
\\h2 {
\\    color: #667eea;
\\    margin-bottom: 1rem;
\\}
\\
\\.api-demo {
\\    margin-top: 2rem;
\\    padding: 1rem;
\\    background-color: #f8f9fa;
\\    border-radius: 5px;
\\}
\\
\\button {
\\    background: #667eea;
\\    color: white;
\\    border: none;
\\    padding: 0.5rem 1rem;
\\    margin: 0.5rem;
\\    border-radius: 5px;
\\    cursor: pointer;
\\    transition: background-color 0.3s;
\\}
\\
\\button:hover {
\\    background: #5a67d8;
\\}
\\
\\#output {
\\    margin-top: 1rem;
\\    padding: 1rem;
\\    background: #fff;
\\    border: 1px solid #ddd;
\\    border-radius: 5px;
\\    min-height: 100px;
\\}
\\
\\pre {
\\    white-space: pre-wrap;
\\    word-wrap: break-word;
\\}
;

const webapp_test = 
\\yeet "testz"
\\yeet "http"
\\yeet "main"
\\
\\test "web application tests" {
\\    test_start("web application test suite")
\\    
\\    // Test server creation
\\    sus server HttpServer = http.create_server()
\\    assert_true(server != NULL)
\\    
\\    // Test route setup
\\    setup_routes(server)
\\    
\\    // Mock request testing would go here
\\    // In a real implementation, would test HTTP endpoints
\\    
\\    print_test_summary()
\\}
;

const api_main = 
\\yeet "http"
\\yeet "json"
\\yeet "handlers"
\\yeet "auth"
\\yeet "models"
\\
\\slay main() {
\\    vibez.spill("Starting CURSED API Server...")
\\    
\\    // Initialize database
\\    models.init_database()
\\    
\\    // Create server
\\    sus server HttpServer = http.create_server()
\\    
\\    // Global middleware
\\    server.use(cors_middleware())
\\    server.use(json_middleware())
\\    server.use(logging_middleware())
\\    
\\    // Public routes
\\    server.post("/api/auth/login", handlers.login)
\\    server.post("/api/auth/register", handlers.register)
\\    
\\    // Protected routes
\\    server.use("/api", auth.require_token())
\\    server.get("/api/profile", handlers.get_profile)
\\    server.put("/api/profile", handlers.update_profile)
\\    
\\    // Resource routes
\\    server.get("/api/items", handlers.get_items)
\\    server.post("/api/items", handlers.create_item)
\\    server.get("/api/items/:id", handlers.get_item)
\\    server.put("/api/items/:id", handlers.update_item)
\\    server.delete("/api/items/:id", handlers.delete_item)
\\    
\\    // Health check
\\    server.get("/health", handlers.health_check)
\\    
\\    // Start server
\\    sus port normie = 3000
\\    vibez.spillf("API server listening on port {}", port)
\\    server.listen(port)
\\}
;

const api_handlers = 
\\yeet "json"
\\yeet "auth"
\\yeet "models"
\\
\\// Authentication handlers
\\slay login(req HttpRequest, res HttpResponse) {
\\    sus credentials = req.body
\\    
\\    bestie (!validate_login(credentials)) {
\\        res.status(400).json({"error": "Invalid credentials format"})
\\        damn
\\    }
\\    
\\    sus user = models.find_user_by_email(credentials.email)
\\    bestie (!user || !auth.verify_password(credentials.password, user.password_hash)) {
\\        res.status(401).json({"error": "Invalid email or password"})
\\        damn
\\    }
\\    
\\    sus token tea = auth.generate_token(user.id)
\\    res.json({
\\        "token": token,
\\        "user": {
\\            "id": user.id,
\\            "email": user.email,
\\            "name": user.name
\\        }
\\    })
\\}
\\
\\slay register(req HttpRequest, res HttpResponse) {
\\    sus user_data = req.body
\\    
\\    bestie (!validate_registration(user_data)) {
\\        res.status(400).json({"error": "Invalid registration data"})
\\        damn
\\    }
\\    
\\    // Check if user already exists
\\    bestie (models.user_exists(user_data.email)) {
\\        res.status(409).json({"error": "User already exists"})
\\        damn
\\    }
\\    
\\    // Create new user
\\    sus password_hash tea = auth.hash_password(user_data.password)
\\    sus new_user = models.create_user({
\\        "email": user_data.email,
\\        "name": user_data.name,
\\        "password_hash": password_hash
\\    })
\\    
\\    sus token tea = auth.generate_token(new_user.id)
\\    res.status(201).json({
\\        "token": token,
\\        "user": {
\\            "id": new_user.id,
\\            "email": new_user.email,
\\            "name": new_user.name
\\        }
\\    })
\\}
\\
\\// Profile handlers
\\slay get_profile(req HttpRequest, res HttpResponse) {
\\    sus user = models.find_user_by_id(req.user.id)
\\    res.json({
\\        "id": user.id,
\\        "email": user.email,
\\        "name": user.name,
\\        "created_at": user.created_at
\\    })
\\}
\\
\\slay update_profile(req HttpRequest, res HttpResponse) {
\\    sus update_data = req.body
\\    sus updated_user = models.update_user(req.user.id, update_data)
\\    
\\    res.json({
\\        "id": updated_user.id,
\\        "email": updated_user.email,
\\        "name": updated_user.name
\\    })
\\}
\\
\\// Item handlers
\\slay get_items(req HttpRequest, res HttpResponse) {
\\    sus page normie = req.query.get("page") || 1
\\    sus limit normie = req.query.get("limit") || 10
\\    
\\    sus items = models.get_items(page, limit)
\\    res.json(items)
\\}
\\
\\slay create_item(req HttpRequest, res HttpResponse) {
\\    sus item_data = req.body
\\    item_data.user_id = req.user.id
\\    
\\    sus new_item = models.create_item(item_data)
\\    res.status(201).json(new_item)
\\}
\\
\\slay get_item(req HttpRequest, res HttpResponse) {
\\    sus item_id normie = req.params.get("id")
\\    sus item = models.find_item_by_id(item_id)
\\    
\\    bestie (!item) {
\\        res.status(404).json({"error": "Item not found"})
\\        damn
\\    }
\\    
\\    res.json(item)
\\}
\\
\\slay update_item(req HttpRequest, res HttpResponse) {
\\    sus item_id normie = req.params.get("id")
\\    sus update_data = req.body
\\    
\\    sus updated_item = models.update_item(item_id, update_data)
\\    res.json(updated_item)
\\}
\\
\\slay delete_item(req HttpRequest, res HttpResponse) {
\\    sus item_id normie = req.params.get("id")
\\    models.delete_item(item_id)
\\    
\\    res.status(204).send("")
\\}
\\
\\// Health check
\\slay health_check(req HttpRequest, res HttpResponse) {
\\    res.json({
\\        "status": "healthy",
\\        "timestamp": get_timestamp(),
\\        "version": "1.0.0"
\\    })
\\}
\\
\\// Validation helpers
\\slay validate_login(data) lit {
\\    damn data.has("email") && data.has("password")
\\}
\\
\\slay validate_registration(data) lit {
\\    damn data.has("email") && data.has("name") && data.has("password") && data.password.len >= 6
\\}
;

const api_models = 
\\yeet "database"
\\yeet "json"
\\
\\// User model
\\squad User {
\\    spill id normie
\\    spill email tea
\\    spill name tea
\\    spill password_hash tea
\\    spill created_at tea
\\    spill updated_at tea
\\}
\\
\\// Item model
\\squad Item {
\\    spill id normie
\\    spill title tea
\\    spill description tea
\\    spill user_id normie
\\    spill created_at tea
\\    spill updated_at tea
\\}
\\
\\// Database initialization
\\slay init_database() {
\\    database.execute("
\\        CREATE TABLE IF NOT EXISTS users (
\\            id INTEGER PRIMARY KEY AUTOINCREMENT,
\\            email TEXT UNIQUE NOT NULL,
\\            name TEXT NOT NULL,
\\            password_hash TEXT NOT NULL,
\\            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
\\            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
\\        )
\\    ")
\\    
\\    database.execute("
\\        CREATE TABLE IF NOT EXISTS items (
\\            id INTEGER PRIMARY KEY AUTOINCREMENT,
\\            title TEXT NOT NULL,
\\            description TEXT,
\\            user_id INTEGER NOT NULL,
\\            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
\\            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
\\            FOREIGN KEY (user_id) REFERENCES users (id)
\\        )
\\    ")
\\}
\\
\\// User operations
\\slay find_user_by_email(email tea) ?User {
\\    sus result = database.query_one("SELECT * FROM users WHERE email = ?", [email])
\\    damn result ? User{
\\        id: result.id,
\\        email: result.email,
\\        name: result.name,
\\        password_hash: result.password_hash,
\\        created_at: result.created_at,
\\        updated_at: result.updated_at
\\    } : NULL
\\}
\\
\\slay find_user_by_id(id normie) ?User {
\\    sus result = database.query_one("SELECT * FROM users WHERE id = ?", [id])
\\    damn result ? User{
\\        id: result.id,
\\        email: result.email,
\\        name: result.name,
\\        password_hash: result.password_hash,
\\        created_at: result.created_at,
\\        updated_at: result.updated_at
\\    } : NULL
\\}
\\
\\slay user_exists(email tea) lit {
\\    sus count = database.count("SELECT COUNT(*) FROM users WHERE email = ?", [email])
\\    damn count > 0
\\}
\\
\\slay create_user(data) User {
\\    sus result = database.insert("
\\        INSERT INTO users (email, name, password_hash) 
\\        VALUES (?, ?, ?)
\\    ", [data.email, data.name, data.password_hash])
\\    
\\    damn find_user_by_id(result.last_insert_id)
\\}
\\
\\slay update_user(id normie, data) User {
\\    database.execute("
\\        UPDATE users 
\\        SET name = ?, updated_at = CURRENT_TIMESTAMP 
\\        WHERE id = ?
\\    ", [data.name, id])
\\    
\\    damn find_user_by_id(id)
\\}
\\
\\// Item operations
\\slay get_items(page normie, limit normie) []Item {
\\    sus offset normie = (page - 1) * limit
\\    sus results = database.query("
\\        SELECT * FROM items 
\\        ORDER BY created_at DESC 
\\        LIMIT ? OFFSET ?
\\    ", [limit, offset])
\\    
\\    sus items []Item = []
\\    sos result in results {
\\        items.append(Item{
\\            id: result.id,
\\            title: result.title,
\\            description: result.description,
\\            user_id: result.user_id,
\\            created_at: result.created_at,
\\            updated_at: result.updated_at
\\        })
\\    }
\\    
\\    damn items
\\}
\\
\\slay find_item_by_id(id normie) ?Item {
\\    sus result = database.query_one("SELECT * FROM items WHERE id = ?", [id])
\\    damn result ? Item{
\\        id: result.id,
\\        title: result.title,
\\        description: result.description,
\\        user_id: result.user_id,
\\        created_at: result.created_at,
\\        updated_at: result.updated_at
\\    } : NULL
\\}
\\
\\slay create_item(data) Item {
\\    sus result = database.insert("
\\        INSERT INTO items (title, description, user_id) 
\\        VALUES (?, ?, ?)
\\    ", [data.title, data.description, data.user_id])
\\    
\\    damn find_item_by_id(result.last_insert_id)
\\}
\\
\\slay update_item(id normie, data) Item {
\\    database.execute("
\\        UPDATE items 
\\        SET title = ?, description = ?, updated_at = CURRENT_TIMESTAMP 
\\        WHERE id = ?
\\    ", [data.title, data.description, id])
\\    
\\    damn find_item_by_id(id)
\\}
\\
\\slay delete_item(id normie) {
\\    database.execute("DELETE FROM items WHERE id = ?", [id])
\\}
;

const api_auth = 
\\yeet "crypto"
\\yeet "json"
\\
\\// Authentication utilities
\\
\\slay hash_password(password tea) tea {
\\    damn crypto.hash_bcrypt(password, 12)
\\}
\\
\\slay verify_password(password tea, hash tea) lit {
\\    damn crypto.verify_bcrypt(password, hash)
\\}
\\
\\slay generate_token(user_id normie) tea {
\\    sus payload = {
\\        "user_id": user_id,
\\        "issued_at": get_timestamp(),
\\        "expires_at": get_future_timestamp(24 * 60 * 60) // 24 hours
\\    }
\\    
\\    damn crypto.sign_jwt(payload, get_jwt_secret())
\\}
\\
\\slay verify_token(token tea) ?TokenPayload {
\\    ready {
\\        sus payload = crypto.verify_jwt(token, get_jwt_secret())
\\        
\\        // Check expiration
\\        bestie (payload.expires_at < get_timestamp()) {
\\            damn NULL
\\        }
\\        
\\        damn TokenPayload{
\\            user_id: payload.user_id,
\\            issued_at: payload.issued_at,
\\            expires_at: payload.expires_at
\\        }
\\    } bummer {
\\        damn NULL
\\    }
\\}
\\
\\slay require_token() Middleware {
\\    damn slay(req HttpRequest, res HttpResponse, next NextFunction) {
\\        sus auth_header tea = req.headers.get("authorization")
\\        
\\        bestie (!auth_header || !auth_header.starts_with("Bearer ")) {
\\            res.status(401).json({"error": "Missing or invalid authorization header"})
\\            damn
\\        }
\\        
\\        sus token tea = auth_header[7..] // Remove "Bearer "
\\        sus payload = verify_token(token)
\\        
\\        bestie (!payload) {
\\            res.status(401).json({"error": "Invalid or expired token"})
\\            damn
\\        }
\\        
\\        req.user = {id: payload.user_id}
\\        next()
\\    }
\\}
\\
\\// Token payload structure
\\squad TokenPayload {
\\    spill user_id normie
\\    spill issued_at tea
\\    spill expires_at tea
\\}
\\
\\// Utility functions
\\slay get_jwt_secret() tea {
\\    // In production, load from environment variable
\\    damn "your-super-secret-jwt-key-change-in-production"
\\}
\\
\\slay get_timestamp() tea {
\\    damn "2025-01-15T10:30:00Z"
\\}
\\
\\slay get_future_timestamp(seconds normie) tea {
\\    damn "2025-01-16T10:30:00Z"
\\}
;

const api_test = 
\\yeet "testz"
\\yeet "http"
\\yeet "auth"
\\yeet "models"
\\
\\test "API functionality tests" {
\\    test_start("API test suite")
\\    
\\    // Test authentication
\\    sus token tea = auth.generate_token(1)
\\    assert_true(token.len > 0)
\\    
\\    sus payload = auth.verify_token(token)
\\    assert_true(payload != NULL)
\\    assert_eq_int(payload.user_id, 1)
\\    
\\    // Test password hashing
\\    sus password tea = "test123"
\\    sus hash tea = auth.hash_password(password)
\\    assert_true(auth.verify_password(password, hash))
\\    assert_false(auth.verify_password("wrong", hash))
\\    
\\    print_test_summary()
\\}
;

const api_docs = 
\\openapi: 3.0.0
\\info:
\\  title: CURSED API
\\  version: 1.0.0
\\  description: REST API built with CURSED
\\
\\paths:
\\  /api/auth/login:
\\    post:
\\      summary: User login
\\      requestBody:
\\        required: true
\\        content:
\\          application/json:
\\            schema:
\\              type: object
\\              properties:
\\                email:
\\                  type: string
\\                  format: email
\\                password:
\\                  type: string
\\              required:
\\                - email
\\                - password
\\      responses:
\\        '200':
\\          description: Login successful
\\          content:
\\            application/json:
\\              schema:
\\                type: object
\\                properties:
\\                  token:
\\                    type: string
\\                  user:
\\                    $ref: '#/components/schemas/User'
\\
\\  /api/auth/register:
\\    post:
\\      summary: User registration
\\      requestBody:
\\        required: true
\\        content:
\\          application/json:
\\            schema:
\\              type: object
\\              properties:
\\                email:
\\                  type: string
\\                  format: email
\\                name:
\\                  type: string
\\                password:
\\                  type: string
\\                  minLength: 6
\\              required:
\\                - email
\\                - name
\\                - password
\\      responses:
\\        '201':
\\          description: Registration successful
\\
\\  /api/items:
\\    get:
\\      summary: Get all items
\\      security:
\\        - bearerAuth: []
\\      parameters:
\\        - name: page
\\          in: query
\\          schema:
\\            type: integer
\\            default: 1
\\        - name: limit
\\          in: query
\\          schema:
\\            type: integer
\\            default: 10
\\      responses:
\\        '200':
\\          description: List of items
\\          content:
\\            application/json:
\\              schema:
\\                type: array
\\                items:
\\                  $ref: '#/components/schemas/Item'
\\
\\    post:
\\      summary: Create new item
\\      security:
\\        - bearerAuth: []
\\      requestBody:
\\        required: true
\\        content:
\\          application/json:
\\            schema:
\\              type: object
\\              properties:
\\                title:
\\                  type: string
\\                description:
\\                  type: string
\\              required:
\\                - title
\\      responses:
\\        '201':
\\          description: Item created
\\
\\components:
\\  schemas:
\\    User:
\\      type: object
\\      properties:
\\        id:
\\          type: integer
\\        email:
\\          type: string
\\        name:
\\          type: string
\\        created_at:
\\          type: string
\\          format: date-time
\\
\\    Item:
\\      type: object
\\      properties:
\\        id:
\\          type: integer
\\        title:
\\          type: string
\\        description:
\\          type: string
\\        user_id:
\\          type: integer
\\        created_at:
\\          type: string
\\          format: date-time
\\
\\  securitySchemes:
\\    bearerAuth:
\\      type: http
\\      scheme: bearer
\\      bearerFormat: JWT
;

const cli_main = 
\\yeet "commands"
\\yeet "config"
\\yeet "stringz"
\\
\\slay main() {
\\    sus args []tea = get_command_args()
\\    
\\    bestie (args.len == 0) {
\\        show_help()
\\        damn
\\    }
\\    
\\    sus command tea = args[0]
\\    sus cmd_args []tea = args[1..]
\\    
\\    // Load configuration
\\    sus cfg Config = config.load_config()
\\    
\\    ready command drip {
\\        "help" => show_help()
\\        "version" => show_version()
\\        "init" => commands.init_command(cmd_args, cfg)
\\        "build" => commands.build_command(cmd_args, cfg)
\\        "test" => commands.test_command(cmd_args, cfg)
\\        "run" => commands.run_command(cmd_args, cfg)
\\        "clean" => commands.clean_command(cmd_args, cfg)
\\        "config" => commands.config_command(cmd_args, cfg)
\\        _ => {
\\            vibez.spillf("Unknown command: {}", command)
\\            vibez.spill("Run 'tool help' for available commands")
\\        }
\\    }
\\}
\\
\\slay show_help() {
\\    vibez.spill("CURSED CLI Tool")
\\    vibez.spill("==============")
\\    vibez.spill("")
\\    vibez.spill("Usage: tool <command> [options] [args]")
\\    vibez.spill("")
\\    vibez.spill("Commands:")
\\    vibez.spill("  init     Initialize new project")
\\    vibez.spill("  build    Build project")
\\    vibez.spill("  test     Run tests")
\\    vibez.spill("  run      Execute project")
\\    vibez.spill("  clean    Clean build artifacts")
\\    vibez.spill("  config   Manage configuration")
\\    vibez.spill("  help     Show this help")
\\    vibez.spill("  version  Show version info")
\\    vibez.spill("")
\\    vibez.spill("Global Options:")
\\    vibez.spill("  --verbose    Enable verbose output")
\\    vibez.spill("  --quiet      Suppress output")
\\    vibez.spill("  --config     Specify config file")
\\}
\\
\\slay show_version() {
\\    vibez.spill("CURSED CLI Tool v1.0.0")
\\    vibez.spill("Built with CURSED programming language")
\\}
\\
\\slay get_command_args() []tea {
\\    // Mock command line arguments
\\    damn ["help"]
\\}
;

const cli_commands = 
\\yeet "config"
\\yeet "stringz"
\\
\\// Command implementations
\\
\\slay init_command(args []tea, cfg Config) {
\\    vibez.spill("🎯 Initializing new project...")
\\    
\\    sus project_name tea = "new-project"
\\    bestie (args.len > 0) {
\\        project_name = args[0]
\\    }
\\    
\\    // Create project structure
\\    create_directory(project_name)
\\    create_directory(project_name + "/src")
\\    create_directory(project_name + "/tests")
\\    create_directory(project_name + "/docs")
\\    
\\    // Create main file
\\    sus main_content tea = "slay main() {\n    vibez.spill(\"Hello from " + project_name + "!\")\n}"
\\    write_file(project_name + "/src/main.csd", main_content)
\\    
\\    // Create config file
\\    sus config_content tea = generate_config_file(project_name)
\\    write_file(project_name + "/project.toml", config_content)
\\    
\\    vibez.spillf("✅ Project '{}' initialized successfully", project_name)
\\}
\\
\\slay build_command(args []tea, cfg Config) {
\\    vibez.spill("🔨 Building project...")
\\    
\\    sus build_type tea = "debug"
\\    bestie (has_flag(args, "--release")) {
\\        build_type = "release"
\\    }
\\    
\\    vibez.spillf("Build type: {}", build_type)
\\    
\\    // Simulate build process
\\    vibez.spill("  Compiling sources...")
\\    vibez.spill("  Linking...")
\\    vibez.spill("  Optimizing...")
\\    
\\    bestie (cfg.verbose) {
\\        vibez.spill("  Generated build artifacts:")
\\        vibez.spill("    - target/main")
\\        vibez.spill("    - target/main.debug")
\\    }
\\    
\\    vibez.spill("✅ Build completed successfully")
\\}
\\
\\slay test_command(args []tea, cfg Config) {
\\    vibez.spill("🧪 Running tests...")
\\    
\\    sus test_pattern tea = "*"
\\    bestie (args.len > 0) {
\\        test_pattern = args[0]
\\    }
\\    
\\    vibez.spillf("Test pattern: {}", test_pattern)
\\    
\\    // Simulate test execution
\\    sus tests_run normie = 0
\\    sus tests_passed normie = 0
\\    
\\    sus test_files []tea = ["basic_test", "integration_test", "performance_test"]
\\    
\\    sos test_file in test_files {
\\        bestie (stringz.contains(test_file, test_pattern) || test_pattern == "*") {
\\            vibez.spillf("  Running {}...", test_file)
\\            tests_run = tests_run + 1
\\            
\\            // Mock test result
\\            bestie (test_file != "performance_test") {
\\                vibez.spill("    ✅ PASSED")
\\                tests_passed = tests_passed + 1
\\            } else {
\\                vibez.spill("    ⚠️  SKIPPED (performance)")
\\            }
\\        }
\\    }
\\    
\\    vibez.spillf("\\n📊 Test Results: {}/{} passed", tests_passed, tests_run)
\\}
\\
\\slay run_command(args []tea, cfg Config) {
\\    vibez.spill("🚀 Running project...")
\\    
\\    sus target tea = "target/main"
\\    bestie (args.len > 0) {
\\        target = args[0]
\\    }
\\    
\\    vibez.spillf("Executing: {}", target)
\\    
\\    // Simulate program execution
\\    vibez.spill("--- Program Output ---")
\\    vibez.spill("Hello from CURSED CLI project!")
\\    vibez.spill("Program completed successfully")
\\    vibez.spill("--- End Output ---")
\\}
\\
\\slay clean_command(args []tea, cfg Config) {
\\    vibez.spill("🧹 Cleaning build artifacts...")
\\    
\\    sus directories []tea = ["target", "build", ".cache"]
\\    
\\    sos dir in directories {
\\        bestie (directory_exists(dir)) {
\\            vibez.spillf("  Removing {}/", dir)
\\            remove_directory(dir)
\\        }
\\    }
\\    
\\    vibez.spill("✅ Clean completed")
\\}
\\
\\slay config_command(args []tea, cfg Config) {
\\    bestie (args.len == 0) {
\\        // Show current config
\\        vibez.spill("Current Configuration:")
\\        vibez.spillf("  Verbose: {}", cfg.verbose)
\\        vibez.spillf("  Output: {}", cfg.output_dir)
\\        vibez.spillf("  Compiler: {}", cfg.compiler_path)
\\        damn
\\    }
\\    
\\    sus action tea = args[0]
\\    
\\    ready action drip {
\\        "set" => {
\\            bestie (args.len < 3) {
\\                vibez.spill("Usage: tool config set <key> <value>")
\\                damn
\\            }
\\            
\\            sus key tea = args[1]
\\            sus value tea = args[2]
\\            
\\            vibez.spillf("Setting {} = {}", key, value)
\\            config.set_config_value(key, value)
\\        }
\\        "get" => {
\\            bestie (args.len < 2) {
\\                vibez.spill("Usage: tool config get <key>")
\\                damn
\\            }
\\            
\\            sus key tea = args[1]
\\            sus value tea = config.get_config_value(key)
\\            vibez.spillf("{} = {}", key, value)
\\        }
\\        "reset" => {
\\            vibez.spill("Resetting configuration to defaults...")
\\            config.reset_config()
\\        }
\\        _ => {
\\            vibez.spillf("Unknown config action: {}", action)
\\        }
\\    }
\\}
\\
\\// Helper functions
\\slay has_flag(args []tea, flag tea) lit {
\\    sos arg in args {
\\        bestie (arg == flag) {
\\            damn based
\\        }
\\    }
\\    damn cringe
\\}
\\
\\slay generate_config_file(project_name tea) tea {
\\    damn "[project]\\nname = \"" + project_name + "\"\\nversion = \"0.1.0\"\\n\\n[build]\\ntarget = \"bin\"\\noptimize = true"
\\}
\\
\\// Mock file system operations
\\slay create_directory(path tea) {
\\    // Mock implementation
\\}
\\
\\slay write_file(path tea, content tea) {
\\    // Mock implementation
\\}
\\
\\slay directory_exists(path tea) lit {
\\    damn based // Mock implementation
\\}
\\
\\slay remove_directory(path tea) {
\\    // Mock implementation
\\}
;

const cli_config = 
\\yeet "stringz"
\\
\\// Configuration management
\\
\\squad Config {
\\    spill verbose lit = cap
\\    spill quiet lit = cap
\\    spill output_dir tea = "target"
\\    spill compiler_path tea = "cursed"
\\    spill max_parallel normie = 4
\\    spill optimization_level normie = 2
\\}
\\
\\slay load_config() Config {
\\    sus cfg Config = Config{}
\\    
\\    // Try to load from config file
\\    ready {
\\        sus config_content tea = read_file("tool.toml")
\\        cfg = parse_config_toml(config_content)
\\    } bummer {
\\        // Use defaults if config file doesn't exist
\\        vibez.spill("Using default configuration")
\\    }
\\    
\\    // Override with environment variables
\\    cfg = apply_env_overrides(cfg)
\\    
\\    damn cfg
\\}
\\
\\slay save_config(cfg Config) {
\\    sus toml_content tea = config_to_toml(cfg)
\\    write_file("tool.toml", toml_content)
\\}
\\
\\slay parse_config_toml(content tea) Config {
\\    // Simple TOML parser for config
\\    sus cfg Config = Config{}
\\    
\\    sus lines []tea = stringz.split(content, "\\n")
\\    
\\    sos line in lines {
\\        sus trimmed tea = stringz.trim(line)
\\        bestie (trimmed.len == 0 || trimmed[0] == '#') {
\\            continue
\\        }
\\        
\\        bestie (stringz.contains(trimmed, "=")) {
\\            sus parts []tea = stringz.split(trimmed, "=")
\\            bestie (parts.len == 2) {
\\                sus key tea = stringz.trim(parts[0])
\\                sus value tea = stringz.trim(parts[1])
\\                
\\                // Remove quotes from string values
\\                bestie (value[0] == '"' && value[value.len - 1] == '"') {
\\                    value = value[1..value.len - 1]
\\                }
\\                
\\                ready key drip {
\\                    "verbose" => cfg.verbose = parse_bool(value)
\\                    "quiet" => cfg.quiet = parse_bool(value)
\\                    "output_dir" => cfg.output_dir = value
\\                    "compiler_path" => cfg.compiler_path = value
\\                    "max_parallel" => cfg.max_parallel = parse_int(value)
\\                    "optimization_level" => cfg.optimization_level = parse_int(value)
\\                }
\\            }
\\        }
\\    }
\\    
\\    damn cfg
\\}
\\
\\slay config_to_toml(cfg Config) tea {
\\    sus content tea = ""
\\    content = content + "# CURSED CLI Tool Configuration\\n"
\\    content = content + "\\n"
\\    content = content + "verbose = " + bool_to_string(cfg.verbose) + "\\n"
\\    content = content + "quiet = " + bool_to_string(cfg.quiet) + "\\n"
\\    content = content + "output_dir = \\"" + cfg.output_dir + "\\"\\n"
\\    content = content + "compiler_path = \\"" + cfg.compiler_path + "\\"\\n"
\\    content = content + "max_parallel = " + int_to_string(cfg.max_parallel) + "\\n"
\\    content = content + "optimization_level = " + int_to_string(cfg.optimization_level) + "\\n"
\\    
\\    damn content
\\}
\\
\\slay apply_env_overrides(cfg Config) Config {
\\    // Check for environment variable overrides
\\    sus env_verbose tea = get_env("CURSED_VERBOSE")
\\    bestie (env_verbose.len > 0) {
\\        cfg.verbose = parse_bool(env_verbose)
\\    }
\\    
\\    sus env_output tea = get_env("CURSED_OUTPUT_DIR")
\\    bestie (env_output.len > 0) {
\\        cfg.output_dir = env_output
\\    }
\\    
\\    damn cfg
\\}
\\
\\slay set_config_value(key tea, value tea) {
\\    sus cfg Config = load_config()
\\    
\\    ready key drip {
\\        "verbose" => cfg.verbose = parse_bool(value)
\\        "quiet" => cfg.quiet = parse_bool(value)
\\        "output_dir" => cfg.output_dir = value
\\        "compiler_path" => cfg.compiler_path = value
\\        "max_parallel" => cfg.max_parallel = parse_int(value)
\\        "optimization_level" => cfg.optimization_level = parse_int(value)
\\        _ => {
\\            vibez.spillf("Unknown config key: {}", key)
\\            damn
\\        }
\\    }
\\    
\\    save_config(cfg)
\\    vibez.spillf("Configuration updated: {} = {}", key, value)
\\}
\\
\\slay get_config_value(key tea) tea {
\\    sus cfg Config = load_config()
\\    
\\    ready key drip {
\\        "verbose" => damn bool_to_string(cfg.verbose)
\\        "quiet" => damn bool_to_string(cfg.quiet)
\\        "output_dir" => damn cfg.output_dir
\\        "compiler_path" => damn cfg.compiler_path
\\        "max_parallel" => damn int_to_string(cfg.max_parallel)
\\        "optimization_level" => damn int_to_string(cfg.optimization_level)
\\        _ => damn "unknown key"
\\    }
\\}
\\
\\slay reset_config() {
\\    sus default_cfg Config = Config{}
\\    save_config(default_cfg)
\\    vibez.spill("Configuration reset to defaults")
\\}
\\
\\// Utility functions
\\slay parse_bool(value tea) lit {
\\    damn value == "true" || value == "1" || value == "yes"
\\}
\\
\\slay parse_int(value tea) normie {
\\    // Simple integer parsing
\\    damn 0 // Mock implementation
\\}
\\
\\slay bool_to_string(value lit) tea {
\\    damn bestie (value) "true" else "false"
\\}
\\
\\slay int_to_string(value normie) tea {
\\    damn "0" // Mock implementation
\\}
\\
\\slay read_file(path tea) tea {
\\    // Mock file reading
\\    damn ""
\\}
\\
\\slay write_file(path tea, content tea) {
\\    // Mock file writing
\\}
\\
\\slay get_env(name tea) tea {
\\    // Mock environment variable access
\\    damn ""
\\}
;

const cli_test = 
\\yeet "testz"
\\yeet "config"
\\yeet "commands"
\\
\\test "CLI tool functionality" {
\\    test_start("CLI tool test suite")
\\    
\\    // Test configuration
\\    sus cfg Config = load_config()
\\    assert_true(cfg.output_dir.len > 0)
\\    assert_true(cfg.max_parallel > 0)
\\    
\\    // Test config parsing
\\    sus test_toml tea = "verbose = true\\noutput_dir = \\"test\\""
\\    sus parsed_cfg Config = parse_config_toml(test_toml)
\\    assert_true(parsed_cfg.verbose)
\\    assert_eq_string(parsed_cfg.output_dir, "test")
\\    
\\    // Test utility functions
\\    assert_true(parse_bool("true"))
\\    assert_false(parse_bool("false"))
\\    assert_eq_string(bool_to_string(based), "true")
\\    assert_eq_string(bool_to_string(cringe), "false")
\\    
\\    print_test_summary()
\\}
;

const cli_readme = 
\\# CURSED CLI Tool
\\
\\A powerful command-line interface tool built with CURSED.
\\
\\## Installation
\\
\\```bash
\\cursed-pkg add cursed-cli-tool
\\cursed-pkg build
\\```
\\
\\## Usage
\\
\\### Basic Commands
\\
\\```bash
\\tool init myproject        # Initialize new project
\\tool build                 # Build project
\\tool build --release       # Release build
\\tool test                  # Run tests
\\tool run                   # Execute project
\\tool clean                 # Clean artifacts
\\```
\\
\\### Configuration
\\
\\```bash
\\tool config                # Show current config
\\tool config set verbose true
\\tool config get output_dir
\\tool config reset
\\```
\\
\\### Global Options
\\
\\- `--verbose` - Enable verbose output
\\- `--quiet` - Suppress output
\\- `--config <file>` - Use custom config file
\\
\\## Configuration File
\\
\\Create `tool.toml` in your project:
\\
\\```toml
\\verbose = false
\\quiet = false
\\output_dir = "target"
\\compiler_path = "cursed"
\\max_parallel = 4
\\optimization_level = 2
\\```
\\
\\## Environment Variables
\\
\\- `CURSED_VERBOSE` - Enable verbose mode
\\- `CURSED_OUTPUT_DIR` - Set output directory
\\- `CURSED_COMPILER` - Set compiler path
\\
\\## Development
\\
\\```bash
\\cursed-pkg test
\\cursed-pkg build --release
\\cursed-pkg publish
\\```
;

pub fn createFromTemplate(allocator: Allocator, template_type: TemplateType, project_name: []const u8) !void {
    const template = switch (template_type) {
        .library => templates.library,
        .binary => templates.binary,
        .webapp => templates.webapp,
        .api_server => templates.api_server,
        .cli_tool => templates.cli_tool,
        .testing_framework => templates.library, // Fallback to library
    };
    
    std.debug.writer().print("🎯 Creating {s} project: {s}\n", .{{template.template_type, project_name});
    
    // Create project directory
    try std.fs.cwd().makeDir(project_name);
    var project_dir = try std.fs.cwd().openDir(project_name, .{});
    defer project_dir.close();
    
    // Create file structure
    for (template.files) |file_template| {
        const dir_path = std.fs.path.dirname(file_template.path);
        if (dir_path) |dir| {
            project_dir.makePath(dir) catch {}; // Ignore if already exists
        }
        
        try project_dir.writeFile(.{ .sub_path = file_template.path, .data = file_template.content });
        std.debug.writer().print("  📄 Created {s}\n", .{file_template.path});
    }
    
    // Create package manifest
    const manifest_content = try createManifestContent(allocator, project_name, template);
    defer allocator.free(manifest_content);
    try project_dir.writeFile(.{ .sub_path = "CursedPackage.toml", .data = manifest_content });
    
    std.debug.writer().print("✅ Project '{s}' created successfully\n", .{project_name});
    std.debug.writer().print("📁 Change to directory: cd {s}\n", .{project_name});
    std.debug.writer().print("🏗️  Install dependencies: cursed-pkg install\n", .{});
    std.debug.writer().print("🧪 Run tests: cursed-pkg test\n", .{});
}

fn createManifestContent(allocator: Allocator, project_name: []const u8, template: Template) ![]const u8 {
    var content = ArrayList(u8){};
    var writer = content.writer();
    
    try writer.writer().print("name = \"{s}\"\n", .{project_name});
    try writer.writer().writeAll("version = \"0.1.0\"\n");
    try writer.writer().print("description = \"{s}\"\n", .{template.description});
    try writer.writer().writeAll("authors = [\"Your Name <your.email@example.com>\"]\n");
    try writer.writer().writeAll("license = \"MIT\"\n");
    
    switch (template.template_type) {
        .library => try writer.writer().writeAll("main = \"src/lib.csd\"\n"),
        .binary, .cli_tool => try writer.writer().writeAll("main = \"src/main.csd\"\n"),
        .webapp, .api_server => try writer.writer().writeAll("main = \"src/main.csd\"\n"),
        .testing_framework => try writer.writer().writeAll("main = \"src/lib.csd\"\n"),
    }
    
    // Dependencies
    if (template.dependencies.len > 0) {
        try writer.writer().writeAll("\n[dependencies]\n");
        for (template.dependencies) |dep| {
            try writer.writer().print("{s} = \"^1.0.0\"\n", .{dep});
        }
    }
    
    // Dev dependencies
    if (template.dev_dependencies.len > 0) {
        try writer.writer().writeAll("\n[dev_dependencies]\n");
        for (template.dev_dependencies) |dep| {
            try writer.writer().print("{s} = \"^1.0.0\"\n", .{dep});
        }
    }
    
    return content.toOwnedSlice();
}
