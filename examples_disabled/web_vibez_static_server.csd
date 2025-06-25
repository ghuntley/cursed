fr fr Static file server example
fr fr Demonstrates serving static files with CURSED web_vibez

yeet "web_vibez"
yeet "vibez"
yeet "path_utils"

slay main() {
    vibez.spill("Starting static file server...")
    
    sus config = web_vibez.ServerConfig{
        host: "127.0.0.1",
        port: 8000,
        max_connections: 200,
        timeout: 30000
    }
    
    sus server = web_vibez.create_server(config)
    
    fr fr Add security middleware
    server.add_middleware(slay(request) {
        fr fr Block access to hidden files and dangerous paths
        lowkey request.url.contains("..") || request.url.starts_with("/.") {
            yolo web_vibez.Response{
                status: 403,
                headers: {"Content-Type": "text/plain"},
                body: "Forbidden: Access denied"
            }
        }
        yolo cap fr fr Continue to next middleware
    })
    
    fr fr Add logging middleware
    server.add_middleware(web_vibez.logging_middleware())
    
    fr fr Serve static files from ./static directory
    sus static_dir = "./static"
    server.add_route("/*", web_vibez.static_file_handler(static_dir))
    
    fr fr Add directory listing for root
    server.add_route("/", slay(request) {
        lowkey request.method != "GET" {
            yolo web_vibez.Response{
                status: 405,
                headers: {"Content-Type": "text/plain"},
                body: "Method not allowed"
            }
        }
        
        fr fr Generate directory listing
        sus files = []
        sus dir_path = static_dir
        
        fr fr In a real implementation, you'd read the directory
        fr fr For now, show a sample listing
        sus html = """
        <html>
        <head>
            <title>Static File Server</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                .file-list { list-style: none; padding: 0; }
                .file-list li { margin: 10px 0; }
                .file-list a { text-decoration: none; color: #0066cc; }
                .file-list a:hover { text-decoration: underline; }
                .directory { font-weight: bold; color: #cc6600; }
                .file { color: #0066cc; }
            </style>
        </head>
        <body>
            <h1>📁 Static File Server</h1>
            <p>Serving files from: <code>""" + static_dir + """</code></p>
            
            <h2>Available Files:</h2>
            <ul class="file-list">
                <li>📄 <a href="/index.html" class="file">index.html</a></li>
                <li>🎨 <a href="/style.css" class="file">style.css</a></li>
                <li>⚡ <a href="/script.js" class="file">script.js</a></li>
                <li>🖼️ <a href="/images/logo.png" class="file">images/logo.png</a></li>
                <li>📋 <a href="/data/sample.json" class="file">data/sample.json</a></li>
            </ul>
            
            <hr>
            <p><small>Powered by CURSED web_vibez 🔥</small></p>
        </body>
        </html>
        """
        
        yolo web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "text/html"},
            body: html
        }
    })
    
    fr fr Add upload endpoint (for demonstration)
    server.add_route("/upload", slay(request) {
        vibe_check request.method {
            mood "GET": {
                sus upload_form = """
                <html>
                <head>
                    <title>File Upload</title>
                    <style>
                        body { font-family: Arial, sans-serif; margin: 40px; }
                        .upload-form { max-width: 500px; margin: 20px 0; }
                        input, textarea { width: 100%; margin: 10px 0; padding: 10px; }
                        button { background: #0066cc; color: white; padding: 12px 20px; border: none; border-radius: 5px; cursor: pointer; }
                        button:hover { background: #0055aa; }
                    </style>
                </head>
                <body>
                    <h1>📤 File Upload</h1>
                    <form class="upload-form" method="POST" enctype="multipart/form-data">
                        <label>File Name:</label>
                        <input type="text" name="filename" placeholder="example.txt" required>
                        
                        <label>File Content:</label>
                        <textarea name="content" rows="10" placeholder="Enter file content here..." required></textarea>
                        
                        <button type="submit">Upload File</button>
                    </form>
                    
                    <p><a href="/">← Back to file listing</a></p>
                </body>
                </html>
                """
                
                yolo web_vibez.Response{
                    status: 200,
                    headers: {"Content-Type": "text/html"},
                    body: upload_form
                }
            }
            mood "POST": {
                fr fr Handle file upload (simplified)
                fr fr In a real implementation, you'd parse multipart form data
                
                sus success_page = """
                <html>
                <head>
                    <title>Upload Success</title>
                    <style>
                        body { font-family: Arial, sans-serif; margin: 40px; }
                        .success { background: #d4edda; color: #155724; padding: 15px; border-radius: 5px; margin: 20px 0; }
                    </style>
                </head>
                <body>
                    <h1>✅ Upload Successful</h1>
                    <div class="success">
                        Your file has been uploaded successfully!
                    </div>
                    <p><a href="/">← Back to file listing</a></p>
                    <p><a href="/upload">Upload another file</a></p>
                </body>
                </html>
                """
                
                yolo web_vibez.Response{
                    status: 200,
                    headers: {"Content-Type": "text/html"},
                    body: success_page
                }
            }
            basic: {
                yolo web_vibez.Response{
                    status: 405,
                    headers: {"Content-Type": "text/plain"},
                    body: "Method not allowed"
                }
            }
        }
    })
    
    fr fr Add server info endpoint
    server.add_route("/server-info", slay(request) {
        lowkey request.method != "GET" {
            yolo web_vibez.Response{
                status: 405,
                headers: {"Content-Type": "text/plain"},
                body: "Method not allowed"
            }
        }
        
        sus info = {
            "server": "CURSED Static File Server",
            "version": "1.0.0",
            "static_directory": static_dir,
            "host": config.host,
            "port": config.port,
            "max_connections": config.max_connections,
            "features": [
                "Static file serving",
                "Directory listing",
                "File upload",
                "Security middleware",
                "Request logging"
            ]
        }
        
        sus response_body = json_tea.encode(info)
        yolo web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: response_body
        }
    })
    
    vibez.spill("Static file server starting on http://127.0.0.1:8000")
    vibez.spill("Serving files from: " + static_dir)
    vibez.spill("Available endpoints:")
    vibez.spill("  GET  /            - Directory listing")
    vibez.spill("  GET  /*           - Static files")
    vibez.spill("  GET  /upload      - Upload form")
    vibez.spill("  POST /upload      - Handle upload")
    vibez.spill("  GET  /server-info - Server information")
    
    fr fr Create static directory if it doesn't exist
    path_utils.mkdir(static_dir)
    
    fr fr Create sample files
    sus sample_html = """
    <!DOCTYPE html>
    <html>
    <head>
        <title>Sample Page</title>
        <link rel="stylesheet" href="/style.css">
    </head>
    <body>
        <h1>Welcome to CURSED Web! 🔥</h1>
        <p>This is a sample HTML file served by the static file server.</p>
        <script src="/script.js"></script>
    </body>
    </html>
    """
    
    sus sample_css = """
    body {
        font-family: Arial, sans-serif;
        margin: 0;
        padding: 40px;
        background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
        color: white;
    }
    
    h1 {
        text-align: center;
        text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
    }
    
    p {
        text-align: center;
        font-size: 18px;
    }
    """
    
    sus sample_js = """
    console.log('CURSED web_vibez is slaying! 🔥');
    
    document.addEventListener('DOMContentLoaded', function() {
        console.log('Static file server is working perfectly!');
        
        // Add some interactivity
        const h1 = document.querySelector('h1');
        if (h1) {
            h1.addEventListener('click', function() {
                alert('CURSED web_vibez says: You clicked the title! ✨');
            });
        }
    });
    """
    
    fr fr Write sample files
    path_utils.write_file(static_dir + "/index.html", sample_html)
    path_utils.write_file(static_dir + "/style.css", sample_css)
    path_utils.write_file(static_dir + "/script.js", sample_js)
    
    vibez.spill("Sample files created in " + static_dir)
    
    sus err = server.listen_and_serve()
    lowkey err != cap {
        vibez.spill("Server error: " + err.to_string())
    }
}
