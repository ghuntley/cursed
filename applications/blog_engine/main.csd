fr fr BLOG ENGINE - Complete Content Management System
fr fr Features: Posts, categories, tags, comments, static site generation, templates

yeet "database_enhanced"
yeet "webz"
yeet "json"
yeet "stringz"
yeet "timez"
yeet "fs"
yeet "vibez"

fr fr ===== APPLICATION CONFIGURATION =====

squad BlogConfig {
    sus database_url tea
    sus server_port drip
    sus web_root tea
    sus content_path tea
    sus template_path tea
    sus static_path tea
    sus site_title tea
    sus site_description tea
    sus author_name tea
}

sus blog_config BlogConfig = BlogConfig{
    database_url: "file://./blog_data",
    server_port: 3000,
    web_root: "./public",
    content_path: "./content",
    template_path: "./templates",
    static_path: "./static",
    site_title: "My CURSED Blog",
    site_description: "A blog engine built with CURSED programming language",
    author_name: "CURSED Developer"
}

fr fr ===== DATA MODELS =====

squad Post {
    sus id drip
    sus title tea
    sus slug tea
    sus content tea
    sus excerpt tea
    sus author tea
    sus status tea
    sus category_id drip
    sus tags tea
    sus featured_image tea
    sus published_at tea
    sus created_at tea
    sus updated_at tea
    sus view_count drip
}

squad Category {
    sus id drip
    sus name tea
    sus slug tea
    sus description tea
    sus color tea
    sus post_count drip
    sus created_at tea
}

squad Comment {
    sus id drip
    sus post_id drip
    sus author_name tea
    sus author_email tea
    sus content tea
    sus status tea
    sus created_at tea
    sus parent_id drip
}

squad Tag {
    sus id drip
    sus name tea
    sus slug tea
    sus post_count drip
    sus created_at tea
}

fr fr ===== DATABASE INITIALIZATION =====

slay initialize_blog_database() database_enhanced.DatabaseConnection {
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(blog_config.database_url)
    
    ready (!conn.is_connected) {
        vibez.spill("FATAL: Could not connect to blog database")
        sus empty database_enhanced.DatabaseConnection = database_enhanced.DatabaseConnection{}
        damn empty
    }
    
    fr fr Create all tables
    create_posts_table(conn)
    create_categories_table(conn)
    create_comments_table(conn)
    create_tags_table(conn)
    create_post_tags_table(conn)
    
    fr fr Insert default data
    insert_default_blog_data(conn)
    
    vibez.spill("Blog database initialized successfully")
    damn conn
}

slay create_posts_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "title": "TEXT NOT NULL",
        "slug": "TEXT UNIQUE NOT NULL",
        "content": "TEXT NOT NULL",
        "excerpt": "TEXT",
        "author": "TEXT NOT NULL",
        "status": "TEXT DEFAULT 'draft'",
        "category_id": "INTEGER",
        "tags": "TEXT",
        "featured_image": "TEXT",
        "published_at": "TEXT",
        "created_at": "TEXT NOT NULL",
        "updated_at": "TEXT NOT NULL",
        "view_count": "INTEGER DEFAULT 0"
    })
    
    ready (database_enhanced.create_table(conn, "posts", schema)) {
        vibez.spill("Created posts table")
    }
}

slay create_categories_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "name": "TEXT NOT NULL UNIQUE",
        "slug": "TEXT UNIQUE NOT NULL",
        "description": "TEXT",
        "color": "TEXT DEFAULT '#007bff'",
        "post_count": "INTEGER DEFAULT 0",
        "created_at": "TEXT NOT NULL"
    })
    
    ready (database_enhanced.create_table(conn, "categories", schema)) {
        vibez.spill("Created categories table")
    }
}

slay create_comments_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "post_id": "INTEGER NOT NULL",
        "author_name": "TEXT NOT NULL",
        "author_email": "TEXT NOT NULL",
        "content": "TEXT NOT NULL",
        "status": "TEXT DEFAULT 'pending'",
        "created_at": "TEXT NOT NULL",
        "parent_id": "INTEGER DEFAULT 0"
    })
    
    ready (database_enhanced.create_table(conn, "comments", schema)) {
        vibez.spill("Created comments table")
    }
}

