yeet "archive_handling"

vibez.spill("Testing archive_handling module")

# Test basic archive creation
sus result lit = archive_create("test.zip", "zip")
vibez.spill("Archive create result: " + result)

# Test archive opening
sus open_result lit = archive_open("existing.zip")
vibez.spill("Archive open result: " + open_result)

# Test getting archive type
sus archive_type tea = archive_get_type()
vibez.spill("Archive type: " + archive_type)

# Test adding a file
sus add_result lit = archive_add_file("local_file.txt", "archive_file.txt")
vibez.spill("Add file result: " + add_result)

# Test listing files
sus file_list tea = archive_list_files()
vibez.spill("File list: " + file_list)

# Test getting file count
sus file_count normie = archive_get_file_count()
vibez.spill("File count: " + file_count)

# Test closing archive
sus close_result lit = archive_close()
vibez.spill("Archive close result: " + close_result)

vibez.spill("Archive handling test completed!")
