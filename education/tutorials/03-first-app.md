# Building Your First CURSED Web Application

Create a complete web application with routing, templates, and database integration. This tutorial takes you from zero to a deployed web app in CURSED.

## What We're Building

A **Task Manager Web App** with:
- ✅ RESTful API endpoints
- ✅ HTML templating
- ✅ JSON database storage
- ✅ Static file serving
- ✅ Real-time updates

**Final result:** A fully functional task manager accessible at `http://localhost:8080`

## Project Setup

```bash
# Create project directory
mkdir cursed-taskman
cd cursed-taskman

# Create project structure
mkdir -p {src,static,templates,data}
touch src/main.csd src/tasks.csd src/server.csd
touch templates/{index.html,task.html}
touch static/{style.css,app.js}
```

## Step 1: Basic Web Server

Let's start with a simple HTTP server:

```cursed
# src/server.csd
yeet "vibez"
yeet "networkz"
yeet "filez"
yeet "stringz"

squad Server {
    port drip,
    routes map<tea, slay(Request) Response>
}

squad Request {
    method tea,
    path tea,
    headers map<tea, tea>,
    body tea
}

squad Response {
    status drip,
    headers map<tea, tea>,
    body tea
}

slay new_server(port drip) Server {
    damn Server{
        port: port,
        routes: make_map()
    }
}

slay add_route(server Server, method tea, path tea, handler slay(Request) Response) {
    sus key tea = method + " " + path
    server.routes[key] = handler
}

slay start_server(server Server) yikes<tea> {
    vibez.spill("Starting server on port", server.port)
    
    sus listener = networkz.listen("localhost", server.port) fam {
        when err -> yikes "Failed to start server: " + err
    }
    
    vibez.spill("Server running at http://localhost:" + string(server.port))
    
    bestie (based) {
        sus conn = networkz.accept(listener) fam {
            when err -> {
                vibez.spill("Connection error:", err)
                continue
            }
        }
        
        # Handle connection in goroutine
        go {
            handle_connection(server, conn)
        }
    }
}

slay handle_connection(server Server, conn Connection) {
    sus request_data tea = networkz.read_request(conn) fam {
        when err -> {
            vibez.spill("Failed to read request:", err)
            damn
        }
    }
    
    sus request Request = parse_request(request_data)
    sus key tea = request.method + " " + request.path
    
    sus response Response = ready (server.routes.has_key(key)) {
        damn server.routes[key](request)
    } otherwise {
        damn Response{
            status: 404,
            headers: make_map(),
            body: "404 Not Found"
        }
    }
    
    send_response(conn, response)
    networkz.close(conn)
}

slay parse_request(data tea) Request {
    sus lines []tea = stringz.split(data, "\n")
    sus first_line []tea = stringz.split(lines[0], " ")
    
    damn Request{
        method: first_line[0],
        path: first_line[1],
        headers: parse_headers(lines),
        body: extract_body(lines)
    }
}

slay send_response(conn Connection, response Response) {
    sus status_line tea = "HTTP/1.1 " + string(response.status) + " OK\r\n"
    sus headers tea = "Content-Length: " + string(len(response.body)) + "\r\n"
    headers = headers + "Content-Type: text/html\r\n\r\n"
    
    sus full_response tea = status_line + headers + response.body
    networkz.write(conn, full_response)
}
```

## Step 2: Task Data Model

Create a task management system:

