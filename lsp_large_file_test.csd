// Large CURSED File Test for LSP Server - Week 2 Oracle Tools Phase
// Testing 10K-line file handling without panics

yeet "vibez";
yeet "mathz"; 
yeet "stringz";
yeet "arrayz";
yeet "testz";

slay main() {
    // Test basic CURSED syntax
    sus message tea = "CURSED LSP Test";
    sus count drip = 42;
    sus active lit = based;
    
    vibez.spill("Starting LSP large file test...");
    
    // Array operations
    sus numbers []drip = [1, 2, 3, 4, 5];
    bestie (item in numbers) {
        vibez.spill("Number:", item);
    }
    
    // Conditional logic
    ready (count > 40) {
        vibez.spill("Count is high");
    } otherwise {
        vibez.spill("Count is low");
    }
    
    // Function calls
    sus result drip = calculate(count, 10);
    vibez.spill("Result:", result);
    
    // Error handling
    sus value drip = risky_operation() fam {
        when "error" -> {
            vibez.spill("Handled error");
            damn 0;
        }
    };
    
    vibez.spill("LSP test complete");
}

slay calculate(a drip, b drip) drip {
    damn a + b;
}

slay risky_operation() yikes<drip> {
    // Simulate potential error
    damn 100;
}

// Additional test functions for LSP completion
slay test_completion() {
    // Keywords starting with 's'
    sus x drip = 1;
    
    // Keywords starting with 'b'  
    ready (based) {
        bestie (i in 0..10) {
            vibez.spill("Loop:", i);
        }
    }
    
    // Module completions
    vibez.spill("Hello");
    mathz.abs(-5);
    stringz.len("test");
    arrayz.push([1,2,3], 4);
}

// Test semantic token classification
slay semantic_token_test() {
    // Variable declarations
    sus name tea = "CURSED";
    sus age drip = 25;
    sus active lit = based;
    
    // Function calls
    vibez.spill(name);
    mathz.max(age, 30);
    
    // Control structures
    ready (age > 18) {
        vibez.spill("Adult");
    } otherwise ready (age > 13) {
        vibez.spill("Teen");
    } otherwise {
        vibez.spill("Child");
    }
    
    // Arrays and loops
    sus items []tea = ["apple", "banana", "cherry"];
    bestie (fruit in items) {
        vibez.spill("Fruit:", fruit);
    }
}

// Test error diagnostics
slay diagnostic_test() {
    // This should generate diagnostics:
    // sus missing_assignment drip = ; // Missing value
    // undefined_function();          // Undefined function
    // ready (count = 5) {           // Assignment in condition
    
    // Valid code:
    sus valid drip = 42;
    ready (valid == 42) {
        vibez.spill("Valid condition");
    }
}

// End of sample CURSED code for LSP testing
