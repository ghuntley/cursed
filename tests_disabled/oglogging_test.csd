vibe main

vibe vibe_test_formatting
vibe vibe_test_log_levels

yeet "oglogging"
yeet "vibez"

slay main() {
    fr fr // Test the advanced formatting capabilities
    vibe_test_formatting()
    
    fr fr // Test the log levels
    vibe_test_log_levels()
    
    vibez.spill("All oglogging tests passed!")
}

slay vibe_test_formatting() {
    fr fr // Basic formatting
    oglogging.spillf("Integer: %d", 42)
    oglogging.spillf("Float: %f", 3.14159)
    oglogging.spillf("String: %s", "hello")
    oglogging.spillf("Boolean: %t", true)
    
    fr fr // Advanced formatting with width and alignment
    oglogging.spillf("Right aligned integer: %5d", 42)      fr fr // Should pad left with spaces
    oglogging.spillf("Left aligned integer: %-5d", 42)      fr fr // Should pad right with spaces
    oglogging.spillf("Precision for float: %.2f", 3.14159)  fr fr // Should show 3.14
    
    fr fr // Positional arguments
    oglogging.spillf("Reusing args: %[1]v %[1]v %[2]v", "first", "second")
    
    fr fr // Hexadecimal, octal and binary formatting
    oglogging.spillf("Hex: %x Octal: %o Binary: %b", 255, 255, 255)
}

slay vibe_test_log_levels() {
    fr fr // Set minimum level to debug to see all messages
    oglogging.set_level(oglogging.LDEBUG)
    
    fr fr // Test each log level
    oglogging.debug("This is a debug message")
    oglogging.info("This is an info message")
    oglogging.warning("This is a warning message")
    oglogging.error("This is an error message")
    
    fr fr // Test with formatted messages
    oglogging.debugf("Debug with number: %d", 1)
    oglogging.infof("Info with number: %d", 2)
    oglogging.warningf("Warning with number: %d", 3)
    oglogging.errorf("Error with number: %d", 4)
    
    fr fr // Set level to warning to filter out debug and info
    oglogging.set_level(oglogging.LWARNING)
    oglogging.debug("This debug message should NOT appear")
    oglogging.info("This info message should NOT appear")
    oglogging.warning("This warning message should appear")
    oglogging.error("This error message should appear")
    
    fr fr // Reset level back to info
    oglogging.set_level(oglogging.LINFO)
}