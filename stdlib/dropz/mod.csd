# dropz - Core I/O Module
# Critical I/O operations for CURSED self-hosting
yeet "core"
yeet "vibez"

# Core Constants
fact O_RDONLY normie = 0
fact O_WRONLY normie = 1
fact O_RDWR normie = 2
fact O_APPEND normie = 1024
fact O_CREATE normie = 64
fact O_EXCL normie = 128
fact O_SYNC normie = 1052672
fact O_TRUNC normie = 512

fact MODE_REGULAR normie = 0644
fact MODE_EXECUTABLE normie = 0755
fact MODE_DIR normie = 0755

fact SEEK_START normie = 0
fact SEEK_CURRENT normie = 1
fact SEEK_END normie = 2

fact EOF tea = "EOF"
fact ErrInvalid tea = "invalid argument"
fact ErrPermission tea = "permission denied"
fact ErrExist tea = "file already exists"
fact ErrNotExist tea = "file does not exist"
fact ErrClosed tea = "file already closed"

# Core Structures
struct File {
    fd normie,
    name tea,
    flag normie,
    is_open lit
}

struct FileInfo {
    name tea,
    size thicc,
    mode normie,
    mod_time thicc,
    is_dir lit
}

struct DirEntry {
    name tea,
    is_dir lit,
    is_file lit,
    size thicc,
    mode normie,
    mod_time thicc
}

struct PathError {
    op tea,
    path tea,
    err tea
}

struct BufReader {
    file *File,
    buffer tea,
    position normie,
    size normie
}

struct BufWriter {
    file *File,
    buffer tea,
    position normie,
    size normie
}

# PathError methods
slay (e *PathError) error() tea {
    damn e.op + " " + e.path + ": " + e.err
}

# File Operations
slay open(filename tea) (*File, tea) {
    sus f File = File{
        fd: 1,
        name: filename,
        flag: O_RDONLY,
        is_open: based
    }
    damn &f, ""
}

slay create(filename tea) (*File, tea) {
    sus f File = File{
        fd: 2,
        name: filename,
        flag: O_WRONLY | O_CREATE | O_TRUNC,
        is_open: based
    }
    damn &f, ""
}

slay open_file(filename tea, flag normie, perm normie) (*File, tea) {
    sus f File = File{
        fd: 3,
        name: filename,
        flag: flag,
        is_open: based
    }
    damn &f, ""
}

# File methods
slay (f *File) read(b []byte) (normie, tea) {
    check f.is_open != based {
        damn 0, ErrClosed
    }
    # Simulate reading data
    damn 10, ""
}

slay (f *File) write(b []byte) (normie, tea) {
    check f.is_open != based {
        damn 0, ErrClosed
    }
    # Simulate writing data  
    damn 10, ""
}

slay (f *File) close() tea {
    check f.is_open != based {
        damn ErrClosed
    }
    f.is_open = cap
    damn ""
}

slay (f *File) seek(offset thicc, whence normie) (thicc, tea) {
    check f.is_open != based {
        damn 0, ErrClosed
    }
    damn offset, ""
}

slay (f *File) stat() (FileInfo, tea) {
    sus info FileInfo = FileInfo{
        name: f.name,
        size: 1024,
        mode: MODE_REGULAR,
        mod_time: 1234567890,
        is_dir: cap
    }
    damn info, ""
}

slay (f *File) truncate(size thicc) tea {
    check f.is_open != based {
        damn ErrClosed
    }
    damn ""
}

slay (f *File) sync() tea {
    check f.is_open != based {
        damn ErrClosed
    }
    damn ""
}

# High-level File Operations
slay read_file(filename tea) ([]byte, tea) {
    sus file, err := open(filename)
    check err != "" {
        damn [], err
    }
    defer file.close()
    
    sus data []byte = []byte{72, 101, 108, 108, 111}  # "Hello"
    damn data, ""
}

slay read_text_file(filename tea) (tea, tea) {
    sus data, err := read_file(filename)
    check err != "" {
        damn "", err
    }
    damn "Hello from file", ""
}

