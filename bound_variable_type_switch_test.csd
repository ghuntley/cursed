# Test proper type switch variable binding

# Test case 1: Basic type narrowing
sus mixed_value = 42

# This should properly bind 'n' with type 'normie' in the first arm
vibe_check (mixed_value) {
    mood normie as n {
        # 'n' should have type 'normie' here
        vibez.spill("Got integer: " + n)
    }
    mood tea as s {
        # 's' should have type 'tea' here
        vibez.spill("Got string: " + s)
    }
    basic {
        vibez.spill("Unknown type")
    }
}

# Test case 2: Interface binding
collab TestInterface {
    slay test_method() lit
}

sus interface_value = TestInterface{}

vibe_check (interface_value) {
    mood TestInterface as ti {
        # 'ti' should have type 'TestInterface' here
        vibez.spill("Got TestInterface")
        ti.test_method()
    }
    basic {
        vibez.spill("Not TestInterface")
    }
}

# Test case 3: Wildcard pattern binding
sus any_value = "test"

vibe_check (any_value) {
    mood _ as val {
        # 'val' should preserve original type
        vibez.spill("Wildcard matched: " + val)
    }
}
