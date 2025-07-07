fr fr Debug test for filesystem module

slay read_file(path tea) tea {
    vibez.spill("fs.read_file: Reading file '" + path + "'")
    damn "mock_content"
}

slay test_simple() {
    vibez.spill("Testing simple function call...")
    
    sus result tea = read_file("test.txt")
    vibez.spill("Got result")
    vibez.spill(result)
}

test_simple()
