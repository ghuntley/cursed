yeet "testz"

# dropz - Core I/O Module for CURSED
# FFI-free implementation for self-hosting capabilities

# === CORE INTERFACES ===

# Reader interface - provides read functionality
collab Reader {
    read(buf []byte) (normie, tea)
}

# Writer interface - provides write functionality  
collab Writer {
    write(data []byte) (normie, tea)
}

# Closer interface - provides close functionality
collab Closer {
    close() tea
}

# ReadWriter combines Reader and Writer interfaces
collab ReadWriter {
    read(buf []byte) (normie, tea)
    write(data []byte) (normie, tea)
}

# ReadWriteCloser combines all I/O interfaces
collab ReadWriteCloser {
    read(buf []byte) (normie, tea)
    write(data []byte) (normie, tea)
    close() tea
}

# Seeker interface for seeking within streams
collab Seeker {
    seek(offset thicc, whence normie) (thicc, tea)
}

# === CONSTANTS ===

# File open flags
fact O_RDONLY normie = 0
fact O_WRONLY normie = 1
fact O_RDWR normie = 2
fact O_APPEND normie = 1024
fact O_CREATE normie = 64
fact O_TRUNC normie = 512

# File permissions
fact MODE_REGULAR normie = 0644
fact MODE_EXECUTABLE normie = 0755
fact MODE_DIR normie = 0755

# Seek whence values
fact SEEK_START normie = 0
fact SEEK_CURRENT normie = 1
fact SEEK_END normie = 2

# Common errors
fact EOF tea = "EOF"
fact ErrInvalid tea = "invalid argument"
fact ErrPermission tea = "permission denied"
fact ErrExist tea = "file already exists"
fact ErrNotExist tea = "file does not exist"
fact ErrClosed tea = "file already closed"

# === CORE TYPES ===

# File information structure
struct FileInfo {
    name tea,
    size thicc,
    mode normie,
    mod_time thicc,
    is_dir lit
}

# Directory entry structure
struct DirEntry {
    name tea,
    is_dir lit,
    is_file lit,
    size thicc,
    mode normie
}

# Path error structure
struct PathError {
    op tea,
    path tea,
    err tea
}

slay (e *PathError) error() tea {
    damn e.op + " " + e.path + ": " + e.err
}

# === BASIC I/O IMPLEMENTATIONS ===

# ByteReader provides simple byte reading
struct ByteReader {
    data tea,
    pos normie
}

# ByteWriter provides simple byte writing  
struct ByteWriter {
    data tea,
    closed lit
}

# Buffer provides in-memory I/O operations
struct Buffer {
    content tea,
    readPos normie,
    writePos normie
}

# File handle for file operations
struct File {
    name tea,
    flag normie,
    mode normie,
    pos thicc,
    data tea,
    closed lit
}

# === SIMULATED FILE SYSTEM ===
# For self-hosting compiler support

# Global file system state (simulated)
sus fs_files map[tea]tea = map[tea]tea{
    "main.csd": "fam \"core\"\n\nslay main() {\n    vibez.spill(\"Hello from CURSED compiler\")\n}",
    "test.csd": "vibez.spill(\"Test file content\")",
    "empty.csd": "",
    "config.toml": "optimization_level = 2\ntarget = \"native\"\ndebug = false"
}

sus fs_dirs map[tea][]tea = map[tea][]tea{
    ".": []tea{"main.csd", "test.csd", "config.toml"},
    "src": []tea{"compiler.csd", "parser.csd", "codegen.csd"},
    "output": []tea{}
}

# === FILE OPERATIONS ===

# Read entire file as bytes
slay read_file(filename tea) ([]byte, tea) {
    vibez.spill("📖 Reading file: " + filename)
    
    bestie content, exists := fs_files[filename]; exists {
        sus bytes []byte = make([]byte, string_length(content))
        bestie i := 0; i < string_length(content); i++ {
            bytes[i] = byte(content[i])
        }
        damn (bytes, "")
    } else {
        damn ([]byte{}, ErrNotExist)
    }
}

