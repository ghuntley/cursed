yeet "json"
yeet "vibez"

slay main_character() {
    vibez.vibez.spillln("=== CURSED JSON Module Test ===")
    
    fr fr Test JSON parsing
    vibez.vibez.spillln("Testing JSON parsing...")
    
    sus simple_string tea = "\"hello\""
    sus parsed_string tea = json.parse(simple_string)
    vibez.vibez.spillln("Parsed string: " + parsed_string)
    
    sus simple_number tea = "42"
    sus parsed_number tea = json.parse(simple_number)
    vibez.spillln("Parsed number: " + parsed_number)
    
    sus simple_bool tea = "true"
    sus parsed_bool tea = json.parse(simple_bool)
    vibez.spillln("Parsed boolean: " + parsed_bool)
    
    sus simple_null tea = "null"
    sus parsed_null tea = json.parse(simple_null)
    vibez.spillln("Parsed null: " + parsed_null)
    
    fr fr Test JSON stringification
    vibez.spillln("\nTesting JSON stringification...")
    
    sus str_result tea = json.stringify("hello world")
    vibez.spillln("Stringified string: " + str_result)
    
    sus int_result tea = json.stringify(123)
    vibez.spillln("Stringified integer: " + int_result)
    
    sus bool_result tea = json.stringify(based)
    vibez.spillln("Stringified boolean: " + bool_result)
    
    fr fr Test JSON validation
    vibez.spillln("\nTesting JSON validation...")
    
    sus valid_json tea = "{\"name\":\"test\"}"
    sus is_valid lit = json.validate(valid_json)
    vibez.spillln("Valid JSON check: " + is_valid)
    
    sus invalid_json tea = "{name:test}"
    sus is_invalid lit = json.validate(invalid_json)
    vibez.spillln("Invalid JSON check: " + is_invalid)
    
    fr fr Test object and array parsing
    vibez.spillln("\nTesting object and array parsing...")
    
    sus json_object tea = "{\"name\":\"John\",\"age\":30}"
    sus parsed_object tea = json.parse_object(json_object)
    vibez.spillln("Parsed object: " + parsed_object)
    
    sus json_array tea = "[1,2,3,4,5]"
    sus parsed_array tea = json.parse_array(json_array)
    vibez.spillln("Parsed array: " + parsed_array)
    
    vibez.spillln("=== JSON Module Test Complete ===")
}
