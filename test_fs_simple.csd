fr fr Simple test of fs module basic functions

import fs

sus main() lit {
    vibez.spill("Testing basic fs operations...")
    
    fr fr Test read_file
    sus content tea = fs.read_file("test.txt")
    vibez.spill("Content: %s", content)
    
    fr fr Test exists
    sus file_exists lit = fs.exists("test.txt")
    vibez.spill("File exists: %s", file_exists)
    
    fr fr Test is_file
    sus is_file_result lit = fs.is_file("test.txt")
    vibez.spill("Is file: %s", is_file_result)
    
    fr fr Test is_dir
    sus is_dir_result lit = fs.is_dir("testdir")
    vibez.spill("Is dir: %s", is_dir_result)
    
    fr fr Test get_size
    sus size drip = fs.get_size("test.txt")
    vibez.spill("File size: %d", size)
    
    vibez.spill("Basic fs test completed!")
    damn based
}