slay create_tags_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "name": "TEXT NOT NULL UNIQUE",
        "slug": "TEXT UNIQUE NOT NULL",
        "post_count": "INTEGER DEFAULT 0",
        "created_at": "TEXT NOT NULL"
    })
    
    ready (database_enhanced.create_table(conn, "tags", schema)) {
        vibez.spill("Created tags table")
    }
}

slay create_post_tags_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "post_id": "INTEGER NOT NULL",
        "tag_id": "INTEGER NOT NULL",
        "created_at": "TEXT NOT NULL"
    })
    
    ready (database_enhanced.create_table(conn, "post_tags", schema)) {
        vibez.spill("Created post_tags table")
    }
}

slay insert_default_blog_data(conn database_enhanced.DatabaseConnection) {
    sus now tea = timez.format_iso8601(timez.now_millis())
    
    fr fr Default categories
    sus default_categories []tea = [
        json.object_to_string({
            "name": "Technology",
            "slug": "technology",
            "description": "Latest tech news and tutorials",
            "color": "#007bff",
            "post_count": "0",
            "created_at": now
        }),
        json.object_to_string({
            "name": "Programming",
            "slug": "programming", 
            "description": "Coding tips and best practices",
            "color": "#28a745",
            "post_count": "0",
            "created_at": now
        }),
        json.object_to_string({
            "name": "CURSED Language",
            "slug": "cursed-language",
            "description": "All about the CURSED programming language",
            "color": "#dc3545",
            "post_count": "0",
            "created_at": now
        })
    ]
    
    sus i drip = 0
    bestie (i < default_categories.length) {
        database_enhanced.insert_record(conn, "categories", default_categories[i])
        i = i + 1
    }
    
    fr fr Welcome post
    sus welcome_post tea = json.object_to_string({
        "title": "Welcome to My CURSED Blog",
        "slug": "welcome-to-my-cursed-blog",
        "content": `# Welcome to My CURSED Blog

This is the first post on my blog built with the CURSED programming language! 

## Features

- **Content Management**: Create, edit, and manage blog posts
- **Categories & Tags**: Organize your content effectively  
- **Comments System**: Engage with your readers
- **Static Site Generation**: Generate static HTML files
- **Template System**: Customize your blog's appearance

## Getting Started

You can create new posts through the admin interface or by using the API endpoints.

Happy blogging with CURSED! 🚀`,
        "excerpt": "Welcome to my new blog built with CURSED programming language!",
        "author": blog_config.author_name,
        "status": "published",
        "category_id": "3",
        "tags": "cursed,welcome,blog,programming",
        "featured_image": "",
        "published_at": now,
        "created_at": now,
        "updated_at": now,
        "view_count": "0"
    })
    
    database_enhanced.insert_record(conn, "posts", welcome_post)
    
    vibez.spill("Inserted default blog data")
}

fr fr ===== CONTENT MANAGEMENT =====

slay create_post(conn database_enhanced.DatabaseConnection, title tea, content tea, author tea, category_id drip, tags tea) drip {
    sus now tea = timez.format_iso8601(timez.now_millis())
    sus slug tea = create_slug(title)
    sus excerpt tea = generate_excerpt(content)
    
    sus post_data tea = json.object_to_string({
        "title": title,
        "slug": slug,
        "content": content,
        "excerpt": excerpt,
        "author": author,
        "status": "draft",
        "category_id": stringz.from_int(category_id),
        "tags": tags,
        "featured_image": "",
        "published_at": "",
        "created_at": now,
        "updated_at": now,
        "view_count": "0"
    })
    
    ready (database_enhanced.insert_record(conn, "posts", post_data)) {
        vibez.spill("Created post: " + title)
        damn mathz.random_int(10000)
    }
    
    damn 0
}

slay publish_post(conn database_enhanced.DatabaseConnection, id drip) lit {
    sus now tea = timez.format_iso8601(timez.now_millis())
    sus updates tea = json.object_to_string({
        "status": "published",
        "published_at": now,
        "updated_at": now
    })
    
    ready (database_enhanced.update_record(conn, "posts", id, updates)) {
        vibez.spill("Published post ID: " + stringz.from_int(id))
        damn based
    }
    
    damn cringe
}

slay get_published_posts(conn database_enhanced.DatabaseConnection) []tea {
    sus conditions tea = json.object_to_string({
        "status": "published"
    })
    
    sus posts []tea = database_enhanced.find_records(conn, "posts", conditions)
    vibez.spill("Retrieved " + stringz.from_int(posts.length) + " published posts")
    damn posts
}

