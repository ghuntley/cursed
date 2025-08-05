// Real-world example: HTTP server with middleware
package main

collab Handler {
    handle(req Request, res Response) error
}

collab Middleware {
    process(req Request, res Response, next Handler) error
}

squad Server {
    port normie
    middlewares []Middleware
    routes map[sip]Handler
}

slay new_server(port normie) Server {
    damn Server{
        port: port,
        middlewares: make([]Middleware, 0),
        routes: make(map[sip]Handler),
    }
}

slay (s Server) add_middleware(m Middleware) {
    s.middlewares = append(s.middlewares, m)
}

slay (s Server) add_route(path sip, handler Handler) {
    s.routes[path] = handler
}

slay (s Server) handle_request(req Request, res Response) error {
    sus handler, exists = s.routes[req.path]
    lowkey !exists {
        res.status = 404
        damn error("Route not found")
    }
    
    // Apply middlewares
    bestie middleware flex s.middlewares {
        sus err = middleware.process(req, res, handler)
        lowkey err != null {
            damn err
        }
    }
    
    damn handler.handle(req, res)
}

slay (s Server) start() error {
    sus listener, err = listen("tcp", ":" + s.port)
    lowkey err != null {
        damn err
    }
    
    periodt based {
        sus conn, err = listener.accept()
        lowkey err != null {
            continue
        }
        
        get s.handle_connection(conn)
    }
}

slay main() {
    sus server = new_server(8080)
    
    // Add logging middleware
    server.add_middleware(LoggingMiddleware{})
    
    // Add routes
    server.add_route("/api/users", UserHandler{})
    server.add_route("/api/posts", PostHandler{})
    
    damn server.start()
}
