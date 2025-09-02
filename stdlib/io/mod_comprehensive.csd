fr fr CURSED I/O Module - Comprehensive Implementation
fr fr Complete I/O operations with proper error handling and buffering
fr fr Pure CURSED implementation with robust file and stream operations

yeet "stringz"
yeet "vibez"

fr fr ================================
fr fr Core Constants and Flags
fr fr ================================

fr fr File opening flags (POSIX compatible)
fact O_RDONLY normie = 0
fact O_WRONLY normie = 1
fact O_RDWR normie = 2
fact O_APPEND normie = 1024
fact O_CREATE normie = 64
fact O_EXCL normie = 128
fact O_SYNC normie = 1052672
fact O_TRUNC normie = 512
fact O_NONBLOCK normie = 2048
fact O_DIRECTORY normie = 65536
fact O_NOFOLLOW normie = 131072

fr fr File permissions (octal)
fact MODE_REGULAR normie = 0644    fr fr rw-r--r--
fact MODE_EXECUTABLE normie = 0755  fr fr rwxr-xr-x
fact MODE_PRIVATE normie = 0600     fr fr rw-------
fact MODE_DIR normie = 0755         fr fr rwxr-xr-x

fr fr Seek origins
fact SEEK_START normie = 0
fact SEEK_CURRENT normie = 1
fact SEEK_END normie = 2

fr fr Buffer sizes
fact DEFAULT_BUFFER_SIZE thicc = 4096
fact LARGE_BUFFER_SIZE thicc = 65536
fact SMALL_BUFFER_SIZE thicc = 512

fr fr Error constants
fact EOF tea = "EOF"
fact ErrInvalid tea = "invalid argument"
fact ErrPermission tea = "permission denied"
fact ErrExist tea = "file already exists"
fact ErrNotExist tea = "file does not exist"
fact ErrClosed tea = "file already closed"
fact ErrTimeout tea = "operation timed out"
fact ErrInterrupted tea = "operation interrupted"
fact ErrTooManyFiles tea = "too many open files"
fact ErrDiskFull tea = "disk full"
fact ErrIOError tea = "input/output error"

fr fr ================================
fr fr Core Data Structures
fr fr ================================

be_like FileDescriptor squad {
    fd normie
    path tea
    flags normie
    mode normie
    is_open lit
    position thicc
    size thicc
    last_error tea
}

be_like BufferedReader squad {
    fd *FileDescriptor
    buffer byte[value]
    buffer_size thicc
    buffer_pos thicc
    buffer_len thicc
    eof_reached lit
}

be_like BufferedWriter squad {
    fd *FileDescriptor
    buffer byte[value]
    buffer_size thicc
    buffer_pos thicc
    auto_flush lit
}

be_like FileInfo squad {
    name tea
    path tea
    size thicc
    mode normie
    mod_time thicc
    access_time thicc
    create_time thicc
    is_dir lit
    is_file lit
    is_symlink lit
    is_readable lit
    is_writable lit
    is_executable lit
}

be_like IOError squad {
    operation tea
    path tea
    message tea
    error_code normie
    inner_error tea
    is_temporary lit
    is_timeout lit
    is_permission lit
}

be_like StreamPosition squad {
    absolute_pos thicc
    relative_pos thicc
    line_number thicc
    column_number thicc
}

fr fr ================================
fr fr Low-Level File Operations
fr fr ================================

slay open_file(path tea, flags normie, mode normie) (*FileDescriptor, IOError) { fr fr Open file with comprehensive error handling
    lowkey path == "" {
        sus error IOError = create_io_error("open", path, ErrInvalid, -1)
        damn null, error
    }
    
    fr fr Validate path length
    lowkey string_length(path) > 4096 {
        sus error IOError = create_io_error("open", path, "path too long", -1)
        damn null, error
    }
    
    fr fr Check for invalid characters in path
    lowkey contains_invalid_path_chars(path) {
        sus error IOError = create_io_error("open", path, ErrInvalid, -1)
        damn null, error
    }
    
    fr fr Perform actual system call
    sus fd normie = syscall_open(path, flags, mode)
    
    lowkey fd < 0 {
        sus error_msg tea = get_system_error_message(fd)
        sus error IOError = create_io_error("open", path, error_msg, fd)
        damn null, error
    }
    
    fr fr Get file information
    sus file_size thicc = get_file_size_by_fd(fd)
    
    sus file_desc FileDescriptor = {
        fd: fd,
        path: path,
        flags: flags,
        mode: mode,
        is_open: true,
        position: 0,
        size: file_size,
        last_error: ""
    }
    
    sus empty_error IOError
    damn &file_desc, empty_error
}