slay get_posts_by_category(conn database_enhanced.DatabaseConnection, category_slug tea) []tea {
    fr fr First find category ID
    sus category_conditions tea = json.object_to_string({
        "slug": category_slug
    })
    
    sus categories []tea = database_enhanced.find_records(conn, "categories", category_conditions)
    ready (categories.length == 0) {
        sus empty []tea = []
        damn empty
    }
    
    sus category_data map[tea]tea = json.parse_object(categories[0])
    sus category_id tea = category_data["id"]
    
    fr fr Find posts for this category
    sus post_conditions tea = json.object_to_string({
        "category_id": category_id,
        "status": "published"
    })
    
    damn database_enhanced.find_records(conn, "posts", post_conditions)
}

slay get_post_by_slug(conn database_enhanced.DatabaseConnection, slug tea) tea {
    sus conditions tea = json.object_to_string({
        "slug": slug
    })
    
    sus posts []tea = database_enhanced.find_records(conn, "posts", conditions)
    ready (posts.length > 0) {
        fr fr Increment view count
        sus post_data map[tea]tea = json.parse_object(posts[0])
        sus current_views drip = stringz.to_int(post_data["view_count"])
        sus id drip = stringz.to_int(post_data["id"])
        
        sus view_update tea = json.object_to_string({
            "view_count": stringz.from_int(current_views + 1)
        })
        
        database_enhanced.update_record(conn, "posts", id, view_update)
        damn posts[0]
    }
    
    damn ""
}

fr fr ===== COMMENT SYSTEM =====

slay add_comment(conn database_enhanced.DatabaseConnection, post_id drip, author_name tea, author_email tea, content tea, parent_id drip) drip {
    sus now tea = timez.format_iso8601(timez.now_millis())
    
    sus comment_data tea = json.object_to_string({
        "post_id": stringz.from_int(post_id),
        "author_name": author_name,
        "author_email": author_email,
        "content": content,
        "status": "pending",
        "created_at": now,
        "parent_id": stringz.from_int(parent_id)
    })
    
    ready (database_enhanced.insert_record(conn, "comments", comment_data)) {
        vibez.spill("Added comment from: " + author_name)
        damn mathz.random_int(10000)
    }
    
    damn 0
}

slay get_approved_comments(conn database_enhanced.DatabaseConnection, post_id drip) []tea {
    sus conditions tea = json.object_to_string({
        "post_id": stringz.from_int(post_id),
        "status": "approved"
    })
    
    damn database_enhanced.find_records(conn, "comments", conditions)
}

slay approve_comment(conn database_enhanced.DatabaseConnection, comment_id drip) lit {
    sus updates tea = json.object_to_string({
        "status": "approved"
    })
    
    damn database_enhanced.update_record(conn, "comments", comment_id, updates)
}

fr fr ===== TEMPLATE SYSTEM =====

slay render_blog_template(template_name tea, data map[tea]tea) tea {
    sus template_path tea = blog_config.template_path + "/" + template_name + ".html"
    
    ready (!fs.file_exists(template_path)) {
        damn get_default_template(template_name, data)
    }
    
    sus template_content tea = fs.read_file(template_path)
    damn process_template(template_content, data)
}

slay process_template(template tea, data map[tea]tea) tea {
    sus result tea = template
    
    fr fr Replace template variables {{variable}}
    bestie key, value := range data {
        sus placeholder tea = "{{" + key + "}}"
        result = stringz.replace_all(result, placeholder, value)
    }
    
    fr fr Replace common site variables
    result = stringz.replace_all(result, "{{SITE_TITLE}}", blog_config.site_title)
    result = stringz.replace_all(result, "{{SITE_DESCRIPTION}}", blog_config.site_description)
    result = stringz.replace_all(result, "{{AUTHOR_NAME}}", blog_config.author_name)
    result = stringz.replace_all(result, "{{CURRENT_YEAR}}", stringz.from_int(2024))
    
    damn result
}

slay get_default_template(template_name tea, data map[tea]tea) tea {
    ready (template_name == "post") {
        damn get_default_post_template(data)
    } otherwise ready (template_name == "index") {
        damn get_default_index_template(data)
    } otherwise ready (template_name == "category") {
        damn get_default_category_template(data)
    }
    
    damn get_default_base_template(data)
}

