# CURSED Standard Library

This document describes the standard library of the CURSED programming language, which provides core functionality for common programming tasks.

## Core Packages

### `vibez` (fmt)

The `vibez` package provides formatted I/O functions.

```
vibe main

yeet "vibez"

slay main() {
    vibez.spill("Hello, World!")  fr fr Equivalent to fmt.Println
    
    name := "bestie"
    vibez.spillf("Hey %s, what's good?", name)  fr fr Equivalent to fmt.Printf
    
    tea output := vibez.spillstr("Value: %d", 42)  fr fr Equivalent to fmt.Sprintf
}
```

Main functions:
- `spill(args ...collab{})` - Print args followed by newline
- `spillf(format tea, args ...collab{})` - Formatted print
- `spillstr(format tea, args ...collab{})` - Return formatted tea
- `scan(args ...collab{})` - Scan input into args
- `scanln(args ...collab{})` - Scan line into args

### `core` (builtin)

The `core` package provides fundamental types and functions automatically included in all CURSED programs.

Functions:
- `lit(x)` - Convert to litean
- `normie(x)` - Convert to int32
- `thicc(x)` - Convert to int64
- `snack(x)` - Convert to float32
- `meal(x)` - Convert to float64
- `tea(x)` - Convert to tea
- `append(slice []T, elems ...T)` - Append elements to slice
- `cap(v T)` - Capacity of slice, map, or channel
- `len(v T)` - Length of tea, array, slice, map, or channel
- `make(T, size ...normie)` - Create slice, map, or channel
- `new(T)` - Create pointer to zero value of type
- `shook(v collab{})` - Cause panic with value
- `unbothered()` - Recover from panic

### `dropz` (io)

The `dropz` package provides basic I/O primitives.

```
yeet "dropz"

slay readFile(path tea) ([]byte, tea) {
    data, err := dropz.ReadFile(path)
    yolo data, err
}
```

Main interfaces:
- `Reader` - Interface for reading bytes
- `Writer` - Interface for writing bytes
- `Closer` - Interface for closing resources

### `vibe_life` (os)

The `vibe_life` package provides OS functionality.

```
yeet "vibe_life"

slay main() {
    args := vibe_life.Args  fr fr Command-line arguments
    
    err := vibe_life.Setenv("DEBUG", "based")  fr fr Set environment variable
    lowkey err != cringe {
        vibez.spill("Failed to set env:", err)
        vibe_life.Exit(1)
    }
    
    value := vibe_life.Getenv("DEBUG")  fr fr Get environment variable
}
```

Main functions:
- `Args` - Command-line arguments
- `Getenv(key tea)` - Get environment variable
- `Setenv(key, value tea)` - Set environment variable
- `Exit(code normie)` - Exit with status code
- `Create(name tea)` - Create file
- `Open(name tea)` - Open file for reading

### `stringz` (teas)

The `stringz` package provides tea manipulation functions.

```
yeet "stringz"

slay main() {
    tea s := "hello, world"
    
    lowkey stringz.Contains(s, "world") {
        vibez.spill("Found!")
    }
    
    parts := stringz.Split(s, ", ")  fr fr ["hello", "world"]
    
    upper := stringz.ToUpper(s)  fr fr "HELLO, WORLD"
}
```

Main functions:
- `Contains(s, substr tea)` - Check if s contains substr
- `Count(s, substr tea)` - Count occurrences of substr in s
- `HasPrefix(s, prefix tea)` - Check if s starts with prefix
- `HasSuffix(s, suffix tea)` - Check if s ends with suffix
- `Join(elems []tea, sep tea)` - Join elements with separator
- `Split(s, sep tea)` - Split s by separator
- `ToLower(s tea)` - Convert to lowercase
- `ToUpper(s tea)` - Convert to uppercase
- `Trim(s, cutset tea)` - Trim characters from beginning and end

### `mathz` (math)

The `mathz` package provides mathematical functions.