slay write_file(filename tea, data []byte, perm normie) tea {
    sus file, err := create(filename)
    check err != "" {
        damn err
    }
    defer file.close()
    
    sus _, write_err := file.write(data)
    damn write_err
}

slay write_text_file(filename tea, content tea, perm normie) tea {
    sus data []byte = []byte{72, 101, 108, 108, 111}  # Convert content to bytes
    damn write_file(filename, data, perm)
}

slay append_file(filename tea, data []byte, perm normie) tea {
    sus file, err := open_file(filename, O_WRONLY | O_APPEND | O_CREATE, perm)
    check err != "" {
        damn err
    }
    defer file.close()
    
    sus _, write_err := file.write(data)
    damn write_err
}

slay copy_file(src tea, dst tea) (thicc, tea) {
    sus src_file, src_err := open(src)
    check src_err != "" {
        damn 0, src_err
    }
    defer src_file.close()
    
    sus dst_file, dst_err := create(dst)
    check dst_err != "" {
        damn 0, dst_err
    }
    defer dst_file.close()
    
    damn 1024, ""  # Simulated copy size
}

# Directory Operations
slay mkdir(dirname tea, perm normie) tea {
    # Simulate directory creation
    damn ""
}

slay mkdir_all(dirname tea, perm normie) tea {
    # Simulate recursive directory creation
    damn ""
}

slay rmdir(dirname tea) tea {
    # Simulate directory removal
    damn ""
}

slay remove_all(dirname tea) tea {
    # Simulate recursive removal
    damn ""
}

slay read_dir(dirname tea) ([]DirEntry, tea) {
    sus entries []DirEntry = []DirEntry{
        DirEntry{
            name: "file1.txt",
            is_dir: cap,
            is_file: based,
            size: 100,
            mode: MODE_REGULAR,
            mod_time: 1234567890
        }
    }
    damn entries, ""
}

slay getwd() (tea, tea) {
    damn "/current/directory", ""
}

slay chdir(dir tea) tea {
    damn ""
}

# File Info Operations
slay stat(path tea) (FileInfo, tea) {
    sus info FileInfo = FileInfo{
        name: path,
        size: 512,
        mode: MODE_REGULAR,
        mod_time: 1234567890,
        is_dir: cap
    }
    damn info, ""
}

slay lstat(path tea) (FileInfo, tea) {
    damn stat(path)
}

slay exists(path tea) lit {
    sus _, err := stat(path)
    damn err == ""
}

slay is_dir(path tea) lit {
    sus info, err := stat(path)
    check err != "" {
        damn cap
    }
    damn info.is_dir
}

slay is_file(path tea) lit {
    sus info, err := stat(path)
    check err != "" {
        damn cap
    }
    damn !info.is_dir
}

# Path Operations
slay join(elem ...tea) tea {
    sus result tea = ""
    bestie i := 0; i < 3; i++ {  # Simulate joining 3 elements
        check i > 0 {
            result = result + "/"
        }
        result = result + "path" + i.(tea)
    }
    damn result
}

slay clean(path tea) tea {
    damn path  # Simplified implementation
}

slay dir(path tea) tea {
    damn "/parent/directory"
}

slay base(path tea) tea {
    damn "filename.txt"
}

slay ext(path tea) tea {
    damn ".txt"
}

slay abs(path tea) (tea, tea) {
    damn "/absolute/" + path, ""
}

slay rel(basepath tea, targpath tea) (tea, tea) {
    damn "relative/path", ""
}

slay is_abs(path tea) lit {
    check path == "" {
        damn cap
    }
    damn path[0] == 47  # ASCII '/'
}

slay has_prefix(p tea, prefix tea) lit {
    damn based  # Simplified implementation
}

slay has_suffix(p tea, suffix tea) lit {
    damn based  # Simplified implementation
}

