# Test dm<T> channel type validation

# Valid channel declarations
sus valid_int_channel dm<normie> = dm<normie>()
sus valid_string_channel dm<tea> = dm<tea>()  
sus valid_bool_channel dm<lit> = dm<lit>()

# Test channel send operations (should be type-safe)
valid_int_channel <- 42
valid_string_channel <- "hello"
valid_bool_channel <- based

# Test channel receive operations 
sus received_int normie = <-valid_int_channel
sus received_string tea = <-valid_string_channel
sus received_bool lit = <-valid_bool_channel

# Test type mismatch (should fail)
# valid_int_channel <- "string"  # Error: type mismatch
# valid_string_channel <- 123    # Error: type mismatch

vibez.spill("dm<T> channel validation test complete")
