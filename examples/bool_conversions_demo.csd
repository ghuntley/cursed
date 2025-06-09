// Boolean conversion demonstration program for CURSED
// This example showcases comprehensive bool type conversions

slay main() {
    // Basic bool literals
    sus truthy = based;
    sus falsy = cap;
    
    print("=== Boolean Literal Examples ===");
    print(truthy);  // Should print "based" 
    print(falsy);   // Should print "sus"
    
    // Bool to integer conversions
    print("\n=== Bool to Integer Conversions ===");
    sus true_as_int = normie(truthy);    // Should be 1
    sus false_as_int = normie(falsy);    // Should be 0
    print("true as int:");
    print(true_as_int);
    print("false as int:");
    print(false_as_int);
    
    // Bool to float conversions  
    print("\n=== Bool to Float Conversions ===");
    sus true_as_float = meal(truthy);    // Should be 1.0
    sus false_as_float = meal(falsy);    // Should be 0.0
    print("true as float:");
    print(true_as_float);
    print("false as float:");
    print(false_as_float);
    
    // Bool to string conversions
    print("\n=== Bool to String Conversions ===");
    sus true_as_string = tea(truthy);    // Should be "true"
    sus false_as_string = tea(falsy);    // Should be "false"
    print("true as string:");
    print(true_as_string);
    print("false as string:");
    print(false_as_string);
    
    // Integer to bool conversions (0 = false, non-zero = true)
    print("\n=== Integer to Bool Conversions ===");
    sus zero = 0;
    sus non_zero = 42;
    sus negative = -1;
    
    sus zero_as_bool = lit(zero);
    sus non_zero_as_bool = lit(non_zero);
    sus negative_as_bool = lit(negative);
    
    print("0 as bool:");
    print(zero_as_bool);
    print("42 as bool:");
    print(non_zero_as_bool);
    print("-1 as bool:");
    print(negative_as_bool);
    
    // Float to bool conversions (0.0 = false, non-zero = true)
    print("\n=== Float to Bool Conversions ===");
    sus zero_float = 0.0;
    sus pi = 3.14159;
    sus negative_float = -2.5;
    
    sus zero_float_as_bool = lit(zero_float);
    sus pi_as_bool = lit(pi);
    sus negative_float_as_bool = lit(negative_float);
    
    print("0.0 as bool:");
    print(zero_float_as_bool);
    print("3.14159 as bool:");
    print(pi_as_bool);
    print("-2.5 as bool:");
    print(negative_float_as_bool);
    
    // String to bool conversions (empty = false, non-empty = true)
    print("\n=== String to Bool Conversions ===");
    sus empty_string = "";
    sus hello_string = "hello";
    
    sus empty_as_bool = lit(empty_string);
    sus hello_as_bool = lit(hello_string);
    
    print("Empty string as bool:");
    print(empty_as_bool);
    print("'hello' as bool:");
    print(hello_as_bool);
    
    // Logical operations with mixed types
    print("\n=== Logical Operations with Auto-Conversion ===");
    
    // AND operations
    sus and_bool_int = truthy && 42;      // true && true = true
    sus and_bool_zero = truthy && 0;      // true && false = false
    sus and_float_bool = 3.14 && falsy;   // true && false = false
    
    print("true && 42:");
    print(and_bool_int);
    print("true && 0:");
    print(and_bool_zero);
    print("3.14 && false:");
    print(and_float_bool);
    
    // OR operations
    sus or_bool_int = falsy || 42;        // false || true = true
    sus or_zero_bool = 0 || truthy;       // false || true = true
    sus or_false_false = falsy || 0;      // false || false = false
    
    print("false || 42:");
    print(or_bool_int);
    print("0 || true:");
    print(or_zero_bool);
    print("false || 0:");
    print(or_false_false);
    
    // NOT operations
    print("\n=== NOT Operations with Auto-Conversion ===");
    sus not_true = !truthy;               // !true = false
    sus not_false = !falsy;               // !false = true
    sus not_int = !42;                    // !true = false
    sus not_zero = !0;                    // !false = true
    sus not_float = !3.14;                // !true = false
    sus not_zero_float = !0.0;            // !false = true
    
    print("!true:");
    print(not_true);
    print("!false:");
    print(not_false);
    print("!42:");
    print(not_int);
    print("!0:");
    print(not_zero);
    print("!3.14:");
    print(not_float);
    print("!0.0:");
    print(not_zero_float);
    
    // Control flow with auto-conversion
    print("\n=== Control Flow with Auto-Conversion ===");
    
    lowkey (42) {
        print("42 is truthy - this should execute");
    } highkey {
        print("This should not execute");
    }
    
    lowkey (0) {
        print("This should not execute");
    } highkey {
        print("0 is falsy - this should execute");
    }
    
    lowkey (3.14) {
        print("3.14 is truthy - this should execute");
    }
    
    lowkey (0.0) {
        print("This should not execute");
    } highkey {
        print("0.0 is falsy - this should execute");
    }
    
    lowkey ("hello") {
        print("Non-empty string is truthy - this should execute");
    }
    
    lowkey ("") {
        print("This should not execute");
    } highkey {
        print("Empty string is falsy - this should execute");
    }
    
    // Loops with auto-conversion
    print("\n=== Loops with Auto-Conversion ===");
    
    sus counter = 3;
    periodt (counter) {
        print("Counter:");
        print(counter);
        counter = counter - 1;
    }
    print("Loop finished when counter became 0 (falsy)");
    
    // Complex boolean expressions
    print("\n=== Complex Boolean Expressions ===");
    
    sus a = 5;
    sus b = 0;
    sus c = 3.14;
    sus d = "";
    sus e = "test";
    
    // Complex expression: (a && c) || (b && e)
    // Evaluates to: (true && true) || (false && true) = true || false = true
    sus complex1 = (a && c) || (b && e);
    print("(5 && 3.14) || (0 && 'test'):");
    print(complex1);
    
    // Complex expression: !(a && b) && (c || d)
    // Evaluates to: !(true && false) && (true || false) = !false && true = true && true = true
    sus complex2 = !(a && b) && (c || d);
    print("!(5 && 0) && (3.14 || ''):");
    print(complex2);
    
    // Comparison operations (return bool)
    print("\n=== Comparison Operations ===");
    
    sus greater = a > 3;                  // 5 > 3 = true
    sus equal = b == 0;                   // 0 == 0 = true
    sus less_equal = c <= 4.0;            // 3.14 <= 4.0 = true
    
    print("5 > 3:");
    print(greater);
    print("0 == 0:");
    print(equal);
    print("3.14 <= 4.0:");
    print(less_equal);
    
    // Chained bool operations
    sus chained = greater && equal && less_equal;  // true && true && true = true
    print("Chained: (5 > 3) && (0 == 0) && (3.14 <= 4.0):");
    print(chained);
    
    print("\n=== Bool Conversion Demo Complete ===");
}

// Helper function to demonstrate bool parameters
slay test_bool_param(lit should_print) {
    lowkey (should_print) {
        print("Function called with truthy parameter");
    } highkey {
        print("Function called with falsy parameter");
    }
}

// Helper function to demonstrate bool return values
slay is_positive(normie value) -> lit {
    bestie value > 0 {
        flex based;
    } highkey {
        flex cap;
    }
}
