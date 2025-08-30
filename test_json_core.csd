yeet "json"
yeet "vibez"

slay main_character() {
    vibez.spillln("=== JSON Core Test ===")
    
    fr fr Test string parsing
    sus json_str tea = "\"hello\""
    sus result1 tea = json.parse(json_str)
    vibez.spillln(result1)
    
    fr fr Test number parsing  
    sus json_num tea = "42"
    sus result2 tea = json.parse(json_num)
    vibez.spillln(result2)
    
    fr fr Test boolean parsing
    sus json_bool tea = "true"
    sus result3 tea = json.parse(json_bool)
    vibez.spillln(result3)
    
    fr fr Test null parsing
    sus json_null tea = "null"
    sus result4 tea = json.parse(json_null)
    vibez.spillln(result4)
    
    fr fr Test stringify
    sus str_out tea = json.stringify("world")
    vibez.spillln(str_out)
    
    vibez.spillln("=== JSON Test Complete ===")
}