```cursed
# src/tasks.csd
yeet "vibez"
yeet "jsonz"
yeet "filez"
yeet "timez"

squad Task {
    id drip,
    title tea,
    description tea,
    completed lit,
    created_at tea,
    updated_at tea
}

squad TaskManager {
    tasks []Task,
    next_id drip,
    file_path tea
}

slay new_task_manager(file_path tea) TaskManager {
    sus manager TaskManager = TaskManager{
        tasks: [],
        next_id: 1,
        file_path: file_path
    }
    
    load_tasks(manager)
    damn manager
}

slay create_task(manager TaskManager, title tea, description tea) Task {
    sus now tea = timez.format_iso(timez.now())
    
    sus task Task = Task{
        id: manager.next_id,
        title: title,
        description: description,
        completed: cringe,
        created_at: now,
        updated_at: now
    }
    
    manager.next_id = manager.next_id + 1
    push(manager.tasks, task)
    save_tasks(manager)
    
    damn task
}

slay get_task(manager TaskManager, id drip) Task yikes<tea> {
    bestie (task in manager.tasks) {
        ready (task.id == id) {
            damn task
        }
    }
    yikes "Task not found"
}

slay update_task(manager TaskManager, id drip, title tea, description tea, completed lit) yikes<tea> {
    bestie (i, task in manager.tasks) {
        ready (task.id == id) {
            manager.tasks[i].title = title
            manager.tasks[i].description = description
            manager.tasks[i].completed = completed
            manager.tasks[i].updated_at = timez.format_iso(timez.now())
            save_tasks(manager)
            damn
        }
    }
    yikes "Task not found"
}

slay delete_task(manager TaskManager, id drip) yikes<tea> {
    bestie (i, task in manager.tasks) {
        ready (task.id == id) {
            remove_at(manager.tasks, i)
            save_tasks(manager)
            damn
        }
    }
    yikes "Task not found"
}

slay list_tasks(manager TaskManager) []Task {
    damn manager.tasks
}

slay save_tasks(manager TaskManager) {
    sus json_data tea = jsonz.stringify(manager.tasks, indent: 2)
    filez.write_file(manager.file_path, json_data) fam {
        when err -> vibez.spill("Error saving tasks:", err)
    }
}

slay load_tasks(manager TaskManager) {
    sus json_data tea = filez.read_file(manager.file_path) fam {
        when _ -> {
            # File doesn't exist, start with empty tasks
            damn
        }
    }
    
    manager.tasks = jsonz.parse_array(json_data, Task) fam {
        when err -> {
            vibez.spill("Error loading tasks:", err)
            manager.tasks = []
        }
    }
    
    # Update next_id based on existing tasks
    sus max_id drip = 0
    bestie (task in manager.tasks) {
        ready (task.id > max_id) {
            max_id = task.id
        }
    }
    manager.next_id = max_id + 1
}
```

## Step 3: HTML Templates

Create dynamic HTML templates:

```html
<!-- templates/index.html -->
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Task Manager</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <div class="container">
        <h1>🔥 CURSED Task Manager</h1>
        
        <form id="task-form" class="task-form">
            <input type="text" id="title" placeholder="Task title" required>
            <textarea id="description" placeholder="Task description"></textarea>
            <button type="submit">Add Task</button>
        </form>
        
        <div id="tasks" class="tasks">
            {{#each tasks}}
            <div class="task {{#if completed}}completed{{/if}}" data-id="{{id}}">
                <div class="task-header">
                    <h3>{{title}}</h3>
                    <div class="task-actions">
                        <button class="toggle-btn" onclick="toggleTask({{id}})">
                            {{#if completed}}✅{{else}}⭕{{/if}}
                        </button>
                        <button class="delete-btn" onclick="deleteTask({{id}})">🗑️</button>
                    </div>
                </div>
                <p>{{description}}</p>
                <small>Created: {{created_at}}</small>
            </div>
            {{/each}}
        </div>
    </div>
    
    <script src="/static/app.js"></script>
</body>
</html>
```

```css
/* static/style.css */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
    padding: 20px;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    background: white;
    border-radius: 10px;
    padding: 20px;
    box-shadow: 0 10px 30px rgba(0,0,0,0.2);
}

h1 {
    text-align: center;
    color: #333;
    margin-bottom: 30px;
}

.task-form {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 30px;
    padding: 20px;
    background: #f8f9fa;
    border-radius: 8px;
}

.task-form input,
.task-form textarea {
    padding: 12px;
    border: 2px solid #ddd;
    border-radius: 6px;
    font-size: 16px;
}

.task-form button {
    padding: 12px;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 16px;
    cursor: pointer;
    transition: background 0.3s;
}

.task-form button:hover {
    background: #5a6fd8;
}

.task {
    border: 2px solid #eee;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 15px;
    transition: all 0.3s;
}

.task:hover {
    border-color: #667eea;
    transform: translateY(-2px);
}

.task.completed {
    opacity: 0.7;
    background: #f0f8f0;
}

.task-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
}

.task-actions {
    display: flex;
    gap: 10px;
}

.toggle-btn,
.delete-btn {
    background: none;
    border: none;
    font-size: 20px;
    cursor: pointer;
    padding: 5px;
    border-radius: 4px;
    transition: background 0.3s;
}

.toggle-btn:hover {
    background: #e8f5e8;
}

.delete-btn:hover {
    background: #ffebee;
}
```

