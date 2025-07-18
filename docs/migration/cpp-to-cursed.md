# Migrating from C/C++ to CURSED

Moving from C/C++ to CURSED involves transitioning from manual memory management to garbage collection, from low-level control to higher-level abstractions, while maintaining performance and adding modern safety features.

## Table of Contents

1. [Philosophy Shift](#philosophy-shift)
2. [Memory Management Revolution](#memory-management-revolution)
3. [Type System Evolution](#type-system-evolution)
4. [Syntax Translation](#syntax-translation)
5. [Object-Oriented Programming](#object-oriented-programming)
6. [Error Handling](#error-handling)
7. [Concurrency Models](#concurrency-models)
8. [Performance Considerations](#performance-considerations)
9. [Migration Strategy](#migration-strategy)
10. [Common Pitfalls](#common-pitfalls)
11. [Working Examples](#working-examples)

## Philosophy Shift

### C/C++ Philosophy
- **Manual control**: Direct memory and resource management
- **Zero overhead**: Pay only for what you use
- **Explicit everything**: Explicit resource management and control flow
- **Systems programming**: Low-level control over hardware
- **Performance first**: Optimize for speed and memory usage

### CURSED Philosophy
- **Managed resources**: Automatic memory management with GC
- **Safe abstractions**: Performance with safety guarantees
- **Modern convenience**: Implicit resource management where safe
- **Systems with safety**: High-level programming with low-level performance
- **Productivity and performance**: Balance between speed and development ease

## Memory Management Revolution

### Manual Memory Management vs Garbage Collection

**C/C++:**
```cpp
#include <iostream>
#include <memory>
#include <vector>

// Manual memory management
class Person {
    char* name;
    int age;
    
public:
    Person(const char* n, int a) : age(a) {
        name = new char[strlen(n) + 1];
        strcpy(name, n);
    }
    
    ~Person() {
        delete[] name;  // Manual cleanup
    }
    
    // Copy constructor needed
    Person(const Person& other) : age(other.age) {
        name = new char[strlen(other.name) + 1];
        strcpy(name, other.name);
    }
    
    // Assignment operator needed
    Person& operator=(const Person& other) {
        if (this != &other) {
            delete[] name;
            name = new char[strlen(other.name) + 1];
            strcpy(name, other.name);
            age = other.age;
        }
        return *this;
    }
};

int main() {
    Person* p1 = new Person("Alice", 25);
    Person p2 = *p1;  // Copy
    
    delete p1;  // Manual cleanup
    
    // p2 automatically cleaned up
    return 0;
}
```

**CURSED:**
```cursed
// Automatic memory management
vibes Person struct {
    name tea
    age normie
}

slay new_person(name tea, age normie) Person {
    damn Person{name: name, age: age}
}

slay main() {
    p1 := new_person("Alice", 25)
    p2 := p1  // Simple copy, GC handles memory
    
    // No manual cleanup needed
    vibez.spill("Person: ", p1.name, ", Age: ", p1.age)
    vibez.spill("Person: ", p2.name, ", Age: ", p2.age)
}
```

### Pointers vs References

**C/C++:**
```cpp
#include <iostream>

void modifyValue(int* ptr) {
    if (ptr != nullptr) {
        *ptr = 42;
    }
}

void modifyValueRef(int& ref) {
    ref = 42;
}

int main() {
    int x = 10;
    int* ptr = &x;
    
    std::cout << "Before: " << x << std::endl;
    modifyValue(ptr);
    std::cout << "After pointer: " << x << std::endl;
    
    modifyValueRef(x);
    std::cout << "After reference: " << x << std::endl;
    
    return 0;
}
```

**CURSED:**
```cursed
slay modify_value(ptr *normie) {
    *ptr = 42
}

slay modify_value_copy(val normie) normie {
    damn 42
}

slay main() {
    x := 10
    
    vibez.spill("Before: ", x)
    modify_value(&x)
    vibez.spill("After pointer: ", x)
    
    x = modify_value_copy(x)
    vibez.spill("After copy: ", x)
}
```

### Arrays and Dynamic Memory

**C/C++:**
```cpp
#include <iostream>
#include <vector>

int main() {
    // Stack array
    int stack_array[5] = {1, 2, 3, 4, 5};
    
    // Heap array (manual management)
    int* heap_array = new int[5];
    for (int i = 0; i < 5; i++) {
        heap_array[i] = i + 1;
    }
    
    // Modern C++ - automatic management
    std::vector<int> vector_array = {1, 2, 3, 4, 5};
    vector_array.push_back(6);
    
    // Print arrays
    for (int i = 0; i < 5; i++) {
        std::cout << stack_array[i] << " ";
    }
    std::cout << std::endl;
    
    for (int i = 0; i < 5; i++) {
        std::cout << heap_array[i] << " ";
    }
    std::cout << std::endl;
    
    for (int val : vector_array) {
        std::cout << val << " ";
    }
    std::cout << std::endl;
    
    delete[] heap_array;  // Manual cleanup
    
    return 0;
}
```

**CURSED:**
```cursed
slay main() {
    // Static array
    stack_array := [5]normie{1, 2, 3, 4, 5}
    
    // Dynamic array (GC managed)
    heap_array := make([]normie, 5)
    bestie i := 0; i < 5; i++ {
        heap_array[i] = i + 1
    }
    
    // Growable slice
    vector_array := []normie{1, 2, 3, 4, 5}
    vector_array = append(vector_array, 6)
    
    // Print arrays
    bestie i := 0; i < 5; i++ {
        vibez.spill(stack_array[i], " ")
    }
    vibez.spill()
    
    bestie i := 0; i < 5; i++ {
        vibez.spill(heap_array[i], " ")
    }
    vibez.spill()
    
    bestie val := range vector_array {
        vibez.spill(val, " ")
    }
    vibez.spill()
    
    // No manual cleanup needed
}
```

## Type System Evolution

### C Types vs CURSED Types

| C/C++ Type | CURSED Type | Description |
|------------|-------------|-------------|
| `char` | `sip` | 8-bit character |
| `int` | `normie` | 32-bit signed integer |
| `short` | `mid` | 16-bit signed integer |
| `long` | `thicc` | 64-bit signed integer |
| `unsigned char` | `byte` | 8-bit unsigned integer |
| `float` | `drip` | 32-bit floating point |
| `double` | `meal` | 64-bit floating point |
| `char*` | `tea` | String type |
| `bool` | `lit` | Boolean type |
| `void*` | `interface{}` | Generic pointer |

### Structs and Classes

**C/C++:**
```cpp
#include <iostream>
#include <string>

// C-style struct
struct Point {
    int x, y;
};

// C++ class
class Rectangle {
private:
    int width, height;
    
public:
    Rectangle(int w, int h) : width(w), height(h) {}
    
    int getArea() const {
        return width * height;
    }
    
    void setDimensions(int w, int h) {
        width = w;
        height = h;
    }
    
    // Friend function
    friend std::ostream& operator<<(std::ostream& os, const Rectangle& rect);
};

std::ostream& operator<<(std::ostream& os, const Rectangle& rect) {
    os << "Rectangle(" << rect.width << "x" << rect.height << ")";
    return os;
}

int main() {
    Point p = {10, 20};
    Rectangle rect(5, 10);
    
    std::cout << "Point: (" << p.x << ", " << p.y << ")" << std::endl;
    std::cout << "Rectangle area: " << rect.getArea() << std::endl;
    std::cout << rect << std::endl;
    
    return 0;
}
```

**CURSED:**
```cursed
yeet "stringz"

// Struct definition
vibes Point struct {
    x normie
    y normie
}

vibes Rectangle struct {
    width normie
    height normie
}

// Constructor function
slay new_rectangle(w normie, h normie) Rectangle {
    damn Rectangle{width: w, height: h}
}

// Methods
slay (r Rectangle) get_area() normie {
    damn r.width * r.height
}

slay (r *Rectangle) set_dimensions(w normie, h normie) {
    r.width = w
    r.height = h
}

slay (r Rectangle) to_string() tea {
    damn "Rectangle(" + stringz.from_int(r.width) + "x" + stringz.from_int(r.height) + ")"
}

slay main() {
    p := Point{x: 10, y: 20}
    rect := new_rectangle(5, 10)
    
    vibez.spill("Point: (", p.x, ", ", p.y, ")")
    vibez.spill("Rectangle area: ", rect.get_area())
    vibez.spill(rect.to_string())
}
```

### Templates vs Interfaces

**C++:**
```cpp
#include <iostream>
#include <vector>

// Template function
template<typename T>
T max(T a, T b) {
    return (a > b) ? a : b;
}

// Template class
template<typename T>
class Stack {
private:
    std::vector<T> elements;
    
public:
    void push(const T& elem) {
        elements.push_back(elem);
    }
    
    T pop() {
        if (elements.empty()) {
            throw std::runtime_error("Stack is empty");
        }
        T elem = elements.back();
        elements.pop_back();
        return elem;
    }
    
    bool empty() const {
        return elements.empty();
    }
    
    size_t size() const {
        return elements.size();
    }
};

int main() {
    std::cout << "Max of 5 and 10: " << max(5, 10) << std::endl;
    std::cout << "Max of 3.14 and 2.71: " << max(3.14, 2.71) << std::endl;
    
    Stack<int> intStack;
    intStack.push(1);
    intStack.push(2);
    intStack.push(3);
    
    while (!intStack.empty()) {
        std::cout << intStack.pop() << " ";
    }
    std::cout << std::endl;
    
    return 0;
}
```

**CURSED:**
```cursed
// Generic function using interfaces
vibes Comparable interface {
    compare(other Comparable) normie
}

slay max[T Comparable](a T, b T) T {
    lowkey a.compare(b) > 0 {
        damn a
    }
    damn b
}

// Generic struct
vibes Stack[T] struct {
    elements []T
}

slay new_stack[T]() Stack[T] {
    damn Stack[T]{elements: make([]T, 0)}
}

slay (s *Stack[T]) push(elem T) {
    s.elements = append(s.elements, elem)
}

slay (s *Stack[T]) pop() T {
    lowkey len(s.elements) == 0 {
        yikes "Stack is empty"
    }
    
    elem := s.elements[len(s.elements)-1]
    s.elements = s.elements[:len(s.elements)-1]
    damn elem
}

slay (s Stack[T]) empty() lit {
    damn len(s.elements) == 0
}

slay (s Stack[T]) size() normie {
    damn len(s.elements)
}

// Implement Comparable for integers
slay (a normie) compare(other Comparable) normie {
    b := other.(normie)
    lowkey a > b {
        damn 1
    } sus lowkey a < b {
        damn -1
    }
    damn 0
}

slay main() {
    vibez.spill("Max of 5 and 10: ", max(5, 10))
    
    int_stack := new_stack[normie]()
    int_stack.push(1)
    int_stack.push(2)
    int_stack.push(3)
    
    bestie !int_stack.empty() {
        vibez.spill(int_stack.pop(), " ")
    }
    vibez.spill()
}
```

## Syntax Translation

### Function Definitions

**C/C++:**
```cpp
#include <iostream>

// C-style function
int add(int a, int b) {
    return a + b;
}

// C++ function with default parameters
int multiply(int a, int b = 2) {
    return a * b;
}

// Function overloading
int process(int x) {
    return x * 2;
}

double process(double x) {
    return x * 2.0;
}

// Function pointers
int (*operation)(int, int) = add;

int main() {
    std::cout << "Add: " << add(5, 3) << std::endl;
    std::cout << "Multiply: " << multiply(5) << std::endl;
    std::cout << "Process int: " << process(5) << std::endl;
    std::cout << "Process double: " << process(5.5) << std::endl;
    std::cout << "Function pointer: " << operation(10, 20) << std::endl;
    
    return 0;
}
```

**CURSED:**
```cursed
// Function definitions
slay add(a normie, b normie) normie {
    damn a + b
}

// Default parameters via function overloading
slay multiply(a normie) normie {
    damn multiply_with_factor(a, 2)
}

slay multiply_with_factor(a normie, b normie) normie {
    damn a * b
}

// Function overloading with different names
slay process_int(x normie) normie {
    damn x * 2
}

slay process_double(x meal) meal {
    damn x * 2.0
}

// Function type
vibes Operation func(normie, normie) normie

slay main() {
    vibez.spill("Add: ", add(5, 3))
    vibez.spill("Multiply: ", multiply(5))
    vibez.spill("Process int: ", process_int(5))
    vibez.spill("Process double: ", process_double(5.5))
    
    // Function assignment
    operation := Operation(add)
    vibez.spill("Function pointer: ", operation(10, 20))
}
```

### Control Flow

**C/C++:**
```cpp
#include <iostream>

int main() {
    // If statements
    int x = 10;
    if (x > 5) {
        std::cout << "x is greater than 5" << std::endl;
    } else if (x == 5) {
        std::cout << "x is equal to 5" << std::endl;
    } else {
        std::cout << "x is less than 5" << std::endl;
    }
    
    // Switch statement
    int day = 2;
    switch (day) {
        case 1:
            std::cout << "Monday" << std::endl;
            break;
        case 2:
            std::cout << "Tuesday" << std::endl;
            break;
        default:
            std::cout << "Other day" << std::endl;
    }
    
    // For loops
    for (int i = 0; i < 5; i++) {
        std::cout << "i = " << i << std::endl;
    }
    
    // While loop
    int count = 0;
    while (count < 3) {
        std::cout << "Count: " << count << std::endl;
        count++;
    }
    
    // Do-while loop
    int j = 0;
    do {
        std::cout << "j = " << j << std::endl;
        j++;
    } while (j < 3);
    
    return 0;
}
```

**CURSED:**
```cursed
slay main() {
    // If statements
    x := 10
    lowkey x > 5 {
        vibez.spill("x is greater than 5")
    } sus lowkey x == 5 {
        vibez.spill("x is equal to 5")
    } sus {
        vibez.spill("x is less than 5")
    }
    
    // Match statement (similar to switch)
    day := 2
    match day {
        1 => vibez.spill("Monday")
        2 => vibez.spill("Tuesday")
        basic => vibez.spill("Other day")
    }
    
    // For loops
    bestie i := 0; i < 5; i++ {
        vibez.spill("i = ", i)
    }
    
    // While loop
    count := 0
    bestie count < 3 {
        vibez.spill("Count: ", count)
        count++
    }
    
    // Do-while equivalent
    j := 0
    bestie {
        vibez.spill("j = ", j)
        j++
        lowkey j >= 3 {
            ghosted
        }
    }
}
```

## Object-Oriented Programming

### Inheritance vs Composition

**C++:**
```cpp
#include <iostream>
#include <string>

// Base class
class Animal {
protected:
    std::string name;
    
public:
    Animal(const std::string& n) : name(n) {}
    
    virtual void makeSound() const {
        std::cout << name << " makes a sound" << std::endl;
    }
    
    virtual ~Animal() = default;
};

// Derived class
class Dog : public Animal {
private:
    std::string breed;
    
public:
    Dog(const std::string& n, const std::string& b) 
        : Animal(n), breed(b) {}
    
    void makeSound() const override {
        std::cout << name << " barks" << std::endl;
    }
    
    void wagTail() const {
        std::cout << name << " wags tail" << std::endl;
    }
};

// Multiple inheritance
class Pet {
protected:
    std::string owner;
    
public:
    Pet(const std::string& o) : owner(o) {}
    
    void showOwner() const {
        std::cout << "Owner: " << owner << std::endl;
    }
};

class DomesticDog : public Dog, public Pet {
public:
    DomesticDog(const std::string& n, const std::string& b, const std::string& o)
        : Dog(n, b), Pet(o) {}
    
    void introduce() const {
        makeSound();
        wagTail();
        showOwner();
    }
};

int main() {
    DomesticDog dog("Buddy", "Golden Retriever", "Alice");
    dog.introduce();
    
    return 0;
}
```

**CURSED:**
```cursed
// Interface-based approach
vibes Animal interface {
    make_sound()
    get_name() tea
}

vibes Pet interface {
    show_owner()
    get_owner() tea
}

// Struct with composition
vibes Dog struct {
    name tea
    breed tea
    owner tea
}

// Implement Animal interface
slay (d Dog) make_sound() {
    vibez.spill(d.name, " barks")
}

slay (d Dog) get_name() tea {
    damn d.name
}

// Implement Pet interface
slay (d Dog) show_owner() {
    vibez.spill("Owner: ", d.owner)
}

slay (d Dog) get_owner() tea {
    damn d.owner
}

// Dog-specific methods
slay (d Dog) wag_tail() {
    vibez.spill(d.name, " wags tail")
}

slay (d Dog) introduce() {
    d.make_sound()
    d.wag_tail()
    d.show_owner()
}

slay main() {
    dog := Dog{
        name: "Buddy",
        breed: "Golden Retriever",
        owner: "Alice",
    }
    
    dog.introduce()
    
    // Can be used as either interface
    sus animal Animal = dog
    animal.make_sound()
    
    sus pet Pet = dog
    pet.show_owner()
}
```

### Polymorphism

**C++:**
```cpp
#include <iostream>
#include <vector>
#include <memory>

class Shape {
public:
    virtual double getArea() const = 0;
    virtual void draw() const = 0;
    virtual ~Shape() = default;
};

class Circle : public Shape {
private:
    double radius;
    
public:
    Circle(double r) : radius(r) {}
    
    double getArea() const override {
        return 3.14159 * radius * radius;
    }
    
    void draw() const override {
        std::cout << "Drawing a circle with radius " << radius << std::endl;
    }
};

class Rectangle : public Shape {
private:
    double width, height;
    
public:
    Rectangle(double w, double h) : width(w), height(h) {}
    
    double getArea() const override {
        return width * height;
    }
    
    void draw() const override {
        std::cout << "Drawing a rectangle " << width << "x" << height << std::endl;
    }
};

int main() {
    std::vector<std::unique_ptr<Shape>> shapes;
    shapes.push_back(std::make_unique<Circle>(5.0));
    shapes.push_back(std::make_unique<Rectangle>(4.0, 6.0));
    
    for (const auto& shape : shapes) {
        shape->draw();
        std::cout << "Area: " << shape->getArea() << std::endl;
    }
    
    return 0;
}
```

**CURSED:**
```cursed
// Interface definition
vibes Shape interface {
    get_area() meal
    draw()
}

// Circle implementation
vibes Circle struct {
    radius meal
}

slay new_circle(r meal) Circle {
    damn Circle{radius: r}
}

slay (c Circle) get_area() meal {
    damn 3.14159 * c.radius * c.radius
}

slay (c Circle) draw() {
    vibez.spill("Drawing a circle with radius ", c.radius)
}

// Rectangle implementation
vibes Rectangle struct {
    width meal
    height meal
}

slay new_rectangle(w meal, h meal) Rectangle {
    damn Rectangle{width: w, height: h}
}

slay (r Rectangle) get_area() meal {
    damn r.width * r.height
}

slay (r Rectangle) draw() {
    vibez.spill("Drawing a rectangle ", r.width, "x", r.height)
}

slay main() {
    shapes := []Shape{
        new_circle(5.0),
        new_rectangle(4.0, 6.0),
    }
    
    bestie shape := range shapes {
        shape.draw()
        vibez.spill("Area: ", shape.get_area())
    }
}
```

## Error Handling

### Exceptions vs Error Values

**C++:**
```cpp
#include <iostream>
#include <stdexcept>
#include <fstream>

class FileProcessor {
public:
    void processFile(const std::string& filename) {
        std::ifstream file(filename);
        
        if (!file.is_open()) {
            throw std::runtime_error("Cannot open file: " + filename);
        }
        
        std::string line;
        int lineNumber = 0;
        
        while (std::getline(file, line)) {
            lineNumber++;
            
            if (line.empty()) {
                throw std::invalid_argument("Empty line at line " + std::to_string(lineNumber));
            }
            
            // Process line
            std::cout << "Processing line " << lineNumber << ": " << line << std::endl;
        }
        
        file.close();
    }
};

int main() {
    FileProcessor processor;
    
    try {
        processor.processFile("input.txt");
        std::cout << "File processed successfully" << std::endl;
    } catch (const std::runtime_error& e) {
        std::cerr << "Runtime error: " << e.what() << std::endl;
    } catch (const std::invalid_argument& e) {
        std::cerr << "Invalid argument: " << e.what() << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "General error: " << e.what() << std::endl;
    }
    
    return 0;
}
```

**CURSED:**
```cursed
yeet "dropz"
yeet "stringz"

vibes FileProcessor struct{}

slay (fp FileProcessor) process_file(filename tea) {
    shook {
        file := dropz.open(filename)
        defer file.close()
        
        content := file.read_all()
        lines := stringz.split(content, "\n")
        
        bestie line_number, line := range lines {
            lowkey line == "" {
                yikes "Empty line at line " + stringz.from_int(line_number + 1)
            }
            
            // Process line
            vibez.spill("Processing line ", line_number + 1, ": ", line)
        }
    } fam err {
        // Re-throw with context
        yikes "Cannot process file " + filename + ": " + err.message()
    }
}

slay main() {
    processor := FileProcessor{}
    
    shook {
        processor.process_file("input.txt")
        vibez.spill("File processed successfully")
    } fam err {
        vibez.spill("Error: ", err.message())
    }
}
```

### Resource Management

**C++:**
```cpp
#include <iostream>
#include <memory>
#include <fstream>

// RAII wrapper
class FileHandle {
private:
    std::unique_ptr<std::ifstream> file;
    
public:
    FileHandle(const std::string& filename) {
        file = std::make_unique<std::ifstream>(filename);
        if (!file->is_open()) {
            throw std::runtime_error("Cannot open file: " + filename);
        }
        std::cout << "File opened: " << filename << std::endl;
    }
    
    ~FileHandle() {
        if (file && file->is_open()) {
            file->close();
            std::cout << "File closed automatically" << std::endl;
        }
    }
    
    std::string readLine() {
        std::string line;
        if (file && std::getline(*file, line)) {
            return line;
        }
        return "";
    }
    
    bool isOpen() const {
        return file && file->is_open();
    }
};

int main() {
    try {
        FileHandle handle("input.txt");
        
        std::string line;
        while (!(line = handle.readLine()).empty()) {
            std::cout << "Read: " << line << std::endl;
        }
        
        // File automatically closed when handle goes out of scope
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }
    
    return 0;
}
```

**CURSED:**
```cursed
yeet "dropz"

vibes FileHandle struct {
    file dropz.File
    filename tea
}

slay new_file_handle(filename tea) FileHandle {
    shook {
        file := dropz.open(filename)
        vibez.spill("File opened: ", filename)
        damn FileHandle{file: file, filename: filename}
    } fam err {
        yikes "Cannot open file: " + filename + ": " + err.message()
    }
}

slay (fh *FileHandle) close() {
    fh.file.close()
    vibez.spill("File closed: ", fh.filename)
}

slay (fh FileHandle) read_line() tea {
    line := fh.file.read_line()
    damn line
}

slay (fh FileHandle) is_open() lit {
    damn fh.file.is_open()
}

slay main() {
    shook {
        handle := new_file_handle("input.txt")
        defer handle.close()  // Automatic cleanup
        
        bestie handle.is_open() {
            line := handle.read_line()
            lowkey line == "" {
                ghosted
            }
            vibez.spill("Read: ", line)
        }
    } fam err {
        vibez.spill("Error: ", err.message())
    }
}
```

## Concurrency Models

### Threads vs Goroutines

**C++:**
```cpp
#include <iostream>
#include <thread>
#include <mutex>
#include <vector>
#include <queue>
#include <condition_variable>

class ThreadPool {
private:
    std::vector<std::thread> workers;
    std::queue<std::function<void()>> tasks;
    std::mutex queue_mutex;
    std::condition_variable condition;
    bool stop;
    
public:
    ThreadPool(size_t num_threads) : stop(false) {
        for (size_t i = 0; i < num_threads; ++i) {
            workers.emplace_back([this] {
                while (true) {
                    std::function<void()> task;
                    
                    {
                        std::unique_lock<std::mutex> lock(queue_mutex);
                        condition.wait(lock, [this] { return stop || !tasks.empty(); });
                        
                        if (stop && tasks.empty()) return;
                        
                        task = std::move(tasks.front());
                        tasks.pop();
                    }
                    
                    task();
                }
            });
        }
    }
    
    template<typename F>
    void enqueue(F&& task) {
        {
            std::unique_lock<std::mutex> lock(queue_mutex);
            tasks.emplace(std::forward<F>(task));
        }
        condition.notify_one();
    }
    
    ~ThreadPool() {
        {
            std::unique_lock<std::mutex> lock(queue_mutex);
            stop = true;
        }
        condition.notify_all();
        for (std::thread& worker : workers) {
            worker.join();
        }
    }
};

int main() {
    ThreadPool pool(4);
    
    // Submit tasks
    for (int i = 0; i < 10; ++i) {
        pool.enqueue([i] {
            std::cout << "Task " << i << " executed by thread " 
                      << std::this_thread::get_id() << std::endl;
            std::this_thread::sleep_for(std::chrono::seconds(1));
        });
    }
    
    // Wait a bit for tasks to complete
    std::this_thread::sleep_for(std::chrono::seconds(3));
    
    return 0;
}
```

**CURSED:**
```cursed
yeet "timez"

slay worker(id normie, tasks <-chan normie) {
    bestie task := range tasks {
        vibez.spill("Task ", task, " executed by worker ", id)
        timez.sleep(timez.second)
    }
}

slay main() {
    tasks := make(chan normie, 10)
    
    // Start workers
    bestie i := 0; i < 4; i++ {
        yolo worker(i, tasks)
    }
    
    // Submit tasks
    bestie i := 0; i < 10; i++ {
        tasks <- i
    }
    
    // Close channel to signal completion
    close(tasks)
    
    // Wait for workers to finish
    timez.sleep(3 * timez.second)
}
```

### Mutexes vs Channels

**C++:**
```cpp
#include <iostream>
#include <thread>
#include <mutex>
#include <vector>

class Counter {
private:
    int count;
    mutable std::mutex mtx;
    
public:
    Counter() : count(0) {}
    
    void increment() {
        std::lock_guard<std::mutex> lock(mtx);
        count++;
    }
    
    int getValue() const {
        std::lock_guard<std::mutex> lock(mtx);
        return count;
    }
};

int main() {
    Counter counter;
    std::vector<std::thread> threads;
    
    // Create threads that increment counter
    for (int i = 0; i < 10; ++i) {
        threads.emplace_back([&counter] {
            for (int j = 0; j < 1000; ++j) {
                counter.increment();
            }
        });
    }
    
    // Wait for all threads to complete
    for (auto& t : threads) {
        t.join();
    }
    
    std::cout << "Final count: " << counter.getValue() << std::endl;
    
    return 0;
}
```

**CURSED:**
```cursed
slay counter_service(increment_ch <-chan lit, get_ch <-chan chan<- normie) {
    count := 0
    
    bestie {
        ready {
            <-increment_ch:
                count++
            
            result_ch := <-get_ch:
                result_ch <- count
        }
    }
}

slay worker(increment_ch chan<- lit) {
    bestie j := 0; j < 1000; j++ {
        increment_ch <- based
    }
}

slay main() {
    increment_ch := make(chan lit, 100)
    get_ch := make(chan chan<- normie)
    
    // Start counter service
    yolo counter_service(increment_ch, get_ch)
    
    // Create workers
    bestie i := 0; i < 10; i++ {
        yolo worker(increment_ch)
    }
    
    // Get final value
    result_ch := make(chan normie)
    get_ch <- result_ch
    final_count := <-result_ch
    
    vibez.spill("Final count: ", final_count)
}
```

## Performance Considerations

### Optimization Levels

**C++:**
```cpp
// Compile with: g++ -O2 -march=native program.cpp
#include <iostream>
#include <chrono>
#include <vector>

// Example that benefits from optimization
int fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

void vector_operations() {
    std::vector<int> vec(1000000);
    
    // Initialize
    for (size_t i = 0; i < vec.size(); ++i) {
        vec[i] = i * 2;
    }
    
    // Process
    long long sum = 0;
    for (int val : vec) {
        sum += val;
    }
    
    std::cout << "Sum: " << sum << std::endl;
}

int main() {
    auto start = std::chrono::high_resolution_clock::now();
    
    int result = fibonacci(35);
    vector_operations();
    
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    
    std::cout << "Fibonacci(35) = " << result << std::endl;
    std::cout << "Time: " << duration.count() << "ms" << std::endl;
    
    return 0;
}
```

**CURSED:**
```cursed
// Compile with: cargo run --bin cursed -- compile --optimize program.csd
yeet "timez"

slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay vector_operations() {
    vec := make([]normie, 1000000)
    
    // Initialize
    bestie i := 0; i < len(vec); i++ {
        vec[i] = i * 2
    }
    
    // Process
    sum := 0
    bestie val := range vec {
        sum += val
    }
    
    vibez.spill("Sum: ", sum)
}

slay main() {
    start := timez.now()
    
    result := fibonacci(35)
    vector_operations()
    
    end := timez.now()
    duration := end.sub(start)
    
    vibez.spill("Fibonacci(35) = ", result)
    vibez.spill("Time: ", duration.milliseconds(), "ms")
}
```

## Migration Strategy

### Phase 1: Memory Management Simplification
1. **Remove manual memory management**: Replace `new`/`delete` with GC
2. **Eliminate RAII patterns**: Use `defer` for cleanup
3. **Simplify smart pointers**: Use regular references and values
4. **Remove move semantics**: GC handles resource management

### Phase 2: Type System Modernization
1. **Update primitive types**: Map C++ types to CURSED types
2. **Convert classes to structs**: Use composition over inheritance
3. **Replace templates with generics**: Use interface-based generics
4. **Modernize error handling**: Use `yikes`/`shook`/`fam` pattern

### Phase 3: Concurrency Transformation
1. **Replace threads with goroutines**: Use `yolo` for concurrent execution
2. **Convert mutexes to channels**: Use message passing for synchronization
3. **Simplify thread pools**: Use goroutines and channels
4. **Modernize async patterns**: Use channel-based communication

### Phase 4: Performance Optimization
1. **Leverage LLVM optimizations**: Use compilation flags
2. **Optimize data structures**: Use efficient algorithms
3. **Profile and tune**: Use CURSED's profiling tools
4. **Benchmark performance**: Compare with C++ versions

## Common Pitfalls

### 1. Memory Management Overthinking
**Problem:** Trying to manually manage memory like in C++
```cpp
// C++ thinking (unnecessary in CURSED)
char* buffer = new char[1024];
// ... use buffer ...
delete[] buffer;
```

**Solution:** Trust the GC
```cursed
// CURSED approach
buffer := make([]byte, 1024)
// GC handles cleanup automatically
```

### 2. RAII Pattern Overuse
**Problem:** Creating complex RAII wrappers
```cpp
// C++ RAII
class FileHandle {
    std::ifstream file;
public:
    FileHandle(const std::string& name) : file(name) {}
    ~FileHandle() { file.close(); }
};
```

**Solution:** Use defer for cleanup
```cursed
// CURSED approach
slay process_file(filename tea) {
    file := dropz.open(filename)
    defer file.close()  // Automatic cleanup
    
    // Use file...
}
```

### 3. Template Complexity
**Problem:** Overly complex template metaprogramming
```cpp
// C++ template complexity
template<typename T, typename = std::enable_if_t<std::is_arithmetic<T>::value>>
T complex_function(T value) {
    return value * 2;
}
```

**Solution:** Use simple interfaces
```cursed
// CURSED approach
vibes Numeric interface {
    multiply(factor Numeric) Numeric
}

slay complex_function[T Numeric](value T) T {
    damn value.multiply(2)
}
```

### 4. Exception Handling Complexity
**Problem:** Complex exception hierarchies
```cpp
// C++ exception complexity
try {
    // code
} catch (const SpecificError& e) {
    // handle
} catch (const BaseError& e) {
    // handle
} catch (...) {
    // handle all
}
```

**Solution:** Use structured error handling
```cursed
// CURSED approach
shook {
    // code
} fam err {
    match err.type() {
        "SpecificError" => // handle
        "BaseError" => // handle
        basic => // handle all
    }
}
```

## Working Examples

### Example 1: Network Server

**C++:**
```cpp
#include <iostream>
#include <thread>
#include <vector>
#include <asio.hpp>

using asio::ip::tcp;

class Server {
private:
    asio::io_context io_context;
    tcp::acceptor acceptor;
    
public:
    Server(short port) : acceptor(io_context, tcp::endpoint(tcp::v4(), port)) {}
    
    void start() {
        accept();
        io_context.run();
    }
    
private:
    void accept() {
        auto socket = std::make_shared<tcp::socket>(io_context);
        
        acceptor.async_accept(*socket,
            [this, socket](std::error_code ec) {
                if (!ec) {
                    std::thread(&Server::handle_client, this, socket).detach();
                }
                accept();
            });
    }
    
    void handle_client(std::shared_ptr<tcp::socket> socket) {
        try {
            char buffer[1024];
            size_t length = socket->read_some(asio::buffer(buffer));
            
            std::string response = "HTTP/1.1 200 OK\r\n\r\nHello from C++!";
            socket->write_some(asio::buffer(response));
            
            socket->close();
        } catch (std::exception& e) {
            std::cerr << "Client error: " << e.what() << std::endl;
        }
    }
};

int main() {
    try {
        Server server(8080);
        std::cout << "Server starting on port 8080..." << std::endl;
        server.start();
    } catch (std::exception& e) {
        std::cerr << "Server error: " << e.what() << std::endl;
    }
    
    return 0;
}
```

**CURSED:**
```cursed
yeet "vibe_net"

slay handle_client(conn vibe_net.TcpConnection) {
    shook {
        buffer := make([]byte, 1024)
        length := conn.read(buffer)
        
        response := "HTTP/1.1 200 OK\r\n\r\nHello from CURSED!"
        conn.write(response.as_bytes())
        
        conn.close()
    } fam err {
        vibez.spill("Client error: ", err.message())
    }
}

slay main() {
    shook {
        listener := vibe_net.listen("tcp", ":8080")
        defer listener.close()
        
        vibez.spill("Server starting on port 8080...")
        
        bestie {
            conn := listener.accept()
            yolo handle_client(conn)
        }
    } fam err {
        vibez.spill("Server error: ", err.message())
    }
}
```

### Example 2: Data Processing Pipeline

**C++:**
```cpp
#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <numeric>
#include <thread>
#include <future>

struct DataPoint {
    int id;
    double value;
    std::string category;
};

class DataProcessor {
public:
    std::vector<DataPoint> loadData(const std::string& filename) {
        std::vector<DataPoint> data;
        std::ifstream file(filename);
        
        if (!file.is_open()) {
            throw std::runtime_error("Cannot open file: " + filename);
        }
        
        std::string line;
        while (std::getline(file, line)) {
            // Parse CSV line (simplified)
            DataPoint point;
            // ... parsing logic ...
            data.push_back(point);
        }
        
        return data;
    }
    
    std::vector<DataPoint> processData(const std::vector<DataPoint>& data) {
        std::vector<DataPoint> processed;
        
        // Process in parallel
        std::vector<std::future<std::vector<DataPoint>>> futures;
        
        size_t chunk_size = data.size() / 4;
        for (size_t i = 0; i < data.size(); i += chunk_size) {
            auto end = std::min(i + chunk_size, data.size());
            
            futures.push_back(std::async(std::launch::async, [&data, i, end] {
                std::vector<DataPoint> chunk_result;
                for (size_t j = i; j < end; ++j) {
                    DataPoint processed_point = data[j];
                    processed_point.value *= 2.0;  // Example processing
                    chunk_result.push_back(processed_point);
                }
                return chunk_result;
            }));
        }
        
        // Collect results
        for (auto& future : futures) {
            auto chunk_result = future.get();
            processed.insert(processed.end(), chunk_result.begin(), chunk_result.end());
        }
        
        return processed;
    }
    
    void saveData(const std::vector<DataPoint>& data, const std::string& filename) {
        std::ofstream file(filename);
        
        if (!file.is_open()) {
            throw std::runtime_error("Cannot create file: " + filename);
        }
        
        for (const auto& point : data) {
            file << point.id << "," << point.value << "," << point.category << std::endl;
        }
    }
};

int main() {
    try {
        DataProcessor processor;
        
        auto data = processor.loadData("input.csv");
        auto processed = processor.processData(data);
        processor.saveData(processed, "output.csv");
        
        std::cout << "Processed " << processed.size() << " data points" << std::endl;
        
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }
    
    return 0;
}
```

**CURSED:**
```cursed
yeet "dropz"
yeet "stringz"
yeet "encode_mood"

vibes DataPoint struct {
    id normie
    value meal
    category tea
}

vibes DataProcessor struct{}

slay (dp DataProcessor) load_data(filename tea) []DataPoint {
    shook {
        file := dropz.open(filename)
        defer file.close()
        
        content := file.read_all()
        lines := stringz.split(content, "\n")
        
        data := make([]DataPoint, 0)
        bestie line := range lines {
            lowkey line != "" {
                fields := stringz.split(line, ",")
                lowkey len(fields) >= 3 {
                    point := DataPoint{
                        id: stringz.to_int(fields[0]),
                        value: stringz.to_float(fields[1]),
                        category: fields[2],
                    }
                    data = append(data, point)
                }
            }
        }
        
        damn data
    } fam err {
        yikes "Cannot load data: " + err.message()
    }
}

slay process_chunk(data []DataPoint, results chan<- []DataPoint) {
    processed := make([]DataPoint, len(data))
    
    bestie i, point := range data {
        processed[i] = DataPoint{
            id: point.id,
            value: point.value * 2.0,
            category: point.category,
        }
    }
    
    results <- processed
}

slay (dp DataProcessor) process_data(data []DataPoint) []DataPoint {
    chunk_size := len(data) / 4
    lowkey chunk_size == 0 {
        chunk_size = 1
    }
    
    results := make(chan []DataPoint, 4)
    chunk_count := 0
    
    // Process in parallel
    bestie i := 0; i < len(data); i += chunk_size {
        end := i + chunk_size
        lowkey end > len(data) {
            end = len(data)
        }
        
        chunk := data[i:end]
        yolo process_chunk(chunk, results)
        chunk_count++
    }
    
    // Collect results
    processed := make([]DataPoint, 0)
    bestie i := 0; i < chunk_count; i++ {
        chunk_result := <-results
        processed = append(processed, chunk_result...)
    }
    
    damn processed
}

slay (dp DataProcessor) save_data(data []DataPoint, filename tea) {
    shook {
        file := dropz.create(filename)
        defer file.close()
        
        bestie point := range data {
            line := stringz.from_int(point.id) + "," + 
                   stringz.from_float(point.value) + "," + 
                   point.category + "\n"
            file.write(line)
        }
    } fam err {
        yikes "Cannot save data: " + err.message()
    }
}

slay main() {
    shook {
        processor := DataProcessor{}
        
        data := processor.load_data("input.csv")
        processed := processor.process_data(data)
        processor.save_data(processed, "output.csv")
        
        vibez.spill("Processed ", len(processed), " data points")
    } fam err {
        vibez.spill("Error: ", err.message())
    }
}
```

## Testing Your Migration

### Running Examples

```bash
# Test basic syntax
cargo run --bin cursed examples/basic_syntax.csd

# Test memory management
cargo run --bin cursed examples/memory_demo.csd

# Test concurrency
cargo run --bin cursed examples/concurrent_processing.csd

# Test performance
cargo run --bin cursed examples/performance_test.csd

# Compile to native
cargo run --bin cursed -- compile examples/network_server.csd
./network_server
```

### Performance Comparison

```bash
# C++ compilation
g++ -O2 -std=c++17 program.cpp -o cpp_program

# CURSED compilation
cargo run --bin cursed -- compile --optimize program.csd

# Compare execution times
time ./cpp_program
time ./program
```

## Next Steps

1. **Embrace automatic memory management**: Stop thinking about manual memory management
2. **Learn goroutines and channels**: Master CURSED's concurrency model
3. **Use interfaces effectively**: Replace complex inheritance with composition
4. **Leverage the type system**: Use static typing for better error catching
5. **Optimize with LLVM**: Use compiler optimizations for performance

The migration from C/C++ to CURSED involves learning to trust higher-level abstractions while maintaining performance through modern compiler technology and efficient runtime systems.
