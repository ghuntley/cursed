vibe main

yeet "vibez"

fr fr Regression test validates correct CURSED keyword usage and syntax
fr fr Tests canonical keywords vs deprecated forms to ensure parser correctness
fr fr Validates 'otherwise' instead of 'else' and proper 'periodt' usage

slay main_character() {
    vibez.spill("=== Keyword Regression Test ===");
    
    sus test_value normie = 42;
    
    fr fr Test 'ready' with 'otherwise' (canonical form)
    ready test_value > 40 {
        vibez.spill("Using canonical 'otherwise' keyword");
    } otherwise ready test_value > 20 {
        vibez.spill("Chained 'otherwise ready'");
    } otherwise {
        vibez.spill("Final 'otherwise' clause");
    };
    
    fr fr Test 'periodt' while loop (canonical form)
    sus counter normie = 3;
    periodt counter > 0 {
        vibez.spill(counter);
        counter = counter - 1;
    };
    
    fr fr Test 'bestie' for loop with proper syntax
    sus i normie = 0;
    bestie i < 3 {
        vibez.spill("bestie loop iteration");
        i = i + 1;
    };
    
    fr fr Test basic conditional flow
    sus is_friday lit = based;
    ready is_friday {
        vibez.spill("End of week vibes");
    } otherwise {
        vibez.spill("Other day");
    };
    
    fr fr Test 'damn' return keyword
    sus return_test normie = test_return(5);
    vibez.spill(return_test);
    
    vibez.spill("=== All Keywords Validated ===");
};

slay test_return(x normie) normie {
    ready x > 0 {
        damn 1;
    } otherwise ready x < 0 {
        damn -1;
    } otherwise {
        damn 0;
    };
};