```javascript
// static/app.js
document.addEventListener('DOMContentLoaded', function() {
    const taskForm = document.getElementById('task-form');
    
    taskForm.addEventListener('submit', async function(e) {
        e.preventDefault();
        
        const title = document.getElementById('title').value;
        const description = document.getElementById('description').value;
        
        try {
            const response = await fetch('/api/tasks', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ title, description })
            });
            
            if (response.ok) {
                location.reload(); // Refresh page to show new task
            }
        } catch (error) {
            console.error('Error creating task:', error);
        }
    });
});

async function toggleTask(id) {
    try {
        const response = await fetch(`/api/tasks/${id}/toggle`, {
            method: 'POST'
        });
        
        if (response.ok) {
            location.reload();
        }
    } catch (error) {
        console.error('Error toggling task:', error);
    }
}

async function deleteTask(id) {
    if (confirm('Are you sure you want to delete this task?')) {
        try {
            const response = await fetch(`/api/tasks/${id}`, {
                method: 'DELETE'
            });
            
            if (response.ok) {
                location.reload();
            }
        } catch (error) {
            console.error('Error deleting task:', error);
        }
    }
}
```

## Step 4: Main Application

Bring everything together:

```cursed
# src/main.csd
yeet "vibez"
yeet "server"
yeet "tasks"
yeet "stringz"
yeet "jsonz"
yeet "filez"

sus task_manager TaskManager = new_task_manager("data/tasks.json")

# Template rendering
slay render_template(template_path tea, data map<tea, any>) tea yikes<tea> {
    sus template_content tea = filez.read_file(template_path) fam {
        when err -> yikes "Template not found: " + template_path
    }
    
    # Simple template engine (replace with handlebars-style)
    sus rendered tea = template_content
    
    # Replace {{#each tasks}} blocks
    ready (data.has_key("tasks")) {
        sus tasks_html tea = ""
        sus tasks_list []Task = data["tasks"]
        
        bestie (task in tasks_list) {
            sus task_html tea = `
            <div class="task ${task.completed ? "completed" : ""}" data-id="${task.id}">
                <div class="task-header">
                    <h3>${task.title}</h3>
                    <div class="task-actions">
                        <button class="toggle-btn" onclick="toggleTask(${task.id})">
                            ${task.completed ? "✅" : "⭕"}
                        </button>
                        <button class="delete-btn" onclick="deleteTask(${task.id})">🗑️</button>
                    </div>
                </div>
                <p>${task.description}</p>
                <small>Created: ${task.created_at}</small>
            </div>`
            
            tasks_html = tasks_html + task_html
        }
        
        rendered = stringz.replace(rendered, "{{#each tasks}}.*{{/each}}", tasks_html)
    }
    
    damn rendered
}

# Route handlers
slay home_handler(request Request) Response {
    sus tasks []Task = list_tasks(task_manager)
    sus data map<tea, any> = {"tasks": tasks}
    
    sus html tea = render_template("templates/index.html", data) fam {
        when err -> {
            damn Response{
                status: 500,
                headers: make_map(),
                body: "Error rendering template: " + err
            }
        }
    }
    
    damn Response{
        status: 200,
        headers: {"Content-Type": "text/html"},
        body: html
    }
}

slay static_handler(request Request) Response {
    sus file_path tea = "." + request.path
    sus content tea = filez.read_file(file_path) fam {
        when _ -> {
            damn Response{
                status: 404,
                headers: make_map(),
                body: "File not found"
            }
        }
    }
    
    sus content_type tea = ready (stringz.ends_with(request.path, ".css")) {
        damn "text/css"
    } otherwise ready (stringz.ends_with(request.path, ".js")) {
        damn "application/javascript"
    } otherwise {
        damn "text/plain"
    }
    
    damn Response{
        status: 200,
        headers: {"Content-Type": content_type},
        body: content
    }
}

slay api_create_task(request Request) Response {
    sus data map<tea, any> = jsonz.parse(request.body) fam {
        when err -> {
            damn Response{
                status: 400,
                headers: {"Content-Type": "application/json"},
                body: jsonz.stringify({"error": "Invalid JSON"})
            }
        }
    }
    
    sus title tea = data["title"]
    sus description tea = data.get("description", "")
    
    sus task Task = create_task(task_manager, title, description)
    
    damn Response{
        status: 201,
        headers: {"Content-Type": "application/json"},
        body: jsonz.stringify(task)
    }
}

slay api_toggle_task(request Request) Response {
    sus path_parts []tea = stringz.split(request.path, "/")
    sus id_str tea = path_parts[3]
    sus id drip = parse_int(id_str) fam {
        when err -> {
            damn Response{
                status: 400,
                headers: {"Content-Type": "application/json"},
                body: jsonz.stringify({"error": "Invalid task ID"})
            }
        }
    }
    
    sus task Task = get_task(task_manager, id) fam {
        when err -> {
            damn Response{
                status: 404,
                headers: {"Content-Type": "application/json"},
                body: jsonz.stringify({"error": "Task not found"})
            }
        }
    }
    
    update_task(task_manager, id, task.title, task.description, !task.completed) fam {
        when err -> {
            damn Response{
                status: 500,
                headers: {"Content-Type": "application/json"},
                body: jsonz.stringify({"error": "Failed to update task"})
            }
        }
    }
    
    damn Response{
        status: 200,
        headers: {"Content-Type": "application/json"},
        body: jsonz.stringify({"success": based})
    }
}

slay api_delete_task(request Request) Response {
    sus path_parts []tea = stringz.split(request.path, "/")
    sus id_str tea = path_parts[3]
    sus id drip = parse_int(id_str) fam {
        when err -> {
            damn Response{
                status: 400,
                headers: {"Content-Type": "application/json"},
                body: jsonz.stringify({"error": "Invalid task ID"})
            }
        }
    }
    
    delete_task(task_manager, id) fam {
        when err -> {
            damn Response{
                status: 404,
                headers: {"Content-Type": "application/json"},
                body: jsonz.stringify({"error": "Task not found"})
            }
        }
    }
    
    damn Response{
        status: 200,
        headers: {"Content-Type": "application/json"},
        body: jsonz.stringify({"success": based})
    }
}

# Main application
slay main() yikes<tea> {
    vibez.spill("🔥 Starting CURSED Task Manager...")
    
    # Ensure data directory exists
    filez.create_dir("data") fam { when _ -> { } }
    
    sus server Server = new_server(8080)
    
    # Register routes
    add_route(server, "GET", "/", home_handler)
    add_route(server, "GET", "/static/*", static_handler)
    add_route(server, "POST", "/api/tasks", api_create_task)
    add_route(server, "POST", "/api/tasks/*/toggle", api_toggle_task)
    add_route(server, "DELETE", "/api/tasks/*", api_delete_task)
    
    vibez.spill("Task Manager ready! 🚀")
    vibez.spill("Open http://localhost:8080 in your browser")
    
    start_server(server) fam {
        when err -> yikes "Server error: " + err
    }
}

main() fam {
    when err -> vibez.spill("Application error:", err)
}
```

## Step 5: Build & Run

```bash
# Build the application
cursed-zig --compile src/main.csd -o taskman

# Run the server
./taskman
```

**Open your browser to `http://localhost:8080`** and see your CURSED web app in action! 🎉

## Features Demonstrated

✅ **HTTP Server**: Custom web server with routing  
✅ **JSON API**: RESTful endpoints for CRUD operations  
✅ **Template Rendering**: Dynamic HTML generation  
✅ **Static Files**: CSS and JavaScript serving  
✅ **Error Handling**: Graceful error management  
✅ **File I/O**: JSON database persistence  
✅ **Concurrency**: Goroutines for connection handling  

## Next Steps

**Enhance your app:**
- [Add user authentication](./04-authentication.md)
- [Connect to PostgreSQL](./05-database.md)
- [Deploy to production](./06-deployment.md)
- [Add real-time WebSocket updates](./07-websockets.md)

**Explore more tutorials:**
- [Systems Programming](./systems/)
- [Game Development](./game-dev/)
- [CLI Tools](./cli-tools/)

🔥 **You've built a complete web application in CURSED!** Ready for the next challenge?