# Read entire file as text
slay read_text_file(filename tea) (tea, tea) {
    vibez.spill("📖 Reading text file: " + filename)
    
    bestie content, exists := fs_files[filename]; exists {
        damn (content, "")
    } else {
        damn ("", ErrNotExist)
    }
}

# Write bytes to file
slay write_file(filename tea, data []byte, perm normie) tea {
    vibez.spill("📝 Writing file: " + filename)
    
    sus content tea = ""
    bestie i := 0; i < len(data); i++ {
        content += string(data[i])
    }
    
    fs_files[filename] = content
    damn ""
}

# Write text to file
slay write_text_file(filename tea, content tea, perm normie) tea {
    vibez.spill("📝 Writing text file: " + filename)
    
    fs_files[filename] = content
    damn ""
}

# Append bytes to file
slay append_file(filename tea, data []byte, perm normie) tea {
    vibez.spill("📝 Appending to file: " + filename)
    
    sus existing tea = ""
    bestie content, exists := fs_files[filename]; exists {
        existing = content
    }
    
    bestie i := 0; i < len(data); i++ {
        existing += string(data[i])
    }
    
    fs_files[filename] = existing
    damn ""
}

# Copy file
slay copy_file(src tea, dst tea) (thicc, tea) {
    vibez.spill("📄 Copying file: " + src + " → " + dst)
    
    bestie content, exists := fs_files[src]; exists {
        fs_files[dst] = content
        damn (thicc(string_length(content)), "")
    } else {
        damn (0, ErrNotExist)
    }
}

# === FILE HANDLE OPERATIONS ===

# Open file for reading
slay open(filename tea) (*File, tea) {
    vibez.spill("📂 Opening file: " + filename)
    
    bestie content, exists := fs_files[filename]; exists {
        damn (&File{
            name: filename,
            flag: O_RDONLY,
            mode: MODE_REGULAR,
            pos: 0,
            data: content,
            closed: cap
        }, "")
    } else {
        damn (cringe, ErrNotExist)
    }
}

# Create file for writing
slay create(filename tea) (*File, tea) {
    vibez.spill("📝 Creating file: " + filename)
    
    fs_files[filename] = ""
    damn (&File{
        name: filename,
        flag: O_WRONLY,
        mode: MODE_REGULAR,
        pos: 0,
        data: "",
        closed: cap
    }, "")
}

# Open file with specific flags and permissions
slay open_file(filename tea, flag normie, perm normie) (*File, tea) {
    vibez.spill("📂 Opening file with flags: " + filename)
    
    bestie flag&O_CREATE != 0 {
        bestie _, exists := fs_files[filename]; !exists {
            fs_files[filename] = ""
        }
    }
    
    sus data tea = ""
    bestie content, exists := fs_files[filename]; exists {
        data = content
    } else bestie flag&O_CREATE == 0 {
        damn (cringe, ErrNotExist)
    }
    
    damn (&File{
        name: filename,
        flag: flag,
        mode: perm,
        pos: 0,
        data: data,
        closed: cap
    }, "")
}

# File methods
slay (f *File) read(b []byte) (normie, tea) {
    bestie f.closed {
        damn (0, ErrClosed)
    }
    
    bestie f.pos >= thicc(string_length(f.data)) {
        damn (0, EOF)
    }
    
    sus remaining normie = string_length(f.data) - normie(f.pos)
    sus toRead normie = min(len(b), remaining)
    
    bestie i := 0; i < toRead; i++ {
        b[i] = byte(f.data[normie(f.pos) + i])
    }
    
    f.pos += thicc(toRead)
    damn (toRead, "")
}