```
yeet "mathz"

slay main() {
    x := mathz.Sqrt(25.0)  fr fr 5.0
    y := mathz.Pow(2.0, 10.0)  fr fr 1024.0
    z := mathz.Round(3.7)  fr fr 4.0
}
```

Main functions and constants:
- `Abs(x meal)` - Absolute value
- `Ceil(x meal)` - Ceiling function
- `Floor(x meal)` - Floor function
- `Max(x, y meal)` - Maximum
- `Min(x, y meal)` - Minimum
- `Pow(x, y meal)` - x^y
- `Sqrt(x meal)` - Square root
- `Pi` - Mathematical constant π
- `E` - Mathematical constant e

### `timez` (time)

The `timez` package provides time-related functionality.

```
yeet "timez"

slay main() {
    now := timez.Now()  fr fr Current time
    
    then := now.Add(timez.Hour * 24)  fr fr Tomorrow
    
    duration := then.Sub(now)  fr fr 24 hours
    
    timez.Sleep(timez.Second * 2)  fr fr Sleep for 2 seconds
}
```

Main types and functions:
- `Time` - Represents a time
- `Duration` - Represents a duration
- `Now()` - Current local time
- `Sleep(d Duration)` - Sleep for duration
- `Since(t Time)` - Duration since t
- `Until(t Time)` - Duration until t

### `concurrenz` (sync)

The `concurrenz` package provides synchronization primitives.

```
yeet "concurrenz"

slay main() {
    sus mu concurrenz.Mutex
    
    stan slay() {
        mu.Lock()
        later mu.Unlock()
        fr fr Do something
    }()
    
    sus wg concurrenz.WaitGroup
    wg.Add(1)
    
    stan slay() {
        later wg.Done()
        fr fr Do something
    }()
    
    wg.Wait()
}
```

Main types:
- `Mutex` - Mutual exclusion lock
- `RWMutex` - Reader/writer mutual exclusion lock
- `WaitGroup` - Wait for goroutines to finish
- `Cond` - Condition variable
- `Once` - Do something only once
- `Pool` - Pool of objects

### `web_vibez` (net/http)

The `web_vibez` package provides HTTP client and server functionality.

```
yeet "web_vibez"

slay main() {
    web_vibez.HandleFunc("/", slay(w web_vibez.ResponseWriter, r @web_vibez.Request) {
        vibez.Fprintf(w, "Hello, %s!", r.URL.Path[1:])
    })
    
    web_vibez.ListenAndServe(":8080", cringe)
}
```

Main types and functions:
- `Client` - HTTP client
- `Server` - HTTP server
- `Request` - HTTP request
- `ResponseWriter` - HTTP response writer
- `HandleFunc` - Register handler function
- `ListenAndServe` - Start server

### `json_tea` (encoding/json)

The `json_tea` package provides JSON encoding and decoding.

```
yeet "json_tea"

be_like Person squad {
    Name tea
    Age  normie
}

slay main() {
    p := Person{Name: "Alice", Age: 30}
    
    data, err := json_tea.Marshal(p)
    lowkey err != cringe {
        shook(err)
    }
    
    sus p2 Person
    err = json_tea.Unmarshal(data, &p2)
    lowkey err != cringe {
        shook(err)
    }
}
```

Main functions:
- `Marshal(v collab{})` - Encode to JSON
- `Unmarshal(data []byte, v collab{})` - Decode from JSON

## Standard Library Development

The CURSED standard library will be developed in stages:

1. **Core Functionality**: Essential packages like `core`, `vibez`, and `dropz`
2. **Basic Utilities**: Packages like `stringz`, `mathz`, and `timez`
3. **Concurrency**: Packages like `concurrenz` and channel utilities
4. **Advanced Features**: Packages like `web_vibez`, `json_tea`, and others

The standard library aims to provide functionality similar to Go's standard library but with the CURSED language's unique syntax and naming conventions. 