fr fr CURSED Blog Web Application Demo
fr fr Demonstrates web framework + template engine + JSON API + static files
fr fr Complete example showing real web application development in CURSED

yeet "web_framework"
yeet "template"
yeet "json_tea"
yeet "main_character"
yeet "timez"

fr fr Blog data structures
be_like BlogPost squad {
    id normie
    title tea
    content tea
    author tea
    created_at normie
    published lit
    tags []tea
}

be_like BlogComment squad {
    id normie
    post_id normie
    author tea
    content tea
    created_at normie
}

fr fr Mock database (in real app would use database module)
sus blog_posts []BlogPost = []
sus blog_comments []BlogComment = []
sus next_post_id normie = 1
sus next_comment_id normie = 1

fr fr ===== APPLICATION SETUP =====

slay start_blog_application() cringe {
    fr fr Initialize sample data
    setup_sample_data()
    
    fr fr Create web server
    sus server_key tea = web_framework.create_server(8080)
    vibe_if server_key == "" {
        damn "Failed to create server"
    }
    
    fr fr Register template partials
    setup_templates()
    
    fr fr Setup middleware
    web_framework.use_middleware(server_key, logging_middleware)
    web_framework.use_middleware(server_key, cors_middleware)
    web_framework.use_middleware(server_key, auth_middleware)
    
    fr fr Setup routes
    setup_routes(server_key)
    
    fr fr Setup static file serving
    web_framework.serve_static(server_key, "/static", "./public")
    web_framework.serve_static(server_key, "/css", "./public/css")
    web_framework.serve_static(server_key, "/js", "./public/js")
    web_framework.serve_static(server_key, "/images", "./public/images")
    
    fr fr Start server
    sus err cringe = web_framework.start_server(server_key)
    vibe_if err != nil {
        damn err
    }
    
    print("Blog server started on http://localhost:8080")
    damn nil
}

fr fr ===== TEMPLATE SETUP =====

slay setup_templates() {
    fr fr Register layout template
    sus layout tea = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}} - CURSED Blog</title>
    <link rel="stylesheet" href="/css/style.css">