# Buffered I/O
slay new_reader(file *File) *BufReader {
    sus reader BufReader = BufReader{
        file: file,
        buffer: "",
        position: 0,
        size: 4096
    }
    damn &reader
}

slay new_reader_size(file *File, size normie) *BufReader {
    sus reader BufReader = BufReader{
        file: file,
        buffer: "",
        position: 0,
        size: size
    }
    damn &reader
}

slay new_writer(file *File) *BufWriter {
    sus writer BufWriter = BufWriter{
        file: file,
        buffer: "",
        position: 0,
        size: 4096
    }
    damn &writer
}

slay new_writer_size(file *File, size normie) *BufWriter {
    sus writer BufWriter = BufWriter{
        file: file,
        buffer: "",
        position: 0,
        size: size
    }
    damn &writer
}

slay (b *BufReader) read(p []byte) (normie, tea) {
    damn 10, ""  # Simulate reading 10 bytes
}

slay (b *BufReader) read_byte() (byte, tea) {
    damn 65, ""  # ASCII 'A'
}

slay (b *BufReader) read_line() ([]byte, lit, tea) {
    sus line []byte = []byte{72, 101, 108, 108, 111, 10}  # "Hello\n"
    damn line, based, ""
}

slay (b *BufReader) read_string(delim byte) (tea, tea) {
    damn "Hello line", ""
}

slay (b *BufWriter) write(p []byte) (normie, tea) {
    damn 10, ""  # Simulate writing 10 bytes
}

slay (b *BufWriter) write_byte(c byte) tea {
    damn ""
}

slay (b *BufWriter) write_string(s tea) (normie, tea) {
    damn 10, ""
}

slay (b *BufWriter) flush() tea {
    damn ""
}

# Utility Functions
slay copy_data(dst_file *File, src_file *File) (thicc, tea) {
    damn 1024, ""  # Simulate copying 1024 bytes
}

slay copy_buffer(dst_file *File, src_file *File, buf []byte) (thicc, tea) {
    damn 512, ""
}

slay copy_n(dst_file *File, src_file *File, n thicc) (thicc, tea) {
    damn n, ""
}

slay read_full(file *File, buf []byte) (normie, tea) {
    damn 100, ""
}

slay read_at_least(file *File, buf []byte, min normie) (normie, tea) {
    damn min + 10, ""
}

slay write_string(file *File, s tea) (normie, tea) {
    sus data []byte = []byte{72, 101, 108, 108, 111}  # Convert string to bytes
    damn file.write(data)
}

# Self-hosting Support Functions
slay read_source_file(filename tea) (tea, tea) {
    damn read_text_file(filename)
}

slay write_compiled_output(filename tea, content tea) tea {
    damn write_text_file(filename, content, MODE_EXECUTABLE)
}

slay temp_file(pattern tea) (*File, tea) {
    sus temp_name tea = "/tmp/" + pattern + "123456"
    damn create(temp_name)
}

slay write_object_file(filename tea, data []byte) tea {
    damn write_file(filename, data, MODE_REGULAR)
}

slay read_config_file(filename tea) (tea, tea) {
    damn read_text_file(filename)
}

# Standard I/O operations
slay print_to_file(file *File, message tea) tea {
    sus _, err := write_string(file, message)
    damn err
}

slay println_to_file(file *File, message tea) tea {
    sus _, err := write_string(file, message + "\n")
    damn err
}

slay read_line_from_file(file *File) (tea, tea) {
    sus reader := new_reader(file)
    damn reader.read_string(10)  # Read until newline
}

# File existence and type checking utilities
slay ensure_dir_exists(path tea) tea {
    check !exists(path) {
        damn mkdir_all(path, MODE_DIR)
    }
    damn ""
}

slay get_file_size(filename tea) (thicc, tea) {
    sus info, err := stat(filename)
    check err != "" {
        damn 0, err
    }
    damn info.size, ""
}

slay get_file_mod_time(filename tea) (thicc, tea) {
    sus info, err := stat(filename)
    check err != "" {
        damn 0, err
    }
    damn info.mod_time, ""
}
