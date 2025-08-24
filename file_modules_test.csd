# File Modules Functionality Test  
yeet "filez"
yeet "dropz"

vibez.spill("=== FILE MODULES TEST ===")

# Test filez module
vibez.spill("Testing filez.write...")
sus test_content tea = "This is test content for file operations"
sus write_result lit = filez.write("test_file.txt", test_content)
vibez.spill("filez.write result:", write_result)

vibez.spill("Testing filez.read...")
sus read_content tea = filez.read("test_file.txt")
vibez.spill("filez.read result:", read_content)

vibez.spill("Testing filez.exists...")
sus file_exists lit = filez.exists("test_file.txt")
vibez.spill("filez.exists('test_file.txt'):", file_exists)

vibez.spill("Testing filez.size...")
sus file_size drip = filez.size("test_file.txt")
vibez.spill("filez.size('test_file.txt'):", file_size)

vibez.spill("Testing filez.delete...")
sus delete_result lit = filez.delete("test_file.txt")
vibez.spill("filez.delete result:", delete_result)

vibez.spill("Testing filez.list_dir...")
sus dir_contents []tea = filez.list_dir(".")
vibez.spill("filez.list_dir('.') found", arrayz.len(dir_contents), "items")

# Test dropz module (file watching/monitoring)
vibez.spill("Testing dropz.watch...")
sus watch_result lit = dropz.watch(".", "*.csd")
vibez.spill("dropz.watch result:", watch_result)

vibez.spill("Testing dropz.get_changes...")
sus changes []tea = dropz.get_changes()
vibez.spill("dropz.get_changes found", arrayz.len(changes), "changes")

vibez.spill("Testing dropz.stop_watch...")
sus stop_result lit = dropz.stop_watch()
vibez.spill("dropz.stop_watch result:", stop_result)

vibez.spill("=== FILE MODULES TEST COMPLETE ===")
