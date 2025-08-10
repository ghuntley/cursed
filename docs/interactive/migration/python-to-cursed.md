# Migrating from Python to CURSED

This guide helps Python developers transition to CURSED by drawing parallels between familiar Python concepts and their CURSED equivalents.

## 🎯 Quick Comparison Overview

| Python | CURSED | Notes |
|--------|--------|-------|
| `print()` | `vibez.spill()` | Output to console |
| `import module` | `yeet "module"` | Module imports |
| `def function():` | `slay function() {` | Function definition |
| `if condition:` | `ready (condition) {` | Conditionals |
| `for item in list:` | `bestie (item drip: list) {` | Iteration |
| `try/except` | `yikes/fam` | Error handling |
| `class MyClass:` | `squad MyClass {` | Class/struct definition |

## 📚 Syntax Comparison

### Hello World

**Python:**
```python
print("Hello, World!")
```

**CURSED:**
```cursed
yeet "vibez"
vibez.spill("Hello, World!")
```

<interactive-editor>
yeet "vibez"
vibez.spill("Hello, World!")
</interactive-editor>

### Variables and Types

**Python:**
```python
# Dynamic typing
name = "Alice"
age = 30
height = 5.7
is_student = True
numbers = [1, 2, 3, 4]
```

**CURSED:**
```cursed
# Static typing with inference
sus name tea = "Alice"
sus age drip = 30
sus height drip = 5.7  # Floats use drip type
sus is_student lit = based  # Boolean: based/cap
sus numbers []drip = [1, 2, 3, 4]
```

<interactive-editor>
yeet "vibez"

sus name tea = "Alice"
sus age drip = 30
sus height drip = 5.7
sus is_student lit = based
sus numbers []drip = [1, 2, 3, 4]

vibez.spill("Name:", name)
vibez.spill("Age:", age)
vibez.spill("Height:", height)
vibez.spill("Student:", is_student)
vibez.spill("Numbers:", numbers)
</interactive-editor>

### Functions

**Python:**
```python
def greet(name, age=None):
    if age:
        return f"Hello {name}, you are {age} years old"
    return f"Hello {name}"

def add(a, b):
    return a + b
```

**CURSED:**
```cursed
slay greet(name tea, age drip) tea {
    ready (age > 0) {
        damn "Hello " + name + ", you are " + age + " years old"
    }
    damn "Hello " + name
}

slay add(a drip, b drip) drip {
    damn a + b
}
```

<interactive-editor>
yeet "vibez"

slay greet(name tea, age drip) tea {
    ready (age > 0) {
        damn "Hello " + name + ", you are " + age + " years old"
    }
    damn "Hello " + name
}

slay add(a drip, b drip) drip {
    damn a + b
}

vibez.spill(greet("Alice", 25))
vibez.spill(greet("Bob", 0))
vibez.spill("Sum:", add(10, 20))
</interactive-editor>

### Control Flow

**Python:**
```python
# If statements
if age >= 18:
    print("Adult")
elif age >= 13:
    print("Teenager")
else:
    print("Child")

# For loops
for i in range(5):
    print(i)

for item in items:
    print(item)

# While loops
count = 0
while count < 5:
    print(count)
    count += 1
```

**CURSED:**
```cursed
# If statements
ready (age >= 18) {
    vibez.spill("Adult")
} otherwise ready (age >= 13) {
    vibez.spill("Teenager")
} otherwise {
    vibez.spill("Child")
}

# For loops
bestie (i drip: 0..5) {
    vibez.spill(i)
}

bestie (item drip: items) {
    vibez.spill(item)
}

# While loops
sus count drip = 0
bestie (count < 5) {
    vibez.spill(count)
    count = count + 1
}
```

<interactive-editor>
yeet "vibez"

sus age drip = 20

ready (age >= 18) {
    vibez.spill("Adult")
} otherwise ready (age >= 13) {
    vibez.spill("Teenager")
} otherwise {
    vibez.spill("Child")
}

vibez.spill("Counting to 5:")
bestie (i drip: 0..5) {
    vibez.spill(i)
}
</interactive-editor>

### Data Structures

**Python:**
```python
# Lists
numbers = [1, 2, 3, 4, 5]
numbers.append(6)
print(len(numbers))

# Dictionaries
person = {
    "name": "Alice",
    "age": 30,
    "city": "New York"
}
print(person["name"])

# Sets
unique_numbers = {1, 2, 3, 4, 5}
unique_numbers.add(6)
```

