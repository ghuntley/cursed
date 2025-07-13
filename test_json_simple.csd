yeet "testz"
yeet "json"

# Simple JSON test to verify functionality
vibez.spill("🔧 Testing Enhanced JSON Module")

# Test basic parsing
sus result tea = json.parse_json("\"hello\"")
vibez.spill("Parse result: " + result)

# Test basic validation
sus valid lit = json.is_valid_json("\"test\"")
vibez.spill("Validation test: " + (valid && "passed" || "failed"))

# Test stringify
sus stringified tea = json.stringify("world")
vibez.spill("Stringify result: " + stringified)

# Test object parsing
sus obj_result tea = json.parse_object("{}")
vibez.spill("Object parse result: " + obj_result)

# Test array parsing
sus arr_result tea = json.parse_array("[]")
vibez.spill("Array parse result: " + arr_result)

vibez.spill("✅ Simple JSON tests completed")
