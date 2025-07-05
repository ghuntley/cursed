func main() {
    let a = true;
    let b = false;
    let c = true;
    
    // Test logical AND
    let result1 = a && c;
    vibez.spill("a && c = " + result1);
    
    let result2 = a && b;
    vibez.spill("a && b = " + result2);
    
    // Test logical OR
    let result3 = a || b;
    vibez.spill("a || b = " + result3);
    
    let result4 = b || false;
    vibez.spill("b || false = " + result4);
    
    // Test in if statement
    if (a && c) {
        vibez.spill("if (a && c) works!");
    }
    
    if (a || b) {
        vibez.spill("if (a || b) works!");
    }
}
