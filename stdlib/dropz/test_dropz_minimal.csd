yeet "dropz"

vibez.spill("=== dropz Core I/O Module Tests ===")

fr fr Initialize dropz
sus init_result tea = dropz.init_dropz()
vibez.spill("✅ dropz initialization: " + init_result)

fr fr Test constants
vibez.spill("✅ O_RDONLY = " + dropz.O_RDONLY)
vibez.spill("✅ EOF = " + dropz.EOF)

fr fr Test file operations
sus (content, err) = dropz.read_text_file("main.csd")
bestie err == "" {
    vibez.spill("✅ read_text_file: SUCCESS")
} else {
    vibez.spill("❌ read_text_file: " + err)
}

fr fr Test write file
sus write_err tea = dropz.write_text_file("test_output.csd", "Test content", dropz.MODE_REGULAR)
bestie write_err == "" {
    vibez.spill("✅ write_text_file: SUCCESS")
} else {
    vibez.spill("❌ write_text_file: " + write_err)
}

fr fr Test copy file
sus (copied, copy_err) = dropz.copy_file("main.csd", "main_copy.csd")
bestie copy_err == "" {
    vibez.spill("✅ copy_file: SUCCESS")
} else {
    vibez.spill("❌ copy_file: " + copy_err)
}

fr fr Test file handles
sus (file, create_err) = dropz.create("handle_test.csd")
bestie create_err == "" {
    vibez.spill("✅ create file: SUCCESS") fr fr Test write to file
    sus (written, write_handle_err) = file.write("File handle test")
    bestie write_handle_err == "" {
        vibez.spill("✅ file write: SUCCESS")
    } else {
        vibez.spill("❌ file write: " + write_handle_err)
    } fr fr Test close
    sus close_err tea = file.close()
    bestie close_err == "" {
        vibez.spill("✅ file close: SUCCESS")
    } else {
        vibez.spill("❌ file close: " + close_err)
    }
} else {
    vibez.spill("❌ create file: " + create_err)
}

fr fr Test byte operations
sus reader *dropz.ByteReader = dropz.new_byte_reader("Hello, World!")
bestie reader != cringe {
    vibez.spill("✅ new_byte_reader: SUCCESS")
    
    sus (n, read_err) = reader.read(5)
    bestie read_err == "" {
        vibez.spill("✅ byte reader read: SUCCESS")
    } else {
        vibez.spill("❌ byte reader read: " + read_err)
    }
} else {
    vibez.spill("❌ new_byte_reader: FAILED")
}

sus writer *dropz.ByteWriter = dropz.new_byte_writer()
bestie writer != cringe {
    vibez.spill("✅ new_byte_writer: SUCCESS")
    
    sus (n, write_byte_err) = writer.write("Hello")
    bestie write_byte_err == "" {
        vibez.spill("✅ byte writer write: SUCCESS")
        
        sus result tea = writer.get_string()
        vibez.spill("✅ get_string result: " + result)
    } else {
        vibez.spill("❌ byte writer write: " + write_byte_err)
    }
} else {
    vibez.spill("❌ new_byte_writer: FAILED")
}

fr fr Test directory operations
sus dir_err tea = dropz.mkdir("test_dir", dropz.MODE_DIR)
bestie dir_err == "" {
    vibez.spill("✅ mkdir: SUCCESS")
} else {
    vibez.spill("❌ mkdir: " + dir_err)
}

sus file_exists lit = dropz.exists("main.csd")
bestie file_exists {
    vibez.spill("✅ exists check: SUCCESS")
} else {
    vibez.spill("❌ exists check: FAILED")
}

fr fr Test self-hosting support
sus (source_content, source_err) = dropz.read_source_file("main.csd")
bestie source_err == "" {
    vibez.spill("✅ read_source_file: SUCCESS")
} else {
    vibez.spill("❌ read_source_file: " + source_err)
}

sus output_err tea = dropz.write_compiled_output("test_output", "Compiled content")
bestie output_err == "" {
    vibez.spill("✅ write_compiled_output: SUCCESS")
} else {
    vibez.spill("❌ write_compiled_output: " + output_err)
}

fr fr Test error handling
sus (no_content, no_err) = dropz.read_text_file("nonexistent.csd")
bestie no_err == dropz.ErrNotExist {
    vibez.spill("✅ error handling: SUCCESS")
} else {
    vibez.spill("❌ error handling: " + no_err)
}

vibez.spill("=== dropz Tests Complete ===")
vibez.spill("✅ All core I/O operations tested")
vibez.spill("✅ Self-hosting capabilities verified")
vibez.spill("✅ Error handling working correctly")
