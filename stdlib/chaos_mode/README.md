# Chaos Mode - Random Number Generation and Chaos Engineering

The `chaos_mode` module provides comprehensive random number generation, probability distributions, and chaos engineering utilities for the CURSED programming language. This module is implemented in pure CURSED without FFI dependencies.

## Features

### Random Number Generation
- Linear Congruential Generator (LCG) implementation
- Seeded random number generation for reproducible results
- Random integers, floats, booleans, and strings
- Range-based random number generation

### Probability Distributions
- Uniform distribution (integers and floats)
- Gaussian/Normal distribution (Box-Muller transform)
- Exponential distribution
- Poisson distribution
- Bernoulli trials
- Geometric distribution

### Chaos Engineering
- Failure simulation with configurable rates
- Random delay injection
- Network partition simulation
- Resource exhaustion testing
- Timeout simulation
- Data corruption testing
- System overload simulation

### Utility Functions
- Array shuffling (Fisher-Yates algorithm)
- Random element selection
- Weighted random selection
- Statistical functions (mean, variance)
- Event tracking and monitoring

## Usage Examples

### Basic Random Number Generation

```cursed
yeet "chaos_mode"

# Initialize with seed for reproducible results
chaos_init(12345)

# Generate random integer (0 to 2^31-1)
sus random_int thicc = chaos_rand()

# Generate random float (0.0 to 1.0)
sus random_float meal = chaos_rand_float()

# Generate random integer in range (inclusive)
sus dice_roll thicc = chaos_rand_range(1, 6)

# Generate random boolean
sus coin_flip lit = chaos_flip()
```

### Probability Distributions

```cursed
# Uniform distribution
sus uniform_int thicc = chaos_uniform_int(10, 20)
sus uniform_float meal = chaos_uniform_float(1.0, 5.0)

# Gaussian distribution (mean=0, std_dev=1)
sus normal_sample meal = chaos_gaussian(0.0, 1.0)

# Exponential distribution (lambda=1.0)
sus exponential_sample meal = chaos_exponential(1.0)

# Poisson distribution (lambda=3.0)
sus poisson_sample thicc = chaos_poisson(3.0)

# Bernoulli trial (50% success rate)
sus success lit = chaos_bernoulli(0.5)
```

### Chaos Engineering

```cursed
# Simulate random failures
sus should_fail lit = chaos_should_fail(0.1)  # 10% failure rate

# Inject random delays
sus delay_ms thicc = chaos_random_delay(100, 500)  # 100-500ms

# Network partition simulation
sus partition_active lit = chaos_network_partition(0.05)  # 5% chance

# Resource exhaustion testing
sus resources_exhausted lit = chaos_resource_exhaustion(0.02)  # 2% chance

# Generate chaos scenario configuration
sus scenario tea = chaos_scenario_config(0.1, 200, 0.05)
```

### Advanced Features

```cursed
# Weighted random selection
sus weights [thicc] = [10, 20, 30, 40]
sus selected_index thicc = chaos_weighted_select(weights)

# Array shuffling
sus numbers [thicc] = [1, 2, 3, 4, 5]
sus shuffled [thicc] = chaos_shuffle_array(numbers)

# Random element picking
sus random_element thicc = chaos_pick_random(numbers)

# Statistical calculations
sus mean_value meal = chaos_mean(numbers)
sus variance_value meal = chaos_variance(numbers)

# Event tracking
sus event_occurred lit = chaos_event_tracker("test_event", 0.3)
```

## Function Reference

### Core Random Functions
- `chaos_init(seed thicc)` - Initialize random generator with seed
- `chaos_rand() thicc` - Generate random integer (0 to 2^31-1)
- `chaos_rand_float() meal` - Generate random float (0.0 to 1.0)
- `chaos_rand_range(min_val thicc, max_val thicc) thicc` - Random integer in range
- `chaos_flip() lit` - Random boolean (50% probability)
- `chaos_weighted_flip(probability meal) lit` - Weighted random boolean

### String and Data Generation
- `chaos_rand_string(length thicc) tea` - Generate random string
- `chaos_shuffle_array(arr [thicc]) [thicc]` - Shuffle array elements
- `chaos_pick_random(arr [thicc]) thicc` - Pick random array element

