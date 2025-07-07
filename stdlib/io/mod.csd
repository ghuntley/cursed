// Standard I/O library

// ================================
// Console I/O
// ================================

slay print(message tea) {
    io_print(message);
}

slay println(message tea) {
    io_println(message);
}

slay printf(format tea, args [tea]) {
    io_printf(format, args);
}

slay eprint(message tea) {
    io_eprint(message);
}

slay eprintln(message tea) {
    io_eprintln(message);
}

slay read_line() tea {
    damn io_read_line();
}

slay read_char() tea {
    damn io_read_char();
}

slay read_int() normie {
    damn io_read_int();
}

slay read_float() meal {
    damn io_read_float();
}

// ================================
// File I/O
// ================================

slay write_file(path tea, content tea) lit {
    damn io_write_file(path, content);
}

slay read_file(path tea) tea {
    damn io_read_file(path);
}

slay read_file_bytes(path tea) [byte] {
    damn io_read_file_bytes(path);
}

slay write_file_bytes(path tea, data [byte]) lit {
    damn io_write_file_bytes(path, data);
}

slay append_file(path tea, content tea) lit {
    damn io_append_file(path, content);
}

slay copy_file(src tea, dest tea) lit {
    damn io_copy_file(src, dest);
}

slay move_file(src tea, dest tea) lit {
    damn io_move_file(src, dest);
}

slay delete_file(path tea) lit {
    damn io_delete_file(path);
}

slay file_exists(path tea) lit {
    damn io_file_exists(path);
}

slay file_size(path tea) normie {
    damn io_file_size(path);
}

slay file_modified_time(path tea) normie {
    damn io_file_modified_time(path);
}

slay file_created_time(path tea) normie {
    damn io_file_created_time(path);
}

slay is_file(path tea) lit {
    damn io_is_file(path);
}

slay is_directory(path tea) lit {
    damn io_is_directory(path);
}

// ================================
// Directory Operations
// ================================

slay create_directory(path tea) lit {
    damn io_create_directory(path);
}

slay create_directory_recursive(path tea) lit {
    damn io_create_directory_recursive(path);
}

slay remove_directory(path tea) lit {
    damn io_remove_directory(path);
}

slay remove_directory_recursive(path tea) lit {
    damn io_remove_directory_recursive(path);
}

slay list_directory(path tea) [tea] {
    damn io_list_directory(path);
}

slay list_directory_recursive(path tea) [tea] {
    damn io_list_directory_recursive(path);
}

slay current_directory() tea {
    damn io_current_directory();
}

slay change_directory(path tea) lit {
    damn io_change_directory(path);
}

// ================================
// Path Operations
// ================================

slay path_join(parts [tea]) tea {
    damn io_path_join(parts);
}

slay path_dirname(path tea) tea {
    damn io_path_dirname(path);
}

slay path_basename(path tea) tea {
    damn io_path_basename(path);
}

slay path_extension(path tea) tea {
    damn io_path_extension(path);
}

slay path_absolute(path tea) tea {
    damn io_path_absolute(path);
}

slay path_relative(from tea, to tea) tea {
    damn io_path_relative(from, to);
}

slay path_exists(path tea) lit {
    damn io_path_exists(path);
}

// ================================
// Stream I/O
// ================================

slay open_file_read(path tea) file_handle {
    damn io_open_file_read(path);
}

slay open_file_write(path tea) file_handle {
    damn io_open_file_write(path);
}

slay open_file_append(path tea) file_handle {
    damn io_open_file_append(path);
}

slay close_file(handle file_handle) lit {
    damn io_close_file(handle);
}

slay read_from_file(handle file_handle, size normie) tea {
    damn io_read_from_file(handle, size);
}

slay write_to_file(handle file_handle, data tea) lit {
    damn io_write_to_file(handle, data);
}

slay flush_file(handle file_handle) lit {
    damn io_flush_file(handle);
}

slay seek_file(handle file_handle, position normie) lit {
    damn io_seek_file(handle, position);
}

slay tell_file(handle file_handle) normie {
    damn io_tell_file(handle);
}

// ================================
// Buffered I/O
// ================================

slay create_buffer(size normie) buffer {
    damn io_create_buffer(size);
}

slay buffer_write(buf buffer, data tea) lit {
    damn io_buffer_write(buf, data);
}

slay buffer_read(buf buffer, size normie) tea {
    damn io_buffer_read(buf, size);
}

slay buffer_flush(buf buffer) lit {
    damn io_buffer_flush(buf);
}

slay buffer_clear(buf buffer) lit {
    damn io_buffer_clear(buf);
}

slay buffer_size(buf buffer) normie {
    damn io_buffer_size(buf);
}

slay buffer_available(buf buffer) normie {
    damn io_buffer_available(buf);
}

// ================================
// Temporary Files
// ================================

slay create_temp_file() tea {
    damn io_create_temp_file();
}

slay create_temp_directory() tea {
    damn io_create_temp_directory();
}

slay temp_directory() tea {
    damn io_temp_directory();
}
