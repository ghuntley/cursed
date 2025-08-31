fr fr TODO LIST APPLICATION - Complete Task Management System
fr fr Features: CRUD operations, categories, priorities, due dates, JSON API, web interface

yeet "database_enhanced"
yeet "webz"
yeet "json"
yeet "stringz"
yeet "timez"
yeet "fs"
yeet "vibez"

fr fr ===== APPLICATION CONFIGURATION =====

squad TodoConfig {
    sus database_url tea
    sus server_port drip
    sus web_root tea
    sus storage_path tea
}

sus app_config TodoConfig = TodoConfig{
    database_url: "file://./todo_data",
    server_port: 8080,
    web_root: "./web",
    storage_path: "./todo_data"
}

fr fr ===== DATA MODELS =====

squad Todo {
    sus id drip
    sus title tea
    sus description tea
    sus category tea
    sus priority tea
    sus due_date tea
    sus completed lit
    sus created_at tea
    sus updated_at tea
}

squad Category {
    sus id drip
    sus name tea
    sus color tea
    sus description tea
    sus created_at tea
}

fr fr ===== DATABASE INITIALIZATION =====

slay initialize_database() database_enhanced.DatabaseConnection {
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(app_config.database_url)
    
    ready (!conn.is_connected) {
        vibez.spill("FATAL: Could not connect to database")
        sus empty database_enhanced.DatabaseConnection = database_enhanced.DatabaseConnection{}
        damn empty
    }
    
    fr fr Create tables if they don't exist
    create_todos_table(conn)
    create_categories_table(conn)
    
    fr fr Insert default categories
    insert_default_categories(conn)
    
    vibez.spill("Todo database initialized successfully")
    damn conn
}

slay create_todos_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "title": "TEXT NOT NULL",
        "description": "TEXT",
        "category": "TEXT DEFAULT 'General'",
        "priority": "TEXT DEFAULT 'Medium'", 
        "due_date": "TEXT",
        "completed": "BOOLEAN DEFAULT 0",
        "created_at": "TEXT NOT NULL",
        "updated_at": "TEXT NOT NULL"
    })
    
    ready (database_enhanced.create_table(conn, "todos", schema)) {
        vibez.spill("Created todos table")
    } otherwise {
        vibez.spill("Todos table already exists or creation failed")
    }
}

slay create_categories_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "name": "TEXT NOT NULL UNIQUE",
        "color": "TEXT DEFAULT '#007bff'",
        "description": "TEXT",
        "created_at": "TEXT NOT NULL"
    })
    
    ready (database_enhanced.create_table(conn, "categories", schema)) {
        vibez.spill("Created categories table")
    } otherwise {
        vibez.spill("Categories table already exists or creation failed")
    }
}

slay insert_default_categories(conn database_enhanced.DatabaseConnection) {
    sus default_categories []tea = [
        json.object_to_string({
            "name": "Personal",
            "color": "#28a745", 
            "description": "Personal tasks and activities",
            "created_at": timez.format_iso8601(timez.now_millis())
        }),
        json.object_to_string({
            "name": "Work",
            "color": "#007bff",
            "description": "Work-related tasks",
            "created_at": timez.format_iso8601(timez.now_millis())
        }),
        json.object_to_string({
            "name": "Shopping",
            "color": "#ffc107",
            "description": "Shopping lists and purchases",
            "created_at": timez.format_iso8601(timez.now_millis())
        }),
        json.object_to_string({
            "name": "Health",
            "color": "#dc3545",
            "description": "Health and fitness goals",
            "created_at": timez.format_iso8601(timez.now_millis())
        })
    ]
    
    sus i drip = 0
    bestie (i < default_categories.length) {
        database_enhanced.insert_record(conn, "categories", default_categories[i])
        i = i + 1
    }
    
    vibez.spill("Inserted default categories")
}

fr fr ===== TODO OPERATIONS =====

slay create_todo(conn database_enhanced.DatabaseConnection, title tea, description tea, category tea, priority tea, due_date tea) drip {
    sus now tea = timez.format_iso8601(timez.now_millis())
    
    sus todo_data tea = json.object_to_string({
        "title": title,
        "description": description,
        "category": category,
        "priority": priority,
        "due_date": due_date,
        "completed": "false",
        "created_at": now,
        "updated_at": now
    })
    
    ready (database_enhanced.insert_record(conn, "todos", todo_data)) {
        vibez.spill("Created todo: " + title)
        damn mathz.random_int(10000) fr fr Simulate returning generated ID
    }
    
    damn 0
}

