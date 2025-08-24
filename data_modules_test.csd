# Data Modules Functionality Test
yeet "jsonz"
yeet "xmlz"
yeet "csv_mood"

vibez.spill("=== DATA MODULES TEST ===")

# Test jsonz module
vibez.spill("Testing jsonz.parse...")
sus json_str tea = '{"name": "John", "age": 30, "active": true}'
sus parsed_json dict = jsonz.parse(json_str)
vibez.spill("jsonz.parse completed - checking result...")

vibez.spill("Testing jsonz.stringify...")
sus test_dict dict = {"key": "value", "number": 42}
sus json_string tea = jsonz.stringify(test_dict)
vibez.spill("jsonz.stringify result:", json_string)

vibez.spill("Testing jsonz.get...")
sus json_value tea = jsonz.get(parsed_json, "name")
vibez.spill("jsonz.get('name'):", json_value)

# Test xmlz module
vibez.spill("Testing xmlz.parse...")
sus xml_str tea = "<root><item>value1</item><item>value2</item></root>"
sus parsed_xml dict = xmlz.parse(xml_str)
vibez.spill("xmlz.parse completed")

vibez.spill("Testing xmlz.to_string...")
sus xml_result tea = xmlz.to_string(parsed_xml)
vibez.spill("xmlz.to_string result length:", stringz.len(xml_result))

# Test csv_mood module
vibez.spill("Testing csv_mood.parse...")
sus csv_str tea = "name,age,city\nJohn,30,NYC\nJane,25,SF"
sus parsed_csv [][]tea = csv_mood.parse(csv_str)
vibez.spill("csv_mood.parse completed")

vibez.spill("Testing csv_mood.to_string...")
sus csv_data [][]tea = [["name", "age"], ["Bob", "35"], ["Alice", "28"]]
sus csv_result tea = csv_mood.to_string(csv_data)
vibez.spill("csv_mood.to_string result:", csv_result)

vibez.spill("=== DATA MODULES TEST COMPLETE ===")
