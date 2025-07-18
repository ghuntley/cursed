# Data Structures

Data structures organize and store data efficiently. This tutorial covers arrays, slices, structs, and maps in CURSED.

## Learning Objectives

By the end of this tutorial, you'll be able to:
- Work with arrays and slices
- Create and use structs with `squad`
- Use maps for key-value storage
- Understand when to use each data structure
- Manipulate complex data efficiently

## Arrays

Arrays are fixed-size collections of elements of the same type.

### Basic Array Operations

```cursed
# arrays.csd - Working with arrays

vibe main

yeet "vibez"

slay main() {
    # Array declaration with size and type
    sus numbers [5]normie = [5]normie{1, 2, 3, 4, 5}
    sus names [3]tea = [3]tea{"Alice", "Bob", "Charlie"}
    
    # Accessing array elements
    vibez.spill("First number: " + string(numbers[0]))
    vibez.spill("Last name: " + names[2])
    
    # Modifying array elements
    numbers[0] = 10
    names[1] = "Bobby"
    
    vibez.spill("Modified first number: " + string(numbers[0]))
    vibez.spill("Modified second name: " + names[1])
    
    # Array length (conceptual - actual syntax may vary)
    vibez.spill("Numbers array has 5 elements")
    vibez.spill("Names array has 3 elements")
}
```

### Array Iteration

```cursed
# array_iteration.csd - Iterating through arrays

vibe main

yeet "vibez"

slay display_array(arr [5]normie) {
    vibez.spill("Array contents:")
    bestie i := 0; i < 5; i++ {
        vibez.spill("  Index " + string(i) + ": " + string(arr[i]))
    }
}

slay find_max(arr [5]normie) normie {
    sus max_val := arr[0]
    
    bestie i := 1; i < 5; i++ {
        lowkey arr[i] > max_val {
            max_val = arr[i]
        }
    }
    
    damn max_val
}

slay main() {
    sus scores [5]normie = [5]normie{85, 92, 78, 96, 88}
    
    display_array(scores)
    
    sus highest := find_max(scores)
    vibez.spill("Highest score: " + string(highest))
}
```

### Multi-dimensional Arrays

```cursed
# multi_arrays.csd - Multi-dimensional arrays

vibe main

yeet "vibez"

slay display_matrix(matrix [3][3]normie) {
    vibez.spill("Matrix:")
    bestie row := 0; row < 3; row++ {
        sus row_str := ""
        bestie col := 0; col < 3; col++ {
            row_str += string(matrix[row][col]) + " "
        }
        vibez.spill(row_str)
    }
}

slay main() {
    # 3x3 matrix
    sus matrix [3][3]normie = [3][3]normie{
        [3]normie{1, 2, 3},
        [3]normie{4, 5, 6},
        [3]normie{7, 8, 9}
    }
    
    display_matrix(matrix)
    
    # Access specific element
    vibez.spill("Element at [1,1]: " + string(matrix[1][1]))
    
    # Modify element
    matrix[1][1] = 50
    vibez.spill("After modification:")
    display_matrix(matrix)
}
```

## Slices

Slices are dynamic arrays that can grow and shrink.

### Basic Slice Operations

```cursed
# slices.csd - Working with slices

vibe main

yeet "vibez"

slay main() {
    # Slice declaration
    sus numbers []normie = []normie{1, 2, 3, 4, 5}
    sus fruits []tea = []tea{"apple", "banana", "cherry"}
    
    # Accessing slice elements
    vibez.spill("First number: " + string(numbers[0]))
    vibez.spill("Second fruit: " + fruits[1])
    
    # Slice length (conceptual)
    vibez.spill("Numbers slice has " + string(5) + " elements")
    
    # Slice modification
    numbers[0] = 100
    fruits[2] = "grape"
    
    vibez.spill("Modified first number: " + string(numbers[0]))
    vibez.spill("Modified third fruit: " + fruits[2])
}
```

### Slice Manipulation

