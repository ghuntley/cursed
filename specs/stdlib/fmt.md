# fmt Module

## Overview
The fmt module provides formatted I/O functions similar to Go's fmt package. It handles string formatting, printing, and scanning with type-safe operations.

## Core Functions

### Printing Functions

```cursed
fr fr Print formats using default formats and writes to standard output
slay Print(a ...interface{}) (n normie, err yikes)

fr fr Printf formats according to format specifier and writes to standard output
slay Printf(format tea, a ...interface{}) (n normie, err yikes)

fr fr Println formats using default formats and writes to standard output with newline
slay Println(a ...interface{}) (n normie, err yikes)
```

### String Formatting

```cursed
fr fr Sprint formats using default formats and returns resulting string
slay Sprint(a ...interface{}) tea

fr fr Sprintf formats according to format specifier and returns resulting string
slay Sprintf(format tea, a ...interface{}) tea

fr fr Sprintln formats using default formats and returns resulting string with newline
slay Sprintln(a ...interface{}) tea
```

### Error Formatting

```cursed
fr fr Errorf formats according to format specifier and returns error
slay Errorf(format tea, a ...interface{}) yikes
```

### Writer Functions

```cursed
fr fr Fprint formats using default formats and writes to w
slay Fprint(w io.Writer, a ...interface{}) (n normie, err yikes)

fr fr Fprintf formats according to format specifier and writes to w
slay Fprintf(w io.Writer, format tea, a ...interface{}) (n normie, err yikes)

fr fr Fprintln formats using default formats and writes to w with newline
slay Fprintln(w io.Writer, a ...interface{}) (n normie, err yikes)
```

### Scanning Functions

```cursed
fr fr Scan scans text from standard input
slay Scan(a ...interface{}) (n normie, err yikes)

fr fr Scanf scans text from standard input according to format
slay Scanf(format tea, a ...interface{}) (n normie, err yikes)

fr fr Scanln scans text from standard input stopping at newline
slay Scanln(a ...interface{}) (n normie, err yikes)
```

### String Scanning

```cursed
fr fr Sscan scans text from string s
slay Sscan(s tea, a ...interface{}) (n normie, err yikes)

fr fr Sscanf scans text from string s according to format
slay Sscanf(s tea, format tea, a ...interface{}) (n normie, err yikes)

fr fr Sscanln scans text from string s stopping at newline
slay Sscanln(s tea, a ...interface{}) (n normie, err yikes)
```

### Reader Scanning

```cursed
fr fr Fscan scans text from reader r
slay Fscan(r io.Reader, a ...interface{}) (n normie, err yikes)

fr fr Fscanf scans text from reader r according to format
slay Fscanf(r io.Reader, format tea, a ...interface{}) (n normie, err yikes)

fr fr Fscanln scans text from reader r stopping at newline
slay Fscanln(r io.Reader, a ...interface{}) (n normie, err yikes)
```

## Format Specifiers

### General
- `%v` - default format
- `%+v` - default format with field names for structs
- `%#v` - Go-syntax representation
- `%T` - type representation
- `%%` - literal percent sign

### Boolean
- `%t` - true or false

### Integer
- `%b` - binary
- `%c` - character
- `%d` - decimal
- `%o` - octal
- `%x` - hexadecimal (lowercase)
- `%X` - hexadecimal (uppercase)

### Floating-point
- `%e` - scientific notation (lowercase)
- `%E` - scientific notation (uppercase)
- `%f` - decimal floating point
- `%F` - decimal floating point (uppercase)
- `%g` - compact format (lowercase)
- `%G` - compact format (uppercase)

### String
- `%s` - string
- `%q` - quoted string
- `%x` - hex dump

### Pointer
- `%p` - pointer address

### Width and Precision
- `%9s` - width 9, default precision
- `%.2f` - default width, precision 2
- `%9.2f` - width 9, precision 2
- `%-9.2f` - left-justified

## Custom Formatting

```cursed
fr fr Stringer interface for custom string formatting
collab Stringer {
    String() tea
}

fr fr Formatter interface for custom format support
collab Formatter {
    Format(f State, c rune)
}

fr fr State interface for format state
collab State {
    Write(b []byte) (normie, yikes)
    Width() (wid normie, ok lit)
    Precision() (prec normie, ok lit)
    Flag(c normie) lit
}
```

## Usage Examples

```cursed
yeet "fmt"

fr fr Basic printing
fmt.Print("Hello ", "world")
fmt.Println("Hello", "world")
fmt.Printf("Hello %s\n", "world")

fr fr String formatting
message := fmt.Sprintf("User %s has %d points", "alice", 100)
error := fmt.Errorf("connection failed: %v", originalError)

fr fr Numeric formatting
fmt.Printf("Binary: %b\n", 42)
fmt.Printf("Hex: %x\n", 42)
fmt.Printf("Float: %.2f\n", 3.14159)

fr fr Scanning input
sus name tea
sus age normie
fmt.Printf("Enter name and age: ")
fmt.Scanf("%s %d", &name, &age)
fmt.Printf("Hello %s, you are %d years old\n", name, age)

fr fr Custom formatting
be_like Person squad {
    name tea
    age normie
}

slay (p Person) String() tea {
    damn fmt.Sprintf("Person{name: %s, age: %d}", p.name, p.age)
}

person := Person{name: "Alice", age: 30}
fmt.Println(person) // Output: Person{name: Alice, age: 30}
```

## Advanced Features

### Format States

```cursed
fr fr Custom formatter example
slay (p Person) Format(f fmt.State, c rune) {
    ready c == 'v' {
        ready f.Flag('+') {
            fmt.Fprintf(f, "Person{name: %s, age: %d}", p.name, p.age)
        } otherwise {
            fmt.Fprintf(f, "%s (%d)", p.name, p.age)
        }
    } otherwise {
        fmt.Fprintf(f, "%%!%c(Person=%s)", c, p.name)
    }
}
```

### Error Handling

```cursed
fr fr Safe printing with error handling
slay SafePrint(format tea, args ...interface{}) {
    defer slay() {
        ready r := recover(); r != cringe {
            fmt.Println("Error in formatting:", r)
        }
    }()
    
    fmt.Printf(format, args...)
}
```

## Implementation Guidelines

1. **Thread Safety**: All formatting functions must be thread-safe
2. **Performance**: Optimize common format specifiers
3. **Error Handling**: Provide clear error messages for invalid formats
4. **Type Safety**: Validate format specifiers against argument types
5. **Memory Management**: Efficient string building and buffer management
6. **Compatibility**: Maintain Go fmt package compatibility
