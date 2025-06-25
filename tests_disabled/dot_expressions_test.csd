vibe main

yeet "vibez"
yeet "htmlrizzler"
yeet "timez"

// Test multiple dot expression types
slay test_dot_expressions() {
    // Test vibez.spill (already working)
    vibez.spill("Testing dot expressions...");
    
    // Test htmlrizzler.escape_html
    sus html_input := "<script>alert('XSS');</script>";
    sus escaped_html := htmlrizzler.escape_html(html_input);
    vibez.spill("HTML escaped: " + escaped_html);
    
    // Test timez.Now
    sus current_time := timez.Now();
    vibez.spill("Current time: " + current_time);
    
    yolo 0;
}

slay main() {
    test_dot_expressions();
    yolo 0;
}