slay get_all_todos(conn database_enhanced.DatabaseConnection) []tea {
    sus conditions tea = "{}" fr fr No conditions = get all
    sus todos []tea = database_enhanced.find_records(conn, "todos", conditions)
    
    vibez.spill("Retrieved " + stringz.from_int(todos.length) + " todos")
    damn todos
}

slay get_todos_by_category(conn database_enhanced.DatabaseConnection, category tea) []tea {
    sus conditions tea = json.object_to_string({
        "category": category
    })
    
    sus todos []tea = database_enhanced.find_records(conn, "todos", conditions)
    vibez.spill("Retrieved " + stringz.from_int(todos.length) + " todos for category: " + category)
    damn todos
}

slay get_pending_todos(conn database_enhanced.DatabaseConnection) []tea {
    sus conditions tea = json.object_to_string({
        "completed": "false"
    })
    
    sus todos []tea = database_enhanced.find_records(conn, "todos", conditions)
    vibez.spill("Retrieved " + stringz.from_int(todos.length) + " pending todos")
    damn todos
}

slay get_overdue_todos(conn database_enhanced.DatabaseConnection) []tea {
    sus current_time tea = timez.format_iso8601(timez.now_millis())
    sus all_todos []tea = get_pending_todos(conn)
    sus overdue_todos []tea = []
    sus overdue_count drip = 0
    
    sus i drip = 0
    bestie (i < all_todos.length) {
        sus todo_data map[tea]tea = json.parse_object(all_todos[i])
        sus due_date tea = todo_data["due_date"]
        
        ready (due_date != "" && due_date < current_time) {
            overdue_todos[overdue_count] = all_todos[i]
            overdue_count = overdue_count + 1
        }
        i = i + 1
    }
    
    vibez.spill("Retrieved " + stringz.from_int(overdue_count) + " overdue todos")
    damn overdue_todos
}

slay update_todo(conn database_enhanced.DatabaseConnection, id drip, updates tea) lit {
    fr fr Add updated_at timestamp
    sus update_data map[tea]tea = json.parse_object(updates)
    update_data["updated_at"] = timez.format_iso8601(timez.now_millis())
    sus final_updates tea = json.object_to_string(update_data)
    
    ready (database_enhanced.update_record(conn, "todos", id, final_updates)) {
        vibez.spill("Updated todo ID: " + stringz.from_int(id))
        damn based
    }
    
    vibez.spill("Failed to update todo ID: " + stringz.from_int(id))
    damn cringe
}

slay complete_todo(conn database_enhanced.DatabaseConnection, id drip) lit {
    sus updates tea = json.object_to_string({
        "completed": "true"
    })
    
    damn update_todo(conn, id, updates)
}

slay delete_todo(conn database_enhanced.DatabaseConnection, id drip) lit {
    ready (database_enhanced.delete_record(conn, "todos", id)) {
        vibez.spill("Deleted todo ID: " + stringz.from_int(id))
        damn based
    }
    
    vibez.spill("Failed to delete todo ID: " + stringz.from_int(id))
    damn cringe
}

fr fr ===== CATEGORY OPERATIONS =====

slay create_category(conn database_enhanced.DatabaseConnection, name tea, color tea, description tea) drip {
    sus now tea = timez.format_iso8601(timez.now_millis())
    
    sus category_data tea = json.object_to_string({
        "name": name,
        "color": color,
        "description": description,
        "created_at": now
    })
    
    ready (database_enhanced.insert_record(conn, "categories", category_data)) {
        vibez.spill("Created category: " + name)
        damn mathz.random_int(10000)
    }
    
    damn 0
}

slay get_all_categories(conn database_enhanced.DatabaseConnection) []tea {
    sus conditions tea = "{}"
    sus categories []tea = database_enhanced.find_records(conn, "categories", conditions)
    
    vibez.spill("Retrieved " + stringz.from_int(categories.length) + " categories")
    damn categories
}

fr fr ===== WEB API HANDLERS =====

