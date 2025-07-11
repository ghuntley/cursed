// Simple Math Test
// Basic test of mathematical functions

slay PI() meal {
    damn 3.14159;
}

slay abs(x meal) meal {
    damn x >= 0.0 ? x : -x;
}

slay test_simple() {
    vibez.spill("Testing simple math functions");
    
    sus pi_val meal = PI();
    vibez.spill("Pi: " + pi_val.(tea));
    
    sus abs_result meal = abs(-5.0);
    vibez.spill("Absolute value of -5.0: " + abs_result.(tea));
    
    vibez.spill("Test completed successfully!");
}

test_simple();
