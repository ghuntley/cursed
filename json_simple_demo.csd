yeet "json_tea"

vibez.spill("JSON Tea Module Test")

fr fr Test basic marshaling
sus data tea = "hello"
sus result tea = json_tea.Marshal(data)
vibez.spill(result)

fr fr Test number marshaling
sus num tea = "42"
sus num_result tea = json_tea.Marshal(num)
vibez.spill(num_result)

fr fr Test type checking
sus json_str tea = "\"test\""
bestie json_tea.is_string(json_str) {
    vibez.spill("String type detected")
}

sus json_num tea = "123"
bestie json_tea.is_number(json_num) {
    vibez.spill("Number type detected")
}

fr fr Test validation
bestie json_tea.validate_json("{\"key\": \"value\"}") {
    vibez.spill("Valid JSON object")
}

vibez.spill("JSON Tea test complete!")
