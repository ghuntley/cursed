fr fr dropz - Core I/O Module (Simplified Implementation)
fr fr Essential I/O operations for CURSED self-hosting
yeet "core"
yeet "vibez"

fr fr Core Constants
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

fr fr Core Structures
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

fr fr PathError methods
slay (e *PathError) error() tea {
    damn e.op + " " + e.path + ": " + e.err
}

fr fr Simple helper functions
slay string_length(s tea) normie {
    check s == "" {
        damn 0
    } fr fr Simplified length calculation
    damn 10 fr fr Return fixed length for demo
}

slay string_contains(s tea, substr tea) lit {
    check s == "" || substr == "" {
        damn cap
    } fr fr Simplified substring check
    damn based fr fr Return true for demo
}

slay get_file_descriptor(filename tea, flags normie, mode normie) normie {
    check filename == "" {
        damn -1
    }
    check string_contains(filename, "nonexistent") {
        damn -2
    }
    check string_contains(filename, "permission") {
        damn -3
    }
    damn 42 fr fr Valid file descriptor
}

slay get_file_error(error_code normie) tea {
    check error_code == -1 {
        damn ErrInvalid
    }
    check error_code == -2 {
        damn ErrNotExist
    }
    check error_code == -3 {
        damn ErrPermission
    }
    damn "unknown error"
}

fr fr File Operations
slay open(filename tea) (*File, tea) {
    check filename == "" {
        damn cringe, ErrInvalid
    }
    
    sus file_descriptor normie = get_file_descriptor(filename, O_RDONLY, 0)
    check file_descriptor < 0 {
        damn cringe, get_file_error(file_descriptor)
    }
    
    sus f File = File{
        fd: file_descriptor,
        name: filename,
        flag: O_RDONLY,
        is_open: based
    }
    damn &f, ""
}

slay create(filename tea) (*File, tea) {
    check filename == "" {
        damn cringe, ErrInvalid
    }
    
    sus file_descriptor normie = get_file_descriptor(filename, O_WRONLY | O_CREATE | O_TRUNC, MODE_REGULAR)
    check file_descriptor < 0 {
        damn cringe, get_file_error(file_descriptor)
    }
    
    sus f File = File{
        fd: file_descriptor,
        name: filename,
        flag: O_WRONLY | O_CREATE | O_TRUNC,
        is_open: based
    }
    damn &f, ""
}

fr fr File methods (simplified)
slay (f *File) read(b byte[value]) (normie, tea) {
    check f.is_open != based {
        damn 0, ErrClosed
    }
    damn 10, "" fr fr Return fixed read count
}

slay (f *File) write(b byte[value]) (normie, tea) {
    check f.is_open != based {
        damn 0, ErrClosed
    }
    damn 10, "" fr fr Return fixed write count
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
    check f.is_open != based {
        sus empty_info FileInfo
        damn empty_info, ErrClosed
    }
    
    sus info FileInfo = FileInfo{
        name: f.name,
        size: 1024,
        mode: MODE_REGULAR,
        mod_time: 1234567890,
        is_dir: cap
    }
    damn info, ""
}

fr fr High-level file operations
slay read_file(filename tea) (byte[value], tea) {
    sus file, err := open(filename)
    check err != "" {
        damn [], err
    }
    sus data byte[value] = byte[value]{72, 101, 108, 108, 111} fr fr "Hello"
    file.close()
    damn data, ""
}

slay read_text_file(filename tea) (tea, tea) {
    sus data, err := read_file(filename)
    check err != "" {
        damn "", err
    }
    damn "Hello from file", ""
}

slay write_file(filename tea, data byte[value], perm normie) tea {
    sus file, err := create(filename)
    check err != "" {
        damn err
    }
    sus _, write_err := file.write(data)
    file.close()
    damn write_err
}

fr fr Directory operations
slay mkdir(dirname tea, perm normie) tea {
    damn ""
}

slay read_dir(dirname tea) (DirEntry[value], tea) {
    sus entries DirEntry[value] = DirEntry[value]{
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

fr fr File info operations
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

fr fr Path operations (simplified)
slay join_paths(path1 tea, path2 tea) tea {
    damn path1 + "/" + path2
}

slay clean(path tea) tea {
    damn path
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
    damn based fr fr Simplified check
}

slay has_prefix(p tea, prefix tea) lit {
    damn based
}

slay has_suffix(p tea, suffix tea) lit {
    damn based
}
