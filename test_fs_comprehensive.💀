fr fr Comprehensive test of the CURSED fs module
fr fr Tests all core filesystem operations

import fs

sus main() lit {
    vibez.spill("CURSED FS Module - Comprehensive Test")
    vibez.spill("=====================================")
    
    fr fr Test file reading
    vibez.spill("Testing file reading...")
    sus content1 tea = fs.read_file("test.txt")
    vibez.spill("read_file('test.txt'): '%s'", content1)
    
    sus content2 tea = fs.read_file("data.txt") 
    vibez.spill("read_file('data.txt'): '%s'", content2)
    
    sus content3 tea = fs.read_file("nonexistent.txt")
    vibez.spill("read_file('nonexistent.txt'): '%s'", content3)
    
    fr fr Test file existence
    vibez.spill("\nTesting file existence...")
    sus exists1 lit = fs.exists("test.txt")
    vibez.spill("exists('test.txt'): %s", exists1)
    
    sus exists2 lit = fs.exists("nonexistent.txt")
    vibez.spill("exists('nonexistent.txt'): %s", exists2)
    
    sus exists3 lit = fs.exists("testdir")
    vibez.spill("exists('testdir'): %s", exists3)
    
    fr fr Test file writing
    vibez.spill("\nTesting file writing...")
    sus write_result1 lit = fs.write_file("new_file.txt", "This is new content")
    vibez.spill("write_file('new_file.txt', content): %s", write_result1)
    
    sus new_content tea = fs.read_file("new_file.txt")
    vibez.spill("Reading back new file: '%s'", new_content)
    
    fr fr Test directory operations
    vibez.spill("\nTesting directory operations...")
    sus create_dir_result lit = fs.create_dir("newdir")
    vibez.spill("create_dir('newdir'): %s", create_dir_result)
    
    sus is_dir1 lit = fs.is_dir("newdir")
    vibez.spill("is_dir('newdir'): %s", is_dir1)
    
    sus is_dir2 lit = fs.is_dir("test.txt")
    vibez.spill("is_dir('test.txt'): %s", is_dir2)
    
    fr fr Test directory listing
    vibez.spill("\nTesting directory listing...")
    sus entries1 [tea] = fs.list_dir("testdir")
    vibez.spill("list_dir('testdir') has %d entries:", len(entries1))
    bestie i := 0; i < len(entries1); i++ {
        vibez.spill("  - %s", entries1[i])
    }
    
    sus entries2 [tea] = fs.list_dir("empty_dir")
    vibez.spill("list_dir('empty_dir') has %d entries", len(entries2))
    
    fr fr Test file type checks
    vibez.spill("\nTesting file type checks...")
    sus is_file1 lit = fs.is_file("test.txt")
    vibez.spill("is_file('test.txt'): %s", is_file1)
    
    sus is_file2 lit = fs.is_file("testdir")
    vibez.spill("is_file('testdir'): %s", is_file2)
    
    fr fr Test file size
    vibez.spill("\nTesting file size...")
    sus size1 drip = fs.get_size("test.txt")
    vibez.spill("get_size('test.txt'): %d", size1)
    
    sus size2 drip = fs.get_size("large.txt")
    vibez.spill("get_size('large.txt'): %d", size2)
    
    sus size3 drip = fs.get_size("empty.txt")
    vibez.spill("get_size('empty.txt'): %d", size3)
    
    fr fr Test file operations
    vibez.spill("\nTesting file operations...")
    sus copy_result lit = fs.copy_file("test.txt", "test_copy.txt")
    vibez.spill("copy_file('test.txt', 'test_copy.txt'): %s", copy_result)
    
    sus copied_content tea = fs.read_file("test_copy.txt")
    vibez.spill("Reading copied file: '%s'", copied_content)
    
    sus move_result lit = fs.move_file("test_copy.txt", "test_moved.txt")
    vibez.spill("move_file('test_copy.txt', 'test_moved.txt'): %s", move_result)
    
    sus moved_exists lit = fs.exists("test_moved.txt")
    vibez.spill("exists('test_moved.txt'): %s", moved_exists)
    
    sus old_exists lit = fs.exists("test_copy.txt")
    vibez.spill("exists('test_copy.txt') after move: %s", old_exists)
    
    fr fr Test file deletion
    vibez.spill("\nTesting file deletion...")
    sus delete_result lit = fs.delete_file("test_moved.txt")
    vibez.spill("delete_file('test_moved.txt'): %s", delete_result)
    
    sus deleted_exists lit = fs.exists("test_moved.txt")
    vibez.spill("exists('test_moved.txt') after delete: %s", deleted_exists)
    
    fr fr Test permissions (simplified)
    vibez.spill("\nTesting permissions...")
    sus readable lit = fs.is_readable("test.txt")
    vibez.spill("is_readable('test.txt'): %s", readable)
    
    sus writable lit = fs.is_writable("test.txt")
    vibez.spill("is_writable('test.txt'): %s", writable)
    
    sus executable lit = fs.is_executable("script.sh")
    vibez.spill("is_executable('script.sh'): %s", executable)
    
    fr fr Test metadata
    vibez.spill("\nTesting metadata...")
    sus mod_time drip = fs.get_modified_time("test.txt")
    vibez.spill("get_modified_time('test.txt'): %d", mod_time)
    
    sus create_time drip = fs.get_created_time("test.txt")
    vibez.spill("get_created_time('test.txt'): %d", create_time)
    
    fr fr Test utility functions
    vibez.spill("\nTesting utility functions...")
    sus basename tea = fs.get_basename("dir/file.txt")
    vibez.spill("get_basename('dir/file.txt'): '%s'", basename)
    
    sus parent tea = fs.get_parent_dir("dir/file.txt")
    vibez.spill("get_parent_dir('dir/file.txt'): '%s'", parent)
    
    sus joined tea = fs.join_path("dir", "file.txt")
    vibez.spill("join_path('dir', 'file.txt'): '%s'", joined)
    
    fr fr Test empty checks
    vibez.spill("\nTesting empty checks...")
    sus is_empty_file lit = fs.is_empty_file("empty.txt")
    vibez.spill("is_empty_file('empty.txt'): %s", is_empty_file)
    
    sus is_empty_dir lit = fs.is_empty_dir("empty_dir")
    vibez.spill("is_empty_dir('empty_dir'): %s", is_empty_dir)
    
    fr fr Test file extensions
    vibez.spill("\nTesting file extensions...")
    sus ext1 tea = fs.file_extension("test.txt")
    vibez.spill("file_extension('test.txt'): '%s'", ext1)
    
    sus ext2 tea = fs.file_extension("config.json")
    vibez.spill("file_extension('config.json'): '%s'", ext2)
    
    fr fr Test hidden files
    vibez.spill("\nTesting hidden files...")
    sus hidden1 lit = fs.is_hidden(".hidden")
    vibez.spill("is_hidden('.hidden'): %s", hidden1)
    
    sus hidden2 lit = fs.is_hidden("test.txt")
    vibez.spill("is_hidden('test.txt'): %s", hidden2)
    
    vibez.spill("\n=====================================")
    vibez.spill("CURSED FS Module test completed!")
    
    damn based
}