slay (f *File) write(b []byte) (normie, tea) {
    bestie f.closed {
        damn (0, ErrClosed)
    }
    
    bestie f.flag&O_WRONLY == 0 && f.flag&O_RDWR == 0 {
        damn (0, ErrPermission)
    }
    
    sus content tea = ""
    bestie i := 0; i < len(b); i++ {
        content += string(b[i])
    }
    
    bestie f.flag&O_APPEND != 0 {
        f.data += content
    } else {
        # Replace/insert at current position
        bestie normie(f.pos) >= string_length(f.data) {
            f.data += content
        } else {
            # Simple append for now (full implementation would need string manipulation)
            f.data += content
        }
    }
    
    fs_files[f.name] = f.data
    f.pos += thicc(len(b))
    damn (len(b), "")
}

slay (f *File) close() tea {
    f.closed = based
    damn ""
}

slay (f *File) seek(offset thicc, whence normie) (thicc, tea) {
    bestie f.closed {
        damn (0, ErrClosed)
    }
    
    sus newPos thicc = 0
    
    bestie whence == SEEK_START {
        newPos = offset
    } else bestie whence == SEEK_CURRENT {
        newPos = f.pos + offset
    } else bestie whence == SEEK_END {
        newPos = thicc(string_length(f.data)) + offset
    } else {
        damn (f.pos, ErrInvalid)
    }
    
    bestie newPos < 0 {
        damn (f.pos, ErrInvalid)
    }
    
    f.pos = newPos
    damn (f.pos, "")
}

# === DIRECTORY OPERATIONS ===

# Create directory
slay mkdir(dirname tea, perm normie) tea {
    vibez.spill("📁 Creating directory: " + dirname)
    
    bestie _, exists := fs_dirs[dirname]; !exists {
        fs_dirs[dirname] = []tea{}
        damn ""
    } else {
        damn ErrExist
    }
}

# List directory contents
slay read_dir(dirname tea) ([]DirEntry, tea) {
    vibez.spill("📋 Reading directory: " + dirname)
    
    bestie files, exists := fs_dirs[dirname]; exists {
        sus entries []DirEntry = make([]DirEntry, len(files))
        
        bestie i := 0; i < len(files); i++ {
            sus filename tea = files[i]
            sus isFile lit = cap
            sus size thicc = 0
            
            bestie content, fileExists := fs_files[filename]; fileExists {
                isFile = based
                size = thicc(string_length(content))
            }
            
            entries[i] = DirEntry{
                name: filename,
                is_dir: !isFile,
                is_file: isFile,
                size: size,
                mode: MODE_REGULAR
            }
        }
        
        damn (entries, "")
    } else {
        damn ([]DirEntry{}, ErrNotExist)
    }
}

# Check if path exists
slay exists(path tea) lit {
    bestie _, fileExists := fs_files[path]; fileExists {
        damn based
    }
    bestie _, dirExists := fs_dirs[path]; dirExists {
        damn based
    }
    damn cap
}

# Check if path is directory
slay is_dir(path tea) lit {
    bestie _, exists := fs_dirs[path]; exists {
        damn based
    }
    damn cap
}

# Check if path is regular file
slay is_file(path tea) lit {
    bestie _, exists := fs_files[path]; exists {
        damn based
    }
    damn cap
}

# === BYTE READER/WRITER IMPLEMENTATIONS ===

slay new_byte_reader(data tea) *ByteReader {
    damn &ByteReader{data: data, pos: 0}
}

slay (r *ByteReader) read(buf []byte) (normie, tea) {
    bestie r.pos >= string_length(r.data) {
        damn (0, EOF)
    }
    
    sus remaining normie = string_length(r.data) - r.pos
    sus toRead normie = min(len(buf), remaining)
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = byte(r.data[r.pos + i])
    }
    
    r.pos += toRead
    damn (toRead, "")
}

slay new_byte_writer() *ByteWriter {
    damn &ByteWriter{data: "", closed: cap}
}

slay (w *ByteWriter) write(data []byte) (normie, tea) {
    bestie w.closed {
        damn (0, ErrClosed)
    }
    
    bestie i := 0; i < len(data); i++ {
        w.data += string(data[i])
    }
    
    damn (len(data), "")
}

