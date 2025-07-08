// Simple test for math_float_simple module
yeet "mod"

// Test basic constants
vibez.spill("Testing PI: ");
vibez.spill(PI());

vibez.spill("Testing E: ");
vibez.spill(E());

vibez.spill("Testing TAU: ");
vibez.spill(TAU());

// Test basic operations
vibez.spill("Testing abs_float(-5.5): ");
vibez.spill(abs_float(-5.5));

vibez.spill("Testing abs_float(5.5): ");
vibez.spill(abs_float(5.5));

vibez.spill("Testing min_float(3.14, 2.71): ");
vibez.spill(min_float(3.14, 2.71));

vibez.spill("Testing max_float(3.14, 2.71): ");
vibez.spill(max_float(3.14, 2.71));

// Test sqrt
vibez.spill("Testing sqrt_simple(4.0): ");
vibez.spill(sqrt_simple(4.0));

vibez.spill("Testing sqrt_simple(9.0): ");
vibez.spill(sqrt_simple(9.0));

// Test trigonometric functions
vibez.spill("Testing sin_simple(0.0): ");
vibez.spill(sin_simple(0.0));

vibez.spill("Testing cos_simple(0.0): ");
vibez.spill(cos_simple(0.0));

// Test exponential
vibez.spill("Testing exp_simple(0.0): ");
vibez.spill(exp_simple(0.0));

vibez.spill("Testing exp_simple(1.0): ");
vibez.spill(exp_simple(1.0));

// Test logarithm
vibez.spill("Testing ln_simple(1.0): ");
vibez.spill(ln_simple(1.0));

// Test utility
vibez.spill("Testing approximately_equal_simple(3.14, 3.14, 0.01): ");
vibez.spill(approximately_equal_simple(3.14, 3.14, 0.01));

vibez.spill("Simple math float module test completed successfully!");
