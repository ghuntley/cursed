//! CURSED Hygienic Macro System Examples
//! Demonstrates the powerful macro capabilities of CURSED

yeet "vibez"
yeet "testz"
yeet "stringz"
yeet "jsonz"

// =============================================================================
// 1. DECLARATIVE MACROS WITH PATTERN MATCHING
// =============================================================================

// Simple debug printing macro
slay_macro! debug_print {
    ($msg:expr) => {
        ready (DEBUG_MODE) {
            vibez.spill("[DEBUG]", $msg)
        }
    }
}

// Mathematical operations macro with multiple patterns
slay_macro! math_op {
    (add $a:expr, $b:expr) => {
        $a + $b
    },
    (sub $a:expr, $b:expr) => {
        $a - $b
    },
    (mul $a:expr, $b:expr) => {
        $a * $b
    },
    (div $a:expr, $b:expr) => {
        ready ($b != 0) {
            $a / $b
        } otherwise {
            yikes "Division by zero"
        }
    }
}

// Vector creation macro with repetition patterns
slay_macro! vec {
    () => {
        []
    },
    ($($item:expr),* $(,)?) => {
        [$($item),*]
    },
    ($item:expr; $count:literal) => {
        {
            sus result = []
            bestie (sus i drip = 0; i < $count; i++) {
                result.push($item)
            }
            damn result
        }
    }
}

// Conditional compilation macro
slay_macro! cfg {
    (debug) => {
        ready (DEBUG_MODE) { based } otherwise { cringe }
    },
    (release) => {
        ready (!DEBUG_MODE) { based } otherwise { cringe }
    },
    (feature = $feat:literal) => {
        ready (features.contains($feat)) { based } otherwise { cringe }
    }
}

// =============================================================================
// 2. STRUCT DERIVATION MACROS
// =============================================================================

// JSON serialization macro
slay_macro! derive_json {
    (squad $name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        // Generate to_json method
        slay to_json(self: $name) tea {
            sus result tea = "{"
            sus first lit = based
            
            $({
                ready (!first) {
                    result += ", "
                }
                result += "\"" + stringify!($field) + "\": "
                
                ready (is_string_type!($type)) {
                    result += "\"" + self.$field + "\""
                } otherwise {
                    result += tea(self.$field)
                }
                
                first = cringe
            })*
            
            result += "}"
            damn result
        }
        
        // Generate from_json method
        slay from_json(json_str: tea) $name {
            sus obj $name = $name{}
            sus json = jsonz.parse(json_str)
            
            $({
                ready (json.has_key(stringify!($field))) {
                    obj.$field = json.get(stringify!($field))
                }
            })*
            
            damn obj
        }
    }
}

// Debug formatting macro
slay_macro! derive_debug {
    (squad $name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        slay debug_format(self: $name) tea {
            sus result tea = stringify!($name) + " { "
            sus first lit = based
            
            $({
                ready (!first) {
                    result += ", "
                }
                result += stringify!($field) + ": " + debug_value!(self.$field)
                first = cringe
            })*
            
            result += " }"
            damn result
        }
    }
}

// Equality comparison macro
slay_macro! derive_eq {
    (squad $name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        slay equals(self: $name, other: $name) lit {
            damn $(self.$field == other.$field) && *
        }
        
        slay hash(self: $name) drip {
            sus result drip = 0
            $({
                result = result ^ hash_value!(self.$field)
            })*
            damn result
        }
    }
}

// =============================================================================
// 3. CONTROL FLOW MACROS
// =============================================================================

// Try-catch macro with automatic error propagation
slay_macro! try_op {
    ($expr:expr) => {
        $expr fam {
            when _ -> damn yikes "Operation failed"
        }
    },
    ($expr:expr, $error_msg:literal) => {
        $expr fam {
            when _ -> damn yikes $error_msg
        }
    }
}

// Pattern matching macro with guards
slay_macro! pattern_match {
    ($value:expr; $($pattern:pat $(when $guard:expr)? => $result:expr),* $(,)?) => {
        sick $value {
            $($pattern $(when $guard)? => $result),*
        }
    }
}

// Loop with early termination macro
slay_macro! loop_until {
    ($condition:expr, $body:block) => {
        bestie (based) {
            ready ($condition) {
                ghosted
            }
            $body
        }
    }
}

// Retry macro with exponential backoff
slay_macro! retry {
    ($attempts:literal, $body:block) => {
        sus attempt drip = 0
        bestie (attempt < $attempts) {
            $body fam {
                when _ -> {
                    attempt += 1
                    ready (attempt < $attempts) {
                        sleep(100 * (2 ** attempt))  // Exponential backoff
                        simp
                    } otherwise {
                        yikes "Max retry attempts exceeded"
                    }
                }
            }
            ghosted
        }
    }
}

// =============================================================================
// 4. TESTING MACROS
// =============================================================================

// Assert macro with custom messages
slay_macro! assert {
    ($condition:expr) => {
        ready (!($condition)) {
            yikes "Assertion failed: " + stringify!($condition)
        }
    },
    ($condition:expr, $msg:expr) => {
        ready (!($condition)) {
            yikes "Assertion failed: " + $msg + " (" + stringify!($condition) + ")"
        }
    }
}

