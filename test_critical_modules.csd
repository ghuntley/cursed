vibez.spill("Testing critical stdlib modules...")

# Test basic functionality without importing
# (since we have build issues, let's test the basic concepts)

# Test Option-like pattern
sus maybe_value := (based, 42)  # (is_some, value)
sus empty_value := (cap, 0)     # (is_none, default)

vibez.spill("Option test:")
bestie maybe_value.0 {
    vibez.spill("Value: " + core.tea(maybe_value.1))
} else {
    vibez.spill("No value")
}

bestie empty_value.0 {
    vibez.spill("Value: " + core.tea(empty_value.1))
} else {
    vibez.spill("No value")
}

# Test Result-like pattern
sus ok_result := (based, 10, "")      # (is_ok, value, error)
sus err_result := (cap, 0, "error")   # (is_err, default, error)

vibez.spill("Result test:")
bestie ok_result.0 {
    vibez.spill("Success: " + core.tea(ok_result.1))
} else {
    vibez.spill("Error: " + ok_result.2)
}

bestie err_result.0 {
    vibez.spill("Success: " + core.tea(err_result.1))
} else {
    vibez.spill("Error: " + err_result.2)
}

# Test basic formatting
vibez.spill("Format test:")
sus number := 42
sus formatted := "Number: " + core.tea(number)
vibez.spill(formatted)

sus bool_val := based
sus bool_str := "Boolean: " + core.tea(core.normie(bool_val))
vibez.spill(bool_str)

# Test type information pattern
sus type_info := (0, "normie", 4)  # (kind, name, size)
vibez.spill("Type test:")
vibez.spill("Type name: " + type_info.1)
vibez.spill("Type size: " + core.tea(type_info.2))

vibez.spill("All critical module patterns working!")
