yeet "vibez"

vibez.spill("Testing direct runtime file operations...")

fr fr Test file write directly
ready (runtime_write_file("test.txt", "Hello CURSED!")) {
    vibez.spill("File written successfully")
} otherwise {
    vibez.spill("File write failed")
}

fr fr Test file read
sus content tea = runtime_read_file("test.txt")
ready (content == "ERROR") {
    vibez.spill("File read failed - ERROR returned")
} otherwise {
    vibez.spill("File content read successfully")
    vibez.spill(content)
}

fr fr Test file exists
ready (runtime_file_exists("test.txt")) {
    vibez.spill("File exists check passed")
} otherwise {
    vibez.spill("File exists check failed")  
}

vibez.spill("Done!")
