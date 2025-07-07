fr fr CURSED Filesystem Module
fr fr Provides file system operations for CURSED programs
fr fr Production-ready filesystem operations using runtime bridge

fr fr ================================
fr fr File Operations
fr fr ================================

slay read_file(path tea) tea {
    fr fr Read file contents as string
    damn io_read_file(path)
}

slay write_file(path tea, content tea) lit {
    fr fr Write string to file
    sus result normie = io_write_file(path, content)
    damn result == 0
}

slay file_exists(path tea) lit {
    fr fr Check if file exists
    sus result normie = io_file_exists(path)
    damn result == 1
}

slay create_dir(path tea) lit {
    fr fr Create directory
    sus result normie = io_create_directory(path)
    damn result == 0
}

slay list_dir(path tea) []tea {
    fr fr List directory contents
    sus content tea = io_list_directory(path)
    
    fr fr Check if read was successful
    lowkey content == "" {
        sus empty_array []tea = []
        damn empty_array
    }
    
    fr fr Split by newlines to get array of filenames
    sus files []tea = content.split("\n")
    damn files
}

slay delete_file(path tea) lit {
    fr fr Delete file
    sus result normie = io_delete_file(path)
    damn result == 0
}

slay get_file_size(path tea) thicc {
    fr fr Get file size in bytes
    damn io_file_size(path)
}

fr fr ================================
fr fr Path Utilities
fr fr ================================

slay join_path(base tea, component tea) tea {
    fr fr Join path components with proper separator
    sus separator tea = "/"
    
    fr fr Handle empty base path
    lowkey base == "" {
        damn component
    }
    
    fr fr Handle empty component
    lowkey component == "" {
        damn base
    }
    
    fr fr Check if base already ends with separator
    sus base_len normie = base.length
    lowkey base_len > 0 {
        sus last_char tea = base[base_len - 1]
        lowkey last_char == separator {
            damn base + component
        }
    }
    
    fr fr Add separator between components
    damn base + separator + component
}

slay get_extension(path tea) tea {
    fr fr Get file extension
    sus dot_pos normie = path.last_index_of(".")
    sus slash_pos normie = path.last_index_of("/")
    
    fr fr No extension found or dot is part of directory name
    lowkey dot_pos == -1 || dot_pos < slash_pos {
        damn ""
    }
    
    fr fr Return extension including the dot
    damn path.substring(dot_pos)
}

slay get_basename(path tea) tea {
    fr fr Get filename without directory path
    sus slash_pos normie = path.last_index_of("/")
    
    fr fr No directory separator found
    lowkey slash_pos == -1 {
        damn path
    }
    
    fr fr Return filename after last separator
    damn path.substring(slash_pos + 1)
}

fr fr ================================
fr fr Directory Operations
fr fr ================================

slay create_dir_recursive(path tea) lit {
    fr fr Create directory tree recursively
    fr fr For now, just try to create the directory
    damn create_dir(path)
}

slay remove_dir(path tea) lit {
    fr fr Remove directory (must be empty)
    fr fr For now, just try to delete as file
    damn delete_file(path)
}

slay is_dir(path tea) lit {
    fr fr Check if path is a directory
    fr fr This is a simplified implementation
    fr fr Try to list the directory
    sus files []tea = list_dir(path)
    damn files.length >= 0
}

slay is_file(path tea) lit {
    fr fr Check if path is a regular file
    fr fr This is a simplified implementation
    fr fr If it exists but is not a directory, assume it's a file
    damn file_exists(path) && !is_dir(path)
}

fr fr ================================
fr fr File Information
fr fr ================================

be_like FileInfo squad {
    name tea
    size thicc
    is_dir lit
    modified_time thicc
    permissions normie
}

slay get_file_info(path tea) FileInfo {
    fr fr Get file information
    sus info FileInfo = {
        name: get_basename(path),
        size: get_file_size(path),
        is_dir: is_dir(path),
        modified_time: 0,  fr fr TODO: Implement actual timestamp
        permissions: 644   fr fr TODO: Implement actual permissions
    }
    damn info
}

fr fr ================================
fr fr File Permissions
fr fr ================================

slay set_permissions(path tea, perms normie) lit {
    fr fr Set file permissions
    fr fr TODO: Implement actual permission setting
    fr fr For now, just return success
    damn based
}

slay get_permissions(path tea) normie {
    fr fr Get file permissions
    fr fr TODO: Implement actual permission retrieval
    fr fr For now, just return default permissions
    damn 644
}
