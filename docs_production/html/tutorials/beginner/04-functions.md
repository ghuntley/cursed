# Functions

Functions are the building blocks of CURSED programs. This tutorial covers function definitions, parameters, return values, and best practices.

## Learning Objectives

By the end of this tutorial, you'll be able to:
- Define functions with the `slay` keyword
- Use parameters and return values effectively
- Understand function scope and visibility
- Write recursive functions
- Follow function best practices

## Function Basics

Functions in CURSED are defined using the `slay` keyword:

```cursed
# Basic function syntax
slay function_name(parameters) return_type {
    # function body
    damn return_value
}
```

### Simple Function Example

```cursed
# simple_function.csd - Basic function examples

vibe main

yeet "vibez"

# Function with no parameters and no return value
slay greet() {
    vibez.spill("Hello from a function!")
}

# Function with parameters
slay greet_person(name tea) {
    vibez.spill("Hello, " + name + "!")
}

# Function with return value
slay get_greeting() tea {
    damn "Hello from get_greeting!"
}

# Entry point
slay main() {
    greet()
    greet_person("Alice")
    
    sus message := get_greeting()
    vibez.spill(message)
}
```

## Function Parameters

### Single Parameter

```cursed
# parameters.csd - Function parameters

vibe main

yeet "vibez"

slay square(x normie) normie {
    damn x * x
}

slay describe_age(age normie) {
    lowkey age < 18 {
        vibez.spill("You're a minor")
    } highkey lowkey age < 65 {
        vibez.spill("You're an adult")
    } highkey {
        vibez.spill("You're a senior")
    }
}

slay main() {
    sus result := square(5)
    vibez.spill("Square of 5: " + string(result))
    
    describe_age(25)
    describe_age(70)
}
```

### Multiple Parameters

```cursed
# multiple_params.csd - Functions with multiple parameters

vibe main

yeet "vibez"

# Function with multiple parameters of same type
slay add(a normie, b normie) normie {
    damn a + b
}

# Function with multiple parameters of different types
slay introduce(name tea, age normie, is_student lit) {
    vibez.spill("Name: " + name)
    vibez.spill("Age: " + string(age))
    
    lowkey is_student {
        vibez.spill("Status: Student")
    } highkey {
        vibez.spill("Status: Not a student")
    }
}

# Function with many parameters
slay calculate_grade(test1, test2, test3, test4, test5 snack) snack {
    sus total := test1 + test2 + test3 + test4 + test5
    damn total / 5.0
}

slay main() {
    sus sum := add(10, 20)
    vibez.spill("Sum: " + string(sum))
    
    introduce("Bob", 22, based)
    
    sus average := calculate_grade(85.5, 92.0, 78.5, 88.0, 95.5)
    vibez.spill("Average grade: " + string(average))
}
```

## Return Values

### Single Return Value

```cursed
# return_single.csd - Single return values

vibe main

yeet "vibez"

slay multiply(a normie, b normie) normie {
    damn a * b
}

slay is_even(n normie) lit {
    damn n % 2 == 0
}

slay get_absolute(n normie) normie {
    lowkey n < 0 {
        damn -n
    }
    damn n
}

slay main() {
    sus product := multiply(6, 7)
    vibez.spill("Product: " + string(product))
    
    sus even_check := is_even(10)
    vibez.spill("Is 10 even? " + string(even_check))
    
    sus abs_value := get_absolute(-15)
    vibez.spill("Absolute value of -15: " + string(abs_value))
}
```

### Multiple Return Values

