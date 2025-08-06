yeet "json_tea"

vibez.spill("JSON Value Access Demo")

fr fr Test value access
sus obj tea = "{\"name\": \"John\", \"age\": 30}"
sus name tea = json_tea.get_value(obj, "name")
sus age tea = json_tea.get_value(obj, "age")

vibez.spill("Object: " + obj)
vibez.spill("Name: " + name)
vibez.spill("Age: " + age)

fr fr Test type-specific access
sus string_val tea = json_tea.get_string(obj, "name")
sus number_val tea = json_tea.get_number(obj, "age")

vibez.spill("String value: " + string_val)
vibez.spill("Number value: " + number_val)

fr fr Test manipulation
sus new_obj tea = json_tea.set_value(obj, "name", "Jane")
vibez.spill("Modified object: " + new_obj)

fr fr Test array manipulation
sus arr tea = "[]"
sus new_arr tea = json_tea.add_to_array(arr, "item")
vibez.spill("Array with item: " + new_arr)

vibez.spill("Access demo complete!")
