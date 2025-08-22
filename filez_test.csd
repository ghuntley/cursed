fr fr Test file I/O operations with real files

yeet "vibez"
yeet "filez"

fr fr Test basic file operations
vibez.spill("Testing file I/O operations...")

fr fr Test file writing
sus test_filename tea = "test_output.txt"
sus test_content tea = "Hello, CURSED file system!"

sus write_err tea = filez.write_file(test_filename, test_content)
ready (write_err != "") {
    vibez.spill("Write error:", write_err)
} otherwise {
    vibez.spill("Successfully wrote file:", test_filename)
}

fr fr Test file reading
(content, read_err) := filez.read_file(test_filename)
ready (read_err != "") {
    vibez.spill("Read error:", read_err)
} otherwise {
    vibez.spill("Read content:", content)
    ready (content == test_content) {
        vibez.spill("✅ File content matches!")
    } otherwise {
        vibez.spill("❌ File content mismatch!")
    }
}

fr fr Test file existence
sus exists lit = filez.file_exists(test_filename)
ready (exists) {
    vibez.spill("✅ File exists check passed")
} otherwise {
    vibez.spill("❌ File exists check failed")
}

fr fr Test file size
(size, size_err) := filez.file_size(test_filename)
ready (size_err != "") {
    vibez.spill("Size error:", size_err)
} otherwise {
    vibez.spill("File size:", size, "bytes")
    ready (size > 0) {
        vibez.spill("✅ File size is valid")
    }
}

fr fr Test file append
sus append_content tea = "\nAppended line!"
sus append_err tea = filez.append_file(test_filename, append_content)
ready (append_err != "") {
    vibez.spill("Append error:", append_err)
} otherwise {
    vibez.spill("✅ Successfully appended to file")
}

fr fr Read file again to verify append
(final_content, final_read_err) := filez.read_file(test_filename)
ready (final_read_err != "") {
    vibez.spill("Final read error:", final_read_err)
} otherwise {
    vibez.spill("Final content:", final_content)
}

fr fr Test directory operations
sus test_dir tea = "test_directory"

sus create_dir_err tea = filez.create_directory(test_dir)
ready (create_dir_err != "") {
    vibez.spill("Create directory error:", create_dir_err)
} otherwise {
    vibez.spill("✅ Successfully created directory:", test_dir)
}

sus dir_exists lit = filez.directory_exists(test_dir)
ready (dir_exists) {
    vibez.spill("✅ Directory exists check passed")
} otherwise {
    vibez.spill("❌ Directory exists check failed")
}

fr fr Test file copy
sus copy_filename tea = test_dir + "/copied_file.txt"
sus copy_err tea = filez.copy_file(test_filename, copy_filename)
ready (copy_err != "") {
    vibez.spill("Copy error:", copy_err)
} otherwise {
    vibez.spill("✅ Successfully copied file")
}

fr fr Test directory listing
(entries, list_err) := filez.list_directory(test_dir)
ready (list_err != "") {
    vibez.spill("List directory error:", list_err)
} otherwise {
    vibez.spill("Directory entries:", entries)
}

fr fr Clean up test files
sus delete_err1 tea = filez.delete_file(test_filename)
sus delete_err2 tea = filez.delete_file(copy_filename)
sus remove_dir_err tea = filez.remove_directory(test_dir)

ready (delete_err1 == "" && delete_err2 == "" && remove_dir_err == "") {
    vibez.spill("✅ Cleanup successful")
} otherwise {
    vibez.spill("⚠️ Cleanup had some errors")
}

vibez.spill("File I/O testing complete!")
