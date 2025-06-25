#!/usr/bin/env cursed

// Comprehensive demo of the oglogging package - Simple logging with Gen Z vibes

yeet "stdlib::oglogging"
yeet "stdlib::io"

slay main() {
    println("🔥 CURSED OG Logging Demo - Let's get this logging bestie! 🔥\n")?
    
    // Basic logging with standard logger
    println("=== Basic Standard Logger ===")
    oglogging.setFlags(oglogging.LstdFlags | oglogging.Lshortfile)
    oglogging.setPrefix("bestie: ")
    
    oglogging.spill(["Starting our amazing CURSED app..."])
    oglogging.spillf("Hello {}, welcome to CURSED!", ["world"])
    
    // Different log levels simulation
    println("\n=== Different Log Levels ===")
    oglogging.setPrefix("INFO: ")
    oglogging.spill(["Application initialized successfully"])
    
    oglogging.setPrefix("WARN: ")
    oglogging.spill(["This is a warning message - no cap!"])
    
    oglogging.setPrefix("ERROR: ")
    oglogging.spill(["An error occurred, but we're handling it like a queen"])
    
    // Custom logger with different output
    println("\n=== Custom Logger Demo ===")
    sus customLogger := oglogging.new(
        io.stdout,
        "CUSTOM: ",
        oglogging.Ltime | oglogging.Lmicroseconds | oglogging.Lshortfile
    )
    
    customLogger.spill(["This is from our custom logger"])
    customLogger.spillf("Processing {} items with {} threads", [100, 4])
    
    // Format flags demonstration
    println("\n=== Format Flags Demo ===")
    
    // Date only
    oglogging.setFlags(oglogging.Ldate)
    oglogging.setPrefix("DATE: ")
    oglogging.spill(["Just the date please"])
    
    // Time with microseconds
    oglogging.setFlags(oglogging.Ltime | oglogging.Lmicroseconds)
    oglogging.setPrefix("TIME: ")
    oglogging.spill(["Precise timing down to microseconds"])
    
    // Full flags
    oglogging.setFlags(oglogging.Ldate | oglogging.Ltime | oglogging.Lshortfile)
    oglogging.setPrefix("FULL: ")
    oglogging.spill(["Complete logging information"])
    
    // UTC time demo
    oglogging.setFlags(oglogging.Ldate | oglogging.Ltime | oglogging.LUTC)
    oglogging.setPrefix("UTC: ")
    oglogging.spill(["This timestamp is in UTC"])
    
    // Message prefix demo
    oglogging.setFlags(oglogging.Ltime | oglogging.Lmsgprefix)
    oglogging.setPrefix(">> ")
    oglogging.spill(["Prefix appears before the message"])
    
    // Preset configurations demo
    println("\n=== Preset Configurations ===")
    
    sus minimalLogger := oglogging.new(
        io.stdout,
        "MINIMAL: ",
        oglogging.presets.MINIMAL
    )
    minimalLogger.spill(["Just the message, no extras"])
    
    sus detailedLogger := oglogging.new(
        io.stdout,
        "DETAILED: ",
        oglogging.presets.DETAILED
    )
    detailedLogger.spill(["Detailed logging with file info"])
    
    sus productionLogger := oglogging.new(
        io.stdout,
        "PROD: ",
        oglogging.presets.PRODUCTION
    )
    productionLogger.spill(["Production-ready logging format"])
    
    // Formatted logging demo
    println("\n=== Formatted Logging Demo ===")
    oglogging.setFlags(oglogging.LstdFlags)
    oglogging.setPrefix("APP: ")
    
    sus username := "ghuntley"
    sus userId := 42
    sus score := 95.7
    sus isActive := true
    
    oglogging.spillf("User login: {} (ID: {})", [username, userId])
    oglogging.spillf("Performance score: {:.1}%", [score])
    oglogging.spillf("Account status: active={}", [isActive])
    
    // Complex formatted messages
    sus items := ["apple", "banana", "cherry"]
    oglogging.spillf("Processing {} items: {}", [items.length, items])
    
    // Indexed placeholders
    oglogging.spillf("Values: {1} comes after {0}", ["first", "second"])
    
    // Escaped braces
    oglogging.spillf("JSON example: {{\"key\": \"{}\"}}", ["value"])
    
    // Application lifecycle simulation
    println("\n=== Application Lifecycle Simulation ===")
    
    oglogging.setPrefix("LIFECYCLE: ")
    oglogging.setFlags(oglogging.LstdFlags | oglogging.Lmicroseconds)
    
    oglogging.spill(["Application starting..."])
    
    lowkey sus i := 0; i < 3; i++ {
        oglogging.spillf("Processing batch {} of {}", [i + 1, 3])
        
        // Simulate work
        time.sleep(time.milliseconds(100))
        
        oglogging.spillf("Batch {} completed successfully", [i + 1])
    }
    
    oglogging.spill(["All batches processed"])
    oglogging.spill(["Application shutting down gracefully"])
    
    // Error demonstration (but not actually fatal)
    println("\n=== Error Handling Demo ===")
    oglogging.setPrefix("ERROR: ")
    oglogging.spill(["This is a simulated error message"])
    oglogging.spillf("Failed to connect to service '{}' after {} attempts", ["database", 3])
    
    // Note: fatal() and shook() would terminate the program, so we won't demonstrate them here
    
    println("\n✨ OG Logging demo completed! Your logs are absolutely fire! ✨")
}
