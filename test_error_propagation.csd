// Test error propagation with context

slay risky_function(value normie) normie {
    bestie value > 100 {
        yikes overflow_error := "Value too large";
        damn 0;
    }
    damn value * 2;
}

slay calling_function(input normie) normie {
    result := shook risky_function(input);
    damn result;
}

// Test error propagation
vibez.spill("Testing error propagation");
sus final_result normie = calling_function(150);
vibez.spill("Should not reach here");
