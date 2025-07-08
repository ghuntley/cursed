yeet "chaos_mode"

chaos_init(42)

# Test basic random number generation
sus rand_num thicc = chaos_rand()
vibez.spill("Random number: " + rand_num.(tea))

# Test random float
sus rand_float meal = chaos_rand_float()
vibez.spill("Random float: " + rand_float.(tea))

# Test random range
sus dice_roll thicc = chaos_rand_range(1, 6)
vibez.spill("Dice roll: " + dice_roll.(tea))

# Test coin flip
sus coin lit = chaos_flip()
vibez.spill("Coin flip: " + coin.(tea))

# Test chaos engineering
sus should_fail lit = chaos_should_fail(0.2)
vibez.spill("Should fail: " + should_fail.(tea))

# Test delay simulation
sus delay thicc = chaos_random_delay(100, 500)
vibez.spill("Random delay: " + delay.(tea))

vibez.spill("Chaos mode test completed!")
