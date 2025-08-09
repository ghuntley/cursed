# ARRAYZ Module - Array Operations

Essential array manipulation functions for CURSED programs.

## Functions

### Array Arithmetic
- `sum_array(nums []drip) drip` - Sum all elements
- `average_array(nums []drip) drip` - Calculate average
- `product_array(nums []drip) drip` - Multiply all elements

### Search Operations
- `find_max(nums []drip) drip` - Find maximum value
- `find_min(nums []drip) drip` - Find minimum value
- `contains_value(nums []drip, value drip) lit` - Check if value exists
- `find_index(nums []drip, value drip) drip` - Find index of value (-1 if not found)

### Validation
- `is_empty_array(nums []drip) lit` - Check if array is empty
- `array_size(nums []drip) drip` - Get array size (same as len())
- `arrays_equal_size(a []drip, b []drip) lit` - Check if arrays same size
- `is_valid_index(nums []drip, index drip) lit` - Check if index is valid

### Counting
- `count_positive(nums []drip) drip` - Count positive numbers
- `count_negative(nums []drip) drip` - Count negative numbers
- `count_zeros(nums []drip) drip` - Count zeros
- `count_occurrences(nums []drip, value drip) drip` - Count occurrences of value

### Safe Operations
- `safe_get(nums []drip, index drip, default drip) drip` - Get with default value

### Properties
- `all_positive(nums []drip) lit` - Check if all numbers positive
- `all_negative(nums []drip) lit` - Check if all numbers negative
- `has_duplicates(nums []drip) lit` - Check for duplicate values

### String Arrays
- `join_string_array(strings []tea, separator tea) tea` - Join strings with separator
- `concat_string_array(strings []tea) tea` - Concatenate all strings
- `string_array_contains(strings []tea, value tea) lit` - Check if string exists

## Usage

```cursed
yeet "arrayz"

sus numbers []drip = [1, 2, 3, 4, 5]
sus sum drip = sum_array(numbers)
sus max_val drip = find_max(numbers)
sus has_three lit = contains_value(numbers, 3)
sus positive_count drip = count_positive(numbers)

vibez.spill("Sum:", sum)
vibez.spill("Max:", max_val)
vibez.spill("Has 3:", has_three)
vibez.spill("Positive count:", positive_count)

sus strings []tea = ["hello", "world"]
sus joined tea = join_string_array(strings, " ")
vibez.spill("Joined:", joined)
```
