fr fr Comprehensive CURSED Language Test Suite
fr fr This file tests all major language features using the testz framework

yeet "testz"

fr fr ===== BASIC SYNTAX TESTS =====

test_start("Variable Declarations")
sus name tea = "CURSED"
sus age drip = 42
sus flag lit = based
sus height meal = 5.9

assert_eq_string(name, "CURSED")
assert_eq_int(age, 42)
assert_true(flag)

test_start("Short Variable Declarations")
x := 10
y := "hello"
z := cringe

assert_eq_int(x, 10)
assert_eq_string(y, "hello")
assert_false(z)

test_start("Multiple Variable Assignment")
(a, b, c) := (1, 2, 3)
assert_eq_int(a, 1)
assert_eq_int(b, 2)
assert_eq_int(c, 3)

test_start("Constants")
facts PI = 3.14159
facts MESSAGE = "constant string"
assert_eq_string(MESSAGE, "constant string")

fr fr ===== FUNCTION TESTS =====

slay simple_function() tea {
    damn "working"
}

slay add_numbers(x drip, y drip) drip {
    damn x + y
}

slay multiple_returns() (tea, drip) {
    damn "result", 42
}

test_start("Function Definitions and Calls")
sus result1 = simple_function()
assert_eq_string(result1, "working")

sus sum = add_numbers(5, 3)
assert_eq_int(sum, 8)

(text, number) := multiple_returns()
assert_eq_string(text, "result")
assert_eq_int(number, 42)

fr fr ===== CONTROL FLOW TESTS =====

test_start("If Statements")
sus value drip = 5
sus result tea = ""

lowkey value > 0 {
    result = "positive"
} highkey lowkey value < 0 {
    result = "negative"
} highkey {
    result = "zero"
}

assert_eq_string(result, "positive")

test_start("If Statements with Parentheses")
lowkey (value > 0) {
    result = "still positive"
}
assert_eq_string(result, "still positive")

test_start("Switch Statements")
sus day tea = "Friday"
sus mood tea = ""

vibe_check day {
    mood "Monday", "Tuesday":
        mood = "start of week"
    mood "Friday":
        mood = "end of week"
    basic:
        mood = "mid-week"
}

assert_eq_string(mood, "end of week")

test_start("For Loops - Traditional")
sus total drip = 0
bestie i := 0; i < 5; i = i + 1 {
    total = total + i
}
assert_eq_int(total, 10)

test_start("For Loops - While Style")
sus counter drip = 0
bestie counter < 3 {
    counter = counter + 1
}
assert_eq_int(counter, 3)

test_start("For Loops - Infinite with Break")
sus breakCounter drip = 0
bestie {
    breakCounter = breakCounter + 1
    lowkey breakCounter >= 2 {
        ghosted
    }
}
assert_eq_int(breakCounter, 2)

test_start("While Loops")
sus whileCounter drip = 5
periodt whileCounter > 0 {
    whileCounter = whileCounter - 1
}
assert_eq_int(whileCounter, 0)

fr fr ===== DATA STRUCTURE TESTS =====

squad Person {
    spill name tea
    spill age drip
}

test_start("Struct Declaration and Usage")
sus person Person = Person{name: "Alice", age: 30}
assert_eq_string(person.name, "Alice")
assert_eq_int(person.age, 30)

test_start("Array Operations")
sus numbers []drip = [1, 2, 3, 4, 5]
assert_eq_int(numbers[0], 1)
assert_eq_int(numbers[4], 5)

test_start("Slice Operations")
sus slice []drip = numbers[1:4]
assert_eq_int(slice[0], 2)
assert_eq_int(slice[2], 4)

fr fr ===== TYPE SYSTEM TESTS =====

test_start("Type Conversions")
sus intVal drip = 42
sus floatVal meal = meal(intVal)
sus stringVal tea = tea(intVal)

assert_eq_string(stringVal, "42")

test_start("Type Assertions")
sus value any = "hello"
sus stringAssert tea = value.(tea)
assert_eq_string(stringAssert, "hello")

fr fr ===== INTERFACE TESTS =====

collab Drawable {
    slay draw() tea
    slay area() meal
}

squad Circle {
    spill radius meal
}

flex Circle => Drawable {
    slay draw() tea {
        damn "drawing circle"
    }
    
    slay area() meal {
        damn 3.14159 * radius * radius
    }
}

test_start("Interface Implementation")
sus circle Circle = Circle{radius: 5.0}
sus drawable Drawable = circle
sus drawResult = drawable.draw()
assert_eq_string(drawResult, "drawing circle")

sus circleArea = drawable.area()
assert_true(circleArea > 75.0)

fr fr ===== PATTERN MATCHING TESTS =====

test_start("Basic Pattern Matching")
sus testValue drip = 42
sus matchResult tea = ""

match testValue {
    0 => matchResult = "zero",
    x if x > 0 => matchResult = "positive",
    _ => matchResult = "negative"
}

assert_eq_string(matchResult, "positive")

