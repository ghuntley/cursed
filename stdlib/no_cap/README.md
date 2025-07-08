# no_cap - Truth/Assertion Utilities and Boolean Logic Functions

The `no_cap` module provides comprehensive truth/assertion utilities, boolean logic functions, and validation helpers for the CURSED programming language. This module is implemented in pure CURSED without FFI dependencies.

## Features

### Truth/Assertion Utilities
- `assert_truth(value lit) lit` - Assert that a value is true
- `assert_fact(condition lit, message tea) lit` - Assert condition with custom message
- `verify_claim(claim lit) lit` - Verify a claim is true
- `confirm_reality(statement lit) lit` - Confirm a statement is not false

### Boolean Logic Functions
- `logic_and(a lit, b lit) lit` - Logical AND operation
- `logic_or(a lit, b lit) lit` - Logical OR operation  
- `logic_not(value lit) lit` - Logical NOT operation
- `logic_xor(a lit, b lit) lit` - Logical XOR operation
- `logic_nand(a lit, b lit) lit` - Logical NAND operation
- `logic_nor(a lit, b lit) lit` - Logical NOR operation
- `logic_implies(premise lit, conclusion lit) lit` - Logical implication
- `logic_biconditional(a lit, b lit) lit` - Logical biconditional (if and only if)

### Validation Helpers
- `validate_true(value lit) lit` - Validate value is true
- `validate_false(value lit) lit` - Validate value is false
- `validate_not_null(value lit) lit` - Validate value is not null/false
- `validate_equals(a lit, b lit) lit` - Validate two values are equal
- `validate_not_equals(a lit, b lit) lit` - Validate two values are not equal

### Fact Checking Utilities
- `check_consistency(facts []lit) lit` - Check if all facts are consistent (all true)
- `check_contradiction(a lit, b lit) lit` - Check if two values contradict each other
- `check_tautology(propositions []lit) lit` - Check if all propositions are true
- `check_satisfiability(conditions []lit) lit` - Check if at least one condition is true

### Advanced Truth Operations
- `truth_table_and(inputs []lit) lit` - Apply AND operation to array of inputs
- `truth_table_or(inputs []lit) lit` - Apply OR operation to array of inputs
- `majority_vote(votes []lit) lit` - Return true if majority of votes are true

### Utility Functions
- `count_truths(values []lit) normie` - Count number of true values in array
- `count_falsehoods(values []lit) normie` - Count number of false values in array
- `truth_ratio(values []lit) meal` - Calculate ratio of true values (0.0 to 1.0)
- `all_true(values []lit) lit` - Check if all values are true
- `any_true(values []lit) lit` - Check if any value is true
- `none_true(values []lit) lit` - Check if no values are true

## Usage Examples

```cursed
yeet "no_cap"

# Basic truth assertions
assert_truth(based)  # Returns: based
assert_fact(5 > 3, "Five should be greater than three")

# Boolean logic operations
sus result1 lit = logic_and(based, based)  # Returns: based
sus result2 lit = logic_xor(based, cap)    # Returns: based
sus result3 lit = logic_implies(cap, based) # Returns: based

# Validation helpers
validate_true(based)    # Returns: based
validate_false(cap)     # Returns: based
validate_equals(based, based) # Returns: based

# Fact checking with arrays
sus facts []lit = [based, based, based]
check_consistency(facts)  # Returns: based
check_tautology(facts)    # Returns: based

sus votes []lit = [based, based, cap]
majority_vote(votes)      # Returns: based

# Utility functions
count_truths(votes)       # Returns: 2
truth_ratio(votes)        # Returns: 0.666...
all_true(votes)          # Returns: cap
any_true(votes)          # Returns: based
```

## Truth Tables

### Basic Operations
| A | B | AND | OR | XOR | NAND | NOR | IMPLIES | BICONDITIONAL |
|---|---|-----|----|----|------|-----|---------|---------------|
| T | T |  T  | T  | F  |  F   |  F  |    T    |       T       |
| T | F |  F  | T  | T  |  T   |  F  |    F    |       F       |
| F | T |  F  | T  | T  |  T   |  F  |    T    |       F       |
| F | F |  F  | F  | F  |  T   |  T  |    T    |       T       |

### Unary Operations
| A | NOT |
|---|-----|
| T |  F  |
| F |  T  |

## Testing

The module includes comprehensive test coverage with 70+ test cases covering:
- All truth/assertion utilities
- Complete boolean logic function truth tables
- Validation helper edge cases
- Fact checking with various array configurations
- Advanced truth operations
- Utility functions with empty arrays and edge cases

```bash
# Run tests in interpretation mode
cargo run --bin cursed stdlib/no_cap/test_no_cap.csd

# Run tests in compilation mode
cargo run --bin cursed -- compile stdlib/no_cap/test_no_cap.csd
./test_no_cap
```

## Implementation Details

- **Pure CURSED**: No FFI dependencies, fully implemented in CURSED
- **Type Safety**: All functions use proper CURSED type annotations
- **Performance**: Efficient implementations using CURSED's native boolean operations
- **Memory Management**: Automatic memory management through CURSED's GC
- **Error Handling**: Graceful handling of edge cases like empty arrays

## Design Philosophy

The `no_cap` module embodies the principle of "no cap" (no lies/no false statements) by providing robust truth checking and assertion utilities. It enables developers to:

1. **Verify Claims**: Assert and validate boolean conditions with confidence
2. **Logical Reasoning**: Perform complex boolean logic operations
3. **Fact Checking**: Validate consistency and satisfiability of propositions
4. **Truth Analysis**: Analyze collections of boolean values statistically

This module is essential for building reliable, truthful applications in CURSED where correctness and verification are paramount.
