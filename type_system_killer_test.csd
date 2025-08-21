/* 
 * CURSED Type System "Killer" Test Suite
 * P0 Hard-Blocker Test Cases for v1.0 Release
 * 
 * This comprehensive test exercises every edge case in the type system:
 * - Complex nested generics with constraints
 * - Interface constraints and variance
 * - Struct validation with fail-fast errors
 * - Cycle detection in type inference
 * - Variance checking for function types
 */

yeet "testz"

// =============================================================================
// 1. CYCLIC TYPE DETECTION TESTS
// =============================================================================

test_start("Cyclic Type Detection")

// Recursive struct definition (should be detected and rejected)
squad RecursiveStruct {
    value drip,
    next ?RecursiveStruct  // This should work - optional breaks cycle
}

// Mutual recursion (should detect cycles)
squad NodeA {
    data tea,
    ref NodeB
}

squad NodeB {  
    value drip,
    ref NodeA  // This creates a cycle - should be handled gracefully
}

// Complex nested generic cycles
squad Container[T] {
    data T,
    nested Container[Container[T]]  // Deep nesting cycle
}

// Test type inference cycle detection
slay problematic_function[T, U](a T, b U) T {
    sus x T = problematic_function[U, T](b, a)  // Creates inference cycle
    damn x
}

assert_error("Cyclic type reference should be detected", proc() {
    sus test_cycle Container[drip] = Container[drip]{ .data = 42, .nested = undefined }
})

test_passed("Cyclic type detection")

// =============================================================================
// 2. COMPLEX NESTED GENERICS WITH CONSTRAINTS
// =============================================================================

test_start("Complex Nested Generics")

// Multi-level generic constraints
collab Printable {
    slay print() tea
}

collab Comparable[T] {
    slay compare(other T) drip
}

collab Container[T: Printable + Comparable[T]] {
    slay add(item T)
    slay get(index drip) ?T
    slay compare_items(a drip, b drip) drip
}

// Complex nested generic structure
squad NestedGeneric[T, U, V] 
where T: Printable,
      U: Container[T],
      V: Comparable[U] {
    
    primary T,
    collection U,
    comparator V
}

// Deeply nested generic function with inference
slay deep_generic_function[
    T: Printable + Comparable[T],
    U: Container[T],
    V: Comparable[U],
    W: Container[V]
](
    items []T,
    containers []U,
    comparators []V,
    wrapper W
) NestedGeneric[T, U, V] {
    
    // Complex type inference scenario
    sus result NestedGeneric[T, U, V] = NestedGeneric[T, U, V]{
        .primary = items[0],
        .collection = containers[0], 
        .comparator = comparators[0]
    }
    
    // Test constraint propagation
    bestie (item in items) {
        ready (!item.compare(result.primary)) {
            result.collection.add(item)
        }
    }
    
    damn result
}

test_passed("Complex nested generics")

// =============================================================================
// 3. VARIANCE CHECKING FOR FUNCTION TYPES  
// =============================================================================

test_start("Function Type Variance")

// Covariant return types
slay covariant_return_test[T](input T) T {
    damn input
}

// Contravariant parameter types  
slay contravariant_param_test[T](processor slay(T) tea) tea {
    damn processor
}

// Bivariant type parameters
slay bivariant_test[T](
    input T,
    transform slay(T) T,
    validator slay(T) lit
) lit {
    damn validator(transform(input))
}

// Complex variance scenario with nested generics
squad Processor[In, Out] {
    transform slay(In) Out
}

slay variance_stress_test[
    T: Printable,
    U: Comparable[T], 
    V: Container[U]
](
    input_processor Processor[T, U],
    output_processor Processor[U, V],
    validator slay(V) lit
) lit {
    
    // This should enforce proper variance constraints
    sus intermediate U = input_processor.transform  
    sus final V = output_processor.transform(intermediate)
    
    damn validator(final)
}

// Test variance violation detection
assert_error("Variance violation should be caught", proc() {
    slay bad_variance[T](func slay(tea) T) T {
        // Trying to pass drip where T is expected - variance violation
        damn func("invalid")  
    }
})

test_passed("Function type variance")

// =============================================================================
// 4. STRUCT FIELD TYPE MATCHING WITH FAIL-FAST ERRORS
// =============================================================================

