# JSON Tea Module

A comprehensive JSON parsing and generation module for CURSED with enhanced Marshal/Unmarshal functionality.

## Features

- **Marshal/Unmarshal**: Convert between CURSED data and JSON strings
- **Type Detection**: Automatic detection of data types for marshaling
- **Validation**: JSON format validation and schema validation
- **Formatting**: Compact and indented JSON formatting
- **Pure CURSED**: No FFI dependencies, implemented entirely in CURSED

## Core Functions

### Marshal Functions

```cursed
yeet "json_tea"

# Basic marshaling
sus json_str tea = json_tea.Marshal("hello")        # "\"hello\""
sus json_num tea = json_tea.Marshal("42")           # "42"
sus json_bool tea = json_tea.Marshal("based")       # "true"

# Advanced marshaling
sus indented tea = json_tea.MarshalIndent(data, "", "  ")
sus compact tea = json_tea.MarshalCompact(data)
```

### Unmarshal Functions

```cursed
# Basic unmarshaling
sus str_data tea = json_tea.Unmarshal("\"hello\"")  # "hello"
sus num_data tea = json_tea.Unmarshal("42")         # "42"
sus bool_data tea = json_tea.Unmarshal("true")      # "based"

# Type-specific unmarshaling
sus map_data tea = json_tea.UnmarshalToMap("{\"key\": \"value\"}")
sus slice_data tea = json_tea.UnmarshalToSlice("[1, 2, 3]")
```

### Validation

```cursed
# JSON validation
bestie json_tea.IsValidJSON("{\"valid\": true}") {
    vibez.spill("Valid JSON!")
}

# Schema validation
bestie json_tea.ValidateSchema("\"hello\"", "string") {
    vibez.spill("String schema matches!")
}
```

## Type Mapping

| CURSED Type | JSON Type | Example |
|-------------|-----------|---------|
| `"based"` | `true` | `json_tea.Marshal("based")` → `"true"` |
| `"cap"` | `false` | `json_tea.Marshal("cap")` → `"false"` |
| `"cringe"` | `null` | `json_tea.Marshal("cringe")` → `"null"` |
| Numbers | Numbers | `json_tea.Marshal("42")` → `"42"` |
| Strings | Strings | `json_tea.Marshal("hello")` → `"\"hello\""` |

## Error Handling

Functions return error strings prefixed with "ERROR:" for invalid input:

```cursed
sus result tea = json_tea.Unmarshal("invalid json")
bestie json_tea.string_starts_with(result, "ERROR") {
    vibez.spill("Invalid JSON detected")
}
```

## Legacy Compatibility

The module provides legacy compatibility functions:

- `marshal(data)` → `Marshal(data)`
- `unmarshal(json)` → `Unmarshal(json)`
- `parse(json)` → `Unmarshal(json)`
- `stringify(data)` → `Marshal(data)`

## Implementation Status

✅ **Core Functions**: Marshal, Unmarshal, basic type support
✅ **Validation**: JSON validation and schema validation
✅ **Formatting**: Compact and indented output
✅ **Pure CURSED**: No external dependencies
✅ **Legacy Support**: Backward compatibility with existing JSON modules

## Usage Example

```cursed
yeet "json_tea"

# Create some data
sus user_data tea = "John"
sus age_data tea = "30"
sus active_data tea = "based"

# Marshal to JSON
sus json_user tea = json_tea.Marshal(user_data)     # "\"John\""
sus json_age tea = json_tea.Marshal(age_data)       # "30"
sus json_active tea = json_tea.Marshal(active_data) # "true"

vibez.spill("JSON User: " + json_user)
vibez.spill("JSON Age: " + json_age)
vibez.spill("JSON Active: " + json_active)

# Unmarshal from JSON
sus parsed_user tea = json_tea.Unmarshal(json_user)
vibez.spill("Parsed User: " + parsed_user)
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/json_tea/test_json_tea.csd
```

Or test compilation mode:

```bash
cargo run --bin cursed -- compile stdlib/json_tea/test_json_tea.csd
./test_json_tea
```
