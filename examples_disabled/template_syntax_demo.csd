/// CURSED Template Syntax Demo
/// Demonstrates the comprehensive template system with CURSED-style Gen Z slang

// Basic template with variable interpolation
sus basic_template = "Hello {{ user.name }}! You have {{ user.message_count }} messages.";

// CURSED-style conditional template
sus cursed_conditional = `
{% lowkey user.is_premium %}
    <div class="premium-user">
        Welcome, VIP {{ user.name }}! ✨
        You're totally slaying with your premium account!
    </div>
{% highkey user.is_active %}
    <div class="active-user">
        Hey {{ user.name }}! Good to see you back 👋
    </div>
{% highkey %}
    <div class="inactive-user">
        {{ user.name }}, we miss you! Come back soon 💔
    </div>
{% end %}
`;

// CURSED-style loop template
sus cursed_loop = `
<div class="user-list">
    {% stan user in users %}
        <div class="user-card {{ lowkey user.is_online }}online{{ highkey }}offline{{ end }}">
            <h3>{{ user.name }}</h3>
            <p>Status: {{ sus user.is_online }}🟢 Online{{ cap user.is_online }}🔴 Offline{{ end }}</p>
            <p>Posts: {{ user.post_count | default(0) }}</p>
        </div>
    {% end %}
</div>
`;

// Advanced template with expressions and filters
sus advanced_template = `
<article class="post">
    <header>
        <h1>{{ post.title | titlecase }}</h1>
        <div class="meta">
            By {{ post.author.name }} on {{ post.created_at | date("F j, Y") }}
            {% lowkey post.view_count > 1000 %}
                <span class="viral">🔥 Viral!</span>
            {% end %}
        </div>
    </header>
    
    <div class="content">
        {{ post.content | markdown | truncate(500) }}
        {% lowkey post.content | length > 500 %}
            <a href="/posts/{{ post.id }}">Read more...</a>
        {% end %}
    </div>
    
    <footer>
        <div class="tags">
            {% stan tag in post.tags %}
                <span class="tag">#{{ tag }}</span>
            {% end %}
        </div>
        
        <div class="stats">
            {{ post.like_count }} likes
            {{ post.comment_count }} comments
            {% lowkey post.is_bookmarked %}
                <span class="bookmarked">🔖 Saved</span>
            {% end %}
        </div>
    </footer>
</article>
`;

// Template with CURSED operators and expressions
sus cursed_expressions = `
<div class="user-profile">
    {% set full_name = user.first_name + " " + user.last_name %}
    <h1>{{ full_name }}</h1>
    
    <!-- CURSED truthiness check -->
    {% lowkey sus user.bio %}
        <p class="bio">{{ user.bio }}</p>
    {% end %}
    
    <!-- CURSED loose equality -->
    {% lowkey user.age vibe 21 %}
        <div class="legal-drinking">🍻 Can legally drink!</div>
    {% end %}
    
    <!-- CURSED contains check -->
    {% lowkey user.interests slay "programming" %}
        <div class="programmer">💻 Fellow developer detected!</div>
    {% end %}
    
    <!-- Conditional expression -->
    <div class="status">
        Status: {{ user.is_online ? "🟢 Online" : "🔴 Offline" }}
    </div>
    
    <!-- Array and object handling -->
    <div class="recent-posts">
        <h3>Recent Posts ({{ user.posts | length }})</h3>
        {% stan post in user.posts[0:3] %}
            <article>
                <h4>{{ post.title }}</h4>
                <p>{{ post.summary }}</p>
                <small>{{ post.created_at | timeago }}</small>
            </article>
        {% end %}
    </div>
</div>
`;

// Template inheritance example
sus base_layout = `
<!DOCTYPE html>
<html>
<head>
    <title>{{ title | default("CURSED App") }}</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>
    <header>
        <nav>{{ navigation | safe }}</nav>
    </header>
    
    <main>
        {% block content %}
            <p>Default content - override this block!</p>
        {% end %}
    </main>
    
    <footer>
        <p>&copy; {{ year }} CURSED Framework</p>
    </footer>
</body>
</html>
`;