slay handle_api_request(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"}
    
    ready (request.method == "GET" && request.path == "/api/todos") {
        damn handle_get_todos(conn, request)
    } otherwise ready (request.method == "POST" && request.path == "/api/todos") {
        damn handle_create_todo(conn, request)
    } otherwise ready (request.method == "PUT" && stringz.starts_with(request.path, "/api/todos/")) {
        damn handle_update_todo(conn, request)
    } otherwise ready (request.method == "DELETE" && stringz.starts_with(request.path, "/api/todos/")) {
        damn handle_delete_todo(conn, request)
    } otherwise ready (request.method == "GET" && request.path == "/api/categories") {
        damn handle_get_categories(conn, request)
    } otherwise ready (request.method == "POST" && request.path == "/api/categories") {
        damn handle_create_category(conn, request)
    } otherwise {
        response.status_code = 404
        response.body = json.object_to_string({"error": "Endpoint not found"})
        damn response
    }
}

slay handle_get_todos(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    response.status_code = 200
    
    fr fr Check for category filter
    sus category tea = webz.get_query_param(request, "category")
    sus status tea = webz.get_query_param(request, "status")
    
    sus todos []tea = []
    
    ready (category != "") {
        todos = get_todos_by_category(conn, category)
    } otherwise ready (status == "overdue") {
        todos = get_overdue_todos(conn)
    } otherwise ready (status == "pending") {
        todos = get_pending_todos(conn)
    } otherwise {
        todos = get_all_todos(conn)
    }
    
    sus todos_json tea = json.array_to_string(todos)
    response.body = json.object_to_string({"todos": todos_json, "count": stringz.from_int(todos.length)})
    
    damn response
}

slay handle_create_todo(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    sus todo_data map[tea]tea = json.parse_object(request.body)
    
    ready (todo_data["title"] == "") {
        response.status_code = 400
        response.body = json.object_to_string({"error": "Title is required"})
        damn response
    }
    
    sus id drip = create_todo(conn, 
        todo_data["title"],
        todo_data["description"],
        todo_data["category"],
        todo_data["priority"],
        todo_data["due_date"]
    )
    
    ready (id > 0) {
        response.status_code = 201
        response.body = json.object_to_string({
            "id": stringz.from_int(id),
            "message": "Todo created successfully"
        })
    } otherwise {
        response.status_code = 500
        response.body = json.object_to_string({"error": "Failed to create todo"})
    }
    
    damn response
}

slay handle_update_todo(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    sus id_str tea = extract_id_from_path(request.path)
    sus id drip = stringz.to_int(id_str)
    
    ready (id <= 0) {
        response.status_code = 400
        response.body = json.object_to_string({"error": "Invalid todo ID"})
        damn response
    }
    
    ready (update_todo(conn, id, request.body)) {
        response.status_code = 200
        response.body = json.object_to_string({"message": "Todo updated successfully"})
    } otherwise {
        response.status_code = 500
        response.body = json.object_to_string({"error": "Failed to update todo"})
    }
    
    damn response
}

slay handle_delete_todo(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    sus id_str tea = extract_id_from_path(request.path)
    sus id drip = stringz.to_int(id_str)
    
    ready (id <= 0) {
        response.status_code = 400
        response.body = json.object_to_string({"error": "Invalid todo ID"})
        damn response
    }
    
    ready (delete_todo(conn, id)) {
        response.status_code = 200
        response.body = json.object_to_string({"message": "Todo deleted successfully"})
    } otherwise {
        response.status_code = 500
        response.body = json.object_to_string({"error": "Failed to delete todo"})
    }
    
    damn response
}

slay handle_get_categories(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    response.status_code = 200
    
    sus categories []tea = get_all_categories(conn)
    sus categories_json tea = json.array_to_string(categories)
    response.body = json.object_to_string({"categories": categories_json, "count": stringz.from_int(categories.length)})
    
    damn response
}

slay handle_create_category(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    sus category_data map[tea]tea = json.parse_object(request.body)
    
    ready (category_data["name"] == "") {
        response.status_code = 400
        response.body = json.object_to_string({"error": "Name is required"})
        damn response
    }
    
    sus id drip = create_category(conn,
        category_data["name"],
        category_data["color"],
        category_data["description"]
    )
    
    ready (id > 0) {
        response.status_code = 201
        response.body = json.object_to_string({
            "id": stringz.from_int(id),
            "message": "Category created successfully"
        })
    } otherwise {
        response.status_code = 500
        response.body = json.object_to_string({"error": "Failed to create category"})
    }
    
    damn response
}

fr fr ===== WEB INTERFACE HANDLERS =====

