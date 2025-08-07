yeet "jsonz"

vibez.spill("=== CURSED JSON Module (jsonz) Demo ===")
vibez.spill("")

fr fr 1. Parse JSON string
vibez.spill("1. JSON Parsing:")
sus json_data tea = "{\"name\": \"Alice\", \"age\": 30, \"city\": \"New York\"}"
vibez.spill("Input JSON: " + json_data)
sus parsed tea = parse_json(json_data)
vibez.spill("Parsed result: " + parsed)
vibez.spill("")

fr fr 2. Get value from object
vibez.spill("2. Getting values from JSON object:")
sus name tea = json_get(parsed, "name")
vibez.spill("Name: " + name)
sus age tea = json_get(parsed, "age")
vibez.spill("Age: " + age)
vibez.spill("")

fr fr 3. Set value in object
vibez.spill("3. Setting values in JSON object:")
sus updated tea = json_set(parsed, "age", "31")
vibez.spill("Updated object: " + updated)
sus with_new_key tea = json_set(updated, "country", "\"USA\"")
vibez.spill("With new key: " + with_new_key)
vibez.spill("")

fr fr 4. Array operations
vibez.spill("4. JSON Array operations:")
sus array_data tea = "[\"apple\", \"banana\", \"cherry\"]"
vibez.spill("Original array: " + array_data)
sus array_length normie = json_array_length(array_data)
vibez.spill("Array length: " + array_length)
sus with_item tea = json_array_push(array_data, "\"date\"")
vibez.spill("After push: " + with_item)
sus new_length normie = json_array_length(with_item)
vibez.spill("New length: " + new_length)
vibez.spill("")

fr fr 5. Stringify operations
vibez.spill("5. JSON Stringification:")
sus simple_value tea = "Hello World"
sus stringified tea = stringify_json(simple_value)
vibez.spill("String value: " + simple_value + " -> " + stringified)
sus bool_value tea = "true"
sus bool_json tea = stringify_json(bool_value)
vibez.spill("Boolean value: " + bool_value + " -> " + bool_json)
vibez.spill("")

fr fr 6. Error handling
vibez.spill("6. Error Handling:")
sus malformed tea = "{\"broken\": json"
sus error_result tea = parse_json(malformed)
vibez.spill("Malformed JSON: " + malformed)
vibez.spill("Error result: " + error_result)
vibez.spill("")

fr fr 7. Complex JSON structure
vibez.spill("7. Complex JSON Structure:")
sus complex_json tea = "{\"users\": [{\"id\": 1, \"name\": \"Bob\"}, {\"id\": 2, \"name\": \"Carol\"}], \"count\": 2}"
vibez.spill("Complex JSON: " + complex_json)
sus complex_parsed tea = parse_json(complex_json)
vibez.spill("Parsed successfully: " + complex_parsed)
vibez.spill("")

vibez.spill("=== JSON Module Demo Complete! ===")
