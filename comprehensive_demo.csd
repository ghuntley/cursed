fr fr CURSED v3.9.0 Comprehensive Feature Demonstration
fr fr This demonstrates all major CURSED language features working together

fr fr Function with typed parameters and return type
slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1;
    } highkey {
        damn n * factorial(n - 1);
    }
}

fr fr Function with string operations
slay greet(name tea) {
    vibez.spill("Hello, " + name + "!");
    vibez.spill("Welcome to CURSED v3.9.0!");
}

fr fr Main function demonstrating all features
slay main() {
    fr fr Variable declarations with types
    sus name tea = "CURSED Developer";
    sus number normie = 5;
    sus flag lit = based;
    
    fr fr Function calls with arguments
    greet(name);
    
    fr fr Mathematical operations and function calls
    sus result normie = factorial(number);
    
    fr fr Multi-line if statements with complex conditions
    lowkey result > 100 {
        vibez.spill("Factorial result is large!");
        vibez.spill("Result: " + result);
    } highkey {
        vibez.spill("Factorial result is small");
        vibez.spill("Result: " + result);
    }
    
    fr fr Single-line if statements
    lowkey flag { vibez.spill("Flag is true!"); }
    
    fr fr Return the computed result
    damn result;
}
