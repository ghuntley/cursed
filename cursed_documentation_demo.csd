// CURSED Documentation System Demo
// This file demonstrates the comprehensive documentation generation system

yeet "testz"

fr fr This module demonstrates the CURSED documentation system
fr fr 
fr fr The documentation system provides:
fr fr - Automatic extraction of documentation from source code
fr fr - Multiple output formats (HTML, Markdown, JSON)
fr fr - Live development server with hot reload
fr fr - Documentation coverage analysis
fr fr - Build system integration
fr fr
fr fr Usage:
fr fr   cargo run --bin cursed -- doc generate
fr fr   cargo run --bin cursed -- doc serve
fr fr   cargo run --bin cursed -- doc coverage

fr fr Basic function with complete documentation
fr fr 
fr fr This function adds two numbers together and returns the result.
fr fr It serves as an example of well-documented code.
fr fr
fr fr Parameters:
fr fr   a - The first number to add
fr fr   b - The second number to add
fr fr
fr fr Returns:
fr fr   The sum of a and b
fr fr
fr fr Examples:
fr fr   sus result drip = add_numbers(5, 3)
fr fr   vibez.spill(result) fr fr Outputs: 8
fr fr
fr fr   sus big_result drip = add_numbers(100, 200)
fr fr   vibez.spill(big_result) fr fr Outputs: 300
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

fr fr Function with parameter documentation
fr fr
fr fr Calculates the factorial of a number using recursion.
fr fr
fr fr Parameters:
fr fr   n - The number to calculate factorial for (must be >= 0)
fr fr
fr fr Returns:
fr fr   The factorial of n
fr fr
fr fr Throws:
fr fr   Error if n is negative
fr fr
fr fr Examples:
fr fr   sus fact drip = factorial(5)
fr fr   vibez.spill(fact) fr fr Outputs: 120
slay factorial(n drip) drip {
    bestie n == 0 {
        damn 1
    } yolo {
        damn n * factorial(n - 1)
    }
}

fr fr Global constant with documentation
fr fr
fr fr The maximum number of items allowed in a collection.
fr fr This constant is used throughout the application to enforce limits.
lowkey MAX_ITEMS drip = 1000

fr fr Mutable global variable
fr fr
fr fr Current system status. Can be one of:
fr fr - "initializing"
fr fr - "running" 
fr fr - "stopping"
fr fr - "stopped"
sus current_status tea = "initializing"

fr fr Example struct with documented fields
fr fr
fr fr Represents a user in the system with basic information.
fr fr All fields are required except for the optional middle_name.
slay User {
    fr fr The user's unique identifier
    id drip,
    
    fr fr The user's first name
    first_name tea,
    
    fr fr The user's optional middle name
    middle_name tea?,
    
    fr fr The user's last name
    last_name tea,
    
    fr fr The user's email address (must be valid)
    email tea,
    
    fr fr Whether the user account is currently active
    is_active lit
}

fr fr Method implementation for User struct
impl User {
    fr fr Creates a new user with the given information
    fr fr
    fr fr Parameters:
    fr fr   id - Unique user identifier
    fr fr   first_name - User's first name
    fr fr   last_name - User's last name  
    fr fr   email - User's email address
    fr fr
    fr fr Returns:
    fr fr   A new User instance with default values
    fr fr
    fr fr Examples:
    fr fr   sus user User = User.new(1, "John", "Doe", "john@example.com")
    slay new(id drip, first_name tea, last_name tea, email tea) User {
        damn User {
            id: id,
            first_name: first_name,
            middle_name: facts,
            last_name: last_name,
            email: email,
            is_active: based
        }
    }
    
    fr fr Returns the user's full name
    fr fr
    fr fr Combines first_name, middle_name (if present), and last_name
    fr fr into a single string.
    fr fr
    fr fr Returns:
    fr fr   The user's full name as a string
    fr fr
    fr fr Examples:
    fr fr   sus name tea = user.get_full_name()
    fr fr   vibez.spill(name) fr fr Outputs: "John Doe"
    slay get_full_name(self) tea {
        bestie self.middle_name {
            damn self.first_name + " " + self.middle_name + " " + self.last_name
        } yolo {
            damn self.first_name + " " + self.last_name
        }
    }
    
    fr fr Validates the user's email address
    fr fr
    fr fr Performs basic email validation to ensure the address
    fr fr contains an @ symbol and a domain.
    fr fr
    fr fr Returns:
    fr fr   true if email is valid, false otherwise
    fr fr
    fr fr Examples:
    fr fr   bestie user.validate_email() {
    fr fr       vibez.spill("Email is valid")
    fr fr   }
    slay validate_email(self) lit {
        damn self.email.contains("@") && self.email.contains(".")
    }
}

fr fr Example enum with documented variants
fr fr
fr fr Represents different priority levels for tasks.
fr fr Each variant has a specific meaning and use case.
slay Priority {
    fr fr Lowest priority - can be delayed indefinitely
    Low,
    
    fr fr Normal priority - standard processing
    Normal,
    
    fr fr High priority - should be processed soon
    High,
    
    fr fr Critical priority - must be processed immediately
    Critical
}