slay get_default_post_template(data map[tea]tea) tea {
    damn `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}} - {{SITE_TITLE}}</title>
    <meta name="description" content="{{excerpt}}">
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; max-width: 800px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; border-bottom: 1px solid #eee; padding-bottom: 20px; margin-bottom: 30px; }
        .nav { text-align: center; margin: 20px 0; }
        .nav a { margin: 0 15px; text-decoration: none; color: #007bff; }
        .post-meta { color: #666; font-size: 0.9em; margin-bottom: 20px; }
        .post-content { line-height: 1.8; }
        .post-content h1, .post-content h2, .post-content h3 { color: #2c3e50; margin-top: 30px; }
        .post-content pre { background: #f8f9fa; padding: 15px; border-radius: 5px; overflow-x: auto; }
        .post-content blockquote { border-left: 4px solid #007bff; margin: 0; padding-left: 20px; color: #666; }
        .comments { margin-top: 40px; border-top: 1px solid #eee; padding-top: 30px; }
        .comment { background: #f8f9fa; padding: 15px; margin: 10px 0; border-radius: 5px; }
        .comment-form { background: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }
        .comment-form input, .comment-form textarea { width: 100%; padding: 10px; margin: 5px 0; border: 1px solid #ddd; border-radius: 3px; }
        .btn { background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 3px; cursor: pointer; }
        .btn:hover { background: #0056b3; }
        .category-badge { background: {{category_color}}; color: white; padding: 2px 8px; border-radius: 12px; font-size: 0.8em; text-decoration: none; }
    </style>
</head>
<body>
    <div class="header">
        <h1>{{SITE_TITLE}}</h1>
        <p>{{SITE_DESCRIPTION}}</p>
    </div>
    
    <div class="nav">
        <a href="/">Home</a>
        <a href="/blog">Blog</a>
        <a href="/categories">Categories</a>
        <a href="/admin">Admin</a>
    </div>
    
    <article>
        <header>
            <h1>{{title}}</h1>
            <div class="post-meta">
                Published on {{published_at}} by {{author}}
                <span class="category-badge">{{category_name}}</span>
                | {{view_count}} views
            </div>
        </header>
        
        <div class="post-content">
            {{content}}
        </div>
        
        <div class="post-tags">
            <strong>Tags:</strong> {{tags}}
        </div>
    </article>
    
    <div class="comments">
        <h3>Comments</h3>
        {{comments_html}}
        
        <div class="comment-form">
            <h4>Leave a Comment</h4>
            <form method="POST" action="/comments">
                <input type="hidden" name="post_id" value="{{id}}">
                <input type="text" name="author_name" placeholder="Your Name" required>
                <input type="email" name="author_email" placeholder="Your Email" required>
                <textarea name="content" placeholder="Your Comment" rows="4" required></textarea>
                <button type="submit" class="btn">Submit Comment</button>
            </form>
        </div>
    </div>
</body>
</html>`
}

