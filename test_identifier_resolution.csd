# Test complex identifier resolution scenarios

# Global variables
sus global_var drip = 100
sus global_string tea = "global value"

# Function with local variables and parameter access
slay test_scoping(param1 drip, param2 tea) {
    sus local_var drip = 200
    vibez.spill("Parameter 1:", param1)
    vibez.spill("Parameter 2:", param2)
    vibez.spill("Local var:", local_var)
    vibez.spill("Global from function:", global_var)
    
    # Nested scope
    ready (based) {
        sus nested_var drip = 300
        vibez.spill("Nested var:", nested_var)
        vibez.spill("Local from nested:", local_var)
        vibez.spill("Global from nested:", global_var)
    }
    
    # This should fail if identifier resolution is broken
    sus result drip = global_var + local_var
    vibez.spill("Result calculation:", result)
}

# Test variable shadowing
slay test_shadowing() {
    sus global_var drip = 500  # Shadow global
    vibez.spill("Shadowed global:", global_var)
}

# Call functions
test_scoping(42, "test parameter")
test_shadowing()
vibez.spill("Global after functions:", global_var)
