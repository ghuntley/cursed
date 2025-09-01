vibe main
yeet "vibez"
yeet "collections"

fr fr Comprehensive validation test for the collections stdlib module
fr fr Tests array and collection operations and edge cases
fr fr Expected: Collection operations with proper manipulation results

slay main_character() lit {
    vibez.spill("=== COLLECTIONS MODULE VALIDATION ===")
    
    fr fr Test array creation and length
    sus numbers []normie = []normie{1, 2, 3, 4, 5}
    sus array_len normie = collections.length(numbers)
    vibez.spill("Array length test:")
    vibez.spill(array_len)
    
    fr fr Test array access
    sus first_element normie = collections.get(numbers, 0)
    sus last_element normie = collections.get(numbers, 4)
    vibez.spill("First element:")
    vibez.spill(first_element)
    vibez.spill("Last element:")
    vibez.spill(last_element)
    
    fr fr Test array modification
    collections.set(numbers, 2, 99)
    sus modified_element normie = collections.get(numbers, 2)
    vibez.spill("Modified element at index 2:")
    vibez.spill(modified_element)
    
    fr fr Test array push
    collections.push(numbers, 42)
    sus new_length normie = collections.length(numbers)
    vibez.spill("Length after push:")
    vibez.spill(new_length)
    
    fr fr Test array contains
    sus has_value normie = collections.contains(numbers, 99)
    vibez.spill("Contains 99:")
    vibez.spill(has_value)
    
    fr fr Test array sum
    sus total normie = collections.sum(numbers)
    vibez.spill("Sum of array:")
    vibez.spill(total)
    
    fr fr Test empty array
    sus empty_array []normie = []normie{}
    sus empty_len normie = collections.length(empty_array)
    vibez.spill("Empty array length:")
    vibez.spill(empty_len)
    
    vibez.spill("=== COLLECTIONS VALIDATION COMPLETE ===")
    damn based
}