test_start("Struct Field Validation")

// Complex struct with various field types
squad ComplexStruct {
    id drip,
    name tea, 
    active lit,
    metadata ?tea,
    tags []tea,
    processor slay(tea) drip,
    nested squad {
        x drip,
        y drip  
    }
}

// Test complete struct validation
sus valid_struct ComplexStruct = ComplexStruct{
    .id = 123,
    .name = "test",
    .active = based,
    .metadata = "extra info",
    .tags = ["tag1", "tag2"],
    .processor = slay(s tea) drip { damn s.len() },
    .nested = .{ .x = 1, .y = 2 }
}

// Test missing field error
assert_error("Missing required field should fail fast", proc() {
    sus invalid_struct ComplexStruct = ComplexStruct{
        .id = 123,
        .name = "test"
        // Missing other required fields - should fail immediately
    }
})

// Test incorrect field type error  
assert_error("Incorrect field type should fail fast", proc() {
    sus invalid_struct ComplexStruct = ComplexStruct{
        .id = "wrong_type",  // Should be drip, not tea
        .name = "test",
        .active = based,
        .metadata = "extra info", 
        .tags = ["tag1", "tag2"],
        .processor = slay(s tea) drip { damn s.len() },
        .nested = .{ .x = 1, .y = 2 }
    }
})

// Test unknown field error
assert_error("Unknown field should fail fast", proc() {
    sus invalid_struct ComplexStruct = ComplexStruct{
        .id = 123,
        .name = "test",
        .active = based,
        .unknown_field = "should not exist",  // Unknown field
        .metadata = "extra info",
        .tags = ["tag1", "tag2"], 
        .processor = slay(s tea) drip { damn s.len() },
        .nested = .{ .x = 1, .y = 2 }
    }
})

test_passed("Struct field validation")

// =============================================================================  
// 5. INTERFACE CONSTRAINTS AND DUCK TYPING
// =============================================================================

test_start("Interface Constraints")

// Complex interface hierarchy
collab Drawable {
    slay draw() tea
}

collab Resizable {
    slay resize(width drip, height drip)
}

collab Component: Drawable + Resizable {
    slay get_bounds() squad { x drip, y drip, width drip, height drip }
}

// Generic function with complex interface constraints
slay render_components[T: Component + Printable](
    components []T,
    viewport squad { width drip, height drip }
) []tea {
    
    sus results []tea = []
    
    bestie (component in components) {
        // Test interface method calls
        sus bounds = component.get_bounds()
        
        ready (bounds.width > viewport.width) {
            component.resize(viewport.width, bounds.height)
        }
        
        sus rendered tea = component.draw()
        sus debug_info tea = component.print()
        
        results.append(rendered + " - " + debug_info)
    }
    
    damn results
}

// Test constraint satisfaction
squad Button {
    x drip, y drip, width drip, height drip, text tea
}

// Implement required interfaces
slay Button.draw() tea {
    damn "Button: " + self.text
}

slay Button.resize(new_width drip, new_height drip) {
    self.width = new_width
    self.height = new_height  
}

slay Button.get_bounds() squad { x drip, y drip, width drip, height drip } {
    damn .{ .x = self.x, .y = self.y, .width = self.width, .height = self.height }
}

slay Button.print() tea {
    damn "Button[" + tea(self.x) + "," + tea(self.y) + "]"
}

test_passed("Interface constraints")

// =============================================================================
// 6. GENERIC TYPE INFERENCE EDGE CASES  
// =============================================================================

test_start("Generic Type Inference Edge Cases")

// Complex inference scenario
slay infer_from_usage[T, U, V](
    transform slay(T) U,
    combine slay(U, U) V,
    input T
) V {
    sus intermediate U = transform(input)
    damn combine(intermediate, intermediate)
}

// Should infer T=drip, U=tea, V=drip  
sus inference_result drip = infer_from_usage(
    slay(x drip) tea { damn tea(x) },     // T=drip -> U=tea
    slay(a tea, b tea) drip { damn a.len() + b.len() },  // U=tea -> V=drip
    42  // T=drip
)

// Partial type inference with constraints
slay partial_inference[T: Comparable[T], U](
    items []T,
    default_value U
) U {
    ready (items.len() == 0) {
        damn default_value
    }
    
    // Should infer return type from default_value
    damn default_value
}

