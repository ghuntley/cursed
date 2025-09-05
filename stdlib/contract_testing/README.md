# Contract Testing Framework

A comprehensive contract testing framework for CURSED, providing robust validation of service contracts, API specifications, and interface compliance.

## Overview

The contract testing framework enables validation of:
- Service contracts and API specifications
- Interface compliance and method signatures
- Schema validation for data structures
- Consumer-driven contract testing
- Performance contract validation
- Contract regression testing

## Features

### Core Functions

#### `contract_test(service_name, contract_spec)`
Tests a service against its contract specification.
- **Parameters**: service name (tea), contract specification (tea)
- **Returns**: validation result (lit)

#### `verify_interface_contract(interface_name, implementation)`
Verifies that an implementation satisfies interface contract requirements.
- **Parameters**: interface name (tea), implementation specification (tea)
- **Returns**: compliance result (lit)

#### `schema_validation_test(schema, data)`
Validates data against a schema definition.
- **Parameters**: schema definition (tea), test data (tea)
- **Returns**: validation result (lit)

### Advanced Features

#### API Contract Validation
```cursed
sus result lit = validate_api_contract("/api/users", schema, response)
```

#### Consumer-Driven Contract Testing
```cursed
sus result lit = consumer_contract_test("WebApp", "UserAPI", contract_spec)
```

#### Performance Contract Testing
```cursed
sus result lit = performance_contract_test("SearchService", perf_spec)
```

#### Contract Regression Testing
```cursed
sus result lit = contract_regression_test("PaymentService", old_contract, new_contract)
```

## Usage Examples

### Basic Contract Testing
```cursed
yeet "contract_testing"

sus contract_spec tea = "{\"version\": \"1.0\", \"endpoints\": [\"/api/v1/users\"]}"
sus result lit = contract_test("UserService", contract_spec)

lowkey result {
    vibez.spill("Contract test passed!")
} else {
    vibez.spill("Contract test failed!")
}
```

### Interface Contract Verification
```cursed
sus interface_result lit = verify_interface_contract("PaymentInterface", "PaymentServiceImpl")
assert_true(interface_result)
```

### Schema Validation
```cursed
sus schema tea = "{\"type\": \"object\", \"properties\": {\"name\": {\"type\": \"string\"}}}"
sus data tea = "{\"name\": \"John Doe\"}"
sus validation_result lit = schema_validation_test(schema, data)
```

### Performance Contract Testing
```cursed
sus perf_spec tea = "{\"max_response_time\": 100, \"min_throughput\": 1000}"
sus perf_result lit = performance_contract_test("SearchService", perf_spec)
```

## Contract Coverage

Calculate test coverage for contracts:
```cursed
sus coverage meal = calculate_contract_coverage(total_contracts, tested_contracts)
vibez.spill("Contract coverage: " + coverage.tea + "%")
```

## Integration with testz

The framework integrates seamlessly with the testz testing framework:
```cursed
yeet "testz"
yeet "contract_testing"

test_start("API contract validation")
sus result lit = validate_api_contract("/api/orders", schema, response)
assert_true(result)
print_test_summary()
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/contract_testing/test_contract_testing.💀
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/contract_testing/test_contract_testing.💀
cargo run --bin cursed -- compile stdlib/contract_testing/test_contract_testing.💀
./test_contract_testing
```

## Implementation Details

- **Pure CURSED**: No FFI dependencies, fully self-contained
- **Comprehensive Coverage**: 20+ test functions covering all aspects
- **Performance Validation**: Built-in performance contract testing
- **Regression Testing**: Backward compatibility validation
- **Integration Ready**: Seamless testz framework integration

## Contract Types Supported

1. **Service Contracts**: API endpoint validation
2. **Interface Contracts**: Method signature compliance
3. **Data Contracts**: Schema and data validation
4. **Performance Contracts**: Response time and throughput
5. **Consumer Contracts**: Consumer-driven testing
6. **Version Contracts**: Backward compatibility

## Benefits

- **Quality Assurance**: Ensures service reliability and compatibility
- **Breaking Change Detection**: Identifies contract violations early
- **Documentation**: Contracts serve as living documentation
- **Integration Testing**: Validates service interactions
- **Performance Monitoring**: Ensures performance requirements are met
- **Regression Prevention**: Detects backward compatibility issues

## Advanced Features

- Contract completeness validation
- Multi-interface contract verification
- Batch API endpoint validation
- Complex nested schema validation
- End-to-end workflow testing
- Comprehensive coverage reporting
