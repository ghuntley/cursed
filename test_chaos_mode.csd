# Test suite for chaos_mode stdlib module
yeet "testz"
yeet "chaos_mode"

# Test basic random number generation
test_start("chaos_rand basic")
chaos_init(12345)
sus first_rand thicc = chaos_rand()
sus second_rand thicc = chaos_rand()
assert_true(first_rand != second_rand)
assert_true(first_rand >= 0)
assert_true(first_rand < 2147483648)

# Test random float generation
test_start("chaos_rand_float")
sus rand_float meal = chaos_rand_float()
assert_true(rand_float >= 0.0)
assert_true(rand_float <= 1.0)

# Test random range
test_start("chaos_rand_range")
sus range_val thicc = chaos_rand_range(10, 20)
assert_true(range_val >= 10)
assert_true(range_val <= 20)

# Test chaos flip
test_start("chaos_flip")
sus flip_result lit = chaos_flip()
assert_true(flip_result == based || flip_result == cap)

# Test weighted flip
test_start("chaos_weighted_flip")
sus weighted_result lit = chaos_weighted_flip(0.5)
assert_true(weighted_result == based || weighted_result == cap)

# Test weighted flip with extreme probabilities
test_start("chaos_weighted_flip extreme")
sus always_true lit = chaos_weighted_flip(1.0)
sus always_false lit = chaos_weighted_flip(0.0)
assert_true(always_true == based)
assert_true(always_false == cap)

# Test random string generation
test_start("chaos_rand_string")
sus rand_str tea = chaos_rand_string(10)
assert_true(len(rand_str) == 10)

# Test chaos initialization with different seeds
test_start("chaos_init seed variation")
chaos_init(54321)
sus seed1_val thicc = chaos_rand()
chaos_init(54321)
sus seed2_val thicc = chaos_rand()
assert_eq_int(seed1_val, seed2_val)

# Test uniform distribution
test_start("chaos_uniform_int")
sus uniform_val thicc = chaos_uniform_int(5, 15)
assert_true(uniform_val >= 5)
assert_true(uniform_val <= 15)

# Test uniform float distribution
test_start("chaos_uniform_float")
sus uniform_float meal = chaos_uniform_float(1.0, 2.0)
assert_true(uniform_float >= 1.0)
assert_true(uniform_float <= 2.0)

# Test Bernoulli distribution
test_start("chaos_bernoulli")
sus bernoulli_result lit = chaos_bernoulli(0.5)
assert_true(bernoulli_result == based || bernoulli_result == cap)

# Test geometric distribution
test_start("chaos_geometric")
sus geometric_result thicc = chaos_geometric(0.5)
assert_true(geometric_result >= 1)

# Test failure simulation
test_start("chaos_should_fail")
sus failure_result lit = chaos_should_fail(0.1)
assert_true(failure_result == based || failure_result == cap)

# Test random delay simulation
test_start("chaos_random_delay")
sus delay_ms thicc = chaos_random_delay(100, 500)
assert_true(delay_ms >= 100)
assert_true(delay_ms <= 500)

# Test network partition simulation
test_start("chaos_network_partition")
sus partition_result lit = chaos_network_partition(0.2)
assert_true(partition_result == based || partition_result == cap)

# Test resource exhaustion simulation
test_start("chaos_resource_exhaustion")
sus exhaustion_result lit = chaos_resource_exhaustion(0.1)
assert_true(exhaustion_result == based || exhaustion_result == cap)

# Test timeout simulation
test_start("chaos_timeout_simulation")
sus timeout_result lit = chaos_timeout_simulation(0.15)
assert_true(timeout_result == based || timeout_result == cap)

# Test scenario configuration generation
test_start("chaos_scenario_config")
sus config tea = chaos_scenario_config(0.1, 200, 0.05)
assert_true(len(config) > 0)

# Test Poisson distribution
test_start("chaos_poisson")
sus poisson_result thicc = chaos_poisson(2.0)
assert_true(poisson_result >= 0)

# Test Gaussian distribution
test_start("chaos_gaussian")
sus gaussian_result meal = chaos_gaussian(0.0, 1.0)
assert_true(gaussian_result != 0.0)  # Very unlikely to be exactly 0

# Test exponential distribution
test_start("chaos_exponential")
sus exponential_result meal = chaos_exponential(1.0)
assert_true(exponential_result >= 0.0)

