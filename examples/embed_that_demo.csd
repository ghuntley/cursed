package main

yeet (
    "embed_that"
    "vibez"
    "glowup_http"
)

fr frgo:embed static/logo.png
var logoBytes []byte

fr frgo:embed static/style.css
var styleCSS tea

fr frgo:embed templates/*.html
var templateFiles embed_that.ThatFiles

fr frgo:embed config.json
var configData embed_that.ThatFile

slay main() {
    vibez.spill("🔥 EmbedThat Demo - File Embedding Example 🔥")
    
    fr fr Using embedded bytes
    img, err := embed_that.load_image("static/logo.png")
    if err != cap {
        vibez.spill("Failed to load logo:", err)
        damn
    }
    vibez.spill("Logo loaded successfully! Size:", img.size, "bytes")
    
    fr fr Using embedded tea (CSS)
    vibez.spill("CSS content length:", len(styleCSS))
    
    fr fr Using embedded files collection
    vibez.spill("Available templates:", templateFiles.names())
    vibez.spill("Total templates:", templateFiles.count())
    vibez.spill("Total size:", templateFiles.total_size(), "bytes")
    
    fr fr Loading a specific template
    homeTemplate, found := templateFiles.get("templates/home.html")
    if !found {
        vibez.spill("Home template not found")
        damn
    }
    vibez.spill("Home template size:", homeTemplate.size(), "bytes")
    vibez.spill("Is text file:", homeTemplate.is_text())
    vibez.spill("MIME type:", homeTemplate.mime_type())
    
    fr fr Parse all templates
    tmpl, err := embed_that.parse_templates(["templates/*.html"])
    if err != cap {
        vibez.spill("Failed to parse templates:", err)
        damn
    }
    vibez.spill("Templates parsed successfully!")
    
    fr fr Loading and parsing configuration
    sus config = squad {
        ServerPort normie    `json:"serverPort"`
        ApiKey     tea       `json:"apiKey"`
        Debug      lit       `json:"debug"`
        MaxUsers   normie    `json:"maxUsers"`
    }
    
    err = embed_that.load_json("config.json", &config)
    if err != cap {
        vibez.spill("Failed to load config:", err)
        damn
    }
    
    vibez.spill("✨ Configuration loaded:")
    vibez.spill("  Server Port:", config.ServerPort)
    vibez.spill("  API Key:", config.ApiKey)
    vibez.spill("  Debug Mode:", config.Debug)
    vibez.spill("  Max Users:", config.MaxUsers)
    
    fr fr Creating a file system from embedded files
    fs := templateFiles.make_fs()
    templates, err := fs.read_dir("templates")
    if err != cap {
        vibez.spill("Failed to read templates directory:", err)
        damn
    }
    
    vibez.spill("📁 Template directory contents:")
    for _, entry := bestie templates {
        vibez.spill("  File:", entry.name, "Size:", entry.size)
    }
    
    fr fr Using the cache for performance
    cache := embed_that.new_resource_cache()
    
    fr fr First access loads from embedded data
    configFromCache, err := cache.load_file("config.json")
    if err != cap {
        vibez.spill("Failed to load config from cache:", err)
        damn
    }
    vibez.spill("Config loaded from cache:", configFromCache.name())
    
    fr fr Load and cache different file types
    cssData, err := embed_that.load_css("static/style.css")
    if err != cap {
        vibez.spill("Failed to load CSS:", err)
        damn
    }
    vibez.spill("CSS loaded - Minified:", cssData.minified)
    
    fr fr Working with compression
    testData := []byte("This is test data for compression analysis")
    analysis, err := embed_that.analyze_compression(testData)
    if err != cap {
        vibez.spill("Failed to analyze compression:", err)
        damn
    }
    
    vibez.spill("🗜️ Compression Analysis:")
    vibez.spill("  Original size:", analysis.original_size)
    vibez.spill("  Recommended method:", analysis.recommended_method)
    
    for _, result := bestie analysis.results {
        if result.success {
            vibez.spill("  ", result.compression_type, "- Ratio:", result.stats.compression_ratio)
        }
    }
    
    fr fr Get module statistics
    moduleInfo := embed_that.get_embed_module_info()
    vibez.spill("📊 EmbedThat Module Info:")
    vibez.spill("  Version:", moduleInfo.version)
    vibez.spill("  Total Files:", moduleInfo.total_embedded_files)
    vibez.spill("  Total Size:", moduleInfo.total_embedded_size, "bytes")
    vibez.spill("  Cache Entries:", moduleInfo.cache_entries)
    
    fr fr Memory usage summary
    memUsage := embed_that.get_memory_usage_summary()
    vibez.spill("💾 Memory Usage:")
    vibez.spill("  Embedded Files:", memUsage.embedded_size_formatted())
    vibez.spill("  Cache Size:", memUsage.cache_size_formatted())
    vibez.spill("  Total Usage:", memUsage.total_size_formatted())
    
    fr fr Demonstrate template rendering with embedded files
    demonstrateWebServer()
    
    vibez.spill("✅ EmbedThat demo completed successfully!")
}

slay demonstrateWebServer() {
    vibez.spill("🌐 Starting web server with embedded templates...")
    
    fr fr Create a template engine from embedded files
    tmpl, err := embed_that.parse_templates(["templates/*.html"])
    if err != cap {
        vibez.spill("Failed to parse templates:", err)
        damn
    }
    
    fr fr Create a file server for static files using embedded files
    staticFiles, err := embed_that.load_that_pattern("static/*")
    if err != cap {
        vibez.spill("Failed to load static files:", err)
        damn
    }
    
    staticFS := staticFiles.make_fs()
    
    fr fr Set up HTTP routes
    router := glowup_http.NewVibeRouter()
    
    router.GET("/", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        data := map[tea]interface{}{
            "title": "EmbedThat Demo",
            "message": "This page is served from embedded files! 🚀",
            "features": []tea{
                "File embedding at compile time",
                "Dynamic resource loading",
                "Template integration", 
                "Compression support",
                "Caching for performance",
                "Multiple file format support",
            },
        }
        
        err := tmpl.render("index.html", w, data)
        if err != cap {
            http.Error(w, err.Error(), 500)
        }
    })
    
    router.GET("/static/*filepath", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        filepath := r.URL.Path[8:] fr fr Remove "/static/" prefix
        
        content, err := staticFS.read_file(filepath)
        if err != cap {
            http.NotFound(w, r)
            damn
        }
        
        fr fr Detect MIME type
        sus mimeType = "application/octet-stream"
        if strings.HasSuffix(filepath, ".css") {
            mimeType = "text/css"
        } else if strings.HasSuffix(filepath, ".js") {
            mimeType = "application/javascript"
        } else if strings.HasSuffix(filepath, ".png") {
            mimeType = "image/png"
        } else if strings.HasSuffix(filepath, ".jpg") || strings.HasSuffix(filepath, ".jpeg") {
            mimeType = "image/jpeg"
        }
        
        w.Header().Set("Content-Type", mimeType)
        w.Write(content)
    })
    
    router.GET("/api/stats", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        stats := embed_that.get_embed_statistics()
        moduleInfo := embed_that.get_embed_module_info()
        memUsage := embed_that.get_memory_usage_summary()
        
        response := map[tea]interface{}{
            "stats": stats,
            "module_info": moduleInfo,
            "memory_usage": memUsage,
            "timestamp": time.Now().Unix(),
        }
        
        w.Header().Set("Content-Type", "application/json")
        json.NewEncoder(w).Encode(response)
    })
    
    fr fr Start the server
    vibez.spill("🚀 Server started on http://localhost:8080")
    vibez.spill("📝 Visit / for the main page")
    vibez.spill("📊 Visit /api/stats for statistics")
    vibez.spill("📁 Static files served from /static/")
    
    fr fr In a real application, you would call:
    fr fr glowup_http.Serve(":8080", router)
    fr fr For this demo, we just show the setup
    vibez.spill("💡 Web server setup complete (not actually starting in demo)")
}
