fr fr Comprehensive CURSED Standard Library Usage Example
fr fr Demonstrating testz, vibez, mathz, stringz, and collections

yeet "testz"
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "collections"

slay main() cringe {
    vibez.spill("🚀 CURSED Standard Library Comprehensive Demo")
    vibez.spill("================================================")
    
    fr fr === Testing Framework Demo ===
    test_start("Standard Library Integration")
    
    fr fr === Math Operations Demo ===
    vibez.spill("\n📐 Mathematics Module (mathz) Demo:")
    
    sus pi_val meal = mathz.PI
    sus e_val meal = mathz.E
    vibez.spillf("Constants - PI: %f, E: %f", pi_val, e_val)
    
    sus sqrt_25 meal = mathz.sqrt_meal(25.0)
    sus abs_neg meal = mathz.abs_meal(-42.5)
    sus max_val meal = mathz.max_meal(15.0, 7.0)
    vibez.spillf("Functions - sqrt(25): %f, abs(-42.5): %f, max(15,7): %f", sqrt_25, abs_neg, max_val)
    
    sus sin_90 meal = mathz.sin_deg(90.0)
    sus cos_0 meal = mathz.cos_deg(0.0)
    vibez.spillf("Trigonometry - sin(90°): %f, cos(0°): %f", sin_90, cos_0)
    
    sus factorial_5 normie = mathz.factorial(5)
    sus random_num normie = mathz.random_int()
    vibez.spillf("Advanced - factorial(5): %d, random: %d", factorial_5, random_num)
    
    fr fr === String Operations Demo ===
    vibez.spill("\n🔤 String Module (stringz) Demo:")
    
    sus test_str tea = "Hello CURSED World"
    sus str_len normie = stringz.length(test_str)
    sus concat_str tea = stringz.concat("CURSED", " Language")
    vibez.spillf("Length of '%s': %d", test_str, str_len)
    vibez.spillf("Concatenation: %s", concat_str)
    
    sus contains_cursed lit = stringz.contains(test_str, "CURSED")
    sus contains_python lit = stringz.contains(test_str, "Python")
    vibez.spillf("Contains 'CURSED': %b, Contains 'Python': %b", contains_cursed, contains_python)
    
    sus char_at_6 sip = stringz.char_at(test_str, 6)
    sus substring_test tea = stringz.substring(test_str, 0, 5)
    vibez.spillf("Character at index 6: %c, Substring(0,5): %s", char_at_6, substring_test)
    
    fr fr === Collections Demo ===
    vibez.spill("\n📦 Collections Module Demo:")
    
    fr fr Vector operations
    vibez.spill("Vector Operations:")
    sus vec [extra] = collections.Vec_new()
    vec = collections.Vec_push(vec, 42)
    vec = collections.Vec_push(vec, 84)
    vec = collections.Vec_push(vec, 126)
    sus vec_len normie = collections.Vec_len(vec)
    vibez.spillf("Vector length after 3 pushes: %d", vec_len)
    
    sus first_elem extra = collections.Vec_get(vec, 0)
    sus last_elem extra = collections.Vec_pop(vec)
    vibez.spillf("First element: %d, Popped element: %d", first_elem, last_elem)
    
    fr fr HashMap operations
    vibez.spill("HashMap Operations:")
    sus map tea = collections.Map_new()
    map = collections.Map_insert(map, "name", "Alice")
    map = collections.Map_insert(map, "age", "25")
    map = collections.Map_insert(map, "city", "San Francisco")
    
    sus name_val tea = collections.Map_get(map, "name")
    sus age_val tea = collections.Map_get(map, "age")
    sus has_email lit = collections.Map_contains_key(map, "email")
    vibez.spillf("Name: %s, Age: %s, Has email: %b", name_val, age_val, has_email)
    
    fr fr Set operations
    vibez.spill("Set Operations:")
    sus set tea = collections.Set_new()
    set = collections.Set_insert(set, "apple")
    set = collections.Set_insert(set, "banana") 
    set = collections.Set_insert(set, "cherry")
    
    sus has_apple lit = collections.Set_contains(set, "apple")
    sus has_grape lit = collections.Set_contains(set, "grape")
    sus set_size normie = collections.Set_len(set)
    vibez.spillf("Set size: %d, Has apple: %b, Has grape: %b", set_size, has_apple, has_grape)
    
    fr fr Stack operations
    vibez.spill("Stack Operations:")
    sus stack tea = collections.Stack_new()
    stack = collections.Stack_push(stack, "first")
    stack = collections.Stack_push(stack, "second")
    
    sus top_elem tea = collections.Stack_peek(stack)
    sus popped tea = collections.Stack_pop(stack)
    sus is_empty lit = collections.Stack_is_empty(stack)
    vibez.spillf("Top element: %s, Popped: %s, Is empty: %b", top_elem, popped, is_empty)
    
    fr fr Sorting demo
    vibez.spill("Sorting Algorithms:")
    sus unsorted [normie] = [3, 1, 4, 1, 5, 9, 2]
    sus sorted_arr [normie] = collections.Collections_bubble_sort(unsorted)
    sus max_elem normie = collections.Collections_max(unsorted)
    sus min_elem normie = collections.Collections_min(unsorted)
    vibez.spillf("Max: %d, Min: %d", max_elem, min_elem)
    
    fr fr === I/O Operations Demo ===
    vibez.spill("\n💬 I/O Module (vibez) Advanced Demo:")
    
    vibez.spillf("Format test with multiple args: %s is %d years old", "Bob", 30)
    vibez.spill_values("Multiple", "values", "with", "spaces")
    vibez.spill_sep(" | ", "Pipe", "separated", "values")
    
    vibez.spill_colored("This text should be green!", "green")
    vibez.spill_colored("This text should be red!", "red")
    vibez.set_color("reset")
    
    fr fr === Test Assertions ===
    vibez.spill("\n🧪 Running Comprehensive Tests:")
    
    assert_true(mathz.PI > 3.0)
    assert_true(mathz.E > 2.0)
    assert_eq_int(mathz.factorial(5), 120)
    assert_true(mathz.abs_meal(-10.0) == 10.0)
    
    assert_eq_int(stringz.length("test"), 4)
    assert_true(stringz.contains("hello world", "world"))
    assert_false(stringz.contains("hello", "goodbye"))
    
    assert_eq_int(collections.Vec_len([1, 2, 3]), 3)
    assert_true(collections.Set_contains("set_one", "apple"))
    assert_false(collections.Stack_is_empty("stack_one"))
    
    print_test_summary()
    
    vibez.spill("\n✨ Standard Library Demo Complete!")
    vibez.spill("All core modules are working correctly!")
    vibez.spill("🎉 CURSED is ready for production use!")
}