test_start("Struct Pattern Matching")
sus point Person = Person{name: "Bob", age: 25}
sus personResult tea = ""

match point {
    Person{name: "Alice", age: _} => personResult = "found Alice",
    Person{name: "Bob", age: a} if a < 30 => personResult = "young Bob",
    _ => personResult = "unknown person"
}

assert_eq_string(personResult, "young Bob")

fr fr ===== GENERIC TESTS =====

slay identity<T>(value T) T {
    damn value
}

slay max<T>(a T, b T) T {
    lowkey a > b {
        damn a
    }
    damn b
}

test_start("Generic Functions")
sus intResult = identity<drip>(42)
sus stringResult = identity<tea>("hello")

assert_eq_int(intResult, 42)
assert_eq_string(stringResult, "hello")

sus maxInt = max<drip>(10, 20)
sus maxFloat = max<meal>(3.14, 2.71)

assert_eq_int(maxInt, 20)
assert_true(maxFloat > 3.0)

fr fr ===== CONCURRENCY TESTS =====

test_start("Basic Goroutines")
sus done lit = cringe

stan {
    done = based
}

fr fr Wait for goroutine to complete
bestie !done {
    fr fr spin wait
}

assert_true(done)

test_start("Channel Operations")
sus ch = make_channel<drip>()

stan {
    dm_send(ch, 100)
}

sus received = dm_recv(ch)
assert_eq_int(received, 100)

test_start("Buffered Channels")
sus bufferedCh = make_channel<tea>(2)
dm_send(bufferedCh, "first")
dm_send(bufferedCh, "second")

sus first = dm_recv(bufferedCh)
sus second = dm_recv(bufferedCh)

assert_eq_string(first, "first")
assert_eq_string(second, "second")

test_start("Select Statements")
sus ch1 = make_channel<drip>()
sus ch2 = make_channel<tea>()
sus selectResult tea = ""

stan {
    dm_send(ch1, 42)
}

ready {
    mood value := dm_recv(ch1):
        selectResult = "received from ch1"
    mood value := dm_recv(ch2):
        selectResult = "received from ch2"
    basic:
        selectResult = "default case"
}

assert_eq_string(selectResult, "received from ch1")

fr fr ===== ERROR HANDLING TESTS =====

slay might_fail(shouldFail lit) (drip, yikes) {
    lowkey shouldFail {
        damn 0, yikes("operation failed")
    }
    damn 42, cringe
}

test_start("Error Handling - Success Case")
sus result, err = might_fail(cringe)
assert_eq_int(result, 42)
assert_true(err == cringe)

test_start("Error Handling - Error Case")
sus result2, err2 = might_fail(based)
assert_eq_int(result2, 0)
assert_false(err2 == cringe)

fr fr ===== MEMORY MANAGEMENT TESTS =====

test_start("Defer Statements")
sus deferCounter drip = 0

slay test_defer() {
    later {
        deferCounter = deferCounter + 1
    }
    later {
        deferCounter = deferCounter + 10
    }
    deferCounter = 1
}

test_defer()
assert_eq_int(deferCounter, 12) fr fr 1 + 10 + 1 (LIFO order)

test_start("Multiple Defer Execution Order")
sus deferOrder []drip = []

slay test_defer_order() {
    later deferOrder = append(deferOrder, 3)
    later deferOrder = append(deferOrder, 2)
    later deferOrder = append(deferOrder, 1)
}

test_defer_order()
assert_eq_int(deferOrder[0], 1)
assert_eq_int(deferOrder[1], 2)
assert_eq_int(deferOrder[2], 3)

fr fr ===== OPERATOR TESTS =====

test_start("Arithmetic Operators")
assert_eq_int(5 + 3, 8)
assert_eq_int(10 - 4, 6)
assert_eq_int(6 * 7, 42)
assert_eq_int(15 / 3, 5)
assert_eq_int(17 % 5, 2)

test_start("Comparison Operators")
assert_true(5 > 3)
assert_true(3 < 5)
assert_true(5 >= 5)
assert_true(3 <= 5)
assert_true(5 == 5)
assert_true(5 != 3)

test_start("Logical Operators")
assert_true(based && based)
assert_false(based && cringe)
assert_true(based || cringe)
assert_false(cringe || cringe)
assert_true(!cringe)
assert_false(!based)

fr fr ===== STRING OPERATIONS =====

test_start("String Concatenation")
sus greeting = "Hello" + " " + "World"
assert_eq_string(greeting, "Hello World")

test_start("String Interpolation")
sus name2 = "CURSED"
sus message = "Welcome to " + name2
assert_eq_string(message, "Welcome to CURSED")

fr fr ===== SCOPE AND CLOSURE TESTS =====

slay create_counter() slay() drip {
    sus count drip = 0
    damn slay() drip {
        count = count + 1
        damn count
    }
}

test_start("Closures")
sus counter = create_counter()
sus first = counter()
sus second = counter()

assert_eq_int(first, 1)
assert_eq_int(second, 2)

fr fr ===== FINAL TEST SUMMARY =====

print_test_summary()
