yeet "arrayz"
sus numbers []drip = [1, 3, 2, 5, 4]
sus length drip = array_length(numbers)
vibez.spill("Array length:", length)

yeet "jsonz"
sus json_data tea = "{\"name\": \"John\", \"age\": 30}"
sus parsed = parse_json(json_data)
vibez.spill("JSON parsed successfully")

yeet "filez"
sus content tea = "Hello File World!"
write_file("test.txt", content)
ready (file_exists("test.txt")) {
    vibez.spill("File created successfully")
} otherwise {
    vibez.spill("File creation failed")
}
