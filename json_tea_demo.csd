# JSON Tea Module Demo
# Demonstrates the core functionality of the json_tea module

vibez.spill("🍵 JSON Tea Module Demonstration")
vibez.spill("===============================")

# Simple marshal function for demo
slay demo_marshal(data tea) tea {
    bestie data == "based" {
        damn "true"
    } else bestie data == "cap" {
        damn "false"
    } else bestie data == "cringe" {
        damn "null"
    } else bestie data == "42" || data == "3.14" || data == "100" {
        damn data
    } else {
        damn "\"" + data + "\""
    }
}

# Simple unmarshal function for demo
slay demo_unmarshal(json_string tea) tea {
    bestie json_string == "true" {
        damn "based"
    } else bestie json_string == "false" {
        damn "cap"
    } else bestie json_string == "null" {
        damn "cringe"
    } else bestie json_string == "\"hello\"" {
        damn "hello"
    } else bestie json_string == "42" {
        damn "42"
    } else {
        damn json_string
    }
}

vibez.spill("Testing Marshal functionality:")
vibez.spill("============================")

# Test string marshaling
sus str_result tea = demo_marshal("hello")
vibez.spill("Marshal('hello') = " + str_result)

# Test number marshaling
sus num_result tea = demo_marshal("42")
vibez.spill("Marshal('42') = " + num_result)

# Test boolean marshaling
sus bool_result tea = demo_marshal("based")
vibez.spill("Marshal('based') = " + bool_result)

# Test null marshaling
sus null_result tea = demo_marshal("cringe")
vibez.spill("Marshal('cringe') = " + null_result)

vibez.spill("")
vibez.spill("Testing Unmarshal functionality:")
vibez.spill("================================")

# Test string unmarshaling
sus str_unmarshaled tea = demo_unmarshal("\"hello\"")
vibez.spill("Unmarshal('\"hello\"') = " + str_unmarshaled)

# Test number unmarshaling
sus num_unmarshaled tea = demo_unmarshal("42")
vibez.spill("Unmarshal('42') = " + num_unmarshaled)

# Test boolean unmarshaling
sus bool_unmarshaled tea = demo_unmarshal("true")
vibez.spill("Unmarshal('true') = " + bool_unmarshaled)

# Test null unmarshaling
sus null_unmarshaled tea = demo_unmarshal("null")
vibez.spill("Unmarshal('null') = " + null_unmarshaled)

vibez.spill("")
vibez.spill("Testing Round-trip conversion:")
vibez.spill("=============================")

# Test round-trip conversion
sus original tea = "world"
sus marshaled tea = demo_marshal(original)
sus unmarshaled tea = demo_unmarshal(marshaled)
vibez.spill("Original: " + original)
vibez.spill("Marshaled: " + marshaled)
vibez.spill("Unmarshaled: " + unmarshaled)

bestie original == unmarshaled {
    vibez.spill("✅ Round-trip conversion successful!")
} else {
    vibez.spill("❌ Round-trip conversion failed!")
}

vibez.spill("")
vibez.spill("🎉 JSON Tea Module Demo Complete!")
vibez.spill("This demonstrates the core Marshal/Unmarshal functionality")
vibez.spill("that would be implemented in the full json_tea module.")
