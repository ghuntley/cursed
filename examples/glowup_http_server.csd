#!/usr/bin/env cursed

import "stdlib::glowup_http";

// Simple HTTP server example using GlowUpHTTP
slay main() tea {
    // Create a new router
    sus router = glowup_http.NewVibeRouter();
    
    // Add some middleware
    router.UseMiddleware(glowup_http.LoggingMiddleware);
    router.UseMiddleware(glowup_http.UnbotheredMiddleware);
    
    // Handle routes
    router.GET("/", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        w.JSON(map[tea]tea{"message": "Welcome to the vibe!"});
    });
    
    router.GET("/users/:id", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        sus id = r.PathParam("id");
        w.JSON(map[tea]tea{"user_id": id});
    });
    
    router.POST("/api/data", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        sus data = r.GetJSON();
        vibez.spill("Received data:", data);
        w.Status(201).JSON(map[tea]tea{"status": "created"});
    });
    
    router.GET("/health", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        w.Text("OK");
    });
    
    // Static file serving
    router.GET("/static/*", glowup_http.FileHandler("/var/www/static"));
    
    // Start the server
    vibez.spill("Starting GlowUpHTTP server on :8080");
    glowup_http.Serve(":8080", router);
}

// HTTP client example
slay client_example() tea {
    sus client = &glowup_http.VibeClient{};
    
    // GET request
    sus resp, err = client.Get("https://api.example.com/data");
    lowkey err != cap {
        vibez.spill("Error:", err);
        periodt;
    }
    defer resp.Body.Close();
    
    // Parse JSON response
    sus data map[tea]interface{};
    lowkey err := resp.ParseJSON(&data); err != cap {
        vibez.spill("JSON parse error:", err);
        periodt;
    }
    
    vibez.spill("Response data:", data["message"]);
    
    // POST request
    sus postData = map[tea]tea{
        "name": "CURSED User",
        "vibe": "immaculate",
    };
    
    sus jsonData, _ = json.Marshal(postData);
    sus postResp, err = client.Post("https://api.example.com/users", "application/json", jsonData);
    
    lowkey err != cap {
        vibez.spill("POST error:", err);
        periodt;
    }
    defer postResp.Body.Close();
    
    vibez.spill("POST response status:", postResp.StatusCode);
}

// WebSocket example
slay websocket_example() tea {
    sus upgrader = glowup_http.NewWebSocketUpgrader();
    
    router.GET("/ws", slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        sus conn, err = upgrader.Upgrade(w, r);
        lowkey err != cap {
            vibez.spill("WebSocket upgrade error:", err);
            periodt;
        }
        defer conn.Close();
        
        // Echo server
        lowkey {
            sus messageType, message, err = conn.ReadMessage();
            lowkey err != cap {
                vibez.spill("Read error:", err);
                flex;
            }
            
            lowkey err := conn.WriteMessage(messageType, message); err != cap {
                vibez.spill("Write error:", err);
                flex;
            }
        }
    });
}

// Middleware example
slay custom_middleware_example() tea {
    sus custom_middleware = slay(next glowup_http.HandlerFunc) glowup_http.HandlerFunc {
        periodt slay(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) tea {
            // Add custom header
            w.Header().Set("X-Powered-By", "CURSED-GlowUpHTTP");
            
            // Log request
            vibez.spill("Custom middleware processing:", r.Method, r.URL.Path);
            
            // Call next handler
            next(w, r);
        };
    };
    
    router.UseMiddleware(custom_middleware);
}
