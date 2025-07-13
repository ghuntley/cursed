yeet "url_parsing"

# Simple test of URL parsing
vibez.spill("Testing URL parsing module...")

# Test basic parsing
bestie url_parse("http://example.com") {
    vibez.spill("URL parsed successfully")
    vibez.spill("Scheme: " + url_get_scheme())
    vibez.spill("Host: " + url_get_host())
    vibez.spill("Port: " + url_get_port())
} otherwise {
    vibez.spill("Failed to parse URL")
}
