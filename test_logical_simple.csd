slay main() {
    let a = true;
    let b = false;
    let c = true;
    
    // Test logical AND
    let result1 = a && c;
    vibez.spill(result1);
    
    let result2 = a && b;
    vibez.spill(result2);
    
    // Test logical OR
    let result3 = a || b;
    vibez.spill(result3);
    
    let result4 = b || false;
    vibez.spill(result4);
    
    // Test in if statement
    if (a && c) {
        vibez.spill("AND works!");
    }
    
    if (a || b) {
        vibez.spill("OR works!");
    }
}