fr fr Function demonstrating error handling documentation
fr fr
fr fr Divides two numbers and handles division by zero.
fr fr
fr fr Parameters:
fr fr   dividend - The number to be divided
fr fr   divisor - The number to divide by
fr fr
fr fr Returns:
fr fr   The result of division as a floating point number
fr fr
fr fr Throws:
fr fr   DivisionByZeroError if divisor is zero
fr fr
fr fr Examples:
fr fr   sus result meal = safe_divide(10.0, 2.0)
fr fr   vibez.spill(result) fr fr Outputs: 5.0
fr fr
fr fr   fr fr This will throw an error:
fr fr   fr fr sus error_result meal = safe_divide(10.0, 0.0)
slay safe_divide(dividend meal, divisor meal) meal {
    bestie divisor == 0.0 {
        damn Error("Cannot divide by zero")
    }
    damn dividend / divisor
}

fr fr Async function example
fr fr
fr fr Fetches user data from a remote API asynchronously.
fr fr
fr fr Parameters:
fr fr   user_id - The ID of the user to fetch
fr fr   timeout_ms - Timeout in milliseconds for the request
fr fr
fr fr Returns:
fr fr   A User object if successful
fr fr
fr fr Throws:
fr fr   NetworkError if connection fails
fr fr   TimeoutError if request times out
fr fr   NotFoundError if user doesn't exist
fr fr
fr fr Examples:
fr fr   sus user User = await fetch_user_async(123, 5000)
fr fr   vibez.spill(user.get_full_name())
async slay fetch_user_async(user_id drip, timeout_ms drip) User {
    fr fr Simulate async operation
    await sleep(100)
    
    fr fr Create mock user data
    damn User.new(user_id, "Async", "User", "async@example.com")
}

fr fr Generic function with type parameters
fr fr
fr fr Finds the maximum value in a collection of comparable items.
fr fr
fr fr Type Parameters:
fr fr   T - Must implement the Comparable interface
fr fr
fr fr Parameters:
fr fr   items - Collection of items to search
fr fr
fr fr Returns:
fr fr   The maximum item if collection is not empty
fr fr   None if collection is empty
fr fr
fr fr Examples:
fr fr   sus numbers drip[] = [1, 5, 3, 9, 2]
fr fr   sus max_num drip? = find_max(numbers)
fr fr   vibez.spill(max_num) fr fr Outputs: 9
slay find_max<T>(items T[]) T? {
    bestie items.length == 0 {
        damn facts
    }
    
    sus max_item T = items[0]
    yeet item bestie items {
        bestie item > max_item {
            max_item = item
        }
    }
    damn max_item
}

fr fr Module-level examples and demonstrations
fr fr
fr fr This section shows how to use the various functions and types
fr fr defined in this module together.

fr fr Example: Creating and working with users
slay demo_user_operations() {
    fr fr Create a new user
    sus user User = User.new(1, "Jane", "Smith", "jane@example.com")
    
    fr fr Display user information
    vibez.spill("User: " + user.get_full_name())
    vibez.spill("Email valid: " + user.validate_email().str())
    
    fr fr Demonstrate arithmetic
    sus sum drip = add_numbers(user.id, 99)
    vibez.spill("User ID + 99 = " + sum.str())
    
    fr fr Demonstrate factorial
    sus fact drip = factorial(5)
    vibez.spill("Factorial of 5: " + fact.str())
}

fr fr Example: Working with priorities and error handling
slay demo_advanced_features() {
    fr fr Create priority levels
    sus priorities Priority[] = [Priority.Low, Priority.High, Priority.Critical]
    
    fr fr Demonstrate safe division
    sus result1 meal = safe_divide(15.0, 3.0)
    vibez.spill("15 / 3 = " + result1.str())
    
    fr fr This would cause an error (commented out for demo):
    fr fr sus result2 meal = safe_divide(10.0, 0.0)
    
    fr fr Demonstrate generic function
    sus numbers drip[] = [42, 17, 99, 3, 88]
    sus max_number drip? = find_max(numbers)
    bestie max_number {
        vibez.spill("Maximum number: " + max_number.str())
    }
}

fr fr Test the documentation system
test_start("Documentation System Demo")

fr fr Test basic functions
assert_eq_int(add_numbers(2, 3), 5)
assert_eq_int(factorial(4), 24)

fr fr Test user creation and methods
sus test_user User = User.new(123, "Test", "User", "test@example.com")
assert_eq_string(test_user.get_full_name(), "Test User")
assert_true(test_user.validate_email())

fr fr Test safe division
sus division_result meal = safe_divide(20.0, 4.0)
assert_eq_int(division_result.int(), 5)

fr fr Test generic function
sus test_numbers drip[] = [1, 3, 2]
sus max_test_number drip? = find_max(test_numbers)
assert_eq_int(max_test_number, 3)

print_test_summary()

fr fr Run the demonstrations
vibez.spill("=== CURSED Documentation System Demo ===")
demo_user_operations()
vibez.spill("")
demo_advanced_features()
vibez.spill("=== Demo Complete ===")