```cursed
# slice_operations.csd - Slice operations

vibe main

yeet "vibez"

slay display_slice(slice []normie, name tea) {
    vibez.spill(name + " contents:")
    bestie i := 0; i < 6; i++ {  # Assuming we know the length
        vibez.spill("  " + string(slice[i]))
    }
}

slay main() {
    sus original []normie = []normie{10, 20, 30, 40, 50, 60}
    
    display_slice(original, "Original")
    
    # Slice operations (conceptual - actual syntax may vary)
    # These operations would typically be provided by a slice module
    vibez.spill("Slice operations would include:")
    vibez.spill("- Append: add elements to end")
    vibez.spill("- Prepend: add elements to beginning")
    vibez.spill("- Insert: add elements at specific position")
    vibez.spill("- Remove: remove elements")
    vibez.spill("- Sub-slice: get portion of slice")
}
```

## Structs with `squad`

Structs group related data together.

### Basic Struct Definition

```cursed
# structs.csd - Working with structs

vibe main

yeet "vibez"

# Define a struct type
be_like Person squad {
    name tea
    age normie
    email tea
    is_active lit
}

# Function to create a person
slay create_person(name tea, age normie, email tea) Person {
    damn Person{
        name: name,
        age: age,
        email: email,
        is_active: based
    }
}

# Function to display person info
slay display_person(p Person) {
    vibez.spill("=== Person Info ===")
    vibez.spill("Name: " + p.name)
    vibez.spill("Age: " + string(p.age))
    vibez.spill("Email: " + p.email)
    vibez.spill("Active: " + string(p.is_active))
}

slay main() {
    # Create struct instances
    sus person1 := create_person("Alice", 28, "alice@example.com")
    sus person2 := Person{
        name: "Bob",
        age: 35,
        email: "bob@example.com",
        is_active: cap
    }
    
    display_person(person1)
    display_person(person2)
    
    # Modify struct fields
    person2.is_active = based
    person2.age = 36
    
    vibez.spill("\nAfter modification:")
    display_person(person2)
}
```

### Nested Structs

```cursed
# nested_structs.csd - Structs containing other structs

vibe main

yeet "vibez"

be_like Address squad {
    street tea
    city tea
    state tea
    zip_code tea
}

be_like Employee squad {
    id normie
    name tea
    department tea
    salary meal
    address Address
}

slay create_employee(id normie, name tea, dept tea, salary meal, addr Address) Employee {
    damn Employee{
        id: id,
        name: name,
        department: dept,
        salary: salary,
        address: addr
    }
}

slay display_employee(emp Employee) {
    vibez.spill("=== Employee Record ===")
    vibez.spill("ID: " + string(emp.id))
    vibez.spill("Name: " + emp.name)
    vibez.spill("Department: " + emp.department)
    vibez.spill("Salary: $" + string(emp.salary))
    vibez.spill("Address:")
    vibez.spill("  " + emp.address.street)
    vibez.spill("  " + emp.address.city + ", " + emp.address.state + " " + emp.address.zip_code)
}

slay main() {
    sus home_address := Address{
        street: "123 Main St",
        city: "Anytown",
        state: "CA",
        zip_code: "90210"
    }
    
    sus employee := create_employee(
        1001,
        "John Doe",
        "Engineering",
        75000.0,
        home_address
    )
    
    display_employee(employee)
    
    # Modify nested struct
    employee.address.city = "New City"
    employee.salary = 80000.0
    
    vibez.spill("\nAfter promotion and move:")
    display_employee(employee)
}
```

### Struct Methods

