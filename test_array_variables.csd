sus a drip = 10
sus b drip = 20  
sus c drip = 30
sus my_array []drip = [a, b, c]
vibez.spill("Array with variables:", my_array)

# Test simple literals
sus literal_array []drip = [1, 2, 3]
vibez.spill("Literal array:", literal_array)

# Test mixed literals and variables
sus mixed_array []drip = [a, 5, c]
vibez.spill("Mixed array:", mixed_array)