**CURSED:**
```cursed
# Arrays
yeet "arrayz"
sus numbers []drip = [1, 2, 3, 4, 5]
arrayz.push(numbers, 6)
vibez.spill(arrayz.len(numbers))

# Structs (similar to dataclasses)
squad Person {
    name tea
    age drip
    city tea
}

sus person Person = Person{
    name: "Alice",
    age: 30,
    city: "New York"
}
vibez.spill(person.name)

# Sets
yeet "collections"
sus unique_numbers collections.Set<drip> = collections.new_set()
collections.set_add(unique_numbers, 1)
collections.set_add(unique_numbers, 2)
```

<interactive-editor>
yeet "vibez"
yeet "arrayz"

# Arrays
sus numbers []drip = [1, 2, 3, 4, 5]
arrayz.push(numbers, 6)
vibez.spill("Array length:", arrayz.len(numbers))
vibez.spill("Numbers:", numbers)

# Structs
squad Person {
    name tea
    age drip
    city tea
}

sus person Person = Person{
    name: "Alice",
    age: 30,
    city: "New York"
}

vibez.spill("Person:", person.name, "from", person.city)
</interactive-editor>

### Error Handling

**Python:**
```python
try:
    result = 10 / 0
    print(result)
except ZeroDivisionError as e:
    print(f"Error: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
finally:
    print("Cleanup")
```

**CURSED:**
```cursed
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

sus result drip = divide(10, 0) fam {
    when "division by zero" -> {
        vibez.spill("Error: Cannot divide by zero")
        damn 0
    }
    when _ -> {
        vibez.spill("Unexpected error occurred")
        damn 0
    }
} shook {
    vibez.spill("Cleanup completed")
}
```

<interactive-editor>
yeet "vibez"

slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

sus result drip = divide(10, 2) fam {
    when "division by zero" -> {
        vibez.spill("Error: Cannot divide by zero")
        damn 0
    }
    when _ -> {
        vibez.spill("Unexpected error occurred")
        damn 0
    }
} shook {
    vibez.spill("Operation completed")
}

vibez.spill("Result:", result)
</interactive-editor>

### Classes vs Structs

**Python:**
```python
class Animal:
    def __init__(self, name, species):
        self.name = name
        self.species = species
    
    def speak(self):
        return f"{self.name} makes a sound"
    
    def describe(self):
        return f"{self.name} is a {self.species}"

# Usage
dog = Animal("Buddy", "Dog")
print(dog.speak())
print(dog.describe())
```

**CURSED:**
```cursed
squad Animal {
    name tea
    species tea
}

slay speak(self Animal) tea {
    damn self.name + " makes a sound"
}

slay describe(self Animal) tea {
    damn self.name + " is a " + self.species
}

# Usage
sus dog Animal = Animal{
    name: "Buddy",
    species: "Dog"
}
vibez.spill(speak(dog))
vibez.spill(describe(dog))
```

<interactive-editor>
yeet "vibez"

squad Animal {
    name tea
    species tea
}

slay speak(self Animal) tea {
    damn self.name + " makes a sound"
}

slay describe(self Animal) tea {
    damn self.name + " is a " + self.species
}

sus dog Animal = Animal{
    name: "Buddy",
    species: "Dog"
}

vibez.spill(speak(dog))
vibez.spill(describe(dog))
</interactive-editor>

## 🔄 Common Patterns Migration

### List Comprehensions to Functional Style

**Python:**
```python
# List comprehensions
squares = [x**2 for x in range(10)]
evens = [x for x in range(10) if x % 2 == 0]
```

**CURSED:**
```cursed
yeet "arrayz"
yeet "mathz"

# Using map and filter
sus numbers []drip = arrayz.range(0, 10)
sus squares []drip = arrayz.map(numbers, slay(x drip) drip { damn mathz.pow(x, 2) })
sus evens []drip = arrayz.filter(numbers, slay(x drip) lit { damn x % 2 == 0 })
```

### File I/O

**Python:**
```python
# Reading files
with open('file.txt', 'r') as f:
    content = f.read()
    lines = f.readlines()

# Writing files
with open('output.txt', 'w') as f:
    f.write("Hello, World!")
```

**CURSED:**
```cursed
yeet "filez"

# Reading files
sus content tea = filez.read_string("file.txt") fam {
    when _ -> damn ""
}

sus lines []tea = filez.read_lines("file.txt") fam {
    when _ -> damn []
}

# Writing files
filez.write_string("output.txt", "Hello, World!") fam {
    when _ -> vibez.spill("Failed to write file")
}
```

### JSON Handling

**Python:**
```python
import json

# Parse JSON
data = json.loads('{"name": "Alice", "age": 30}')
print(data["name"])

# Generate JSON
person = {"name": "Bob", "age": 25}
json_string = json.dumps(person)
```

