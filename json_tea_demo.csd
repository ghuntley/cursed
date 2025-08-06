yeet "json_tea"

vibez.spill("🍵 CURSED JSON Tea Module Demo")
vibez.spill("=====================================")

fr fr Basic JSON marshaling/unmarshaling
vibez.spill("\n--- Basic Marshal/Unmarshal ---")
sus string_data tea = "hello"
sus marshaled_string tea = json_tea.Marshal(string_data)
vibez.spill("String data: " + string_data)
vibez.spill("Marshaled: " + marshaled_string)

sus number_data tea = "42"  
sus marshaled_number tea = json_tea.Marshal(number_data)
vibez.spill("Number data: " + number_data)
vibez.spill("Marshaled: " + marshaled_number)

fr fr JSON parsing
vibez.spill("\n--- JSON Parsing ---")
sus json_string tea = "\"world\""
sus parsed tea = json_tea.parse_json(json_string)
vibez.spill("JSON string: " + json_string)
vibez.spill("Parsed: " + parsed)

fr fr Type checking
vibez.spill("\n--- Type Checking ---")
sus test_string tea = "\"hello\""
sus test_number tea = "42"
sus test_boolean tea = "true"
sus test_object tea = "{\"name\": \"John\"}"
sus test_array tea = "[1, 2, 3]"

bestie json_tea.is_string(test_string) {
    vibez.spill("✅ " + test_string + " is a string")
} else {
    vibez.spill("❌ " + test_string + " is NOT a string")
}

bestie json_tea.is_number(test_number) {
    vibez.spill("✅ " + test_number + " is a number")
} else {
    vibez.spill("❌ " + test_number + " is NOT a number")
}

bestie json_tea.is_boolean_value(test_boolean) {
    vibez.spill("✅ " + test_boolean + " is a boolean")
} else {
    vibez.spill("❌ " + test_boolean + " is NOT a boolean")
}

bestie json_tea.is_object_value(test_object) {
    vibez.spill("✅ " + test_object + " is an object")
} else {
    vibez.spill("❌ " + test_object + " is NOT an object")
}

bestie json_tea.is_array_value(test_array) {
    vibez.spill("✅ " + test_array + " is an array")
} else {
    vibez.spill("❌ " + test_array + " is NOT an array")
}

fr fr Value access
vibez.spill("\n--- Value Access ---")
sus json_obj tea = "{\"name\": \"John\", \"age\": 30}"
sus name_value tea = json_tea.get_value(json_obj, "name")
sus age_value tea = json_tea.get_value(json_obj, "age")

vibez.spill("Object: " + json_obj)
vibez.spill("Name value: " + name_value)
vibez.spill("Age value: " + age_value)

fr fr Validation
vibez.spill("\n--- JSON Validation ---")
sus valid_json tea = "{\"test\": \"value\"}"
sus invalid_json tea = "invalid"

bestie json_tea.validate_json(valid_json) {
    vibez.spill("✅ " + valid_json + " is valid JSON")
} else {
    vibez.spill("❌ " + valid_json + " is NOT valid JSON")
}

bestie json_tea.validate_json(invalid_json) {
    vibez.spill("✅ " + invalid_json + " is valid JSON")
} else {
    vibez.spill("❌ " + invalid_json + " is NOT valid JSON")
}

vibez.spill("\n🎉 JSON Tea Demo Complete!")
