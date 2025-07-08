# Simple chaos_mode demonstration
yeet "chaos_mode"

# Initialize chaos generator
chaos_init(42)

# Demonstrate basic random number generation
vibez.spill("=== Random Number Generation ===")
sus random_num thicc = chaos_rand()
vibez.spill("Random number: " + random_num.(tea))

sus random_float meal = chaos_rand_float()
vibez.spill("Random float: " + random_float.(tea))

sus dice_roll thicc = chaos_rand_range(1, 6)
vibez.spill("Dice roll (1-6): " + dice_roll.(tea))

# Demonstrate chaos engineering features
vibez.spill("=== Chaos Engineering ===")
sus should_fail lit = chaos_should_fail(0.2)
vibez.spill("Should fail (20% chance): " + should_fail.(tea))

sus delay_ms thicc = chaos_random_delay(100, 500)
vibez.spill("Random delay (100-500ms): " + delay_ms.(tea))

sus network_partition lit = chaos_network_partition(0.1)
vibez.spill("Network partition (10% chance): " + network_partition.(tea))

# Demonstrate probability distributions
vibez.spill("=== Probability Distributions ===")
sus uniform_val thicc = chaos_uniform_int(10, 20)
vibez.spill("Uniform int (10-20): " + uniform_val.(tea))

sus bernoulli_trial lit = chaos_bernoulli(0.7)
vibez.spill("Bernoulli trial (70% success): " + bernoulli_trial.(tea))

sus geometric_trials thicc = chaos_geometric(0.3)
vibez.spill("Geometric trials (30% success): " + geometric_trials.(tea))

# Demonstrate utility functions
vibez.spill("=== Utility Functions ===")
sus coin_flip lit = chaos_flip()
vibez.spill("Coin flip: " + coin_flip.(tea))

sus weighted_flip lit = chaos_weighted_flip(0.8)
vibez.spill("Weighted flip (80% true): " + weighted_flip.(tea))

vibez.spill("=== Chaos Mode Demo Complete ===")
