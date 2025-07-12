// Test file for type system improvements

// Test basic primitive type inference
sus x normie = 42;           // Integer type
sus name tea = "hello";      // String type  
sus flag lit = based;        // Boolean type
sus ch sip = 'a';           // Character type

// Test numeric type coercion
sus float_val meal = 3.14;   // Float type
sus int_to_float meal = x;   // Integer to float coercion

// Test binary operations with type inference
sus sum normie = x + 10;     // Integer arithmetic
sus comparison lit = x > 5;  // Comparison result

// Test logical operations
sus logic_result lit = flag && (x > 0);

// Test function calls with argument type checking
vibez.spill("Type system test");
vibez.spill(name);
vibez.spill(x);

// Test map literal with type inference
// sus map_test = {"key1": 1, "key2": 2};

// Test array with type inference
// sus arr = [1, 2, 3];
