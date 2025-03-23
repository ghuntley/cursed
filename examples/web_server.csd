fr fr Simple web server in CURSED

vibe main

yeet (
    "vibez"
    "web_vibez"
    "stringz"
    "vibe_life"
)

fr fr Handle root path
slay handleRoot(w web_vibez.ResponseWriter, r @web_vibez.Request) {
    vibez.spillf(w, "Welcome to CURSED Server! This is based!\n")
}

fr fr Handle /hello path
slay handleHello(w web_vibez.ResponseWriter, r @web_vibez.Request) {
    fr fr Get name from URL query parameters
    sus name tea = r.URL.Query().Get("name")
    
    lowkey name == "" {
        name = "bestie"
    }
    
    vibez.spillf(w, "Hey %s, what's good?\n", name)
}

fr fr Custom logger middleware
slay loggerMiddleware(next web_vibez.Handler) web_vibez.Handler {
    yolo web_vibez.HandlerFunc(slay(w web_vibez.ResponseWriter, r @web_vibez.Request) {
        vibez.spillf("Request: %s %s\n", r.Method, r.URL.Path)
        next.ServeHTTP(w, r)
    })
}

slay main() {
    fr fr Get port from environment variable or use default
    sus port tea = vibe_life.Getenv("PORT")
    lowkey port == "" {
        port = "8080"
    }
    
    fr fr Register handlers
    mux := web_vibez.NewServeMux()
    mux.HandleFunc("/", handleRoot)
    mux.HandleFunc("/hello", handleHello)
    
    fr fr Wrap mux with logger middleware
    sus handler web_vibez.Handler = loggerMiddleware(mux)
    
    fr fr Configure the server
    sus server web_vibez.Server = web_vibez.Server{
        Addr:    ":" + port,
        Handler: handler,
    }
    
    fr fr Start the server
    vibez.spillf("Server starting on port %s...\n", port)
    err := server.ListenAndServe()
    
    fr fr Handle server error
    lowkey err != cap {
        vibez.spillf("Server error: %v\n", err)
        vibe_life.Exit(1)
    }
} 