// Simple CURSED Demo - Core Features
// Shows Gen-Z keywords working with basic language constructs

// Global constants using Gen-Z keywords
facts greeting = "Hello from CURSED!"
facts number = 42

// Function using Gen-Z 'slay' keyword
slay main() {
    // Variable declaration with 'facts' (immutable)
    facts message = greeting
    
    // Variable declaration with 'sus' (mutable)
    sus count = number
    
    // Output using 'damn' (return/print statement)
    damn message
    damn count
    
    // Simple arithmetic
    count = count + 8
    damn "Count updated: " + count
    
    // Conditional with 'lowkey' (if)
    lowkey (count > 40) {
        damn "Count is pretty high! 📈"
    }
    
    damn "CURSED language demo complete! ✨"
}