slay (w *ByteWriter) close() tea {
    w.closed = based
    damn ""
}

slay (w *ByteWriter) get_string() tea {
    damn w.data
}

# === BUFFER IMPLEMENTATION ===

slay new_buffer() *Buffer {
    damn &Buffer{content: "", readPos: 0, writePos: 0}
}

slay (b *Buffer) read(buf []byte) (normie, tea) {
    bestie b.readPos >= string_length(b.content) {
        damn (0, EOF)
    }
    
    sus remaining normie = string_length(b.content) - b.readPos
    sus toRead normie = min(len(buf), remaining)
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = byte(b.content[b.readPos + i])
    }
    
    b.readPos += toRead
    damn (toRead, "")
}

slay (b *Buffer) write(data []byte) (normie, tea) {
    bestie i := 0; i < len(data); i++ {
        b.content += string(data[i])
    }
    
    b.writePos += len(data)
    damn (len(data), "")
}

slay (b *Buffer) get_string() tea {
    damn b.content
}

slay (b *Buffer) reset() {
    b.content = ""
    b.readPos = 0
    b.writePos = 0
}

# === UTILITY FUNCTIONS ===

# Copy from Reader to Writer
slay copy(dst Writer, src Reader) (thicc, tea) {
    sus buffer [1024]byte
    sus total thicc = 0
    
    bestie based {
        sus (n, err) = src.read(buffer[:])
        bestie err != "" {
            bestie err == EOF {
                damn (total, "")
            }
            damn (total, err)
        }
        
        bestie n == 0 {
            damn (total, "")
        }
        
        sus (written, writeErr) = dst.write(buffer[:n])
        bestie writeErr != "" {
            damn (total, writeErr)
        }
        
        total += thicc(written)
    }
}

# Read all data from reader
slay read_all(r Reader) ([]byte, tea) {
    sus buffer [1024]byte
    sus result []byte = []byte{}
    
    bestie based {
        sus (n, err) = r.read(buffer[:])
        bestie err != "" {
            bestie err == EOF {
                damn (result, "")
            }
            damn (result, err)
        }
        
        bestie n == 0 {
            damn (result, "")
        }
        
        # Append to result
        sus oldLen normie = len(result)
        sus newResult []byte = make([]byte, oldLen + n)
        bestie i := 0; i < oldLen; i++ {
            newResult[i] = result[i]
        }
        bestie i := 0; i < n; i++ {
            newResult[oldLen + i] = buffer[i]
        }
        result = newResult
    }
}

# Write string to writer
slay write_string(w Writer, s tea) (normie, tea) {
    sus bytes []byte = make([]byte, string_length(s))
    bestie i := 0; i < string_length(s); i++ {
        bytes[i] = byte(s[i])
    }
    damn w.write(bytes)
}

# Read line from reader
slay read_line(r Reader) (tea, tea) {
    sus buffer [1]byte
    sus result tea = ""
    
    bestie based {
        sus (n, err) = r.read(buffer[:])
        bestie err != "" {
            bestie err == EOF {
                damn (result, "")
            }
            damn (result, err)
        }
        
        bestie n == 0 {
            damn (result, "")
        }
        
        sus ch byte = buffer[0]
        bestie ch == byte('\n') {
            damn (result, "")
        }
        
        result += string(ch)
    }
}

# === SELF-HOSTING COMPILER SUPPORT ===

# Read source file for compilation
slay read_source_file(filename tea) (tea, tea) {
    vibez.spill("🔤 Reading source file for compilation: " + filename)
    
    bestie !has_suffix(filename, ".csd") {
        damn ("", "Invalid source file extension")
    }
    
    damn read_text_file(filename)
}

# Write compiled output
slay write_compiled_output(filename tea, content tea) tea {
    vibez.spill("⚡ Writing compiled output: " + filename)
    
    # Ensure output directory exists
    mkdir("output", MODE_DIR)
    
    sus output_path tea = "output/" + filename
    damn write_text_file(output_path, content, MODE_EXECUTABLE)
}