slay close_file(file_desc *FileDescriptor) IOError { fr fr Close file descriptor
    lowkey file_desc == null || !file_desc.is_open {
        damn create_io_error("close", file_desc.path, ErrClosed, -1)
    }
    
    sus result normie = syscall_close(file_desc.fd)
    
    lowkey result != 0 {
        sus error_msg tea = get_system_error_message(result)
        damn create_io_error("close", file_desc.path, error_msg, result)
    }
    
    file_desc.is_open = false
    file_desc.fd = -1
    
    sus empty_error IOError
    damn empty_error
}

slay read_from_fd(file_desc *FileDescriptor, buffer byte[value], count thicc) (thicc, IOError) { fr fr Read from file descriptor
    lowkey file_desc == null || !file_desc.is_open {
        sus error IOError = create_io_error("read", file_desc.path, ErrClosed, -1)
        damn 0, error
    }
    
    lowkey count == 0 {
        sus empty_error IOError
        damn 0, empty_error
    }
    
    lowkey len(buffer) < count {
        sus error IOError = create_io_error("read", file_desc.path, ErrInvalid, -1)
        damn 0, error
    }
    
    sus bytes_read thicc = syscall_read(file_desc.fd, buffer, count)
    
    lowkey bytes_read < 0 {
        sus error_msg tea = get_system_error_message(normie(bytes_read))
        sus error IOError = create_io_error("read", file_desc.path, error_msg, normie(bytes_read))
        damn 0, error
    }
    
    lowkey bytes_read == 0 {
        fr fr End of file reached
        file_desc.last_error = EOF
    } otherwise {
        file_desc.position += bytes_read
        file_desc.last_error = ""
    }
    
    sus empty_error IOError
    damn bytes_read, empty_error
}

slay write_to_fd(file_desc *FileDescriptor, buffer byte[value], count thicc) (thicc, IOError) { fr fr Write to file descriptor
    lowkey file_desc == null || !file_desc.is_open {
        sus error IOError = create_io_error("write", file_desc.path, ErrClosed, -1)
        damn 0, error
    }
    
    lowkey (file_desc.flags & O_WRONLY) == 0 && (file_desc.flags & O_RDWR) == 0 {
        sus error IOError = create_io_error("write", file_desc.path, ErrPermission, -1)
        damn 0, error
    }
    
    lowkey count == 0 {
        sus empty_error IOError
        damn 0, empty_error
    }
    
    lowkey len(buffer) < count {
        sus error IOError = create_io_error("write", file_desc.path, ErrInvalid, -1)
        damn 0, error
    }
    
    sus bytes_written thicc = syscall_write(file_desc.fd, buffer, count)
    
    lowkey bytes_written < 0 {
        sus error_msg tea = get_system_error_message(normie(bytes_written))
        sus error IOError = create_io_error("write", file_desc.path, error_msg, normie(bytes_written))
        damn 0, error
    }
    
    file_desc.position += bytes_written
    lowkey file_desc.position > file_desc.size {
        file_desc.size = file_desc.position
    }
    file_desc.last_error = ""
    
    sus empty_error IOError
    damn bytes_written, empty_error
}

slay seek_in_file(file_desc *FileDescriptor, offset thicc, whence normie) (thicc, IOError) { fr fr Seek to position in file
    lowkey file_desc == null || !file_desc.is_open {
        sus error IOError = create_io_error("seek", file_desc.path, ErrClosed, -1)
        damn 0, error
    }
    
    sus new_position thicc = syscall_seek(file_desc.fd, offset, whence)
    
    lowkey new_position < 0 {
        sus error_msg tea = get_system_error_message(normie(new_position))
        sus error IOError = create_io_error("seek", file_desc.path, error_msg, normie(new_position))
        damn 0, error
    }
    
    file_desc.position = new_position
    file_desc.last_error = ""
    
    sus empty_error IOError
    damn new_position, empty_error
}