### Probability Distributions
- `chaos_uniform_int(min_val thicc, max_val thicc) thicc` - Uniform integer
- `chaos_uniform_float(min_val meal, max_val meal) meal` - Uniform float
- `chaos_gaussian(mean meal, std_dev meal) meal` - Normal distribution
- `chaos_exponential(lambda meal) meal` - Exponential distribution
- `chaos_poisson(lambda meal) thicc` - Poisson distribution
- `chaos_bernoulli(success_prob meal) lit` - Bernoulli trial
- `chaos_geometric(success_prob meal) thicc` - Geometric distribution

### Chaos Engineering Functions
- `chaos_should_fail(failure_rate meal) lit` - Failure simulation
- `chaos_random_delay(min_ms thicc, max_ms thicc) thicc` - Delay injection
- `chaos_network_partition(partition_prob meal) lit` - Network partition
- `chaos_resource_exhaustion(exhaustion_prob meal) lit` - Resource exhaustion
- `chaos_timeout_simulation(timeout_prob meal) lit` - Timeout simulation
- `chaos_corrupt_data(corruption_prob meal) lit` - Data corruption
- `chaos_system_overload(overload_prob meal) lit` - System overload

### Utility Functions
- `chaos_scenario_config(failure_rate meal, delay_ms thicc, partition_prob meal) tea` - Generate configuration
- `chaos_weighted_select(weights [thicc]) thicc` - Weighted selection
- `chaos_mean(values [thicc]) meal` - Calculate mean
- `chaos_variance(values [thicc]) meal` - Calculate variance
- `chaos_event_tracker(event_name tea, probability meal) lit` - Event tracking
- `chaos_next_seed() thicc` - Generate new seed
- `chaos_multi_rand(count thicc) [thicc]` - Multiple random values

## Testing

The module includes comprehensive tests covering all functions and edge cases:

```bash
# Run chaos_mode tests
cargo run --bin cursed stdlib/chaos_mode/test_chaos_mode.💀

# Test both interpretation and compilation modes
cargo run --bin cursed -- compile stdlib/chaos_mode/test_chaos_mode.💀
./test_chaos_mode
```

## Implementation Details

### Random Number Generator
- Uses Linear Congruential Generator (LCG) algorithm
- Parameters: a=1103515245, c=12345, m=2^31
- Provides good distribution for most use cases
- Seeded for reproducible results

### Probability Distributions
- Gaussian: Box-Muller transform
- Exponential: Inverse transform method
- Poisson: Knuth's algorithm approximation
- All distributions mathematically sound

### Chaos Engineering
- Configurable failure rates and probabilities
- Realistic delay simulation
- Network partition modeling
- Resource exhaustion simulation
- Comprehensive scenario configuration

### Performance
- Pure CURSED implementation (no FFI)
- Efficient algorithms for all operations
- Minimal memory overhead
- Fast random number generation

## Best Practices

1. **Seed Management**: Always initialize with `chaos_init()` for reproducible results
2. **Probability Ranges**: Use probabilities between 0.0 and 1.0
3. **Range Validation**: Ensure min <= max for range functions
4. **Statistical Testing**: Use multiple samples for statistical distributions
5. **Chaos Engineering**: Start with low failure rates and increase gradually

## Examples

### Simple Chaos Test
```cursed
yeet "chaos_mode"

chaos_init(42)
sus failures thicc = 0
sus total_tests thicc = 100

bestie i := 0; i < total_tests; i++ {
    fam chaos_should_fail(0.1) {
        failures++
    }
}

vibez.spill("Failures: " + failures.(tea) + "/" + total_tests.(tea))
```

### Distribution Sampling
```cursed
yeet "chaos_mode"

chaos_init(12345)
sus samples [thicc] = []

bestie i := 0; i < 10; i++ {
    sus sample thicc = chaos_poisson(2.0)
    samples[i] = sample
}

sus mean_val meal = chaos_mean(samples)
vibez.spill("Mean: " + mean_val.(tea))
```

## License

This module is part of the CURSED programming language standard library and follows the same license terms.