# Test data corruption simulation
test_start("chaos_corrupt_data")
sus corruption_result lit = chaos_corrupt_data(0.05)
assert_true(corruption_result == based || corruption_result == cap)

# Test system overload simulation
test_start("chaos_system_overload")
sus overload_result lit = chaos_system_overload(0.1)
assert_true(overload_result == based || overload_result == cap)

# Test next seed generation
test_start("chaos_next_seed")
sus next_seed thicc = chaos_next_seed()
assert_true(next_seed >= 0)

# Test multiple random values
test_start("chaos_multi_rand")
sus multi_values [thicc] = chaos_multi_rand(5)
assert_true(len(multi_values) == 5)

# Test statistical mean calculation
test_start("chaos_mean")
sus test_values [thicc] = [10, 20, 30, 40, 50]
sus mean_result meal = chaos_mean(test_values)
assert_true(mean_result == 30.0)

# Test statistical variance calculation
test_start("chaos_variance")
sus variance_result meal = chaos_variance(test_values)
assert_true(variance_result > 0.0)

# Test event tracker
test_start("chaos_event_tracker")
sus event_occurred lit = chaos_event_tracker("test_event", 0.5)
assert_true(event_occurred == based || event_occurred == cap)

# Test weighted selection
test_start("chaos_weighted_select")
sus weights [thicc] = [10, 20, 30, 40]
sus selection thicc = chaos_weighted_select(weights)
assert_true(selection >= 0)
assert_true(selection < 4)

# Test randomness distribution (basic statistical test)
test_start("chaos randomness distribution")
chaos_init(98765)
sus zero_count thicc = 0
sus one_count thicc = 0
sus total_trials thicc = 100

bestie i := 0; i < total_trials; i++ {
    sus flip_result lit = chaos_flip()
    fam flip_result {
        one_count++
    } else {
        zero_count++
    }
}

# Check that we have reasonable distribution (not all zeros or all ones)
assert_true(zero_count > 0)
assert_true(one_count > 0)
assert_true(zero_count + one_count == total_trials)

# Test array shuffling simulation (simplified)
test_start("chaos_shuffle_array basic")
sus original_array [thicc] = [1, 2, 3, 4, 5]
sus shuffled_array [thicc] = chaos_shuffle_array(original_array)
assert_true(len(shuffled_array) == len(original_array))

# Test random element picking
test_start("chaos_pick_random")
sus test_array [thicc] = [100, 200, 300, 400, 500]
sus picked_element thicc = chaos_pick_random(test_array)
assert_true(picked_element >= 100)
assert_true(picked_element <= 500)

# Test chaos engineering scenario
test_start("chaos engineering scenario")
chaos_init(13579)
sus failure_occurred lit = chaos_should_fail(0.3)
sus delay_needed thicc = chaos_random_delay(50, 200)
sus partition_active lit = chaos_network_partition(0.1)

# Verify all results are valid
assert_true(failure_occurred == based || failure_occurred == cap)
assert_true(delay_needed >= 50)
assert_true(delay_needed <= 200)
assert_true(partition_active == based || partition_active == cap)

# Test probability distributions consistency
test_start("probability distributions consistency")
sus uniform_consistent thicc = chaos_uniform_int(1, 10)
sus gaussian_consistent meal = chaos_gaussian(5.0, 2.0)
sus exponential_consistent meal = chaos_exponential(0.5)

assert_true(uniform_consistent >= 1)
assert_true(uniform_consistent <= 10)
assert_true(gaussian_consistent != 0.0)
assert_true(exponential_consistent >= 0.0)

# Test chaos mode state management
test_start("chaos state management")
sus initial_seed thicc = 42
chaos_init(initial_seed)
sus first_sequence thicc = chaos_rand()
sus second_sequence thicc = chaos_rand()

# Reset with same seed
chaos_init(initial_seed)
sus first_repeat thicc = chaos_rand()
sus second_repeat thicc = chaos_rand()

assert_eq_int(first_sequence, first_repeat)
assert_eq_int(second_sequence, second_repeat)

# Test advanced chaos features
test_start("advanced chaos features")
sus config_string tea = chaos_scenario_config(0.2, 150, 0.08)
sus event_tracked lit = chaos_event_tracker("advanced_test", 0.4)
sus weighted_choice thicc = chaos_weighted_select([5, 10, 15, 20])

assert_true(len(config_string) > 20)
assert_true(event_tracked == based || event_tracked == cap)
assert_true(weighted_choice >= 0)
assert_true(weighted_choice < 4)

print_test_summary()
