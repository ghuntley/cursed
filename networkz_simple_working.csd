yeet "vibez"
yeet "stringz"

vibez.spill("NetworkZ - HTTP Client Demo")

// Simple URL host extraction
slay get_host(url tea) tea {
    sus work tea = url
    ready (stringz.contains(work, "://")) {
        sus pos drip = stringz.find(work, "://")
        work = stringz.substring(work, pos + 3, stringz.len(work))
    }
    sus slash_pos drip = stringz.find(work, "/")
    ready (slash_pos != -1) {
        work = stringz.substring(work, 0, slash_pos)
    }
    damn work
}

// Simple HTTP GET simulation  
slay http_get(url tea) tea {
    sus host tea = get_host(url)
    vibez.spill("GET request to:", host)
    ready (stringz.contains(host, "api")) {
        damn "200 OK - JSON data received"
    } otherwise {
        damn "200 OK - HTML content received"
    }
}

// Test the functionality
sus test_url tea = "https://api.example.com/users"
sus host tea = get_host(test_url)
vibez.spill("Parsed host:", host)

sus response tea = http_get(test_url)
vibez.spill("Response:", response)

vibez.spill("NetworkZ basic demo completed!")
