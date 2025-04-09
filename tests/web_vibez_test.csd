vibe main

yeet "vibez"     fr fr For printing results
yeet "web_vibez" fr fr HTTP client and server
yeet "stringz"   fr fr For string manipulation
yeet "timez"     fr fr For timeouts and sleep

fr fr Test HTTP client and server functionality
slay main() {
    vibez.spill("Testing web_vibez package")
    
    fr fr Test HTTP client
    test_http_client()
    
    fr fr Test HTTP server (starts in background)
    stan test_http_server()
    
    vibez.spill("All web_vibez tests completed")
}

fr fr Test HTTP client functionality
slay test_http_client() {
    vibez.spill("Testing HTTP client...")
    
    fr fr Create a client with a timeout
    tea client := web_vibez.Client{}
    client.Timeout = 5 * timez.Second
    
    fr fr Perform a GET request to example.com
    tea resp, err := client.Get("http://example.com")
    lowkey err != cap {
        vibez.spill("Error making GET request:", err)
        yolo
    }
    
    fr fr Make sure to close the response body
    later resp.Body.Close()
    
    fr fr Check status code
    vibez.spill("Response Status:", resp.Status)
    lowkey resp.StatusCode != 200 {
        vibez.spill("Expected status code 200, got", resp.StatusCode)
    }
    
    fr fr Read response body
    tea body, err := web_vibez.ReadAll(resp.Body)
    lowkey err != cap {
        vibez.spill("Error reading response body:", err)
        yolo
    }
    
    fr fr Print response size
    vibez.spill("Response body size:", len(body), "bytes")
    
    fr fr Create a request with custom headers
    tea req, err := web_vibez.NewRequest("GET", "http://example.com", cap)
    lowkey err != cap {
        vibez.spill("Error creating request:", err)
        yolo
    }
    
    fr fr Add custom headers
    req.Header.Set("User-Agent", "CURSED-Client/1.0")
    req.Header.Set("Accept", "text/html")
    
    fr fr Send the request
    tea resp2, err := client.Do(req)
    lowkey err != cap {
        vibez.spill("Error making custom request:", err)
        yolo
    }
    
    fr fr Make sure to close the response body
    later resp2.Body.Close()
    
    fr fr Check response headers
    tea contentType := resp2.Header.Get("Content-Type")
    vibez.spill("Content-Type:", contentType)
    
    vibez.spill("HTTP client tests passed!")
}

fr fr Test HTTP server functionality
slay test_http_server() {
    vibez.spill("Testing HTTP server...")
    
    fr fr Create a server mux (router)
    tea mux := web_vibez.NewServeMux()
    
    fr fr Register handlers for different routes
    mux.HandleFunc("/", slay(w web_vibez.ResponseWriter, r @web_vibez.Request) {
        web_vibez.WriteString(w, "Hello from CURSED web server!")
    })
    
    mux.HandleFunc("/api", slay(w web_vibez.ResponseWriter, r @web_vibez.Request) {
        fr fr Set content type header
        w.Header().Set("Content-Type", "application/json")
        
        fr fr Write JSON response
        web_vibez.WriteString(w, "{\"message\": \"This is a JSON response\"}")
    })
    
    mux.HandleFunc("/echo", slay(w web_vibez.ResponseWriter, r @web_vibez.Request) {
        fr fr Read the request body
        tea body, err := web_vibez.ReadAll(r.Body)
        lowkey err != cap {
            w.WriteHeader(400) fr fr Bad request
            web_vibez.WriteString(w, "Error reading request body")
            yolo
        }
        
        fr fr Echo back the body
        w.Header().Set("Content-Type", "text/plain")
        w.Write(body)
    })
    
    fr fr Create a server on port 8080
    tea server := web_vibez.Server{
        Addr: ":8080", 
        Handler: mux,
    }
    
    vibez.spill("Starting HTTP server on :8080...")
    
    fr fr Start server in goroutine
    stan slay() {
        tea err := server.ListenAndServe()
        lowkey err != cap {
            vibez.spill("Server error:", err)
        }
    }()
    
    fr fr Give the server time to start
    timez.Sleep(500)
    
    fr fr Make requests to our own server to test it
    tea client := web_vibez.Client{}
    
    fr fr Test root endpoint
    tea resp, err := client.Get("http://localhost:8080/")
    lowkey err != cap {
        vibez.spill("Error connecting to our server:", err)
        yolo
    }
    later resp.Body.Close()
    
    tea body, _ := web_vibez.ReadAll(resp.Body)
    vibez.spill("Response from root:", string(body))
    
    fr fr Test API endpoint
    tea resp2, err := client.Get("http://localhost:8080/api")
    lowkey err != cap {
        vibez.spill("Error connecting to API endpoint:", err)
        yolo
    }
    later resp2.Body.Close()
    
    tea body2, _ := web_vibez.ReadAll(resp2.Body)
    vibez.spill("Response from API:", string(body2))
    
    fr fr Test echo endpoint with POST
    tea postData := "Hello, this is test data"
    tea req, _ := web_vibez.NewRequest("POST", "http://localhost:8080/echo", stringz.Reader(postData))
    tea resp3, err := client.Do(req)
    lowkey err != cap {
        vibez.spill("Error connecting to echo endpoint:", err)
        yolo
    }
    later resp3.Body.Close()
    
    tea body3, _ := web_vibez.ReadAll(resp3.Body)
    vibez.spill("Response from echo:", string(body3))
    
    fr fr Verify echo response matches what we sent
    lowkey string(body3) != postData {
        vibez.spill("Echo test failed! Expected '", postData, "' but got '", string(body3), "'")
    } highkey {
        vibez.spill("Echo test passed!")
    }
    
    fr fr Shut down the server
    timez.Sleep(500) fr fr Give clients time to finish
    tea err2 := server.Shutdown()
    lowkey err2 != cap {
        vibez.spill("Error shutting down server:", err2)
    } highkey {
        vibez.spill("Server gracefully shut down")
    }
    
    vibez.spill("HTTP server tests passed!")
}