**CURSED:**
```cursed
yeet "jsonz"
yeet "vibez"

# Parse JSON
sus json_text tea = '{"name": "Alice", "age": 30}'
sus data jsonz.Object = jsonz.parse(json_text) fam {
    when _ -> damn jsonz.Object{}
}
vibez.spill(jsonz.get_string(data, "name"))

# Generate JSON
sus person jsonz.Object = jsonz.Object{}
jsonz.set_string(person, "name", "Bob")
jsonz.set_number(person, "age", 25)
sus json_string tea = jsonz.stringify(person)
```

## 🚀 Advanced Features

### Concurrency

**Python (asyncio):**
```python
import asyncio

async def fetch_data(url):
    # Simulate async operation
    await asyncio.sleep(1)
    return f"Data from {url}"

async def main():
    tasks = [
        fetch_data("url1"),
        fetch_data("url2"),
        fetch_data("url3")
    ]
    results = await asyncio.gather(*tasks)
    print(results)

asyncio.run(main())
```

**CURSED (goroutines):**
```cursed
yeet "concurrenz"
yeet "timez"

slay fetch_data(url tea) tea {
    timez.sleep(1000)  # Sleep 1 second
    damn "Data from " + url
}

slay main() {
    sus ch chan<tea> = concurrenz.make_channel()
    
    # Start goroutines
    go { ch <- fetch_data("url1") }
    go { ch <- fetch_data("url2") }
    go { ch <- fetch_data("url3") }
    
    # Collect results
    bestie (i drip: 0..3) {
        sus result tea = <-ch
        vibez.spill(result)
    }
}
```

### Decorators vs Higher-Order Functions

**Python:**
```python
def timer(func):
    def wrapper(*args, **kwargs):
        start = time.time()
        result = func(*args, **kwargs)
        end = time.time()
        print(f"{func.__name__} took {end - start} seconds")
        return result
    return wrapper

@timer
def slow_function():
    time.sleep(1)
    return "Done"
```

**CURSED:**
```cursed
yeet "timez"

slay timer<T>(func slay() T) slay() T {
    damn slay() T {
        sus start drip = timez.now()
        sus result T = func()
        sus end drip = timez.now()
        vibez.spill("Function took", end - start, "milliseconds")
        damn result
    }
}

sus slow_function slay() tea = timer(slay() tea {
    timez.sleep(1000)
    damn "Done"
})
```

## 🛠️ Migration Strategy

### Step 1: Start Small
1. **Install CURSED** and set up development environment
2. **Rewrite simple scripts** like file processors or calculators
3. **Learn CURSED equivalents** for Python libraries you use

### Step 2: Key Differences to Understand
- **Static typing**: CURSED is statically typed
- **Memory management**: Automatic with garbage collection
- **Concurrency model**: Goroutines instead of async/await
- **Error handling**: Explicit error types instead of exceptions

### Step 3: Gradual Migration
1. **Identify core logic** that can be ported directly
2. **Rewrite I/O operations** using CURSED's modules
3. **Convert classes to structs** and methods
4. **Adapt error handling** from try/except to yikes/fam

### Step 4: Performance Benefits
- **Compile-time optimization**: CURSED compiles to optimized native code
- **Better concurrency**: Goroutines are more efficient than Python threads
- **Memory efficiency**: Lower memory overhead than Python

## 📚 Learning Resources

### Essential Modules for Python Developers
- [`vibez`](../../api/vibez.md) - I/O operations (like Python's `print`, `input`)
- [`stringz`](../../api/stringz.md) - String manipulation (like Python's `str` methods)
- [`arrayz`](../../api/arrayz.md) - Array operations (like Python's `list` methods)
- [`jsonz`](../../api/jsonz.md) - JSON parsing (like Python's `json`)
- [`filez`](../../api/filez.md) - File operations (like Python's `open`, `pathlib`)
- [`networkz`](../../api/networkz.md) - HTTP/networking (like Python's `requests`)

### Practice Projects
1. **File Organizer** - Migrate a Python script that organizes files
2. **API Client** - Convert a Python requests-based API client
3. **Data Processor** - Port a pandas-style data processing script
4. **Web Scraper** - Rewrite a BeautifulSoup scraper

### Common Gotchas
- **Array indexing**: CURSED uses 0-based indexing like Python
- **String concatenation**: Use `+` operator or string interpolation
- **Variable naming**: Use snake_case like Python
- **Function naming**: CURSED uses snake_case for functions too

## 🔗 Next Steps

- [Complete the CURSED beginner tutorial](../tutorials/beginner/README.md)
- [Learn CURSED-specific features](../tutorials/intermediate/README.md)
- [Join the Python→CURSED migration community](../community/migration-python.md)
- [Contribute Python examples to documentation](../community/contributing.md)

---

**Migration Difficulty**: Medium  
**Estimated Learning Time**: 2-3 weeks for basic proficiency  
**Key Advantage**: Better performance and concurrency than Python

Start migrating: [Set up CURSED development environment](../tutorials/setup/installation.md) →
