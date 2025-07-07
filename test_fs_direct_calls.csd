fr fr Test filesystem operations using direct runtime calls

slay main() {
    vibez.spill("Testing direct filesystem runtime calls...")
    
    fr fr Test basic file operations
    sus path tea = "test_direct_calls.txt"
    sus content tea = "Hello, CURSED!"
    
    vibez.spill("Creating file: " + path)
    sus result normie = io_write_file(path, content)
    vibez.spill("Write result: " + tea(result))
    
    vibez.spill("Checking if file exists...")
    sus exists normie = io_file_exists(path)
    vibez.spill("File exists result: " + tea(exists))
    
    vibez.spill("Reading file...")
    sus read_content tea = io_read_file(path)
    vibez.spill("Read content: " + read_content)
    
    vibez.spill("Getting file size...")
    sus file_size thicc = io_file_size(path)
    vibez.spill("File size: " + tea(file_size))
    
    vibez.spill("Deleting file...")
    sus delete_result normie = io_delete_file(path)
    vibez.spill("Delete result: " + tea(delete_result))
    
    fr fr Test directory operations
    vibez.spill("Testing directory operations...")
    sus test_dir tea = "test_direct_dir"
    
    vibez.spill("Creating directory: " + test_dir)
    sus create_dir_result normie = io_create_directory(test_dir)
    vibez.spill("Create directory result: " + tea(create_dir_result))
    
    vibez.spill("Listing directory...")
    sus dir_contents tea = io_list_directory(test_dir)
    vibez.spill("Directory contents: " + dir_contents)
    
    vibez.spill("Filesystem operations test complete!")
}

main()