// Test case macro
slay_macro! test_case {
    ($name:literal, $body:block) => {
        slay test_ ## $name() {
            vibez.spill("Running test:", $name)
            
            later {
                vibez.spill("Test completed:", $name)
            }
            
            $body
        }
    }
}

// Benchmark macro
slay_macro! benchmark {
    ($name:literal, $iterations:literal, $body:block) => {
        slay bench_ ## $name() {
            sus start_time = get_time_nanos()
            
            bestie (sus i drip = 0; i < $iterations; i++) {
                $body
            }
            
            sus end_time = get_time_nanos()
            sus duration = end_time - start_time
            sus avg_ns = duration / $iterations
            
            vibez.spill("Benchmark", $name, ":", avg_ns, "ns per iteration")
        }
    }
}

// =============================================================================
// 5. UTILITY MACROS
// =============================================================================

// Stringify macro
slay_macro! stringify {
    ($($tokens:tt)*) => {
        "$(token_string!($tokens))*"
    }
}

// Type checking macros
slay_macro! is_string_type {
    (tea) => { based },
    ($other:ty) => { cringe }
}

slay_macro! is_numeric_type {
    (drip) => { based },
    (float) => { based },
    ($other:ty) => { cringe }
}

// Debug value formatting
slay_macro! debug_value {
    ($value:expr) => {
        ready (is_string_type!(typeof!($value))) {
            "\"" + $value + "\""
        } otherwise {
            tea($value)
        }
    }
}

// Hash value generation
slay_macro! hash_value {
    ($value:expr) => {
        ready (is_string_type!(typeof!($value))) {
            string_hash($value)
        } otherwise ready (is_numeric_type!(typeof!($value))) {
            number_hash($value)
        } otherwise {
            object_hash($value)
        }
    }
}

// Compile-time configuration
slay_macro! feature_enabled {
    ($feature:literal) => {
        ready (compile_env!("FEATURES").contains($feature)) {
            based
        } otherwise {
            cringe
        }
    }
}

// =============================================================================
// 6. ADVANCED PROCEDURAL MACROS
// =============================================================================

// SQL query builder (procedural macro)
@proc_macro
slay build_query(input: QueryAST) QueryResult {
    // This would be implemented as a procedural macro
    // that takes AST input and generates SQL queries
    damn QueryResult.from_ast(input)
}

// ORM model generation (procedural macro)
@proc_macro
slay generate_model(input: ModelDefinition) StructDefinition {
    // Generate database model with CRUD operations
    damn generate_orm_struct(input)
}

// HTML template macro (procedural macro)
@proc_macro
slay html_template(input: TemplateAST) FunctionDefinition {
    // Generate HTML rendering function from template
    damn compile_template_to_function(input)
}

// =============================================================================
// 7. USAGE EXAMPLES
// =============================================================================

slay demonstrate_macros() {
    vibez.spill("=== CURSED Macro System Demo ===")
    
    // Debug printing
    sus debug_mode lit = based
    debug_print!("Starting macro demonstration")
    
    // Mathematical operations
    sus result1 = math_op!(add 5, 3)
    sus result2 = math_op!(mul 4, 7)
    sus result3 = math_op!(div 10, 2)
    
    vibez.spill("Math results:", result1, result2, result3)
    
    // Vector creation
    sus empty_vec = vec!()
    sus num_vec = vec![1, 2, 3, 4, 5]
    sus repeat_vec = vec![42; 10]
    
    vibez.spill("Vectors created:", empty_vec.len(), num_vec.len(), repeat_vec.len())
    
    // Conditional compilation
    ready (cfg!(debug)) {
        vibez.spill("Running in debug mode")
    }
    
    // Pattern matching
    sus value drip = 42
    sus match_result = pattern_match!(value;
        0 => "zero",
        1..10 => "small",
        x when x > 100 => "large",
        _ => "medium"
    )
    
    vibez.spill("Pattern match result:", match_result)
    
    // Retry operations
    retry!(3, {
        sus random_fail = random() < 0.5
        ready (random_fail) {
            yikes "Random failure for retry demo"
        }
        vibez.spill("Operation succeeded!")
    })
    
    // Assertions
    assert!(result1 == 8, "Addition should work correctly")
    assert!(num_vec.len() == 5)
    
    vibez.spill("=== Macro Demo Complete ===")
}

// Struct with derived traits
@derive_json
@derive_debug
@derive_eq
squad User {
    id: drip,
    name: tea,
    email: tea,
    active: lit,
}

// Test cases using macros
test_case!("user_serialization", {
    sus user = User{
        id: 1,
        name: "Alice",
        email: "alice@example.com",
        active: based,
    }
    
    sus json = user.to_json()
    sus parsed = User.from_json(json)
    
    assert!(user.equals(parsed), "Serialization should be round-trip safe")
})

benchmark!("vector_operations", 1000, {
    sus v = vec![1, 2, 3, 4, 5]
    v.push(6)
    _ = v.pop()
})

// Main function to run examples
slay main() {
    demonstrate_macros()
    
    // Run tests
    test_user_serialization()
    
    // Run benchmarks
    bench_vector_operations()
}
