# CURSED Code Formatter Test Cases

## Basic Variable Declarations

### Before:
```cursed
sus x drip=42;sus name tea="Hello"
```

### After:
```cursed
sus x drip = 42
sus name tea = "Hello"
```

## Function Definitions

### Before:
```cursed
slay add(a drip,b drip)drip{damn a+b}
```

### After:
```cursed
slay add(a drip, b drip) drip {
    damn a + b
}
```

## Struct Definitions

### Before:
```cursed
squad Person{spill name tea;spill age drip}
```

### After:
```cursed
squad Person {
    spill name tea
    spill age drip
}
```

## Interface Definitions

### Before:
```cursed
collab Drawable{slay draw();slay area()normie}
```

### After:
```cursed
collab Drawable {
    slay draw()
    slay area() normie
}
```

## Control Flow

### Before:
```cursed
bestie(x<100){x=x+1;vibez.spill(x)}
```

### After:
```cursed
bestie (x < 100) {
    x = x + 1
    vibez.spill(x)
}
```

## Current Implementation Status ✅

The CURSED code formatter has been implemented with the following features:

- ✅ Automatic indentation and whitespace normalization
- ✅ Statement separation (semicolon to newline conversion)
- ✅ Function and struct formatting with proper braces
- ✅ Configurable formatting options
- ✅ Integration with main CURSED CLI (`cursed format`)
- ✅ Support for all CURSED keywords and syntax

## Usage

```bash
# Format a single file
./zig-out/bin/cursed format file.csd

# Format with verbose output
./zig-out/bin/cursed format file.csd --verbose

# Check formatting without modifying files
./zig-out/bin/cursed format file.csd --check

# Show formatting differences
./zig-out/bin/cursed format file.csd --diff
```