```cursed
# return_multiple.csd - Multiple return values

vibe main

yeet "vibez"

# Function returning multiple values
slay divide_with_remainder(a normie, b normie) (normie, normie) {
    sus quotient := a / b
    sus remainder := a % b
    damn quotient, remainder
}

# Function returning result and error status
slay safe_divide(a normie, b normie) (meal, lit) {
    lowkey b == 0 {
        damn 0.0, cap  # Return 0 and false (error)
    }
    damn meal(a) / meal(b), based  # Return result and true (success)
}

# Function returning multiple different types
slay get_user_info(user_id normie) (tea, normie, lit) {
    # Simulate database lookup
    vibe_check user_id {
        mood 1:
            damn "Alice", 25, based
        mood 2:
            damn "Bob", 30, cap
        basic:
            damn "Unknown", 0, cap
    }
}

slay main() {
    sus quotient, remainder := divide_with_remainder(17, 5)
    vibez.spill("17 ÷ 5 = " + string(quotient) + " remainder " + string(remainder))
    
    sus result, success := safe_divide(10, 2)
    lowkey success {
        vibez.spill("Division result: " + string(result))
    } highkey {
        vibez.spill("Division failed!")
    }
    
    sus name, age, is_active := get_user_info(1)
    vibez.spill("User: " + name + ", Age: " + string(age) + ", Active: " + string(is_active))
}
```

## Function Scope and Variables

### Local Variables

```cursed
# scope.csd - Variable scope in functions

vibe main

yeet "vibez"

# Global variable (accessible everywhere)
sus global_counter normie = 0

slay increment_counter() {
    # Local variable (only accessible in this function)
    sus local_increment normie = 1
    global_counter = global_counter + local_increment
    
    vibez.spill("Counter incremented by: " + string(local_increment))
}

slay calculate_area(radius meal) meal {
    # Local constant
    facts PI = 3.14159
    
    # Local variable
    sus area := PI * radius * radius
    
    vibez.spill("Calculating area with radius: " + string(radius))
    damn area
}

slay main() {
    vibez.spill("Initial counter: " + string(global_counter))
    
    increment_counter()
    vibez.spill("Counter after increment: " + string(global_counter))
    
    sus circle_area := calculate_area(5.0)
    vibez.spill("Circle area: " + string(circle_area))
}
```

### Parameter Passing

```cursed
# parameter_passing.csd - How parameters are passed

vibe main

yeet "vibez"

slay modify_value(x normie) {
    x = x + 10  # This only changes the local copy
    vibez.spill("Inside function, x = " + string(x))
}

slay process_array(arr [3]normie) {
    arr[0] = 100  # This modifies the local copy
    vibez.spill("Inside function, arr[0] = " + string(arr[0]))
}

slay main() {
    sus original_value normie = 5
    vibez.spill("Before function call: " + string(original_value))
    
    modify_value(original_value)
    vibez.spill("After function call: " + string(original_value))
    
    sus numbers [3]normie = [3]normie{1, 2, 3}
    vibez.spill("Before array function: " + string(numbers[0]))
    
    process_array(numbers)
    vibez.spill("After array function: " + string(numbers[0]))
}
```

## Advanced Function Concepts

### Recursive Functions

```cursed
# recursion.csd - Recursive functions

vibe main

yeet "vibez"

# Factorial function using recursion
slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    damn n * factorial(n - 1)
}

# Fibonacci sequence
slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

# Countdown function
slay countdown(n normie) {
    lowkey n <= 0 {
        vibez.spill("Blast off! 🚀")
        damn
    }
    
    vibez.spill(string(n))
    countdown(n - 1)
}

slay main() {
    sus fact := factorial(5)
    vibez.spill("5! = " + string(fact))
    
    sus fib := fibonacci(7)
    vibez.spill("7th Fibonacci number: " + string(fib))
    
    vibez.spill("Countdown:")
    countdown(5)
}
```

### Helper Functions

