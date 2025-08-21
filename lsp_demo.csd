// CURSED LSP Demo File
// This file demonstrates LSP features like completion, hover, and diagnostics

yeet "vibez"
yeet "mathz"

// Function definition - should provide hover info and go-to-definition
slay calculateArea(width drip, height drip) drip {
    sus area drip = width * height
    damn area
}

// Variable declarations - should appear in completions
sus userName tea = "CURSED Developer"  
sus userAge drip = 25
sus isActive lit = based

// Struct definition
squad Rectangle {
    width drip,
    height drip
}

// Pattern matching
slay processNumber(num drip) tea {
    sick (num) {
        when 0 -> "zero"
        when 1 -> "one"  
        when 2 -> "two"
        otherwise -> "other"
    }
}

// Error handling example
slay divide(a drip, b drip) drip {
    ready (b == 0) {
        yikes "Division by zero!"
    }
    damn a / b
}

// Main function with various CURSED language features
slay main() {
    vibez.spill("Welcome to CURSED LSP Demo!")
    
    // Test completions - try typing "use" or "calc" or "Rectangle."
    sus rect Rectangle = Rectangle { width: 10, height: 20 }
    sus area drip = calculateArea(rect.width, rect.height)
    
    vibez.spill("Area:", area)
    vibez.spill("User:", userName, "Age:", userAge)
    
    // Test hover - hover over function names, variables
    sus result tea = processNumber(42)
    vibez.spill("Result:", result)
    
    // Test error diagnostics - uncomment next line to see error
    // sus broken drip = divide(10, 0)
}

// Interface example
collab Drawable {
    slay draw()
}

// More complex function for testing
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}
