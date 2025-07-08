# Chaos Mode - Pure CURSED Random Number Generation and Chaos Engineering
# Provides comprehensive randomization, probability distributions, and chaos utilities

# Linear Congruential Generator (LCG) state
sus chaos_seed thicc = 1

# Initialize chaos with custom seed
slay chaos_init(seed thicc) {
    chaos_seed = seed
}

# Basic LCG random number generator (0 to 2^31-1)
slay chaos_rand() thicc {
    chaos_seed = (chaos_seed * 1103515245 + 12345) % 2147483648
    damn chaos_seed
}

# Random float between 0.0 and 1.0
slay chaos_rand_float() meal {
    sus rand_val thicc = chaos_rand()
    damn rand_val.(meal) / 2147483648.0
}

# Random integer between min and max (inclusive)
slay chaos_rand_range(min_val thicc, max_val thicc) thicc {
    sus range thicc = max_val - min_val + 1
    sus rand_val thicc = chaos_rand() % range
    damn min_val + rand_val
}

# Random boolean with 50% probability
slay chaos_flip() lit {
    sus rand_val thicc = chaos_rand()
    damn (rand_val % 2) == 0
}

# Random boolean with custom probability (0.0 to 1.0)
slay chaos_weighted_flip(probability meal) lit {
    sus rand_val meal = chaos_rand_float()
    damn rand_val < probability
}

# Generate random string of specified length
slay chaos_rand_string(length thicc) tea {
    sus result tea = ""
    sus chars tea = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    sus chars_len thicc = 62
    
    bestie i := 0; i < length; i++ {
        sus char_index thicc = chaos_rand() % chars_len
        # Simulate character access (simplified for pure CURSED)
        sus char_code thicc = 97 + (char_index % 26)  # a-z range
        result = result + char_code.(tea)
    }
    
    damn result
}

# Shuffle array elements (Fisher-Yates algorithm)
slay chaos_shuffle_array(arr [thicc]) [thicc] {
    sus result [thicc] = arr
    sus n thicc = len(arr)
    
    bestie i := n - 1; i > 0; i-- {
        sus j thicc = chaos_rand() % (i + 1)
        # Swap elements
        sus temp thicc = result[i]
        result[i] = result[j]
        result[j] = temp
    }
    
    damn result
}

# Pick random element from array
slay chaos_pick_random(arr [thicc]) thicc {
    sus n thicc = len(arr)
    sus index thicc = chaos_rand() % n
    damn arr[index]
}

# Gaussian/Normal distribution using Box-Muller transform
slay chaos_gaussian(mean meal, std_dev meal) meal {
    sus u1 meal = chaos_rand_float()
    sus u2 meal = chaos_rand_float()
    
    # Box-Muller transform (simplified)
    sus z0 meal = sqrt(-2.0 * ln(u1)) * cos(2.0 * 3.14159 * u2)
    damn mean + z0 * std_dev
}

# Exponential distribution
slay chaos_exponential(lambda meal) meal {
    sus u meal = chaos_rand_float()
    damn -ln(u) / lambda
}

# Chaos engineering: random failure simulation
slay chaos_should_fail(failure_rate meal) lit {
    damn chaos_weighted_flip(failure_rate)
}

# Chaos engineering: random delay simulation (returns delay in ms)
slay chaos_random_delay(min_ms thicc, max_ms thicc) thicc {
    damn chaos_rand_range(min_ms, max_ms)
}

# Chaos engineering: random network partition
slay chaos_network_partition(partition_prob meal) lit {
    damn chaos_weighted_flip(partition_prob)
}

# Chaos engineering: random resource exhaustion
slay chaos_resource_exhaustion(exhaustion_prob meal) lit {
    damn chaos_weighted_flip(exhaustion_prob)
}

# Chaos engineering: random timeout simulation
slay chaos_timeout_simulation(timeout_prob meal) lit {
    damn chaos_weighted_flip(timeout_prob)
}

# Generate chaos scenario configuration
slay chaos_scenario_config(failure_rate meal, delay_ms thicc, partition_prob meal) tea {
    sus config tea = "chaos_scenario:"
    config = config + " failure_rate=" + failure_rate.(tea)
    config = config + " delay_ms=" + delay_ms.(tea)
    config = config + " partition_prob=" + partition_prob.(tea)
    damn config
}

# Probability distribution utilities
slay chaos_uniform_int(min_val thicc, max_val thicc) thicc {
    damn chaos_rand_range(min_val, max_val)
}

slay chaos_uniform_float(min_val meal, max_val meal) meal {
    sus rand_val meal = chaos_rand_float()
    damn min_val + rand_val * (max_val - min_val)
}

# Poisson distribution approximation
slay chaos_poisson(lambda meal) thicc {
    sus L meal = exp(-lambda)
    sus k thicc = 0
    sus p meal = 1.0
    
    bestie p > L {
        k++
        sus u meal = chaos_rand_float()
        p = p * u
    }
    
    damn k - 1
}

# Bernoulli trial
slay chaos_bernoulli(success_prob meal) lit {
    damn chaos_weighted_flip(success_prob)
}

# Geometric distribution
slay chaos_geometric(success_prob meal) thicc {
    sus trials thicc = 1
    
    bestie cap chaos_bernoulli(success_prob) {
        trials++
    }
    
    damn trials
}

# Chaos testing: random data corruption
slay chaos_corrupt_data(corruption_prob meal) lit {
    damn chaos_weighted_flip(corruption_prob)
}

# Chaos testing: random system overload
slay chaos_system_overload(overload_prob meal) lit {
    damn chaos_weighted_flip(overload_prob)
}

# Generate random seed from current state
slay chaos_next_seed() thicc {
    damn chaos_rand()
}

# Advanced chaos: multiple random values
slay chaos_multi_rand(count thicc) [thicc] {
    sus results [thicc] = []
    
    bestie i := 0; i < count; i++ {
        sus rand_val thicc = chaos_rand()
        # Simulate array append
        results[i] = rand_val
    }
    
    damn results
}

# Statistical utilities
slay chaos_mean(values [thicc]) meal {
    sus sum thicc = 0
    sus count thicc = len(values)
    
    bestie i := 0; i < count; i++ {
        sum = sum + values[i]
    }
    
    damn sum.(meal) / count.(meal)
}

slay chaos_variance(values [thicc]) meal {
    sus mean_val meal = chaos_mean(values)
    sus sum_sq meal = 0.0
    sus count thicc = len(values)
    
    bestie i := 0; i < count; i++ {
        sus diff meal = values[i].(meal) - mean_val
        sum_sq = sum_sq + (diff * diff)
    }
    
    damn sum_sq / count.(meal)
}

# Chaos monitoring: track random events
slay chaos_event_tracker(event_name tea, probability meal) lit {
    sus should_occur lit = chaos_weighted_flip(probability)
    
    fam should_occur {
        vibez.spill("Chaos event occurred: " + event_name)
    }
    
    damn should_occur
}

# Advanced randomization: weighted random selection
slay chaos_weighted_select(weights [thicc]) thicc {
    sus total_weight thicc = 0
    sus count thicc = len(weights)
    
    # Calculate total weight
    bestie i := 0; i < count; i++ {
        total_weight = total_weight + weights[i]
    }
    
    sus random_val thicc = chaos_rand() % total_weight
    sus current_weight thicc = 0
    
    bestie i := 0; i < count; i++ {
        current_weight = current_weight + weights[i]
        fam current_weight > random_val {
            damn i
        }
    }
    
    damn count - 1  # Fallback
}
