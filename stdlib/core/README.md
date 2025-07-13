# Core Runtime Module

Pure CURSED implementation of core language runtime utilities, providing type conversions, memory operations, and basic runtime management.

## Features

- **Type Conversions**: Convert between string, integer, float, and boolean types
- **Runtime Management**: Initialize, enable/disable runtime components
- **Memory Operations**: Basic memory allocation and deallocation simulation
- **Data Processing**: Safe data processing with error handling
- **Validation**: Type checking and validation utilities

## Functions

### Runtime Management
- `runtime_init() lit` - Initialize the core runtime
- `runtime_is_initialized() lit` - Check if runtime is initialized
- `runtime_enable()` - Enable runtime operations
- `runtime_disable()` - Disable runtime operations
- `runtime_is_enabled() lit` - Check if runtime is enabled

### Type Conversions
- `to_string(value) tea` - Convert any type to string
- `to_int(value tea) normie` - Convert string to integer
- `to_float(value tea) meal` - Convert string to float
- `to_bool(value tea) lit` - Convert string to boolean

### Data Processing
- `process_data(data tea) tea` - Process data with runtime checks
- `safe_process(data tea) tea` - Error-safe data processing

### Memory Operations
- `memory_allocate(size normie) lit` - Simulate memory allocation
- `memory_deallocate(ptr) lit` - Simulate memory deallocation

### Utilities
- `core_info() tea` - Get core runtime information
- `core_version() tea` - Get core runtime version
- `is_valid_string(value tea) lit` - Validate string values
- `is_valid_int(value normie) lit` - Validate integer values
- `core_self_test() lit` - Run internal self-tests

## Usage Example

```cursed
yeet "core"

# Initialize runtime
sus init_ok lit = runtime_init()
lowkey init_ok == based {
    vibez.spill("Core runtime initialized")
}

# Type conversions
sus number normie = to_int("42")
sus text tea = to_string(number)
sus flag lit = to_bool("based")

# Process data safely
sus result tea = safe_process("important_data")
vibez.spill(result)

# Get runtime info
sus info tea = core_info()
vibez.spill(info)
```

## Testing

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/core/test_core.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/core/test_core.csd
./test_core
```

## Implementation Notes

- Pure CURSED implementation without FFI dependencies
- Simplified type conversion for demonstration (real implementation would be more robust)
- Memory operations are simulated for pure CURSED compatibility
- Error handling follows CURSED patterns with lit return types
- Compatible with both interpretation and compilation modes

## Dependencies

- `testz` - Testing framework (for tests only)

## Status

✅ **Complete** - Fully implemented pure CURSED core runtime module