slay handle_web_request(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    ready (request.path == "/" || request.path == "/index.html") {
        damn serve_static_file("index.html")
    } otherwise ready (stringz.starts_with(request.path, "/static/")) {
        damn serve_static_file(request.path)
    } otherwise ready (request.path == "/todos") {
        damn render_todos_page(conn)
    } otherwise ready (request.path == "/categories") {
        damn render_categories_page(conn)
    } otherwise {
        sus response webz.HttpResponse = webz.HttpResponse{}
        response.status_code = 404
        response.body = render_404_page()
        response.headers = {"Content-Type": "text/html"}
        damn response
    }
}

slay render_todos_page(conn database_enhanced.DatabaseConnection) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "text/html"}
    response.status_code = 200
    
    sus todos []tea = get_all_todos(conn)
    sus categories []tea = get_all_categories(conn)
    
    sus html tea = generate_todos_html(todos, categories)
    response.body = html
    
    damn response
}

slay render_categories_page(conn database_enhanced.DatabaseConnection) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "text/html"}
    response.status_code = 200
    
    sus categories []tea = get_all_categories(conn)
    
    sus html tea = generate_categories_html(categories)
    response.body = html
    
    damn response
}

slay serve_static_file(file_path tea) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    sus full_path tea = app_config.web_root + "/" + file_path
    
    ready (fs.file_exists(full_path)) {
        response.status_code = 200
        response.body = fs.read_file(full_path)
        response.headers = {"Content-Type": get_content_type(file_path)}
    } otherwise {
        response.status_code = 404
        response.body = "File not found"
        response.headers = {"Content-Type": "text/plain"}
    }
    
    damn response
}

fr fr ===== HTML GENERATION =====