slay flush_file(file_desc *FileDescriptor) IOError { fr fr Flush file buffers to disk
    lowkey file_desc == null || !file_desc.is_open {
        damn create_io_error("flush", file_desc.path, ErrClosed, -1)
    }
    
    sus result normie = syscall_fsync(file_desc.fd)
    
    lowkey result != 0 {
        sus error_msg tea = get_system_error_message(result)
        damn create_io_error("flush", file_desc.path, error_msg, result)
    }
    
    file_desc.last_error = ""
    sus empty_error IOError
    damn empty_error
}

fr fr ================================
fr fr Buffered I/O Operations
fr fr ================================

slay create_buffered_reader(file_desc *FileDescriptor, buffer_size thicc) *BufferedReader { fr fr Create buffered reader
    lowkey file_desc == null {
        damn null
    }
    
    lowkey buffer_size <= 0 {
        buffer_size = DEFAULT_BUFFER_SIZE
    }
    
    sus buffer byte[value] = allocate_byte_array(buffer_size)
    
    sus reader BufferedReader = {
        fd: file_desc,
        buffer: buffer,
        buffer_size: buffer_size,
        buffer_pos: 0,
        buffer_len: 0,
        eof_reached: false
    }
    
    damn &reader
}

slay read_buffered(reader *BufferedReader, output byte[value], count thicc) (thicc, IOError) { fr fr Read with buffering
    lowkey reader == null || reader.fd == null {
        sus error IOError = create_io_error("read", "", ErrInvalid, -1)
        damn 0, error
    }
    
    lowkey count == 0 {
        sus empty_error IOError
        damn 0, empty_error
    }
    
    sus total_read thicc = 0
    sus remaining thicc = count
    
    bestie remaining > 0 && !reader.eof_reached {
        fr fr Check if buffer needs refilling
        lowkey reader.buffer_pos >= reader.buffer_len {
            sus refill_error IOError = refill_buffer(reader)
            lowkey refill_error.error_code != 0 {
                damn total_read, refill_error
            }
            
            lowkey reader.buffer_len == 0 {
                reader.eof_reached = true
                break
            }
        }
        
        fr fr Copy from buffer to output
        sus available thicc = reader.buffer_len - reader.buffer_pos
        sus to_copy thicc = min_thicc(remaining, available)
        
        copy_bytes(output, total_read, reader.buffer, reader.buffer_pos, to_copy)
        
        reader.buffer_pos += to_copy
        total_read += to_copy
        remaining -= to_copy
    }
    
    sus empty_error IOError
    damn total_read, empty_error
}

slay refill_buffer(reader *BufferedReader) IOError { fr fr Refill internal buffer
    lowkey reader == null {
        damn create_io_error("refill", "", ErrInvalid, -1)
    }
    
    sus bytes_read, error := read_from_fd(reader.fd, reader.buffer, reader.buffer_size)
    
    lowkey error.error_code != 0 {
        damn error
    }
    
    reader.buffer_pos = 0
    reader.buffer_len = bytes_read
    
    sus empty_error IOError
    damn empty_error
}

slay create_buffered_writer(file_desc *FileDescriptor, buffer_size thicc, auto_flush lit) *BufferedWriter { fr fr Create buffered writer
    lowkey file_desc == null {
        damn null
    }
    
    lowkey buffer_size <= 0 {
        buffer_size = DEFAULT_BUFFER_SIZE
    }
    
    sus buffer byte[value] = allocate_byte_array(buffer_size)
    
    sus writer BufferedWriter = {
        fd: file_desc,
        buffer: buffer,
        buffer_size: buffer_size,
        buffer_pos: 0,
        auto_flush: auto_flush
    }
    
    damn &writer
}