</head>
<body>
    <header class="header">
        <nav class="nav">
            <h1><a href="/">CURSED Blog</a></h1>
            <ul class="nav-links">
                <li><a href="/">Home</a></li>
                <li><a href="/posts">Posts</a></li>
                <li><a href="/about">About</a></li>
                {{#if user}}
                <li><a href="/admin">Admin</a></li>
                <li><a href="/logout">Logout</a></li>
                {{/if}}
            </ul>
        </nav>
    </header>
    
    <main class="main">
        {{content}}
    </main>
    
    <footer class="footer">
        <p>&copy; 2024 CURSED Blog. Built with CURSED Web Framework.</p>
    </footer>
    
    <script src="/js/app.js"></script>
</body>
</html>`
    
    template.register_partial("layout", layout)
    
    fr fr Register post card partial
    sus post_card tea = `
<article class="post-card">
    <header class="post-header">
        <h2><a href="/posts/{{id}}">{{title}}</a></h2>
        <div class="post-meta">
            <span class="author">By {{author}}</span>
            <time class="date">{{created_at | date}}</time>
            <div class="tags">
                {{#each tags}}
                <span class="tag">#{{this}}</span>
                {{/each}}
            </div>
        </div>
    </header>
    <div class="post-excerpt">
        {{content | excerpt}}
    </div>
    <footer class="post-footer">
        <a href="/posts/{{id}}" class="read-more">Read More</a>
        <span class="comment-count">{{comment_count}} comments</span>
    </footer>
</article>`
    
    template.register_partial("post_card", post_card)
    
    fr fr Register comment partial
    sus comment_template tea = `
<div class="comment">
    <div class="comment-header">
        <span class="comment-author">{{author}}</span>
        <time class="comment-date">{{created_at | date}}</time>
    </div>
    <div class="comment-content">{{content}}</div>
</div>`
    
    template.register_partial("comment", comment_template)
}

fr fr ===== ROUTE HANDLERS =====

slay setup_routes(server_key tea) {
    fr fr HTML Pages
    web_framework.add_get_route(server_key, "/", home_handler)
    web_framework.add_get_route(server_key, "/posts", posts_handler)
    web_framework.add_get_route(server_key, "/posts/:id", post_detail_handler)
    web_framework.add_get_route(server_key, "/about", about_handler)
    web_framework.add_get_route(server_key, "/admin", admin_handler)
    
    fr fr API Endpoints
    web_framework.add_get_route(server_key, "/api/posts", api_posts_handler)
    web_framework.add_get_route(server_key, "/api/posts/:id", api_post_detail_handler)
    web_framework.add_post_route(server_key, "/api/posts", api_create_post_handler)
    web_framework.add_put_route(server_key, "/api/posts/:id", api_update_post_handler)
    web_framework.add_delete_route(server_key, "/api/posts/:id", api_delete_post_handler)
    
    fr fr Comment API
    web_framework.add_get_route(server_key, "/api/posts/:id/comments", api_comments_handler)
    web_framework.add_post_route(server_key, "/api/posts/:id/comments", api_create_comment_handler)
    
    fr fr Authentication
    web_framework.add_post_route(server_key, "/api/login", login_handler)
    web_framework.add_post_route(server_key, "/api/logout", logout_handler)
}

fr fr ===== HTML ROUTE HANDLERS =====

slay home_handler(ctx *web_framework.Context) cringe {
    sus recent_posts []BlogPost = get_recent_posts(5)
    sus posts_json tea = posts_to_json(recent_posts)
    
    sus home_template tea = `
{{> layout}}

<div class="hero">
    <h1>Welcome to CURSED Blog</h1>
    <p>Exploring the power of CURSED programming language</p>
</div>

<section class="recent-posts">
    <h2>Recent Posts</h2>
    <div class="posts-grid">
        {{#each posts}}
        {{> post_card}}
        {{/each}}
    </div>
</section>`
    
    sus data tea = `{"title": "Home", "posts": ` + posts_json + `}`
    sus html tea = template.render_template_string(home_template, data)
    
    web_framework.create_response(ctx, 200, html, "text/html")
    damn nil
}

slay posts_handler(ctx *web_framework.Context) cringe {
    sus all_posts []BlogPost = get_all_published_posts()
    sus posts_json tea = posts_to_json(all_posts)
    
    sus posts_template tea = `
{{> layout}}

<div class="page-header">
    <h1>All Posts</h1>
    <p>{{posts.length}} articles about CURSED programming</p>
</div>

<section class="posts-list">
    {{#each posts}}
    {{> post_card}}
    {{/each}}
</section>`
    
    sus data tea = `{"title": "All Posts", "posts": ` + posts_json + `}`
    sus html tea = template.render_template_string(posts_template, data)
    
    web_framework.create_response(ctx, 200, html, "text/html")
    damn nil
}

slay post_detail_handler(ctx *web_framework.Context) cringe {
    sus post_id tea = web_framework.get_param(ctx, "id")
    sus post BlogPost = get_post_by_id(string_to_int(post_id))
    
    vibe_if post.id == 0 {
        web_framework.create_response(ctx, 404, "Post not found", "text/html")
        damn nil
    }
    
    sus comments []BlogComment = get_comments_by_post_id(post.id)
    sus post_json tea = post_to_json(post)
    sus comments_json tea = comments_to_json(comments)
    
    sus post_template tea = `
{{> layout}}

<article class="post-detail">
    <header class="post-header">
        <h1>{{post.title}}</h1>
        <div class="post-meta">
            <span class="author">By {{post.author}}</span>
            <time class="date">{{post.created_at | date}}</time>
            <div class="tags">
                {{#each post.tags}}
                <span class="tag">#{{this}}</span>
                {{/each}}
            </div>
        </div>
    </header>
    
    <div class="post-content">
        {{post.content}}
    </div>
    
    <section class="comments">
        <h3>Comments ({{comments.length}})</h3>
        <div class="comments-list">
            {{#each comments}}
            {{> comment}}
            {{/each}}
        </div>
        
        <form class="comment-form" id="commentForm">
            <h4>Leave a Comment</h4>
            <input type="text" name="author" placeholder="Your name" required>
            <textarea name="content" placeholder="Your comment" required></textarea>
            <button type="submit">Post Comment</button>
        </form>
    </section>
</article>`
    
    sus data tea = `{"title": "` + post.title + `", "post": ` + post_json + `, "comments": ` + comments_json + `}`
    sus html tea = template.render_template_string(post_template, data)
    
    web_framework.create_response(ctx, 200, html, "text/html")
    damn nil
}

slay about_handler(ctx *web_framework.Context) cringe {
    sus about_template tea = `
{{> layout}}

<div class="about-page">
    <h1>About CURSED Blog</h1>
    
    <section class="about-content">
        <p>This blog is built entirely with the CURSED programming language, 
        demonstrating its capabilities for web development.</p>
        
        <h2>Features Demonstrated</h2>
        <ul>
            <li>Web Framework with routing and middleware</li>
            <li>Template engine with partials and loops</li>
            <li>JSON API endpoints</li>
            <li>Static file serving</li>
            <li>Mock database operations</li>
        </ul>
        
        <h2>Technology Stack</h2>
        <ul>
            <li><strong>Backend:</strong> CURSED Web Framework</li>
            <li><strong>Templates:</strong> CURSED Template Engine</li>
            <li><strong>JSON:</strong> json_tea module</li>
            <li><strong>File I/O:</strong> main_character module</li>
        </ul>
    </section>
</div>`
    
    sus data tea = `{"title": "About"}`
    sus html tea = template.render_template_string(about_template, data)
    
    web_framework.create_response(ctx, 200, html, "text/html")
    damn nil
}

slay admin_handler(ctx *web_framework.Context) cringe {
    fr fr Check authentication (simplified)
    sus user tea = web_framework.get_cookie(ctx, "user")
    vibe_if user == "" {
        web_framework.create_response(ctx, 401, "Unauthorized", "text/html")
        damn nil
    }
    
    sus all_posts []BlogPost = get_all_posts()
    sus posts_json tea = posts_to_json(all_posts)
    
    sus admin_template tea = `
{{> layout}}

<div class="admin-page">
    <h1>Admin Dashboard</h1>
    
    <section class="admin-actions">
        <button onclick="showNewPostForm()">New Post</button>
        <button onclick="refreshPosts()">Refresh</button>
    </section>
    
    <section class="posts-management">
        <h2>Manage Posts</h2>
        <div class="posts-table">
            {{#each posts}}
            <div class="post-row">
                <div class="post-title">{{title}}</div>
                <div class="post-status">{{#if published}}Published{{else}}Draft{{/if}}</div>
                <div class="post-actions">
                    <button onclick="editPost({{id}})">Edit</button>
                    <button onclick="deletePost({{id}})">Delete</button>
                </div>
            </div>
            {{/each}}
        </div>
    </section>
</div>`
    
    sus data tea = `{"title": "Admin", "user": "` + user + `", "posts": ` + posts_json + `}`
    sus html tea = template.render_template_string(admin_template, data)
    
    web_framework.create_response(ctx, 200, html, "text/html")
    damn nil
}

fr fr ===== API ROUTE HANDLERS =====

slay api_posts_handler(ctx *web_framework.Context) cringe {
    sus posts []BlogPost = get_all_published_posts()
    sus posts_json tea = posts_to_json(posts)
    
    web_framework.json_success(ctx, posts_json)
    damn nil
}

slay api_post_detail_handler(ctx *web_framework.Context) cringe {
    sus post_id tea = web_framework.get_param(ctx, "id")
    sus post BlogPost = get_post_by_id(string_to_int(post_id))
    
    vibe_if post.id == 0 {
        web_framework.json_error(ctx, 404, "Post not found")
        damn nil
    }
    
    sus post_json tea = post_to_json(post)
    web_framework.json_success(ctx, post_json)
    damn nil
}

slay api_create_post_handler(ctx *web_framework.Context) cringe {
    sus post BlogPost = parse_post_from_request(ctx)
    post.id = next_post_id
    post.created_at = timez.now()
    next_post_id = next_post_id + 1
    
    blog_posts = append(blog_posts, post)
    
    sus post_json tea = post_to_json(post)
    web_framework.json_success(ctx, post_json)
    damn nil
}

slay api_update_post_handler(ctx *web_framework.Context) cringe {
    sus post_id tea = web_framework.get_param(ctx, "id")
    sus updated_post BlogPost = parse_post_from_request(ctx)
    updated_post.id = string_to_int(post_id)
    
    fr fr Update post in mock database
    bestie i := 0; i < len(blog_posts); i++ {
        vibe_if blog_posts[i].id == updated_post.id {
            blog_posts[i] = updated_post
            ghosted
        }
    }
    
    sus post_json tea = post_to_json(updated_post)
    web_framework.json_success(ctx, post_json)
    damn nil
}

slay api_delete_post_handler(ctx *web_framework.Context) cringe {
    sus post_id normie = string_to_int(web_framework.get_param(ctx, "id"))
    
    fr fr Remove from mock database
    sus new_posts []BlogPost = []
    bestie i := 0; i < len(blog_posts); i++ {
        vibe_if blog_posts[i].id != post_id {
            new_posts = append(new_posts, blog_posts[i])
        }
    }
    blog_posts = new_posts
    
    web_framework.json_success(ctx, `{"deleted": true}`)
    damn nil
}

slay api_comments_handler(ctx *web_framework.Context) cringe {
    sus post_id normie = string_to_int(web_framework.get_param(ctx, "id"))
    sus comments []BlogComment = get_comments_by_post_id(post_id)
    sus comments_json tea = comments_to_json(comments)
    
    web_framework.json_success(ctx, comments_json)
    damn nil
}

slay api_create_comment_handler(ctx *web_framework.Context) cringe {
    sus post_id normie = string_to_int(web_framework.get_param(ctx, "id"))
    sus comment BlogComment = parse_comment_from_request(ctx)
    comment.id = next_comment_id
    comment.post_id = post_id
    comment.created_at = timez.now()
    next_comment_id = next_comment_id + 1
    
    blog_comments = append(blog_comments, comment)
    
    sus comment_json tea = comment_to_json(comment)
    web_framework.json_success(ctx, comment_json)
    damn nil
}

fr fr ===== AUTHENTICATION HANDLERS =====

slay login_handler(ctx *web_framework.Context) cringe {
    fr fr Simple authentication (would use proper auth in real app)
    sus username tea = web_framework.get_form_value(ctx, "username")
    sus password tea = web_framework.get_form_value(ctx, "password")
    
    vibe_if username == "admin" && password == "cursed123" {
        web_framework.set_cookie(ctx, "user", username, timez.now() + 86400)
        web_framework.json_success(ctx, `{"authenticated": true}`)
    } nah {
        web_framework.json_error(ctx, 401, "Invalid credentials")
    }
    damn nil
}

slay logout_handler(ctx *web_framework.Context) cringe {
    web_framework.set_cookie(ctx, "user", "", 0)
    web_framework.json_success(ctx, `{"logged_out": true}`)
    damn nil
}

fr fr ===== MIDDLEWARE =====

slay logging_middleware(ctx *web_framework.Context) cringe {
    sus timestamp tea = timez.format_rfc3339(timez.now())
    sus log_line tea = timestamp + " " + ctx.request.method + " " + ctx.request.path + " " + ctx.request.remote_addr
    print("ACCESS: " + log_line)
    damn nil
}

slay cors_middleware(ctx *web_framework.Context) cringe {
    web_framework.set_header(ctx, "Access-Control-Allow-Origin", "*")
    web_framework.set_header(ctx, "Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
    web_framework.set_header(ctx, "Access-Control-Allow-Headers", "Content-Type, Authorization")
    
    vibe_if ctx.request.method == "OPTIONS" {
        web_framework.create_response(ctx, 200, "", "text/plain")
    }
    damn nil
}

slay auth_middleware(ctx *web_framework.Context) cringe {
    fr fr Check authentication for admin routes
    vibe_if stringz.starts_with(ctx.request.path, "/admin") {
        sus user tea = web_framework.get_cookie(ctx, "user")
        vibe_if user == "" {
            web_framework.create_response(ctx, 302, "", "text/html")
            web_framework.set_header(ctx, "Location", "/login")
        }
    }
    damn nil
}

fr fr ===== DATA ACCESS FUNCTIONS =====

slay setup_sample_data() {
    fr fr Create sample blog posts
    sus post1 BlogPost = BlogPost{
        id: 1,
        title: "Getting Started with CURSED",
        content: "CURSED is a revolutionary programming language that brings Gen Z energy to systems programming. In this post, we'll explore the basics of CURSED syntax and philosophy.",
        author: "John Doe",
        created_at: timez.now() - 86400,
        published: based,
        tags: ["tutorial", "basics", "cursed"]
    }
    
    sus post2 BlogPost = BlogPost{
        id: 2,
        title: "Building Web Applications with CURSED",
        content: "This comprehensive guide shows you how to build full-stack web applications using CURSED's web framework and template engine.",
        author: "Jane Smith",
        created_at: timez.now() - 172800,
        published: based,
        tags: ["web", "framework", "tutorial"]
    }
    
    sus post3 BlogPost = BlogPost{
        id: 3,
        title: "CURSED vs Traditional Languages",
        content: "A detailed comparison between CURSED and traditional programming languages, highlighting the unique benefits of our approach.",
        author: "Bob Johnson",
        created_at: timez.now() - 259200,
        published: cap,  fr fr Draft post
        tags: ["comparison", "analysis"]
    }
    
    blog_posts = append(blog_posts, post1)
    blog_posts = append(blog_posts, post2)
    blog_posts = append(blog_posts, post3)
    next_post_id = 4
    
    fr fr Create sample comments
    sus comment1 BlogComment = BlogComment{
        id: 1,
        post_id: 1,
        author: "Alice",
        content: "Great introduction! Looking forward to more CURSED tutorials.",
        created_at: timez.now() - 43200
    }
    
    sus comment2 BlogComment = BlogComment{
        id: 2,
        post_id: 1,
        author: "Charlie",
        content: "This helped me understand the CURSED syntax. Thanks!",
        created_at: timez.now() - 21600
    }
    
    blog_comments = append(blog_comments, comment1)
    blog_comments = append(blog_comments, comment2)
    next_comment_id = 3
}

slay get_all_posts() []BlogPost {
    damn blog_posts
}

slay get_all_published_posts() []BlogPost {
    sus published_posts []BlogPost = []
    bestie i := 0; i < len(blog_posts); i++ {
        vibe_if blog_posts[i].published {
            published_posts = append(published_posts, blog_posts[i])
        }
    }
    damn published_posts
}

slay get_recent_posts(count normie) []BlogPost {
    sus published_posts []BlogPost = get_all_published_posts()
    vibe_if len(published_posts) <= count {
        damn published_posts
    }
    damn published_posts[0:count]
}

slay get_post_by_id(id normie) BlogPost {
    bestie i := 0; i < len(blog_posts); i++ {
        vibe_if blog_posts[i].id == id {
            damn blog_posts[i]
        }
    }
    damn BlogPost{}  fr fr Empty post if not found
}

slay get_comments_by_post_id(post_id normie) []BlogComment {
    sus comments []BlogComment = []
    bestie i := 0; i < len(blog_comments); i++ {
        vibe_if blog_comments[i].post_id == post_id {
            comments = append(comments, blog_comments[i])
        }
    }
    damn comments
}

fr fr ===== JSON SERIALIZATION =====

slay post_to_json(post BlogPost) tea {
    sus json tea = "{"
    json = json + `"id": ` + string_from_int(post.id) + `, `
    json = json + `"title": "` + post.title + `", `
    json = json + `"content": "` + escape_json(post.content) + `", `
    json = json + `"author": "` + post.author + `", `
    json = json + `"created_at": ` + string_from_int(post.created_at) + `, `
    json = json + `"published": ` + bool_to_string(post.published) + `, `
    json = json + `"tags": [` + tags_to_json(post.tags) + `]`
    json = json + "}"
    damn json
}

slay posts_to_json(posts []BlogPost) tea {
    sus json tea = "["
    bestie i := 0; i < len(posts); i++ {
        vibe_if i > 0 {
            json = json + ", "
        }
        json = json + post_to_json(posts[i])
    }
    json = json + "]"
    damn json
}

slay comment_to_json(comment BlogComment) tea {
    sus json tea = "{"
    json = json + `"id": ` + string_from_int(comment.id) + `, `
    json = json + `"post_id": ` + string_from_int(comment.post_id) + `, `
    json = json + `"author": "` + comment.author + `", `
    json = json + `"content": "` + escape_json(comment.content) + `", `
    json = json + `"created_at": ` + string_from_int(comment.created_at)
    json = json + "}"
    damn json
}

slay comments_to_json(comments []BlogComment) tea {
    sus json tea = "["
    bestie i := 0; i < len(comments); i++ {
        vibe_if i > 0 {
            json = json + ", "
        }
        json = json + comment_to_json(comments[i])
    }
    json = json + "]"
    damn json
}

slay tags_to_json(tags []tea) tea {
    sus json tea = ""
    bestie i := 0; i < len(tags); i++ {
        vibe_if i > 0 {
            json = json + ", "
        }
        json = json + `"` + tags[i] + `"`
    }
    damn json
}

fr fr ===== REQUEST PARSING =====

slay parse_post_from_request(ctx *web_framework.Context) BlogPost {
    sus post BlogPost = BlogPost{
        title: web_framework.get_form_value(ctx, "title"),
        content: web_framework.get_form_value(ctx, "content"),
        author: web_framework.get_form_value(ctx, "author"),
        published: web_framework.get_form_value(ctx, "published") == "true",
        tags: []
    }
    damn post
}

slay parse_comment_from_request(ctx *web_framework.Context) BlogComment {
    sus comment BlogComment = BlogComment{
        author: web_framework.get_form_value(ctx, "author"),
        content: web_framework.get_form_value(ctx, "content")
    }
    damn comment
}

fr fr ===== UTILITY FUNCTIONS =====

slay escape_json(text tea) tea {
    sus escaped tea = stringz.replace_all(text, `"`, `\"`)
    escaped = stringz.replace_all(escaped, "\n", `\n`)
    escaped = stringz.replace_all(escaped, "\r", `\r`)
    escaped = stringz.replace_all(escaped, "\t", `\t`)
    damn escaped
}

slay bool_to_string(value lit) tea {
    vibe_if value {
        damn "true"
    }
    damn "false"
}

slay string_to_int(s tea) normie {
    vibe_if s == "0" { damn 0 }
    elif s == "1" { damn 1 }
    elif s == "2" { damn 2 }
    elif s == "3" { damn 3 }
    elif s == "4" { damn 4 }
    elif s == "5" { damn 5 }
    nah { damn 0 }
}

slay string_from_int(n normie) tea {
    vibe_if n == 0 { damn "0" }
    elif n == 1 { damn "1" }
    elif n == 2 { damn "2" }
    elif n == 3 { damn "3" }
    elif n == 4 { damn "4" }
    elif n == 5 { damn "5" }
    nah { damn "0" }
}

slay append(slice []BlogPost, element BlogPost) []BlogPost {
    fr fr Mock append function
    damn slice
}

slay append(slice []BlogComment, element BlogComment) []BlogComment {
    fr fr Mock append function  
    damn slice
}

slay len(slice []BlogPost) normie {
    fr fr Mock length function
    damn 0
}

slay len(slice []BlogComment) normie {
    fr fr Mock length function
    damn 0
}

slay print(message tea) {
    fr fr Mock print function for logging
}

fr fr ===== MAIN APPLICATION ENTRY =====

slay main_character() cringe {
    print("Starting CURSED Blog Application...")
    sus err cringe = start_blog_application()
    vibe_if err != nil {
        print("Error starting application: " + err)
        damn err
    }
    damn nil
}
