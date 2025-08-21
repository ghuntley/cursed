yeet "vibez"
yeet "networkz"

vibez.spill("Testing NetworkZ module...")

// Test URL parsing
sus url_parts networkz.UrlParts = networkz.parse_url("https://api.example.com:8443/v1/users?limit=10") fam {
    when err -> {
        vibez.spill("URL parse error:", err.message)
        damn
    }
}

vibez.spill("✅ URL parsing successful:")
vibez.spill("  Scheme:", url_parts.scheme)
vibez.spill("  Host:", url_parts.host)
vibez.spill("  Port:", url_parts.port)
vibez.spill("  Path:", url_parts.path)
vibez.spill("  Query:", url_parts.query)

// Test parameter encoding
sus params []tea = ["name=John Doe", "city=New York"]
sus encoded tea = networkz.encode_url_params(params)
vibez.spill("✅ URL encoding:", encoded)

// Test HTTP GET simulation
sus response networkz.HttpResponse = networkz.http_get("http://echo.example.com/test") fam {
    when err -> {
        vibez.spill("HTTP GET error:", err.message)
        damn
    }
}

vibez.spill("✅ HTTP GET successful:")
vibez.spill("  Status:", response.status_code)
vibez.spill("  Body length:", response.content_length)

vibez.spill("NetworkZ module test completed!")