slay write_buffered(writer *BufferedWriter, input byte[value], count thicc) (thicc, IOError) { fr fr Write with buffering
    lowkey writer == null || writer.fd == null {
        sus error IOError = create_io_error("write", "", ErrInvalid, -1)
        damn 0, error
    }
    
    lowkey count == 0 {
        sus empty_error IOError
        damn 0, empty_error
    }
    
    sus total_written thicc = 0
    sus remaining thicc = count
    sus input_pos thicc = 0
    
    bestie remaining > 0 {
        fr fr Check if buffer needs flushing
        lowkey writer.buffer_pos >= writer.buffer_size {
            sus flush_error IOError = flush_buffered_writer(writer)
            lowkey flush_error.error_code != 0 {
                damn total_written, flush_error
            }
        }
        
        fr fr Copy to buffer
        sus available thicc = writer.buffer_size - writer.buffer_pos
        sus to_copy thicc = min_thicc(remaining, available)
        
        copy_bytes(writer.buffer, writer.buffer_pos, input, input_pos, to_copy)
        
        writer.buffer_pos += to_copy
        total_written += to_copy
        remaining -= to_copy
        input_pos += to_copy
        
        fr fr Auto-flush if requested
        lowkey writer.auto_flush && writer.buffer_pos >= writer.buffer_size {
            sus flush_error IOError = flush_buffered_writer(writer)
            lowkey flush_error.error_code != 0 {
                damn total_written, flush_error
            }
        }
    }
    
    sus empty_error IOError
    damn total_written, empty_error
}

slay flush_buffered_writer(writer *BufferedWriter) IOError { fr fr Flush buffered writer
    lowkey writer == null || writer.buffer_pos == 0 {
        sus empty_error IOError
        damn empty_error
    }
    
    sus bytes_written, error := write_to_fd(writer.fd, writer.buffer, writer.buffer_pos)
    
    lowkey error.error_code != 0 {
        damn error
    }
    
    lowkey bytes_written != writer.buffer_pos {
        sus partial_error IOError = create_io_error("flush", writer.fd.path, "partial write", -1)
        damn partial_error
    }
    
    writer.buffer_pos = 0
    
    sus empty_error IOError
    damn empty_error
}

fr fr ================================
fr fr High-Level File Operations
fr fr ================================

slay read_entire_file(path tea) (tea, IOError) { fr fr Read entire file as string
    sus bytes_data, error := read_entire_file_bytes(path)
    
    lowkey error.error_code != 0 {
        damn "", error
    }
    
    sus content tea = bytes_to_utf8_string(bytes_data)
    
    sus empty_error IOError
    damn content, empty_error
}

slay read_entire_file_bytes(path tea) (byte[value], IOError) { fr fr Read entire file as bytes
    sus file_desc, open_error := open_file(path, O_RDONLY, 0)
    
    lowkey open_error.error_code != 0 {
        sus empty_data byte[value]
        damn empty_data, open_error
    }
    
    defer close_file(file_desc)
    
    fr fr Create buffer based on file size
    sus buffer_size thicc = file_desc.size
    lowkey buffer_size <= 0 {
        buffer_size = DEFAULT_BUFFER_SIZE
    } otherwise lowkey buffer_size > 100000000 {  fr fr 100MB limit
        buffer_size = 100000000
    }
    
    sus buffer byte[value] = allocate_byte_array(buffer_size)
    sus total_read thicc = 0
    
    bestie {
        sus bytes_read, read_error := read_from_fd(file_desc, 
            slice_byte_array(buffer, total_read, buffer_size), 
            buffer_size - total_read)
        
        lowkey read_error.error_code != 0 && read_error.message != EOF {
            sus empty_data byte[value]
            damn empty_data, read_error
        }
        
        total_read += bytes_read
        
        lowkey bytes_read == 0 || read_error.message == EOF {
            break  fr fr End of file
        }
        
        fr fr Expand buffer if needed
        lowkey total_read >= buffer_size {
            sus new_size thicc = buffer_size * 2
            lowkey new_size > 100000000 {
                new_size = 100000000
            }
            buffer = expand_byte_array(buffer, new_size)
            buffer_size = new_size
        }
    }
    
    fr fr Return actual data read
    sus result byte[value] = slice_byte_array(buffer, 0, total_read)
    
    sus empty_error IOError
    damn result, empty_error
}