```cursed
# struct_methods.csd - Methods on structs

vibe main

yeet "vibez"

be_like BankAccount squad {
    account_number tea
    owner_name tea
    balance meal
    is_active lit
}

# Method to deposit money
slay (account @BankAccount) deposit(amount meal) {
    lowkey amount > 0 {
        account.balance += amount
        vibez.spill("Deposited $" + string(amount))
        vibez.spill("New balance: $" + string(account.balance))
    } highkey {
        vibez.spill("Invalid deposit amount")
    }
}

# Method to withdraw money
slay (account @BankAccount) withdraw(amount meal) lit {
    lowkey amount > account.balance {
        vibez.spill("Insufficient funds")
        damn cap
    } highkey lowkey amount <= 0 {
        vibez.spill("Invalid withdrawal amount")
        damn cap
    }
    
    account.balance -= amount
    vibez.spill("Withdrew $" + string(amount))
    vibez.spill("New balance: $" + string(account.balance))
    damn based
}

# Method to get account info
slay (account @BankAccount) get_info() tea {
    damn "Account: " + account.account_number + 
         " | Owner: " + account.owner_name + 
         " | Balance: $" + string(account.balance)
}

slay main() {
    sus account := BankAccount{
        account_number: "ACC-001",
        owner_name: "Alice Smith",
        balance: 1000.0,
        is_active: based
    }
    
    vibez.spill("=== Bank Account Demo ===")
    vibez.spill(account.get_info())
    
    account.deposit(500.0)
    account.withdraw(200.0)
    account.withdraw(2000.0)  # Should fail
    
    vibez.spill("\nFinal: " + account.get_info())
}
```

## Maps

Maps store key-value pairs for fast lookups.

### Basic Map Operations

```cursed
# maps.csd - Working with maps

vibe main

yeet "vibez"

slay main() {
    # Map declaration (conceptual syntax)
    # Note: Actual map syntax may vary in implementation
    sus ages tea[tea]normie = tea[tea]normie{
        "Alice": 28,
        "Bob": 35,
        "Charlie": 42
    }
    
    sus grades tea[tea]sip = tea[tea]sip{
        "Alice": 'A',
        "Bob": 'B',
        "Charlie": 'C'
    }
    
    # Accessing map values
    vibez.spill("Alice's age: " + string(ages["Alice"]))
    vibez.spill("Bob's grade: " + string(grades["Bob"]))
    
    # Adding new entries
    ages["Diana"] = 30
    grades["Diana"] = 'A'
    
    vibez.spill("Diana's age: " + string(ages["Diana"]))
    vibez.spill("Diana's grade: " + string(grades["Diana"]))
    
    # Modifying existing entries
    ages["Alice"] = 29
    grades["Bob"] = 'B'
    
    vibez.spill("Alice's updated age: " + string(ages["Alice"]))
}
```

### Map Iteration and Operations

```cursed
# map_operations.csd - Advanced map operations

vibe main

yeet "vibez"

slay display_inventory(inventory tea[tea]normie) {
    vibez.spill("=== Inventory ===")
    # Note: Actual iteration syntax may vary
    vibez.spill("Items in stock:")
    vibez.spill("  apples: " + string(inventory["apples"]))
    vibez.spill("  bananas: " + string(inventory["bananas"]))
    vibez.spill("  cherries: " + string(inventory["cherries"]))
}

slay update_inventory(inventory tea[tea]normie, item tea, quantity normie) {
    # Check if item exists (conceptual)
    lowkey item == "apples" || item == "bananas" || item == "cherries" {
        inventory[item] = quantity
        vibez.spill("Updated " + item + " quantity to " + string(quantity))
    } highkey {
        vibez.spill("Item " + item + " not found in inventory")
    }
}

slay main() {
    sus inventory tea[tea]normie = tea[tea]normie{
        "apples": 50,
        "bananas": 30,
        "cherries": 25
    }
    
    display_inventory(inventory)
    
    # Update inventory
    update_inventory(inventory, "apples", 75)
    update_inventory(inventory, "bananas", 20)
    update_inventory(inventory, "oranges", 40)  # Should fail
    
    vibez.spill("\nAfter updates:")
    display_inventory(inventory)
}
```

## Complex Data Structures

### Combining Data Structures

