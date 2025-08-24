# Networking Modules Functionality Test
yeet "networkz"
yeet "httpz" 
yeet "web_vibez"

vibez.spill("=== NETWORKING MODULES TEST ===")

# Test networkz module
vibez.spill("Testing networkz.create_server...")
sus server_result lit = networkz.create_server("127.0.0.1", 8080)
vibez.spill("networkz.create_server result:", server_result)

vibez.spill("Testing networkz.get_ip...")
sus ip_result tea = networkz.get_ip()
vibez.spill("networkz.get_ip result:", ip_result)

# Test httpz module  
vibez.spill("Testing httpz.get...")
sus http_get_result tea = httpz.get("http://httpbin.org/get")
vibez.spill("httpz.get result length:", stringz.len(http_get_result))
vibez.spill("httpz.get first 100 chars:", stringz.slice(http_get_result, 0, 100))

vibez.spill("Testing httpz.post...")
sus post_data tea = '{"test": "data"}'
sus http_post_result tea = httpz.post("http://httpbin.org/post", post_data)
vibez.spill("httpz.post result length:", stringz.len(http_post_result))

# Test web_vibez module
vibez.spill("Testing web_vibez.start_server...")
sus web_server_result lit = web_vibez.start_server(3000)
vibez.spill("web_vibez.start_server result:", web_server_result)

vibez.spill("Testing web_vibez.route...")
sus route_result lit = web_vibez.route("/test", "GET")
vibez.spill("web_vibez.route result:", route_result)

vibez.spill("=== NETWORKING MODULES TEST COMPLETE ===")