slay get_default_index_template(data map[tea]tea) tea {
    damn `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{SITE_TITLE}}</title>
    <meta name="description" content="{{SITE_DESCRIPTION}}">
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; max-width: 1000px; margin: 0 auto; padding: 20px; background: #f8f9fa; }
        .header { text-align: center; background: white; padding: 40px 20px; border-radius: 10px; margin-bottom: 30px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .nav { text-align: center; margin: 20px 0; }
        .nav a { margin: 0 15px; text-decoration: none; color: #007bff; font-weight: 500; }
        .posts { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .post-card { background: white; padding: 25px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); transition: transform 0.2s; }
        .post-card:hover { transform: translateY(-2px); }
        .post-card h2 { margin: 0 0 10px 0; color: #2c3e50; }
        .post-card h2 a { text-decoration: none; color: inherit; }
        .post-card h2 a:hover { color: #007bff; }
        .post-meta { color: #666; font-size: 0.9em; margin-bottom: 15px; }
        .post-excerpt { color: #555; margin-bottom: 15px; }
        .read-more { color: #007bff; text-decoration: none; font-weight: 500; }
        .sidebar { background: white; padding: 25px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .main-content { display: grid; grid-template-columns: 2fr 1fr; gap: 30px; }
        .category-list { list-style: none; padding: 0; }
        .category-list li { margin: 8px 0; }
        .category-list a { text-decoration: none; color: #333; }
        .category-badge { background: #007bff; color: white; padding: 2px 8px; border-radius: 12px; font-size: 0.8em; margin-left: 5px; }
        @media (max-width: 768px) { .main-content { grid-template-columns: 1fr; } }
    </style>
</head>
<body>
    <div class="header">
        <h1>{{SITE_TITLE}}</h1>
        <p>{{SITE_DESCRIPTION}}</p>
        <p>Built with ❤️ using CURSED programming language</p>
    </div>
    
    <div class="nav">
        <a href="/">Home</a>
        <a href="/blog">Blog</a>
        <a href="/categories">Categories</a>
        <a href="/admin">Admin</a>
    </div>
    
    <div class="main-content">
        <main>
            <h2>Latest Posts</h2>
            <div class="posts">
                {{posts_html}}
            </div>
        </main>
        
        <aside class="sidebar">
            <h3>Categories</h3>
            {{categories_html}}
            
            <h3>Recent Posts</h3>
            {{recent_posts_html}}
        </aside>
    </div>
    
    <footer style="text-align: center; margin: 40px 0; color: #666;">
        <p>&copy; {{CURRENT_YEAR}} {{AUTHOR_NAME}}. Powered by CURSED Blog Engine.</p>
    </footer>
</body>
</html>`
}

fr fr ===== STATIC SITE GENERATION =====

slay generate_static_site(conn database_enhanced.DatabaseConnection) {
    vibez.spill("Generating static site...")
    
    fr fr Create public directory structure
    create_public_directories()
    
    fr fr Generate index page
    generate_index_page(conn)
    
    fr fr Generate all post pages
    generate_post_pages(conn)
    
    fr fr Generate category pages
    generate_category_pages(conn)
    
    fr fr Copy static assets
    copy_static_assets()
    
    vibez.spill("Static site generation completed!")
}

slay create_public_directories() {
    sus directories []tea = [
        blog_config.web_root,
        blog_config.web_root + "/posts",
        blog_config.web_root + "/categories",
        blog_config.web_root + "/static"
    ]
    
    sus i drip = 0
    bestie (i < directories.length) {
        ready (!fs.directory_exists(directories[i])) {
            fs.create_directory(directories[i])
        }
        i = i + 1
    }
}

slay generate_index_page(conn database_enhanced.DatabaseConnection) {
    sus posts []tea = get_published_posts(conn)
    sus categories []tea = database_enhanced.find_records(conn, "categories", "{}")
    
    sus posts_html tea = generate_posts_cards_html(posts)
    sus categories_html tea = generate_sidebar_categories_html(categories)
    
    sus data map[tea]tea = {
        "posts_html": posts_html,
        "categories_html": categories_html,
        "recent_posts_html": generate_recent_posts_html(posts)
    }
    
    sus html tea = render_blog_template("index", data)
    fs.write_file(blog_config.web_root + "/index.html", html)
    vibez.spill("Generated index page")
}

slay generate_post_pages(conn database_enhanced.DatabaseConnection) {
    sus posts []tea = get_published_posts(conn)
    
    sus i drip = 0
    bestie (i < posts.length) {
        sus post_data map[tea]tea = json.parse_object(posts[i])
        sus slug tea = post_data["slug"]
        
        fr fr Get comments for this post
        sus post_id drip = stringz.to_int(post_data["id"])
        sus comments []tea = get_approved_comments(conn, post_id)
        sus comments_html tea = generate_comments_html(comments)
        
        fr fr Get category info
        sus category_id drip = stringz.to_int(post_data["category_id"])
        sus category_info tea = get_category_by_id(conn, category_id)
        
        sus template_data map[tea]tea = post_data
        template_data["comments_html"] = comments_html
        template_data["category_name"] = get_category_name_from_info(category_info)
        template_data["category_color"] = get_category_color_from_info(category_info)
        
        sus html tea = render_blog_template("post", template_data)
        sus file_path tea = blog_config.web_root + "/posts/" + slug + ".html"
        fs.write_file(file_path, html)
        
        i = i + 1
    }
    
    vibez.spill("Generated " + stringz.from_int(posts.length) + " post pages")
}

fr fr ===== WEB API HANDLERS =====

