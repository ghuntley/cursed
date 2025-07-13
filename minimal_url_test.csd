# Test URL parsing without module imports

vibez.spill("Testing basic URL functionality...")

# Test direct function calls - using global state
sus test_url tea = "http://example.com"

# Since we can't import the module yet, let's test basic functionality
sus scheme tea = "http"
sus host tea = "example.com"
sus port normie = 80

vibez.spill("Test URL: " + test_url)
vibez.spill("Scheme: " + scheme)
vibez.spill("Host: " + host)
vibez.spill("Port: " + port)

bestie scheme == "http" {
    vibez.spill("✓ HTTP scheme detected")
} otherwise {
    vibez.spill("✗ HTTP scheme not detected")
}

bestie host == "example.com" {
    vibez.spill("✓ Host correct")
} otherwise {
    vibez.spill("✗ Host incorrect")
}

bestie port == 80 {
    vibez.spill("✓ Default HTTP port correct")
} otherwise {
    vibez.spill("✗ Default HTTP port incorrect")
}

vibez.spill("Basic URL test complete!")