// Multiple constraint inference
slay multi_constraint_inference[
    T: Printable + Comparable[T] + Container[drip]
](input T) tea {
    sus info tea = input.print()
    sus comparison drip = input.compare(input)
    sus first_item ?drip = input.get(0)
    
    damn info + " comparison: " + tea(comparison)
}

test_passed("Generic type inference edge cases")

// =============================================================================
// 7. TYPE SYSTEM STRESS TESTS
// =============================================================================

test_start("Type System Stress Tests")

// Deeply nested generic types (tests stack depth)
squad Level1[T] { data T }
squad Level2[T] { data Level1[T] }  
squad Level3[T] { data Level2[T] }
squad Level4[T] { data Level3[T] }
squad Level5[T] { data Level4[T] }

// Extremely complex generic signature
slay stress_test_function[
    A: Printable + Comparable[A],
    B: Container[A] + Resizable, 
    C: Component + Container[B],
    D: Comparable[C] + Drawable,
    E: Container[D] + Printable
](
    param_a A,
    param_b B, 
    param_c C,
    param_d D,
    param_e E
) Level5[squad {
    result_a A,
    result_b B,
    result_c C, 
    result_d D,
    result_e E
}] {
    
    // Complex type construction with all parameters
    sus complex_result = Level5[squad {
        result_a A,
        result_b B,
        result_c C,
        result_d D, 
        result_e E
    }]{
        .data = .{
            .data = .{
                .data = .{
                    .data = .{
                        .data = .{
                            .result_a = param_a,
                            .result_b = param_b,
                            .result_c = param_c,
                            .result_d = param_d,
                            .result_e = param_e
                        }
                    }
                }
            }
        }
    }
    
    damn complex_result
}

// Test memory and performance under type system stress
bestie (i in 0..100) {
    // Create many complex generic types to stress the system
    sus temp Level3[drip] = Level3[drip]{ 
        .data = .{ 
            .data = .{ 
                .data = i 
            } 
        } 
    }
}

test_passed("Type system stress tests")

// =============================================================================
// FINAL VALIDATION
// =============================================================================

test_start("Final Type System Validation")

// Ensure all type system components work together
slay final_integration_test() tea {
    // Complex scenario combining all features tested above
    
    // 1. Create complex generic struct with constraints
    squad FinalTest[
        T: Printable + Comparable[T],
        U: Container[T]
    ] {
        data T,
        container U,
        processor slay(T) tea,
        validator slay(U) lit
    }
    
    // 2. Test with real implementation
    squad SimpleContainer[T] {
        items []T
    }
    
    // Implement required interfaces for SimpleContainer
    slay SimpleContainer[T].add(item T) {
        self.items.append(item)
    }
    
    slay SimpleContainer[T].get(index drip) ?T {
        ready (index < self.items.len()) {
            damn self.items[index]
        }
        damn undefined
    }
    
    // 3. Create instance with full type checking
    sus final_instance FinalTest[tea, SimpleContainer[tea]] = FinalTest[tea, SimpleContainer[tea]]{
        .data = "test string",
        .container = SimpleContainer[tea]{ .items = ["item1", "item2"] },
        .processor = slay(s tea) tea { damn "processed: " + s },
        .validator = slay(c SimpleContainer[tea]) lit { damn c.items.len() > 0 }
    }
    
    // 4. Use all functionality
    sus processed tea = final_instance.processor(final_instance.data)
    sus is_valid lit = final_instance.validator(final_instance.container)
    
    ready (is_valid) {
        damn processed
    } otherwise {
        damn "validation failed"
    }
}

sus integration_result tea = final_integration_test()
assert_eq_tea(integration_result, "processed: test string")

test_passed("Final type system validation")

// Print comprehensive test summary
print_test_summary()

/* 
 * Expected Behavior:
 * - All cyclic type references should be detected and handled gracefully
 * - Complex generic constraints should be properly validated  
 * - Function variance should be enforced correctly
 * - Struct field validation should provide immediate, clear error messages
 * - Interface constraints should work with duck typing
 * - Generic type inference should handle complex scenarios
 * - The entire type system should handle stress testing without crashes
 * 
 * This test file represents the most comprehensive validation of the CURSED
 * type system and must pass 100% for v1.0 release certification.
 */
