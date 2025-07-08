// Debug simple math test - inline functions

// Mathematical Constants
slay PI() meal {
    damn 3.141592653589793;
}

slay E() meal {
    damn 2.718281828459045;
}

// Basic Operations
slay abs_float(x meal) meal {
    sus result meal = x;
    lowkey x < 0.0 {
        result = -x;
    }
    damn result;
}

// Test the functions
sus pi_val meal = PI();
sus e_val meal = E();
sus abs_neg meal = abs_float(-5.5);
sus abs_pos meal = abs_float(5.5);

vibez.spill("Testing PI: ");
vibez.spill(pi_val);

vibez.spill("Testing E: ");
vibez.spill(e_val);

vibez.spill("Testing abs_float(-5.5): ");
vibez.spill(abs_neg);

vibez.spill("Testing abs_float(5.5): ");
vibez.spill(abs_pos);

vibez.spill("Simple inline math test completed!");