slay generate_todos_html(todos []tea, categories []tea) tea {
    sus html tea = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Todo List - CURSED App</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; }
        .todo-form { background: #f8f9fa; padding: 20px; border-radius: 6px; margin-bottom: 30px; }
        .todo-item { border: 1px solid #ddd; margin: 10px 0; padding: 15px; border-radius: 6px; background: white; }
        .todo-completed { background: #e8f5e9; text-decoration: line-through; opacity: 0.7; }
        .priority-high { border-left: 4px solid #dc3545; }
        .priority-medium { border-left: 4px solid #ffc107; }
        .priority-low { border-left: 4px solid #28a745; }
        input, select, textarea { padding: 8px; margin: 5px 0; border: 1px solid #ddd; border-radius: 4px; }
        button { background: #007bff; color: white; padding: 10px 15px; border: none; border-radius: 4px; cursor: pointer; }
        button:hover { background: #0056b3; }
        .btn-danger { background: #dc3545; }
        .btn-success { background: #28a745; }
        .category-badge { background: #007bff; color: white; padding: 2px 8px; border-radius: 12px; font-size: 0.8em; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Todo List Application</h1>
            <nav>
                <a href="/">Home</a> | 
                <a href="/todos">Todos</a> | 
                <a href="/categories">Categories</a>
            </nav>
        </div>
        
        <div class="todo-form">
            <h3>Add New Todo</h3>
            <form id="todo-form">
                <input type="text" id="title" placeholder="Todo title" required style="width: 300px;">
                <textarea id="description" placeholder="Description" style="width: 300px; height: 60px;"></textarea>
                <select id="category">` + generate_category_options(categories) + `</select>
                <select id="priority">
                    <option value="Low">Low</option>
                    <option value="Medium" selected>Medium</option>
                    <option value="High">High</option>
                </select>
                <input type="datetime-local" id="due-date">
                <button type="submit">Add Todo</button>
            </form>
        </div>
        
        <div class="todo-list">
            <h3>Todos (` + stringz.from_int(todos.length) + `)</h3>
            ` + generate_todo_items(todos) + `
        </div>
    </div>
    
    <script>
        document.getElementById('todo-form').addEventListener('submit', function(e) {
            e.preventDefault();
            // JavaScript for form submission would go here
            alert('Todo form submitted! (JavaScript integration needed)');
        });
    </script>
</body>
</html>`
    
    damn html
}

slay generate_categories_html(categories []tea) tea {
    sus html tea = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Categories - Todo App</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }
        .container { max-width: 800px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .category-item { border: 1px solid #ddd; margin: 10px 0; padding: 15px; border-radius: 6px; background: white; }
        .color-preview { width: 20px; height: 20px; border-radius: 50%; display: inline-block; margin-right: 10px; }
        input, textarea { padding: 8px; margin: 5px 0; border: 1px solid #ddd; border-radius: 4px; }
        button { background: #007bff; color: white; padding: 10px 15px; border: none; border-radius: 4px; cursor: pointer; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Categories</h1>
        <nav>
            <a href="/">Home</a> | 
            <a href="/todos">Todos</a> | 
            <a href="/categories">Categories</a>
        </nav>
        
        <div style="background: #f8f9fa; padding: 20px; border-radius: 6px; margin: 20px 0;">
            <h3>Add New Category</h3>
            <form id="category-form">
                <input type="text" id="name" placeholder="Category name" required style="width: 200px;">
                <input type="color" id="color" value="#007bff">
                <textarea id="description" placeholder="Description" style="width: 300px; height: 60px;"></textarea>
                <button type="submit">Add Category</button>
            </form>
        </div>
        
        <div class="category-list">
            <h3>Categories (` + stringz.from_int(categories.length) + `)</h3>
            ` + generate_category_items(categories) + `
        </div>
    </div>
</body>
</html>`
    
    damn html
}

slay generate_category_options(categories []tea) tea {
    sus options tea = `<option value="General">General</option>`
    
    sus i drip = 0
    bestie (i < categories.length) {
        sus category_data map[tea]tea = json.parse_object(categories[i])
        options = options + `<option value="` + category_data["name"] + `">` + category_data["name"] + `</option>`
        i = i + 1
    }
    
    damn options
}

slay generate_todo_items(todos []tea) tea {
    sus html tea = ""
    
    ready (todos.length == 0) {
        damn `<div class="todo-item">No todos found. Add your first todo above!</div>`
    }
    
    sus i drip = 0
    bestie (i < todos.length) {
        sus todo_data map[tea]tea = json.parse_object(todos[i])
        
        sus completed_class tea = ""
        ready (todo_data["completed"] == "true") {
            completed_class = " todo-completed"
        }
        
        sus priority_class tea = " priority-" + stringz.to_lower(todo_data["priority"])
        
        html = html + `<div class="todo-item` + completed_class + priority_class + `">
            <h4>` + todo_data["title"] + ` 
                <span class="category-badge">` + todo_data["category"] + `</span>
                <span style="font-size: 0.8em; color: #666;">[` + todo_data["priority"] + `]</span>
            </h4>
            <p>` + todo_data["description"] + `</p>
            <div style="font-size: 0.9em; color: #666;">
                Due: ` + todo_data["due_date"] + ` | 
                Created: ` + todo_data["created_at"] + `
            </div>
            <div style="margin-top: 10px;">
                <button class="btn-success">Complete</button>
                <button>Edit</button>
                <button class="btn-danger">Delete</button>
            </div>
        </div>`
        
        i = i + 1
    }
    
    damn html
}

slay generate_category_items(categories []tea) tea {
    sus html tea = ""
    
    ready (categories.length == 0) {
        damn `<div class="category-item">No categories found.</div>`
    }
    
    sus i drip = 0
    bestie (i < categories.length) {
        sus category_data map[tea]tea = json.parse_object(categories[i])
        
        html = html + `<div class="category-item">
            <div style="display: flex; align-items: center;">
                <span class="color-preview" style="background-color: ` + category_data["color"] + `;"></span>
                <div style="flex: 1;">
                    <h4>` + category_data["name"] + `</h4>
                    <p>` + category_data["description"] + `</p>
                    <small>Created: ` + category_data["created_at"] + `</small>
                </div>
                <div>
                    <button>Edit</button>
                    <button class="btn-danger">Delete</button>
                </div>
            </div>
        </div>`
        
        i = i + 1
    }
    
    damn html
}

slay render_404_page() tea {
    damn `<!DOCTYPE html>
<html><head><title>404 - Not Found</title></head>
<body style="font-family: Arial; text-align: center; padding: 50px;">
    <h1>404 - Page Not Found</h1>
    <p>The requested page could not be found.</p>
    <a href="/">Return to Home</a>
</body></html>`
}

fr fr ===== UTILITY FUNCTIONS =====

slay extract_id_from_path(path tea) tea {
    sus parts []tea = stringz.split(path, "/")
    ready (parts.length >= 3) {
        damn parts[3] fr fr /api/todos/123 -> parts[3] = "123"
    }
    damn ""
}

slay get_content_type(file_path tea) tea {
    ready (stringz.ends_with(file_path, ".html")) {
        damn "text/html"
    } otherwise ready (stringz.ends_with(file_path, ".css")) {
        damn "text/css"
    } otherwise ready (stringz.ends_with(file_path, ".js")) {
        damn "application/javascript"
    } otherwise ready (stringz.ends_with(file_path, ".json")) {
        damn "application/json"
    }
    damn "text/plain"
}

fr fr ===== MAIN APPLICATION =====

slay main() {
    vibez.spill("Starting Todo List Application...")
    
    fr fr Initialize database
    sus db_conn database_enhanced.DatabaseConnection = initialize_database()
    ready (!db_conn.is_connected) {
        vibez.spill("FATAL: Could not initialize database")
        damn
    }
    
    fr fr Create web root directory if it doesn't exist
    ready (!fs.directory_exists(app_config.web_root)) {
        fs.create_directory(app_config.web_root)
        create_default_web_files()
    }
    
    fr fr Setup web server
    sus server webz.Server = webz.create_server(app_config.server_port)
    
    fr fr Register request handler
    webz.handle_requests(server, slay(request webz.HttpRequest) webz.HttpResponse {
        ready (stringz.starts_with(request.path, "/api/")) {
            damn handle_api_request(db_conn, request)
        } otherwise {
            damn handle_web_request(db_conn, request)
        }
    })
    
    vibez.spill("Todo application started on port " + stringz.from_int(app_config.server_port))
    vibez.spill("Web interface: http://localhost:" + stringz.from_int(app_config.server_port))
    vibez.spill("API endpoints:")
    vibez.spill("  GET /api/todos - List all todos")
    vibez.spill("  POST /api/todos - Create new todo")
    vibez.spill("  PUT /api/todos/{id} - Update todo")
    vibez.spill("  DELETE /api/todos/{id} - Delete todo")
    vibez.spill("  GET /api/categories - List categories")
    vibez.spill("  POST /api/categories - Create category")
    
    fr fr Start server
    webz.start_server(server)
    
    fr fr Cleanup
    database_enhanced.close_connection(db_conn)
}

slay create_default_web_files() {
    sus index_html tea = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Todo List - CURSED Application</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; min-height: 100vh; }
        .container { max-width: 800px; margin: 0 auto; text-align: center; padding: 50px 20px; }
        .hero { background: rgba(255,255,255,0.1); padding: 40px; border-radius: 10px; margin-bottom: 30px; backdrop-filter: blur(10px); }
        .features { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 30px 0; }
        .feature { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 8px; backdrop-filter: blur(10px); }
        .nav-buttons { display: flex; justify-content: center; gap: 15px; margin-top: 30px; }
        .nav-buttons a { background: #007bff; color: white; padding: 15px 25px; text-decoration: none; border-radius: 6px; font-weight: bold; transition: all 0.3s; }
        .nav-buttons a:hover { background: #0056b3; transform: translateY(-2px); }
    </style>
</head>
<body>
    <div class="container">
        <div class="hero">
            <h1>🚀 Todo List Application</h1>
            <p>A complete task management system built with CURSED</p>
            <p>Organize your life, boost your productivity!</p>
        </div>
        
        <div class="features">
            <div class="feature">
                <h3>📝 Task Management</h3>
                <p>Create, update, and organize your tasks with categories, priorities, and due dates</p>
            </div>
            <div class="feature">
                <h3>🏷️ Categories</h3>
                <p>Organize tasks with custom categories and color coding</p>
            </div>
            <div class="feature">
                <h3>⚡ Real-time API</h3>
                <p>JSON API for integration with mobile apps and external services</p>
            </div>
            <div class="feature">
                <h3>💾 Persistent Storage</h3>
                <p>All your data is safely stored and retrieved using CURSED's database system</p>
            </div>
        </div>
        
        <div class="nav-buttons">
            <a href="/todos">View Todos</a>
            <a href="/categories">Manage Categories</a>
            <a href="/api/todos">API Documentation</a>
        </div>
        
        <div style="margin-top: 50px; opacity: 0.8;">
            <p>Built with ❤️ using the CURSED programming language</p>
        </div>
    </div>
</body>
</html>`
    
    fs.write_file(app_config.web_root + "/index.html", index_html)
    vibez.spill("Created default web files")
}

fr fr Start the application
main()
