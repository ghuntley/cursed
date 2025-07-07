fr fr Test filesystem operations in main function

slay main() {
    vibez.spill("Testing filesystem operations...")
    
    fr fr Test basic file operations
    sus path tea = "test_main.txt"
    sus content tea = "Hello, CURSED filesystem!"
    
    vibez.spill("Creating file: " + path)
    sus result normie = io_write_file(path, content)
    vibez.spill("Write result: " + tea(result))
    
    vibez.spill("Checking if file exists...")
    sus exists normie = io_file_exists(path)
    vibez.spill("File exists result: " + tea(exists))
    
    vibez.spill("Reading file...")
    sus read_content tea = io_read_file(path)
    vibez.spill("Read content: " + read_content)
    
    vibez.spill("Deleting file...")
    sus delete_result normie = io_delete_file(path)
    vibez.spill("Delete result: " + tea(delete_result))
    
    vibez.spill("Filesystem operations test complete!")
}

main()
