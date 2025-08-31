yeet "json"
yeet "vibez"

slay main_character() {
    vibez.spill("Testing JSON module with correct API...")
    
    fr fr Test basic string parsing
    sus json_str tea = "\"hello\""
    sus result tea = parse_json(json_str)
    vibez.spill("JSON string result: " + result)
    
    fr fr Test number parsing
    sus json_num tea = "42"
    sus num_result tea = parse_json(json_num)
    vibez.spill("JSON number result: " + num_result)
    
    fr fr Test object parsing
    sus json_obj tea = "{\"key\":\"value\"}"
    sus obj_result tea = parse_json(json_obj)
    vibez.spill("JSON object result: " + obj_result)
    
    fr fr Test validation
    sus valid_json tea = "{\"test\":true}"
    sus is_valid lit = is_valid_json(valid_json)
    vibez.spill("JSON is valid: " + lit_to_string(is_valid))
    
    vibez.spill("JSON tests completed")
}
