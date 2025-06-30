// Standard I/O library

// ================================
// Console I/O
// ================================

fn print(message: string) -> void {
    io_print(message);
}

fn println(message: string) -> void {
    io_println(message);
}

fn printf(format: string, args: array) -> void {
    io_printf(format, args);
}

fn eprint(message: string) -> void {
    io_eprint(message);
}

fn eprintln(message: string) -> void {
    io_eprintln(message);
}

fn read_line() -> string {
    return io_read_line();
}

fn read_char() -> string {
    return io_read_char();
}

fn read_int() -> int {
    return io_read_int();
}

fn read_float() -> float {
    return io_read_float();
}

// ================================
// File I/O
// ================================

fn write_file(path: string, content: string) -> bool {
    return io_write_file(path, content);
}

fn read_file(path: string) -> string {
    return io_read_file(path);
}

fn read_file_bytes(path: string) -> array {
    return io_read_file_bytes(path);
}

fn write_file_bytes(path: string, data: array) -> bool {
    return io_write_file_bytes(path, data);
}

fn append_file(path: string, content: string) -> bool {
    return io_append_file(path, content);
}

fn copy_file(src: string, dest: string) -> bool {
    return io_copy_file(src, dest);
}

fn move_file(src: string, dest: string) -> bool {
    return io_move_file(src, dest);
}

fn delete_file(path: string) -> bool {
    return io_delete_file(path);
}

fn file_exists(path: string) -> bool {
    return io_file_exists(path);
}

fn file_size(path: string) -> int {
    return io_file_size(path);
}

fn file_modified_time(path: string) -> int {
    return io_file_modified_time(path);
}

fn file_created_time(path: string) -> int {
    return io_file_created_time(path);
}

fn is_file(path: string) -> bool {
    return io_is_file(path);
}

fn is_directory(path: string) -> bool {
    return io_is_directory(path);
}

// ================================
// Directory Operations
// ================================

fn create_directory(path: string) -> bool {
    return io_create_directory(path);
}

fn create_directory_recursive(path: string) -> bool {
    return io_create_directory_recursive(path);
}

fn remove_directory(path: string) -> bool {
    return io_remove_directory(path);
}

fn remove_directory_recursive(path: string) -> bool {
    return io_remove_directory_recursive(path);
}

fn list_directory(path: string) -> array {
    return io_list_directory(path);
}

fn list_directory_recursive(path: string) -> array {
    return io_list_directory_recursive(path);
}

fn current_directory() -> string {
    return io_current_directory();
}

fn change_directory(path: string) -> bool {
    return io_change_directory(path);
}

// ================================
// Path Operations
// ================================

fn path_join(parts: array) -> string {
    return io_path_join(parts);
}

fn path_dirname(path: string) -> string {
    return io_path_dirname(path);
}

fn path_basename(path: string) -> string {
    return io_path_basename(path);
}

fn path_extension(path: string) -> string {
    return io_path_extension(path);
}

fn path_absolute(path: string) -> string {
    return io_path_absolute(path);
}

fn path_relative(from: string, to: string) -> string {
    return io_path_relative(from, to);
}

fn path_exists(path: string) -> bool {
    return io_path_exists(path);
}

// ================================
// Stream I/O
// ================================

fn open_file_read(path: string) -> file_handle {
    return io_open_file_read(path);
}

fn open_file_write(path: string) -> file_handle {
    return io_open_file_write(path);
}

fn open_file_append(path: string) -> file_handle {
    return io_open_file_append(path);
}

fn close_file(handle: file_handle) -> bool {
    return io_close_file(handle);
}

fn read_from_file(handle: file_handle, size: int) -> string {
    return io_read_from_file(handle, size);
}

fn write_to_file(handle: file_handle, data: string) -> bool {
    return io_write_to_file(handle, data);
}

fn flush_file(handle: file_handle) -> bool {
    return io_flush_file(handle);
}

fn seek_file(handle: file_handle, position: int) -> bool {
    return io_seek_file(handle, position);
}

fn tell_file(handle: file_handle) -> int {
    return io_tell_file(handle);
}

// ================================
// Buffered I/O
// ================================

fn create_buffer(size: int) -> buffer {
    return io_create_buffer(size);
}

fn buffer_write(buf: buffer, data: string) -> bool {
    return io_buffer_write(buf, data);
}

fn buffer_read(buf: buffer, size: int) -> string {
    return io_buffer_read(buf, size);
}

fn buffer_flush(buf: buffer) -> bool {
    return io_buffer_flush(buf);
}

fn buffer_clear(buf: buffer) -> bool {
    return io_buffer_clear(buf);
}

fn buffer_size(buf: buffer) -> int {
    return io_buffer_size(buf);
}

fn buffer_available(buf: buffer) -> int {
    return io_buffer_available(buf);
}

// ================================
// Temporary Files
// ================================

fn create_temp_file() -> string {
    return io_create_temp_file();
}

fn create_temp_directory() -> string {
    return io_create_temp_directory();
}

fn temp_directory() -> string {
    return io_temp_directory();
}
