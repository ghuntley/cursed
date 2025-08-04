# CURSED Type System

This document describes the type system of the CURSED programming language.

## Basic Types

CURSED has several built-in basic types:

| Type Category | CURSED Types | Description |
|---------------|--------------|-------------|
| Boolean       | `lit`        | Represents `based` (true) or `cringe` (false) values |
| Numeric       | `smol`       | 8-bit signed integer |
|               | `mid`        | 16-bit signed integer |
|               | `normie`     | 32-bit signed integer |
|               | `thicc`      | 64-bit signed integer |
|               | `snack`      | 32-bit floating point |
|               | `meal`       | 64-bit floating point |
|               | `byte`       | Alias for uint8, represents a byte of data |
|               | `rune`       | Alias for int32, represents a Unicode code point |
| String        | `tea`        | A sequence of bytes representing Unicode text |
| Character     | `sip`        | Single Unicode character (rune) |
| Complex       | `extra`      | Complex number with two floating-point components |

## Composite Types

CURSED provides several ways to construct types from existing types:

| Type Category | Syntax | Description |
|---------------|--------|-------------|
| Array       | `[n]T`   | Array of n elements of type T |
| Slice       | `[]T`    | Dynamic array of elements of type T |
| Map         | `map[K]V` | Map from keys of type K to values of type V |
| Struct      | `squad`  | Collection of fields |
| Interface   | `collab` | Set of method signatures |
| Pointer     | `@T`     | Pointer to a value of type T |
| Function    | `slay`   | Function with parameters and return values |
| Channel     | `dm<T>`  | Channel of type T for goroutine communication |

## Channel Types

Channels are first-class types for goroutine communication:

### Channel Type Syntax
```
dm<T>           // Unbuffered channel of type T
dm<T>[N]        // Buffered channel of type T with capacity N
```

### Channel Operations
```
sus ch dm<normie>             // Unbuffered channel declaration
sus buffered dm<tea>[10]      // Buffered channel with capacity 10

ch <- value                   // Send operation (blocking)
value := <-ch                 // Receive operation (blocking)
value, ok := <-ch             // Receive with closed check
```

### Channel States
- **Open**: Channel can send/receive values
- **Closed**: Channel cannot accept new values but can drain existing ones
- **Nil**: Uninitialized channel (operations block forever)

### Channel Closing
```
close(ch)                     // Close channel
value, ok := <-ch             // ok is false if channel is closed
```

## Type Declarations

Types are declared using the `be_like` keyword:

```
be_like Person squad {
    name tea
    age normie
    vibes []tea
}

be_like Greeter collab {
    greet(name tea) tea
}
```

## Type Conversion

CURSED requires explicit conversion between different types:

```
sus x normie = 10
sus y snack = snack(x)  fr fr Convert int to float
```

## Zero Values

Each type has a zero value that variables of that type are initialized to when no explicit initialization is provided:

| Type | Zero Value |
|------|------------|
| lit  | `cringe` (false) |
| numeric types | `0` |
| tea  | `""` (empty string) |
| sip  | `\0` (null character) |
| pointers | `nah` (nil) |
| slices | `nah` (nil) |
| maps | `nah` (nil) |
| channels | `nah` (nil) |
| structs | Each field has its zero value |
| arrays | Each element has its zero value |

## Type Inference

CURSED supports type inference in variable declarations with the `:=` operator:

```
x := 10        fr fr x is a normie (int32)
y := "hello"   fr fr y is a tea (string)
z := based     fr fr z is a lit (bool)
w := cringe    fr fr w is a lit (bool)
c := 'a'       fr fr c is a sip (char)
```

## Type Assertions and Type Switches

Type assertions and switches allow working with interface values:

```
sus val, ok = x.(normie)  fr fr Type assertion

vibe_check v.(be_like) {  fr fr Type switch
    mood normie:
        fr fr v is an int
    mood tea:
        fr fr v is a string
    mood sip:
        fr fr v is a char
    basic:
        fr fr Other types
}
```

## Generic Types

CURSED supports generic types using square brackets:

```
be_like Stack[T] squad {
    items []T
    size normie
}

slay push[T](s @Stack[T], item T) {
    s.items = append(s.items, item)
    s.size++
}

slay pop[T](s @Stack[T]) T {
    s.size--
    damn s.items[s.size]
}
```

## Character Type Operations

The character type `sip` supports various operations:

```
sus c sip = 'a'
sus is_upper lit = c.is_uppercase()
sus is_lower lit = c.is_lowercase()
sus is_digit lit = c.is_digit()
sus is_alpha lit = c.is_alpha()
sus is_alnum lit = c.is_alnum()
sus as_upper sip = c.to_uppercase()
sus as_lower sip = c.to_lowercase()
sus as_int normie = normie(c)    fr fr Convert char to integer
```