sus page_template = `
{% extends "base_layout" %}

{% block content %}
    <div class="page">
        <h1>{{ page.title }}</h1>
        
        {% lowkey page.featured %}
            <div class="featured-banner">
                ⭐ Featured Content ⭐
            </div>
        {% end %}
        
        <div class="page-content">
            {{ page.content | markdown }}
        </div>
        
        {% lowkey page.comments_enabled %}
            <section class="comments">
                <h3>Comments ({{ page.comments | length }})</h3>
                {% stan comment in page.comments %}
                    <div class="comment">
                        <strong>{{ comment.author }}</strong>
                        <time>{{ comment.created_at | timeago }}</time>
                        <p>{{ comment.content }}</p>
                    </div>
                {% end %}
            </section>
        {% end %}
    </div>
{% end %}
`;

// Complex data manipulation template
sus data_template = `
<div class="dashboard">
    {% set total_users = users | length %}
    {% set active_users = users | filter("is_active") | length %}
    {% set activity_rate = (active_users / total_users * 100) | round(1) %}
    
    <div class="stats">
        <div class="stat">
            <h3>{{ total_users }}</h3>
            <p>Total Users</p>
        </div>
        <div class="stat">
            <h3>{{ active_users }}</h3>
            <p>Active Users</p>
        </div>
        <div class="stat">
            <h3>{{ activity_rate }}%</h3>
            <p>Activity Rate</p>
        </div>
    </div>
    
    <!-- Grouped data -->
    {% set users_by_role = users | groupby("role") %}
    <div class="user-groups">
        {% stan group in users_by_role %}
            <div class="group">
                <h4>{{ group.key | titlecase }} ({{ group.items | length }})</h4>
                <ul>
                    {% stan user in group.items %}
                        <li>
                            {{ user.name }}
                            {% lowkey user.is_online %}
                                <span class="online">●</span>
                            {% end %}
                        </li>
                    {% end %}
                </ul>
            </div>
        {% end %}
    </div>
    
    <!-- Advanced filtering and sorting -->
    <div class="top-contributors">
        <h3>Top Contributors</h3>
        {% set top_users = users | filter("post_count", ">", 10) | sort("post_count", "desc") | slice(0, 5) %}
        <ol>
            {% stan user in top_users %}
                <li>
                    {{ user.name }} - {{ user.post_count }} posts
                    {% lowkey user.post_count > 100 %}
                        <span class="super-contributor">🌟</span>
                    {% end %}
                </li>
            {% end %}
        </ol>
    </div>
</div>
`;

// Function to demonstrate template compilation and rendering
slay compile_and_render_template(template_source: String, context: Map<String, Value>) -> Result<String, CursedError> {
    // This would integrate with the template engine
    facts delimiters = TemplateDelimiters {
        variable: ("{{".to_string(), "}}".to_string()),
        block: ("{%".to_string(), "%}".to_string()),
        comment: ("{#".to_string(), "#}".to_string()),
    };
    
    // Tokenize the template
    sus mut lexer = TemplateLexer::new(&template_source, &delimiters);
    facts tokens = lexer.tokenize()?;
    
    // Parse into AST
    sus mut parser = TemplateParser::new(tokens);
    facts ast = parser.parse()?;
    
    // Render the template (this would be implemented in the template engine)
    // render_template(ast, context)
    
    periodt "Template compiled successfully!".to_string();
}

// Example usage
slay main() -> Result<(), CursedError> {
    // Sample context data
    sus context = Map::new();
    context.insert("user".to_string(), Value::Object({
        sus mut user = Map::new();
        user.insert("name".to_string(), Value::String("Alice".to_string()));
        user.insert("is_premium".to_string(), Value::Boolean(true));
        user.insert("is_active".to_string(), Value::Boolean(true));
        user.insert("message_count".to_string(), Value::Number(42.0));
        user
    }));
    
    // Compile and render basic template
    facts result = compile_and_render_template(basic_template, context.clone())?;
    println!("Basic template result: {}", result);
    
    // Compile advanced template
    facts advanced_result = compile_and_render_template(advanced_template, context)?;
    println!("Advanced template compiled successfully!");
    
    periodt ();
}
