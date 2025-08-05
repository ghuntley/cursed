vibe main

yeet "vibez"
yeet "htmlrizzler"
yeet "timez"

slay main() {
    // Print a message
    vibez.spill("Testing all dot expression functions");
    
    // HTML escaping
    sus html_content := "<script>alert('XSS attack');</script>";
    sus escaped := htmlrizzler.escape_html(html_content);
    vibez.spill("Original: " + html_content);
    vibez.spill("Escaped: " + escaped);
    
    // Get current time
    sus current_time := timez.Now();
    vibez.spill("Current timestamp: " + current_time);
    
    damn 0;
}