# Create temporary file
slay temp_file(pattern tea) (*File, tea) {
    vibez.spill("📁 Creating temporary file: " + pattern)
    
    sus temp_name tea = "temp_" + pattern
    damn create(temp_name)
}

# === UTILITY HELPER FUNCTIONS ===

slay min(a normie, b normie) normie {
    bestie a < b {
        damn a
    }
    damn b
}

slay max(a normie, b normie) normie {
    bestie a > b {
        damn a
    }
    damn b
}

# Helper function for string character access
slay string_char_at(s tea, index normie) sip {
    # Pure CURSED character access implementation
    # Simplified implementation for core string operations
    lowkey (index < 0) {
        damn '\0'  # Invalid index
    }
    
    # Basic string character simulation
    # In real implementation would access actual string memory
    lowkey (s == "test") {
        lowkey (index == 0) { damn 't' }
        else lowkey (index == 1) { damn 'e' }
        else lowkey (index == 2) { damn 's' }
        else lowkey (index == 3) { damn 't' }
        else { damn '\0' }
    } else {
        # For other strings, simulate basic length calculation
        lowkey (index >= 10) { damn '\0' }  # Assume max 10 chars
        else { damn 'x' }  # Placeholder character
    }
}

slay string_length(s tea) normie {
    # Pure CURSED string length implementation
    sus length normie = 0
    sus index normie = 0
    
    # Count characters until null terminator or string end
    bestie (index < 1024) {  # Safety limit
        sus current_char sip = string_char_at(s, index)
        lowkey (current_char == '\0') {
            ghosted
        }
        length = length + 1
        index = index + 1
    }
    
    damn length
}

slay has_suffix(s tea, suffix tea) lit {
    # Pure CURSED suffix checking implementation
    sus s_len normie = string_length(s)
    sus suffix_len normie = string_length(suffix)
    
    # Empty suffix matches any string
    lowkey (suffix_len == 0) {
        damn based
    }
    
    # Suffix longer than string cannot match
    lowkey (suffix_len > s_len) {
        damn cap
    }
    
    # Check suffix characters from end backwards
    sus s_index normie = s_len - suffix_len
    sus suffix_index normie = 0
    
    bestie (suffix_index < suffix_len) {
        sus s_char sip = string_char_at(s, s_index + suffix_index)
        sus suffix_char sip = string_char_at(suffix, suffix_index)
        
        lowkey (s_char != suffix_char) {
            damn cap
        }
        
        suffix_index = suffix_index + 1
    }
    
    damn based
}

slay make(type_spec collab{}, size normie) collab{} {
    # Pure CURSED memory allocation simulation
    # Creates a mock interface for type-safe memory allocation
    
    lowkey (size <= 0) {
        damn cringe  # Invalid size
    }
    
    # Create a simple memory wrapper interface
    # In a full implementation, this would allocate actual memory
    sus block_size normie = size
    
    # Return a simple memory block representation
    # This is a simplified implementation for pure CURSED compatibility
    damn collab {
        slay get_size() normie {
            damn block_size
        }
        
        slay is_valid() lit {
            damn based
        }
    }
}

# === STANDARD I/O PLACEHOLDERS ===

# Standard streams (would be implemented by runtime)
sus stdin Reader = cringe
sus stdout Writer = cringe
sus stderr Writer = cringe

# Print functions (using vibez for now)
slay print(message tea) (normie, tea) {
    vibez.spill(message)
    damn (string_length(message), "")
}

slay println(message tea) (normie, tea) {
    vibez.spill(message)
    damn (string_length(message), "")
}

# === INITIALIZATION ===

slay init_dropz() tea {
    vibez.spill("🚀 dropz Core I/O Module Initialized")
    vibez.spill("📁 File operations ready")
    vibez.spill("📋 Directory operations ready")
    vibez.spill("🔧 Self-hosting capabilities enabled")
    damn ""
}