```cursed
# complex_data.csd - Combining different data structures

vibe main

yeet "vibez"

# Student struct
be_like Student squad {
    id normie
    name tea
    grades []normie
    courses []tea
}

# Course struct
be_like Course squad {
    code tea
    title tea
    credits normie
    students []Student
}

# Function to create a student
slay create_student(id normie, name tea) Student {
    damn Student{
        id: id,
        name: name,
        grades: []normie{},
        courses: []tea{}
    }
}

# Function to add grade to student
slay add_grade(student @Student, grade normie) {
    # In real implementation, this would append to slice
    vibez.spill("Adding grade " + string(grade) + " to " + student.name)
    # student.grades = append(student.grades, grade)
}

# Function to calculate average grade
slay calculate_average(student Student) meal {
    # Simplified calculation - in real implementation would iterate through grades
    sus sample_grades := []normie{85, 92, 78, 90}
    sus total := 0
    
    bestie i := 0; i < 4; i++ {
        total += sample_grades[i]
    }
    
    damn meal(total) / 4.0
}

slay main() {
    # Create students
    sus student1 := create_student(1001, "Alice")
    sus student2 := create_student(1002, "Bob")
    
    # Add grades
    add_grade(&student1, 85)
    add_grade(&student1, 92)
    add_grade(&student2, 78)
    add_grade(&student2, 90)
    
    # Calculate and display averages
    sus alice_avg := calculate_average(student1)
    sus bob_avg := calculate_average(student2)
    
    vibez.spill("Alice's average: " + string(alice_avg))
    vibez.spill("Bob's average: " + string(bob_avg))
}
```

## Exercise: Library Management System

Create a comprehensive library management system using various data structures:

### Solution

