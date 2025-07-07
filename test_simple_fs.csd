fr fr Simple test of direct runtime calls

vibez.spill("Testing direct runtime calls...")

fr fr Test basic file operations
sus path tea = "test_simple_direct.txt"
sus content tea = "Hello, world!"

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

vibez.spill("Test complete!")
