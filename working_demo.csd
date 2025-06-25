
package main

import (
    "vibecheck"
    "stringz" 
    "mathz"
)

func main() {
    // Demo of working CURSED features
    
    // String operations
    let message = "CURSED language features working!";
    let formatted = stringz.format("{}: {}", "Status", message);
    
    // Math operations  
    let result = mathz.add(21, 21);
    
    // System info
    let version = vibecheck.version();
    
    println(formatted);
    println("Math result:", result);
    println("Version:", version);
}