slay write_entire_file(path tea, content tea) IOError { fr fr Write string to file
    sus bytes_data byte[value] = utf8_string_to_bytes(content)
    damn write_entire_file_bytes(path, bytes_data)
}

slay write_entire_file_bytes(path tea, data byte[value]) IOError { fr fr Write bytes to file
    sus file_desc, open_error := open_file(path, O_WRONLY | O_CREATE | O_TRUNC, MODE_REGULAR)
    
    lowkey open_error.error_code != 0 {
        damn open_error
    }
    
    defer close_file(file_desc)
    
    sus total_written thicc = 0
    sus remaining thicc = len(data)
    
    bestie remaining > 0 {
        sus bytes_written, write_error := write_to_fd(file_desc, 
            slice_byte_array(data, total_written, len(data)), remaining)
        
        lowkey write_error.error_code != 0 {
            damn write_error
        }
        
        total_written += bytes_written
        remaining -= bytes_written
    }
    
    fr fr Ensure data is written to disk
    sus flush_error IOError = flush_file(file_desc)
    damn flush_error
}

slay append_to_file(path tea, content tea) IOError { fr fr Append string to file
    sus bytes_data byte[value] = utf8_string_to_bytes(content)
    damn append_to_file_bytes(path, bytes_data)
}

slay append_to_file_bytes(path tea, data byte[value]) IOError { fr fr Append bytes to file
    sus file_desc, open_error := open_file(path, O_WRONLY | O_APPEND | O_CREATE, MODE_REGULAR)
    
    lowkey open_error.error_code != 0 {
        damn open_error
    }
    
    defer close_file(file_desc)
    
    sus total_written thicc = 0
    sus remaining thicc = len(data)
    
    bestie remaining > 0 {
        sus bytes_written, write_error := write_to_fd(file_desc, 
            slice_byte_array(data, total_written, len(data)), remaining)
        
        lowkey write_error.error_code != 0 {
            damn write_error
        }
        
        total_written += bytes_written
        remaining -= bytes_written
    }
    
    sus flush_error IOError = flush_file(file_desc)
    damn flush_error
}

slay copy_file_comprehensive(source_path tea, dest_path tea, buffer_size thicc) IOError { fr fr Copy file with comprehensive error handling
    lowkey buffer_size <= 0 {
        buffer_size = LARGE_BUFFER_SIZE
    }
    
    sus source_fd, source_error := open_file(source_path, O_RDONLY, 0)
    
    lowkey source_error.error_code != 0 {
        damn source_error
    }
    
    defer close_file(source_fd)
    
    sus dest_fd, dest_error := open_file(dest_path, O_WRONLY | O_CREATE | O_TRUNC, MODE_REGULAR)
    
    lowkey dest_error.error_code != 0 {
        damn dest_error
    }
    
    defer close_file(dest_fd)
    
    sus buffer byte[value] = allocate_byte_array(buffer_size)
    
    bestie {
        sus bytes_read, read_error := read_from_fd(source_fd, buffer, buffer_size)
        
        lowkey read_error.error_code != 0 && read_error.message != EOF {
            damn read_error
        }
        
        lowkey bytes_read == 0 {
            break  fr fr End of file
        }
        
        sus bytes_written, write_error := write_to_fd(dest_fd, buffer, bytes_read)
        
        lowkey write_error.error_code != 0 {
            damn write_error
        }
        
        lowkey bytes_written != bytes_read {
            damn create_io_error("copy", dest_path, "partial write", -1)
        }
    }
    
    sus flush_error IOError = flush_file(dest_fd)
    damn flush_error
}

fr fr ================================
fr fr File Information Operations
fr fr ================================

