yeet "url_parsing"

vibez.spill("Testing URL parsing...")

bestie url_parse("http://example.com") {
    vibez.spill("URL parsed successfully!")
    vibez.spill("Scheme: " + url_get_scheme())
    vibez.spill("Host: " + url_get_host())
    vibez.spill("Port: " + url_get_port())
    vibez.spill("Is valid: " + url_is_valid())
    vibez.spill("Is absolute: " + url_is_absolute())
    vibez.spill("Is secure: " + url_is_secure())
} otherwise {
    vibez.spill("Failed to parse URL")
}

vibez.spill("Test complete!")