```cursed
# helpers.csd - Helper functions for organization

vibe main

yeet "vibez"

# Helper function for validation
slay is_valid_age(age normie) lit {
    damn age >= 0 && age <= 150
}

# Helper function for formatting
slay format_currency(amount meal) tea {
    damn "$" + string(amount)
}

# Helper function for calculations
slay calculate_tax(amount meal, rate meal) meal {
    damn amount * rate
}

# Main business logic function
slay process_purchase(item_name tea, price meal, customer_age normie) {
    # Validate inputs using helper
    lowkey !is_valid_age(customer_age) {
        vibez.spill("Error: Invalid age")
        damn
    }
    
    # Calculate tax using helper
    sus tax := calculate_tax(price, 0.08)
    sus total := price + tax
    
    # Format output using helper
    vibez.spill("Item: " + item_name)
    vibez.spill("Price: " + format_currency(price))
    vibez.spill("Tax: " + format_currency(tax))
    vibez.spill("Total: " + format_currency(total))
    
    # Age-based discount
    lowkey customer_age >= 65 {
        sus discount := total * 0.1
        sus discounted_total := total - discount
        vibez.spill("Senior discount: " + format_currency(discount))
        vibez.spill("Final total: " + format_currency(discounted_total))
    }
}

slay main() {
    process_purchase("Coffee", 4.50, 25)
    vibez.spill("---")
    process_purchase("Book", 15.99, 70)
}
```

## Exercise: Library Management System

Create a program called `library.csd` that implements a simple library management system with the following functions:

1. `calculate_fine(days_overdue normie) meal` - Calculate fine for overdue books
2. `get_book_category(book_id normie) tea` - Get book category by ID
3. `is_member_eligible(member_age normie, membership_years normie) lit` - Check borrowing eligibility
4. `process_book_return(book_id normie, days_overdue normie, member_age normie) (tea, meal)` - Process book return

### Solution

```cursed
# library.csd - Library management system

vibe main

yeet "vibez"

# Constants for library rules
facts (
    DAILY_FINE_RATE = 0.50
    MAX_FINE = 10.0
    MIN_AGE_FOR_ADULT_BOOKS = 18
    MIN_MEMBERSHIP_YEARS = 1
)

# Calculate fine for overdue books
slay calculate_fine(days_overdue normie) meal {
    lowkey days_overdue <= 0 {
        damn 0.0
    }
    
    sus fine := meal(days_overdue) * DAILY_FINE_RATE
    
    # Cap the fine at maximum amount
    lowkey fine > MAX_FINE {
        damn MAX_FINE
    }
    
    damn fine
}

# Get book category by ID
slay get_book_category(book_id normie) tea {
    vibe_check book_id {
        mood 1, 2, 3:
            damn "Children"
        mood 4, 5, 6:
            damn "Young Adult"
        mood 7, 8, 9:
            damn "Adult Fiction"
        mood 10, 11, 12:
            damn "Non-Fiction"
        basic:
            damn "Unknown"
    }
}

# Check if member is eligible to borrow
slay is_member_eligible(member_age normie, membership_years normie) lit {
    # Must be at least 13 years old and have membership for minimum time
    damn member_age >= 13 && membership_years >= MIN_MEMBERSHIP_YEARS
}

# Check if member can borrow specific book category
slay can_borrow_category(member_age normie, category tea) lit {
    vibe_check category {
        mood "Children":
            damn based  # Anyone can borrow children's books
        mood "Young Adult":
            damn member_age >= 13
        mood "Adult Fiction", "Non-Fiction":
            damn member_age >= MIN_AGE_FOR_ADULT_BOOKS
        basic:
            damn cap  # Unknown category
    }
}

# Process book return and calculate final amounts
slay process_book_return(book_id normie, days_overdue normie, member_age normie) (tea, meal) {
    sus category := get_book_category(book_id)
    sus fine := calculate_fine(days_overdue)
    
    # Create return message
    sus status tea
    lowkey days_overdue <= 0 {
        status = "Returned on time"
    } highkey lowkey days_overdue <= 7 {
        status = "Returned late"
    } highkey {
        status = "Severely overdue"
    }
    
    damn status + " - Category: " + category, fine
}

# Display member information
slay display_member_info(name tea, age normie, membership_years normie) {
    vibez.spill("=== Member Information ===")
    vibez.spill("Name: " + name)
    vibez.spill("Age: " + string(age))
    vibez.spill("Membership Years: " + string(membership_years))
    
    sus eligible := is_member_eligible(age, membership_years)
    vibez.spill("Eligible to borrow: " + string(eligible))
    
    # Show what categories they can borrow
    vibez.spill("Can borrow:")
    sus categories := []tea{"Children", "Young Adult", "Adult Fiction", "Non-Fiction"}
    
    # Note: This is simplified - in real code we'd iterate through the array
    lowkey can_borrow_category(age, "Children") {
        vibez.spill("  - Children's books")
    }
    lowkey can_borrow_category(age, "Young Adult") {
        vibez.spill("  - Young Adult books")
    }
    lowkey can_borrow_category(age, "Adult Fiction") {
        vibez.spill("  - Adult Fiction")
    }
    lowkey can_borrow_category(age, "Non-Fiction") {
        vibez.spill("  - Non-Fiction")
    }
    
    vibez.spill("")
}

slay main() {
    # Display library information
    vibez.spill("=== Library Management System ===")
    vibez.spill("Daily fine rate: $" + string(DAILY_FINE_RATE))
    vibez.spill("Maximum fine: $" + string(MAX_FINE))
    vibez.spill("")
    
    # Test with different members
    display_member_info("Alice Johnson", 25, 3)
    display_member_info("Bobby Smith", 15, 2)
    display_member_info("Charlie Brown", 8, 1)
    
    # Process some book returns
    vibez.spill("=== Book Returns ===")
    
    sus return_status, fine := process_book_return(1, 0, 25)
    vibez.spill("Book ID 1: " + return_status)
    vibez.spill("Fine: $" + string(fine))
    vibez.spill("")
    
    return_status, fine = process_book_return(7, 5, 25)
    vibez.spill("Book ID 7: " + return_status)
    vibez.spill("Fine: $" + string(fine))
    vibez.spill("")
    
    return_status, fine = process_book_return(10, 25, 30)
    vibez.spill("Book ID 10: " + return_status)
    vibez.spill("Fine: $" + string(fine))
}
```