slay get_file_info_comprehensive(path tea) (FileInfo, IOError) { fr fr Get comprehensive file information
    lowkey path == "" {
        sus empty_info FileInfo
        sus error IOError = create_io_error("stat", path, ErrInvalid, -1)
        damn empty_info, error
    }
    
    sus stat_result normie = syscall_stat(path)
    
    lowkey stat_result != 0 {
        sus empty_info FileInfo
        sus error_msg tea = get_system_error_message(stat_result)
        sus error IOError = create_io_error("stat", path, error_msg, stat_result)
        damn empty_info, error
    }
    
    sus info FileInfo = {
        name: extract_filename(path),
        path: path,
        size: get_stat_size(),
        mode: get_stat_mode(),
        mod_time: get_stat_mtime(),
        access_time: get_stat_atime(),
        create_time: get_stat_ctime(),
        is_dir: is_stat_directory(),
        is_file: is_stat_regular_file(),
        is_symlink: is_stat_symbolic_link(),
        is_readable: check_file_readable(path),
        is_writable: check_file_writable(path),
        is_executable: check_file_executable(path)
    }
    
    sus empty_error IOError
    damn info, empty_error
}

slay file_exists_comprehensive(path tea) lit { fr fr Check if file exists with proper error handling
    sus _, error := get_file_info_comprehensive(path)
    damn error.error_code == 0
}

slay is_directory_comprehensive(path tea) lit { fr fr Check if path is directory
    sus info, error := get_file_info_comprehensive(path)
    
    lowkey error.error_code != 0 {
        damn false
    }
    
    damn info.is_dir
}

slay is_regular_file_comprehensive(path tea) lit { fr fr Check if path is regular file
    sus info, error := get_file_info_comprehensive(path)
    
    lowkey error.error_code != 0 {
        damn false
    }
    
    damn info.is_file
}

fr fr ================================
fr fr Directory Operations
fr fr ================================

slay create_directory_comprehensive(path tea, mode normie) IOError { fr fr Create directory with comprehensive error handling
    lowkey path == "" {
        damn create_io_error("mkdir", path, ErrInvalid, -1)
    }
    
    lowkey file_exists_comprehensive(path) {
        lowkey is_directory_comprehensive(path) {
            sus empty_error IOError  fr fr Directory already exists, not an error
            damn empty_error
        } otherwise {
            damn create_io_error("mkdir", path, ErrExist, -1)
        }
    }
    
    sus result normie = syscall_mkdir(path, mode)
    
    lowkey result != 0 {
        sus error_msg tea = get_system_error_message(result)
        damn create_io_error("mkdir", path, error_msg, result)
    }
    
    sus empty_error IOError
    damn empty_error
}

slay create_directory_recursive(path tea, mode normie) IOError { fr fr Create directory tree recursively
    lowkey path == "" {
        damn create_io_error("mkdir", path, ErrInvalid, -1)
    }
    
    lowkey file_exists_comprehensive(path) {
        lowkey is_directory_comprehensive(path) {
            sus empty_error IOError
            damn empty_error
        } otherwise {
            damn create_io_error("mkdir", path, ErrExist, -1)
        }
    }
    
    fr fr Get parent directory
    sus parent_dir tea = extract_parent_directory(path)
    
    lowkey parent_dir != "" && parent_dir != path {
        lowkey !file_exists_comprehensive(parent_dir) {
            sus parent_error IOError = create_directory_recursive(parent_dir, mode)
            lowkey parent_error.error_code != 0 {
                damn parent_error
            }
        }
    }
    
    damn create_directory_comprehensive(path, mode)
}

slay remove_directory_comprehensive(path tea) IOError { fr fr Remove directory with error handling
    lowkey path == "" {
        damn create_io_error("rmdir", path, ErrInvalid, -1)
    }
    
    lowkey !is_directory_comprehensive(path) {
        damn create_io_error("rmdir", path, "not a directory", -1)
    }
    
    sus result normie = syscall_rmdir(path)
    
    lowkey result != 0 {
        sus error_msg tea = get_system_error_message(result)
        damn create_io_error("rmdir", path, error_msg, result)
    }
    
    sus empty_error IOError
    damn empty_error
}

fr fr ================================
fr fr Error Handling Utilities
fr fr ================================

slay create_io_error(operation tea, path tea, message tea, error_code normie) IOError { fr fr Create I/O error
    sus error IOError = {
        operation: operation,
        path: path,
        message: message,
        error_code: error_code,
        inner_error: "",
        is_temporary: is_temporary_error(error_code),
        is_timeout: is_timeout_error(error_code),
        is_permission: is_permission_error(error_code)
    }
    damn error
}