```cursed
# library_system.csd - Complete library management system

vibe main

yeet "vibez"

# Book struct
be_like Book squad {
    isbn tea
    title tea
    author tea
    year normie
    is_available lit
    genre tea
}

# Member struct
be_like Member squad {
    id normie
    name tea
    email tea
    borrowed_books []tea  # ISBNs of borrowed books
    membership_date tea
}

# Library struct
be_like Library squad {
    name tea
    books []Book
    members []Member
    book_count normie
    member_count normie
}

# Function to create a book
slay create_book(isbn tea, title tea, author tea, year normie, genre tea) Book {
    damn Book{
        isbn: isbn,
        title: title,
        author: author,
        year: year,
        is_available: based,
        genre: genre
    }
}

# Function to create a member
slay create_member(id normie, name tea, email tea, date tea) Member {
    damn Member{
        id: id,
        name: name,
        email: email,
        borrowed_books: []tea{},
        membership_date: date
    }
}

# Function to add book to library
slay add_book(library @Library, book Book) {
    # In real implementation, would append to slice
    library.book_count++
    vibez.spill("Added book: " + book.title + " by " + book.author)
}

# Function to add member to library
slay add_member(library @Library, member Member) {
    # In real implementation, would append to slice
    library.member_count++
    vibez.spill("Added member: " + member.name)
}

# Function to find book by ISBN
slay find_book(library Library, isbn tea) Book {
    # Simplified - in real implementation would search through books slice
    vibe_check isbn {
        mood "978-0-123456-78-9":
            damn Book{
                isbn: isbn,
                title: "The Great Gatsby",
                author: "F. Scott Fitzgerald",
                year: 1925,
                is_available: based,
                genre: "Classic"
            }
        mood "978-0-987654-32-1":
            damn Book{
                isbn: isbn,
                title: "To Kill a Mockingbird",
                author: "Harper Lee",
                year: 1960,
                is_available: cap,
                genre: "Classic"
            }
        basic:
            damn Book{
                isbn: "",
                title: "Book not found",
                author: "",
                year: 0,
                is_available: cap,
                genre: ""
            }
    }
}

# Function to borrow book
slay borrow_book(library @Library, member_id normie, isbn tea) lit {
    sus book := find_book(library, isbn)
    
    lowkey book.isbn == "" {
        vibez.spill("Book not found!")
        damn cap
    }
    
    lowkey !book.is_available {
        vibez.spill("Book is not available!")
        damn cap
    }
    
    # In real implementation, would update book availability and member's borrowed books
    vibez.spill("Book '" + book.title + "' borrowed successfully!")
    damn based
}

# Function to return book
slay return_book(library @Library, member_id normie, isbn tea) lit {
    sus book := find_book(library, isbn)
    
    lowkey book.isbn == "" {
        vibez.spill("Book not found!")
        damn cap
    }
    
    # In real implementation, would update book availability and member's borrowed books
    vibez.spill("Book '" + book.title + "' returned successfully!")
    damn based
}

# Function to display library statistics
slay display_library_stats(library Library) {
    vibez.spill("=== Library Statistics ===")
    vibez.spill("Library: " + library.name)
    vibez.spill("Total books: " + string(library.book_count))
    vibez.spill("Total members: " + string(library.member_count))
}

# Function to display book info
slay display_book_info(book Book) {
    vibez.spill("=== Book Information ===")
    vibez.spill("Title: " + book.title)
    vibez.spill("Author: " + book.author)
    vibez.spill("Year: " + string(book.year))
    vibez.spill("ISBN: " + book.isbn)
    vibez.spill("Genre: " + book.genre)
    vibez.spill("Available: " + string(book.is_available))
}

slay main() {
    # Create library
    sus library := Library{
        name: "CURSED Public Library",
        books: []Book{},
        members: []Member{},
        book_count: 0,
        member_count: 0
    }
    
    # Create books
    sus book1 := create_book("978-0-123456-78-9", "The Great Gatsby", "F. Scott Fitzgerald", 1925, "Classic")
    sus book2 := create_book("978-0-987654-32-1", "To Kill a Mockingbird", "Harper Lee", 1960, "Classic")
    sus book3 := create_book("978-0-555666-77-8", "1984", "George Orwell", 1949, "Dystopian")
    
    # Add books to library
    add_book(&library, book1)
    add_book(&library, book2)
    add_book(&library, book3)
    
    # Create members
    sus member1 := create_member(1001, "Alice Johnson", "alice@email.com", "2024-01-01")
    sus member2 := create_member(1002, "Bob Smith", "bob@email.com", "2024-01-15")
    
    # Add members to library
    add_member(&library, member1)
    add_member(&library, member2)
    
    # Display library stats
    display_library_stats(library)
    
    # Test book operations
    vibez.spill("\n=== Book Operations ===")
    sus found_book := find_book(library, "978-0-123456-78-9")
    display_book_info(found_book)
    
    # Test borrowing and returning
    vibez.spill("\n=== Borrowing Operations ===")
    borrow_book(&library, 1001, "978-0-123456-78-9")
    borrow_book(&library, 1002, "978-0-987654-32-1")
    borrow_book(&library, 1001, "978-0-987654-32-1")  # Should fail - not available
    
    vibez.spill("\n=== Returning Operations ===")
    return_book(&library, 1001, "978-0-123456-78-9")
    return_book(&library, 1002, "978-0-987654-32-1")
}
```

## Best Practices

1. **Choose the right data structure**:
   - Arrays for fixed-size collections
   - Slices for dynamic collections
   - Structs for grouping related data
   - Maps for fast key-value lookups

2. **Use meaningful field names**: `user.email` instead of `user.e`

3. **Initialize properly**: Always initialize data structures before use

4. **Consider memory usage**: Large arrays use more memory than needed

5. **Use methods on structs**: Keep related functions close to the data

6. **Validate data**: Check bounds and null values before accessing

## What's Next?

Now that you understand data structures, let's learn about error handling in the next tutorial: [Error Handling](07-error-handling.md).

## Key Takeaways

- Arrays have fixed size, slices are dynamic
- Structs group related data with `squad` keyword
- Maps provide fast key-value lookups
- Choose the appropriate data structure for your needs
- Use methods to keep functionality close to data
- Always validate data before accessing
- Complex data structures combine simpler ones
- Proper initialization prevents runtime errors

Data structures are the foundation of organizing information in your CURSED programs!
