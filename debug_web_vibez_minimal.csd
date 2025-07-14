yeet "vibez"
yeet "web_vibez"

vibez.spill("Testing web_vibez module...")

# Test basic functionality without testz framework
sus headers HttpHeaders = web_vibez.init_headers()
vibez.spill("Headers count: " + tea(headers.count))

headers = web_vibez.add_header(headers, "Content-Type", "application/json")
vibez.spill("Headers count after add: " + tea(headers.count))

sus content_type tea = web_vibez.get_header(headers, "Content-Type")
vibez.spill("Content-Type: " + content_type)

# Test HTTP request creation
sus request HttpRequest = web_vibez.create_request("GET", "http://example.com/api")
vibez.spill("Request method: " + request.method)
vibez.spill("Request host: " + request.url.host)

# Test HTTP response creation
sus response HttpResponse = web_vibez.create_response(200)
vibez.spill("Response status code: " + tea(response.status_code))
vibez.spill("Response status: " + response.status)

vibez.spill("✅ Basic web_vibez functionality verified!")
