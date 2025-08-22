// simple_network_test.csd - Simple Network Test
yeet "httpz"
yeet "vibez"
yeet "stringz"

vibez.spill("Testing HTTP GET...")
sus response tea = httpz.http_get("http://httpbin.org/get")
vibez.spill("Response:")
vibez.spill(response)

ready (stringz.contains(response, "real_network")) {
    vibez.spill("✓ SUCCESS: Real network operations implemented!")
} otherwise {
    vibez.spill("✗ FAILURE: Still using simulated responses")
}