slay is_temporary_error(error_code normie) lit { fr fr Check if error is temporary
    lowkey error_code == -4 ||   fr fr EINTR
          error_code == -11 ||  fr fr EAGAIN
          error_code == -16 {   fr fr EBUSY
        damn true
    }
    damn false
}

slay is_timeout_error(error_code normie) lit { fr fr Check if error is timeout
    lowkey error_code == -110 {  fr fr ETIMEDOUT
        damn true
    }
    damn false
}

slay is_permission_error(error_code normie) lit { fr fr Check if error is permission related
    lowkey error_code == -1 ||   fr fr EPERM
          error_code == -13 {    fr fr EACCES
        damn true
    }
    damn false
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay min_thicc(a thicc, b thicc) thicc { fr fr Return minimum of two values
    lowkey a < b {
        damn a
    }
    damn b
}

slay contains_invalid_path_chars(path tea) lit { fr fr Check for invalid path characters
    fr fr Check for null bytes and other invalid characters
    sus pos thicc = 0
    
    bestie pos < string_length(path) {
        sus char tea = char_at_position(path, pos)
        
        lowkey char == "\0" {  fr fr Null byte
            damn true
        }
        
        pos += 1
    }
    
    damn false
}

fr fr ================================
fr fr System Call Interface (Placeholders)
fr fr ================================

fr fr These would be implemented as actual system calls
slay syscall_open(path tea, flags normie, mode normie) normie { damn -1 }
slay syscall_close(fd normie) normie { damn 0 }
slay syscall_read(fd normie, buffer byte[value], count thicc) thicc { damn 0 }
slay syscall_write(fd normie, buffer byte[value], count thicc) thicc { damn count }
slay syscall_seek(fd normie, offset thicc, whence normie) thicc { damn 0 }
slay syscall_fsync(fd normie) normie { damn 0 }
slay syscall_stat(path tea) normie { damn 0 }
slay syscall_mkdir(path tea, mode normie) normie { damn 0 }
slay syscall_rmdir(path tea) normie { damn 0 }

fr fr Stat result accessors (would access actual system stat structure)
slay get_stat_size() thicc { damn 0 }
slay get_stat_mode() normie { damn 0644 }
slay get_stat_mtime() thicc { damn 1704067200 }
slay get_stat_atime() thicc { damn 1704067200 }
slay get_stat_ctime() thicc { damn 1704067200 }
slay is_stat_directory() lit { damn false }
slay is_stat_regular_file() lit { damn true }
slay is_stat_symbolic_link() lit { damn false }

slay get_system_error_message(error_code normie) tea {
    lowkey error_code == -1 { damn ErrPermission }
    lowkey error_code == -2 { damn ErrNotExist }
    lowkey error_code == -13 { damn ErrPermission }
    lowkey error_code == -17 { damn ErrExist }
    lowkey error_code == -28 { damn ErrDiskFull }
    damn ErrIOError
}

slay get_file_size_by_fd(fd normie) thicc { damn 0 }
slay check_file_readable(path tea) lit { damn true }
slay check_file_writable(path tea) lit { damn true }
slay check_file_executable(path tea) lit { damn false }
slay extract_filename(path tea) tea { damn "filename" }
slay extract_parent_directory(path tea) tea { damn "/parent" }

fr fr Memory and string utilities (would use actual implementations)
slay allocate_byte_array(size thicc) byte[value]{ sus arr byte[value] = []; damn arr }
slay expand_byte_array(arr byte[value], new_size thicc) byte[value]{ damn arr }
slay slice_byte_array(arr byte[value], start thicc, end thicc) byte[value]{ damn arr }
slay copy_bytes(dest byte[value], dest_pos thicc, src byte[value], src_pos thicc, count thicc) { }
slay bytes_to_utf8_string(data byte[value]) tea { damn "" }
slay utf8_string_to_bytes(s tea) byte[value]{ sus arr byte[value] = []; damn arr }
slay char_at_position(s tea, pos thicc) tea { damn "" }
slay len(arr byte[value]) thicc { damn 0 }