## Common Function Mistakes

### 1. Missing Return Statement
```cursed
# ❌ Wrong - function should return a value
slay get_double(x normie) normie {
    x * 2  # Missing 'damn' keyword
}

# ✅ Correct
slay get_double(x normie) normie {
    damn x * 2
}
```

### 2. Incorrect Parameter Types
```cursed
# ❌ Wrong - type mismatch
slay add_numbers(a normie, b tea) normie {
    damn a + b  # Can't add int and string
}

# ✅ Correct
slay add_numbers(a normie, b normie) normie {
    damn a + b
}
```

### 3. Unused Return Values
```cursed
# ❌ Wrong - ignoring return value
slay main() {
    calculate_area(5.0)  # Result is lost
}

# ✅ Correct
slay main() {
    sus area := calculate_area(5.0)
    vibez.spill("Area: " + string(area))
}
```

## Function Best Practices

1. **Use descriptive names**: `calculate_monthly_payment()` instead of `calc()`
2. **Keep functions focused**: Each function should do one thing well
3. **Use appropriate return types**: Return meaningful values or error indicators
4. **Validate parameters**: Check for valid input ranges
5. **Document complex functions**: Add comments for non-obvious logic
6. **Avoid deep nesting**: Use early returns to reduce complexity
7. **Use helper functions**: Break complex logic into smaller, reusable pieces

## What's Next?

Now that you understand functions, let's explore control flow in the next tutorial: [Control Flow](05-control-flow.md).

## Key Takeaways

- Functions are defined with `slay` and can have parameters and return values
- Use `damn` to return values from functions
- Functions can return multiple values using comma separation
- Parameters are passed by value (copied) in CURSED
- Recursive functions can call themselves to solve complex problems
- Helper functions improve code organization and reusability
- Always validate inputs and handle edge cases
- Good function design makes code more maintainable and testable

Functions are essential for organizing code and creating reusable components in CURSED!