slay handle_blog_api_request(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"}
    
    ready (request.method == "GET" && request.path == "/api/posts") {
        damn handle_get_posts(conn, request)
    } otherwise ready (request.method == "POST" && request.path == "/api/posts") {
        damn handle_create_post(conn, request)
    } otherwise ready (request.method == "GET" && stringz.starts_with(request.path, "/api/posts/")) {
        damn handle_get_post(conn, request)
    } otherwise ready (request.method == "POST" && request.path == "/api/comments") {
        damn handle_create_comment(conn, request)
    } otherwise ready (request.method == "POST" && request.path == "/api/generate-site") {
        damn handle_generate_site(conn, request)
    } otherwise {
        response.status_code = 404
        response.body = json.object_to_string({"error": "API endpoint not found"})
        damn response
    }
}

slay handle_get_posts(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    response.status_code = 200
    
    sus posts []tea = get_published_posts(conn)
    sus posts_json tea = json.array_to_string(posts)
    response.body = json.object_to_string({
        "posts": posts_json,
        "count": stringz.from_int(posts.length)
    })
    
    damn response
}

slay handle_create_post(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    sus post_data map[tea]tea = json.parse_object(request.body)
    
    ready (post_data["title"] == "" || post_data["content"] == "") {
        response.status_code = 400
        response.body = json.object_to_string({"error": "Title and content are required"})
        damn response
    }
    
    sus category_id drip = stringz.to_int(post_data["category_id"])
    sus id drip = create_post(conn,
        post_data["title"],
        post_data["content"],
        post_data["author"],
        category_id,
        post_data["tags"]
    )
    
    ready (id > 0) {
        response.status_code = 201
        response.body = json.object_to_string({
            "id": stringz.from_int(id),
            "message": "Post created successfully"
        })
    } otherwise {
        response.status_code = 500
        response.body = json.object_to_string({"error": "Failed to create post"})
    }
    
    damn response
}

slay handle_generate_site(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    generate_static_site(conn)
    
    response.status_code = 200
    response.body = json.object_to_string({
        "message": "Static site generated successfully",
        "output_directory": blog_config.web_root
    })
    
    damn response
}

fr fr ===== UTILITY FUNCTIONS =====

slay create_slug(title tea) tea {
    sus slug tea = stringz.to_lower(title)
    slug = stringz.replace_all(slug, " ", "-")
    slug = stringz.replace_all(slug, "'", "")
    slug = stringz.replace_all(slug, "\"", "")
    slug = stringz.replace_all(slug, ".", "")
    slug = stringz.replace_all(slug, ",", "")
    slug = stringz.replace_all(slug, "!", "")
    slug = stringz.replace_all(slug, "?", "")
    damn slug
}

slay generate_excerpt(content tea, max_length drip) tea {
    ready (max_length == 0) {
        max_length = 150
    }
    
    ready (stringz.length(content) <= max_length) {
        damn content
    }
    
    sus excerpt tea = stringz.substring(content, 0, max_length)
    sus last_space drip = stringz.last_index_of(excerpt, " ")
    
    ready (last_space > 0) {
        excerpt = stringz.substring(excerpt, 0, last_space)
    }
    
    damn excerpt + "..."
}

slay generate_posts_cards_html(posts []tea) tea {
    sus html tea = ""
    
    ready (posts.length == 0) {
        damn `<div class="post-card">
            <h3>No posts yet</h3>
            <p>Check back soon for new content!</p>
        </div>`
    }
    
    sus i drip = 0
    bestie (i < posts.length) {
        sus post_data map[tea]tea = json.parse_object(posts[i])
        
        html = html + `<div class="post-card">
            <h2><a href="/posts/` + post_data["slug"] + `.html">` + post_data["title"] + `</a></h2>
            <div class="post-meta">
                Published ` + post_data["published_at"] + ` by ` + post_data["author"] + `
            </div>
            <div class="post-excerpt">` + post_data["excerpt"] + `</div>
            <a href="/posts/` + post_data["slug"] + `.html" class="read-more">Read more →</a>
        </div>`
        
        i = i + 1
    }
    
    damn html
}

slay generate_sidebar_categories_html(categories []tea) tea {
    sus html tea = `<ul class="category-list">`
    
    sus i drip = 0
    bestie (i < categories.length) {
        sus category_data map[tea]tea = json.parse_object(categories[i])
        
        html = html + `<li>
            <a href="/categories/` + category_data["slug"] + `.html">` + category_data["name"] + `</a>
            <span class="category-badge">` + category_data["post_count"] + `</span>
        </li>`
        
        i = i + 1
    }
    
    html = html + `</ul>`
    damn html
}

slay generate_comments_html(comments []tea) tea {
    ready (comments.length == 0) {
        damn `<p>No comments yet. Be the first to comment!</p>`
    }
    
    sus html tea = ""
    sus i drip = 0
    bestie (i < comments.length) {
        sus comment_data map[tea]tea = json.parse_object(comments[i])
        
        html = html + `<div class="comment">
            <strong>` + comment_data["author_name"] + `</strong>
            <small> - ` + comment_data["created_at"] + `</small>
            <p>` + comment_data["content"] + `</p>
        </div>`
        
        i = i + 1
    }
    
    damn html
}

fr fr ===== MAIN APPLICATION =====

slay main() {
    vibez.spill("Starting CURSED Blog Engine...")
    
    fr fr Initialize database
    sus db_conn database_enhanced.DatabaseConnection = initialize_blog_database()
    ready (!db_conn.is_connected) {
        vibez.spill("FATAL: Could not initialize blog database")
        damn
    }
    
    fr fr Create directory structure
    create_public_directories()
    
    fr fr Setup web server
    sus server webz.Server = webz.create_server(blog_config.server_port)
    
    fr fr Register request handlers
    webz.handle_requests(server, slay(request webz.HttpRequest) webz.HttpResponse {
        ready (stringz.starts_with(request.path, "/api/")) {
            damn handle_blog_api_request(db_conn, request)
        } otherwise {
            damn handle_blog_web_request(db_conn, request)
        }
    })
    
    vibez.spill("Blog engine started on port " + stringz.from_int(blog_config.server_port))
    vibez.spill("Website: http://localhost:" + stringz.from_int(blog_config.server_port))
    vibez.spill("Admin: http://localhost:" + stringz.from_int(blog_config.server_port) + "/admin")
    vibez.spill("API endpoints:")
    vibez.spill("  GET /api/posts - List posts")
    vibez.spill("  POST /api/posts - Create post")
    vibez.spill("  POST /api/comments - Add comment")
    vibez.spill("  POST /api/generate-site - Generate static site")
    
    fr fr Generate initial static site
    generate_static_site(db_conn)
    
    fr fr Start server
    webz.start_server(server)
    
    fr fr Cleanup
    database_enhanced.close_connection(db_conn)
}

slay handle_blog_web_request(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    ready (request.path == "/" || request.path == "/index.html") {
        damn serve_static_html("index.html")
    } otherwise ready (stringz.starts_with(request.path, "/posts/")) {
        damn serve_post_page(request.path)
    } otherwise ready (stringz.starts_with(request.path, "/categories/")) {
        damn serve_category_page(request.path)
    } otherwise {
        sus response webz.HttpResponse = webz.HttpResponse{}
        response.status_code = 404
        response.body = "Page not found"
        response.headers = {"Content-Type": "text/html"}
        damn response
    }
}

slay serve_static_html(filename tea) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    sus file_path tea = blog_config.web_root + "/" + filename
    
    ready (fs.file_exists(file_path)) {
        response.status_code = 200
        response.body = fs.read_file(file_path)
        response.headers = {"Content-Type": "text/html"}
    } otherwise {
        response.status_code = 404
        response.body = "File not found"
        response.headers = {"Content-Type": "text/plain"}
    }
    
    damn response
}

fr fr Additional helper functions would be defined here...

slay get_category_by_id(conn database_enhanced.DatabaseConnection, id drip) tea {
    sus conditions tea = json.object_to_string({
        "id": stringz.from_int(id)
    })
    sus categories []tea = database_enhanced.find_records(conn, "categories", conditions)
    ready (categories.length > 0) {
        damn categories[0]
    }
    damn ""
}

slay get_category_name_from_info(category_info tea) tea {
    ready (category_info != "") {
        sus data map[tea]tea = json.parse_object(category_info)
        damn data["name"]
    }
    damn "Uncategorized"
}

slay get_category_color_from_info(category_info tea) tea {
    ready (category_info != "") {
        sus data map[tea]tea = json.parse_object(category_info)
        damn data["color"]
    }
    damn "#007bff"
}

fr fr Start the blog engine
main()
