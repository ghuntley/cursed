# SysCore (syscall package)

## Overview
SysCore provides a low-level interface to operating system functionality, allowing direct interaction with the underlying system. It's inspired by Go's syscall package but with improved organization, stronger type safety, and more predictable cross-platform behavior.

## Core Types

### Error Types

```go
type SysError struct {
    Code     int
    Function string
    Message  string
    Internal error
}

// Methods
func (e *SysError) Error() string
func (e *SysError) Unwrap() error
func (e *SysError) Timeout() bool
func (e *SysError) Temporary() bool
func (e *SysError) Is(target error) bool

// Error checking functions
func IsPermissionDenied(err error) bool
func IsNotExist(err error) bool
func IsExist(err error) bool
func IsTimeout(err error) bool
func IsConnectionRefused(err error) bool
func IsInterrupted(err error) bool
func IsTemporary(err error) bool
```

### File Descriptors

```go
type FileHandle int

// File operation constants
const (
    O_RDONLY  int = 0
    O_WRONLY  int = 1
    O_RDWR    int = 2
    O_APPEND  int = 4
    O_CREAT   int = 8
    O_EXCL    int = 16
    O_SYNC    int = 32
    O_TRUNC   int = 64
    O_NONBLOCK int = 128
    
    // File permission bits
    S_IRWXU  = 0700 // User: read, write, execute
    S_IRUSR  = 0400 // User: read
    S_IWUSR  = 0200 // User: write
    S_IXUSR  = 0100 // User: execute
    S_IRWXG  = 070  // Group: read, write, execute
    S_IRGRP  = 040  // Group: read
    S_IWGRP  = 020  // Group: write
    S_IXGRP  = 010  // Group: execute
    S_IRWXO  = 07   // Others: read, write, execute
    S_IROTH  = 04   // Others: read
    S_IWOTH  = 02   // Others: write
    S_IXOTH  = 01   // Others: execute
    
    // Common combinations
    S_DEFAULT_FILE = 0644
    S_DEFAULT_DIR  = 0755
)

// File operations
func Open(path string, mode int, perm uint32) (FileHandle, error)
func Close(fd FileHandle) error
func Read(fd FileHandle, p []byte) (n int, err error)
func Write(fd FileHandle, p []byte) (n int, err error)
func Pread(fd FileHandle, p []byte, offset int64) (n int, err error)
func Pwrite(fd FileHandle, p []byte, offset int64) (n int, err error)
func Seek(fd FileHandle, offset int64, whence int) (ret int64, err error)
func Fsync(fd FileHandle) error
func Ftruncate(fd FileHandle, length int64) error
func Fstat(fd FileHandle) (FileInfo, error)
func Fchmod(fd FileHandle, mode uint32) error
func Fchown(fd FileHandle, uid, gid int) error
func Dup(oldfd FileHandle) (newfd FileHandle, err error)
func Dup2(oldfd, newfd FileHandle) error
```

### File Information

```go
type FileInfo struct {
    Dev      uint64
    Ino      uint64
    Mode     uint32
    Nlink    uint32
    Uid      uint32
    Gid      uint32
    Rdev     uint64
    Size     int64
    Blksize  int64
    Blocks   int64
    Atim     TimeSpec
    Mtim     TimeSpec
    Ctim     TimeSpec
}

type TimeSpec struct {
    Sec  int64
    Nsec int64
}

// File Info Operations
func Stat(path string) (FileInfo, error)
func Lstat(path string) (FileInfo, error)
```

### Directory Operations

```go
type DirEnt struct {
    Ino     uint64
    Off     int64
    Reclen  uint16
    Type    uint8
    Name    string
}

func Mkdir(path string, mode uint32) error
func Rmdir(path string) error
func Rename(oldpath, newpath string) error
func Unlink(path string) error
func Readdir(fd FileHandle) ([]DirEnt, error)
func Chdir(path string) error
func Getcwd() (string, error)
```

### Process Management

```go
type ProcessID int
type GroupID int
type UserID int

// Process information
type ProcInfo struct {
    PID  ProcessID
    PPID ProcessID
    PGID GroupID
    UID  UserID
    GID  GroupID
    Comm string
    State byte
    CWD  string
    Argv []string
    Envp []string
}

// Process functions
func Fork() (pid ProcessID, err error)
func Exec(path string, args []string, env []string) error
func Wait4(pid ProcessID, options int) (wpid ProcessID, status int, err error)
func Kill(pid ProcessID, sig Signal) error
func Getpid() ProcessID
func Getppid() ProcessID
func Getuid() UserID
func Geteuid() UserID
func Getgid() GroupID
func Getegid() GroupID
func Setuid(uid UserID) error
func Seteuid(euid UserID) error
func Setgid(gid GroupID) error
func Setegid(egid GroupID) error
func Getrusage(who int, rusage *Rusage) error
func Getrlimit(resource int, rlim *Rlimit) error
func Setrlimit(resource int, rlim *Rlimit) error
func Setsid() (ProcessID, error)
func Getsid(pid ProcessID) (ProcessID, error)
func GetProcessInfo(pid ProcessID) (ProcInfo, error)

// Process resource usage
type Rusage struct {
    Utime    TimeSpec
    Stime    TimeSpec
    Maxrss   int64
    Ixrss    int64
    Idrss    int64
    Isrss    int64
    Minflt   int64
    Majflt   int64
    Nswap    int64
    Inblock  int64
    Oublock  int64
    Msgsnd   int64
    Msgrcv   int64
    Nsignals int64
    Nvcsw    int64
    Nivcsw   int64
}

// Resource limits
type Rlimit struct {
    Cur uint64
    Max uint64
}
```

### Signal Handling

```go
type Signal int

const (
    SIGHUP    Signal = 1
    SIGINT    Signal = 2
    SIGQUIT   Signal = 3
    SIGILL    Signal = 4
    SIGTRAP   Signal = 5
    SIGABRT   Signal = 6
    SIGBUS    Signal = 7
    SIGFPE    Signal = 8
    SIGKILL   Signal = 9
    SIGUSR1   Signal = 10
    SIGSEGV   Signal = 11
    SIGUSR2   Signal = 12
    SIGPIPE   Signal = 13
    SIGALRM   Signal = 14
    SIGTERM   Signal = 15
    SIGCHLD   Signal = 17
    SIGCONT   Signal = 18
    SIGSTOP   Signal = 19
    SIGTSTP   Signal = 20
    SIGTTIN   Signal = 21
    SIGTTOU   Signal = 22
    SIGURG    Signal = 23
    SIGXCPU   Signal = 24
    SIGXFSZ   Signal = 25
    SIGVTALRM Signal = 26
    SIGPROF   Signal = 27
    SIGWINCH  Signal = 28
    SIGIO     Signal = 29
    SIGPWR    Signal = 30
    SIGSYS    Signal = 31
)

// Signal handling functions
func Sigaction(sig Signal, new, old *Sigaction) error
func Sigprocmask(how int, set, oldset *Sigset) error
func Sigaltstack(new, old *Sigaltstack) error
func Sigsuspend(mask *Sigset) error

type Sigset struct {
    // platform-specific bit array
}

type Sigaction struct {
    Handler  uintptr
    Flags    uint32
    Restorer uintptr
    Mask     Sigset
}

type Sigaltstack struct {
    SS_sp    uintptr
    SS_flags int32
    SS_size  uintptr
}
```

### Network Operations

```go
type Socket FileHandle

// Socket-related constants
const (
    // Socket families
    AF_UNSPEC    = 0
    AF_UNIX      = 1
    AF_INET      = 2
    AF_INET6     = 10
    
    // Socket types
    SOCK_STREAM    = 1
    SOCK_DGRAM     = 2
    SOCK_RAW       = 3
    SOCK_SEQPACKET = 5
    
    // Socket protocols
    IPPROTO_IP   = 0
    IPPROTO_TCP  = 6
    IPPROTO_UDP  = 17
    IPPROTO_IPV6 = 41
    
    // Socket options
    SOL_SOCKET = 1
    SO_REUSEADDR = 2
    SO_KEEPALIVE = 9
    SO_BROADCAST = 6
    SO_LINGER    = 13
    SO_RCVBUF    = 8
    SO_SNDBUF    = 7
    SO_RCVTIMEO  = 20
    SO_SNDTIMEO  = 21
)

// Socket address structures
type SockaddrInet4 struct {
    Port int
    Addr [4]byte
    Zero [8]byte
}

type SockaddrInet6 struct {
    Port   int
    ZoneId uint32
    Addr   [16]byte
    Zero   [8]byte
}

type SockaddrUnix struct {
    Family uint16
    Path   string
}

// Network functions
func Socket(domain, typ, proto int) (Socket, error)
func Connect(fd Socket, sa Sockaddr) error
func Bind(fd Socket, sa Sockaddr) error
func Listen(fd Socket, backlog int) error
func Accept(fd Socket) (Socket, Sockaddr, error)
func Getsockname(fd Socket) (Sockaddr, error)
func Getpeername(fd Socket) (Sockaddr, error)
func Setsockopt(fd Socket, level, opt int, value uintptr, vallen uint32) error
func Getsockopt(fd Socket, level, opt int, value uintptr, vallen *uint32) error
func Recvfrom(fd Socket, p []byte, flags int) (n int, from Sockaddr, err error)
func Sendto(fd Socket, p []byte, flags int, to Sockaddr) (n int, err error)
func Recv(fd Socket, p []byte, flags int) (n int, err error)
func Send(fd Socket, p []byte, flags int) (n int, err error)
func Recvmsg(fd Socket, p, oob []byte, flags int) (n, oobn int, recvflags int, from Sockaddr, err error)
func Sendmsg(fd Socket, p, oob []byte, to Sockaddr, flags int) (n int, err error)
```

### Memory Management

```go
const (
    // Memory protection constants
    PROT_NONE  = 0x00
    PROT_READ  = 0x01
    PROT_WRITE = 0x02
    PROT_EXEC  = 0x04
    
    // Memory mapping flags
    MAP_SHARED    = 0x01
    MAP_PRIVATE   = 0x02
    MAP_FIXED     = 0x10
    MAP_ANONYMOUS = 0x20
)

type MemoryAddress uintptr

func Mmap(fd FileHandle, offset int64, length int, prot, flags int) (MemoryAddress, error)
func Munmap(addr MemoryAddress, length int) error
func Mprotect(addr MemoryAddress, length int, prot int) error
func Madvise(addr MemoryAddress, length int, advice int) error
func Msync(addr MemoryAddress, length int, flags int) error
func Mlock(addr MemoryAddress, length int) error
func Munlock(addr MemoryAddress, length int) error
```

### System Information

```go
type Utsname struct {
    Sysname    string
    Nodename   string
    Release    string
    Version    string
    Machine    string
    Domainname string
}

type Sysinfo struct {
    Uptime    int64
    Loads     [3]uint64
    Totalram  uint64
    Freeram   uint64
    Sharedram uint64
    Bufferram uint64
    Totalswap uint64
    Freeswap  uint64
    Procs     uint16
    Pad       uint16
    Totalhigh uint64
    Freehigh  uint64
    Unit      uint32
}

func Uname() (*Utsname, error)
func Sysinfo() (*Sysinfo, error)
func Gethostname() (string, error)
func Sethostname(name string) error
func Getdomainname() (string, error)
func Setdomainname(name string) error
func Gettimeofday(tv *Timeval) error
func Settimeofday(tv *Timeval) error
```

### IPC Mechanisms

```go
// Shared memory
func Shmget(key int, size int, flag int) (id int, err error)
func Shmat(id int, addr uintptr, flag int) (ret uintptr, err error)
func Shmdt(addr uintptr) error
func Shmctl(id int, cmd int, buf *ShmidDS) error

type ShmidDS struct {
    // Platform-specific fields
}

// Message queues
func Msgget(key int, flag int) (id int, err error)
func Msgsnd(id int, msgp *MsgBuf, msgsz int, flag int) error
func Msgrcv(id int, msgp *MsgBuf, msgsz int, mtype int64, flag int) (int, error)
func Msgctl(id int, cmd int, buf *MsqidDS) error

type MsgBuf struct {
    Mtype int64
    Mtext []byte
}

type MsqidDS struct {
    // Platform-specific fields
}

// Semaphores
func Semget(key int, nsems int, flag int) (id int, err error)
func Semop(id int, ops []Sembuf) error
func Semctl(id int, num int, cmd int, arg uintptr) (int, error)

type Sembuf struct {
    Sem   uint16
    Op    int16
    Flag  int16
}
```

## Platform-Specific Extensions

### Linux-Specific

```go
// Epoll for efficient I/O multiplexing
func EpollCreate1(flag int) (fd FileHandle, err error)
func EpollCtl(epfd FileHandle, op, fd int, event *EpollEvent) error
func EpollWait(epfd FileHandle, events []EpollEvent, timeout int) (n int, err error)

type EpollEvent struct {
    Events uint32
    Fd     int32
    Pad    int32
}

// Inotify for file system event monitoring
func InotifyInit1(flags int) (fd FileHandle, err error)
func InotifyAddWatch(fd FileHandle, pathname string, mask uint32) (int, error)
func InotifyRmWatch(fd FileHandle, wd int) error

// Netlink sockets for kernel communication
func NetlinkSocket(proto int) (fd FileHandle, err error)
func NetlinkSendMessage(fd FileHandle, msg *NetlinkMessage) error
func NetlinkRecvMessage(fd FileHandle) (*NetlinkMessage, error)

type NetlinkMessage struct {
    Header NetlinkMessageHeader
    Data   []byte
}

type NetlinkMessageHeader struct {
    Len   uint32
    Type  uint16
    Flags uint16
    Seq   uint32
    Pid   uint32
}
```

### Windows-Specific

```go
// Windows file and registry functions
func CreateFile(name string, access, share uint32, sa *SecurityAttributes, disposition, attrs, templatefile uint32) (handle FileHandle, err error)
func DeviceIoControl(handle FileHandle, ioControlCode uint32, inBuffer []byte, outBuffer []byte) (err error)
func GetOverlappedResult(handle FileHandle, overlapped *Overlapped, transferred *uint32, wait bool) (err error)

type Overlapped struct {
    Internal     uintptr
    InternalHigh uintptr
    Offset       uint32
    OffsetHigh   uint32
    HEvent       Handle
}

type SecurityAttributes struct {
    Length             uint32
    SecurityDescriptor uintptr
    InheritHandle      bool
}

// Registry operations
func RegOpenKeyEx(key Handle, subkey string, options, access uint32) (result Handle, err error)
func RegCloseKey(key Handle) error
func RegCreateKeyEx(key Handle, subkey string, reserved, class uint32, options, access uint32, sa *SecurityAttributes) (newkey Handle, disposition uint32, err error)
func RegSetValueEx(key Handle, valueName string, reserved, valtype uint32, buf []byte) error
func RegQueryValueEx(key Handle, valueName string) (valueType uint32, data []byte, err error)
func RegEnumKeyEx(key Handle, index uint32) (name string, err error)
```

### Darwin-Specific

```go
// MacOS-specific system calls
func Sysctl(name string) (string, error)
func SysctlUint32(name string) (uint32, error)
func SysctlUint64(name string) (uint64, error)

// KQueue for event notification
func Kqueue() (fd FileHandle, err error)
func Kevent(kq FileHandle, changes, events []Kevent, timeout *Timespec) (n int, err error)

type Kevent struct {
    Ident  uint64
    Filter int16
    Flags  uint16
    Fflags uint32
    Data   int64
    Udata  uintptr
}
```

## GenZ Style Extensions

```go
// No-frills, direct OS access
func NoCap(fn func() error) error

// Yeet-based process termination
func YeetProcess(pid ProcessID, reason string) error

// System state validation
func VibeCheckSystem() (healthy bool, issues []string)

// Resource utilization tracker
func FlexResourceUsage() (*Rusage, error)

// Suspiciously efficient I/O multiplexing
func SusIOPoll(fds []FileHandle, timeout time.Duration) (readyFds []FileHandle, err error)
```

## Usage Example

```go
// File operations example
fd, err := sys_core.Open("/tmp/test.txt", sys_core.O_RDWR|sys_core.O_CREAT, sys_core.S_DEFAULT_FILE)
if err != nil {
    vibez.spill("Open error:", err)
    return
}
defer sys_core.Close(fd)

data := []byte("Hello, syscall interface!")
n, err := sys_core.Write(fd, data)
if err != nil {
    vibez.spill("Write error:", err)
    return
}
vibez.spill("Wrote", n, "bytes")

// Seek to beginning of file
_, err = sys_core.Seek(fd, 0, 0)
if err != nil {
    vibez.spill("Seek error:", err)
    return
}

// Read the content back
buf := make([]byte, 100)
n, err = sys_core.Read(fd, buf)
if err != nil {
    vibez.spill("Read error:", err)
    return
}
vibez.spill("Read", n, "bytes:", string(buf[:n]))

// Get file information
info, err := sys_core.Fstat(fd)
if err != nil {
    vibez.spill("Fstat error:", err)
    return
}
vibez.spill("File size:", info.Size, "bytes")
vibez.spill("File permissions:", info.Mode&0777)

// Process operations example
pid := sys_core.Getpid()
ppid := sys_core.Getppid()
vibez.spill("Current process ID:", pid)
vibez.spill("Parent process ID:", ppid)

// Get process info
procInfo, err := sys_core.GetProcessInfo(pid)
if err != nil {
    vibez.spill("GetProcessInfo error:", err)
} else {
    vibez.spill("Process command:", procInfo.Comm)
    vibez.spill("Process current directory:", procInfo.CWD)
}

// Get resource usage
var rusage sys_core.Rusage
err = sys_core.Getrusage(0, &rusage) // 0 means current process
if err != nil {
    vibez.spill("Getrusage error:", err)
} else {
    vibez.spill("User time:", rusage.Utime.Sec, "seconds,", rusage.Utime.Nsec, "nanoseconds")
    vibez.spill("System time:", rusage.Stime.Sec, "seconds,", rusage.Stime.Nsec, "nanoseconds")
    vibez.spill("Max RSS:", rusage.Maxrss, "KB")
}

// Network operations example
sock, err := sys_core.Socket(sys_core.AF_INET, sys_core.SOCK_STREAM, 0)
if err != nil {
    vibez.spill("Socket creation error:", err)
    return
}
defer sys_core.Close(FileHandle(sock))

// Enable address reuse
err = sys_core.Setsockopt(sock, sys_core.SOL_SOCKET, sys_core.SO_REUSEADDR, 1, 4)
if err != nil {
    vibez.spill("Setsockopt error:", err)
    return
}

// Bind to a port
addr := sys_core.SockaddrInet4{Port: 8080}
sys_core.Bind(sock, &addr)

// Listen for connections
err = sys_core.Listen(sock, 5)
if err != nil {
    vibez.spill("Listen error:", err)
    return
}
vibez.spill("Listening on port 8080...")

// Accept a connection (would block until a client connects)
// client, clientAddr, err := sys_core.Accept(sock)

// Memory mapping example
fd, err = sys_core.Open("/tmp/mmap_example.txt", sys_core.O_RDWR|sys_core.O_CREAT, sys_core.S_DEFAULT_FILE)
if err != nil {
    vibez.spill("Open error:", err)
    return
}
defer sys_core.Close(fd)

// Extend the file to 1MB
err = sys_core.Ftruncate(fd, 1024*1024)
if err != nil {
    vibez.spill("Ftruncate error:", err)
    return
}

// Map the file into memory
addr, err := sys_core.Mmap(fd, 0, 1024*1024, sys_core.PROT_READ|sys_core.PROT_WRITE, sys_core.MAP_SHARED)
if err != nil {
    vibez.spill("Mmap error:", err)
    return
}
defer sys_core.Munmap(addr, 1024*1024)

// Write directly to the mapped memory (simplified example)
// In reality, you would use unsafe.Pointer to access the memory

// System information example
uname, err := sys_core.Uname()
if err != nil {
    vibez.spill("Uname error:", err)
    return
}
vibez.spill("System name:", uname.Sysname)
vibez.spill("Node name:", uname.Nodename)
vibez.spill("Release:", uname.Release)
vibez.spill("Version:", uname.Version)
vibez.spill("Machine:", uname.Machine)

// Using GenZ extensions
healthy, issues := sys_core.VibeCheckSystem()
if !healthy {
    vibez.spill("System vibe check failed with issues:", issues)
} else {
    vibez.spill("System vibe check passed!")
}

// Resource usage with GenZ style
rusage, err = sys_core.FlexResourceUsage()
if err != nil {
    vibez.spill("Error flexing resource usage:", err)
} else {
    vibez.spill("Memory usage flex:", rusage.Maxrss, "KB")
}

// No-frills direct system call
err = sys_core.NoCap(func() error {
    return sys_core.Mkdir("/tmp/nocap_dir", sys_core.S_DEFAULT_DIR)
})
if err != nil {
    vibez.spill("NoCap mkdir error:", err)
} else {
    vibez.spill("Directory created with NoCap!")
}
```

## Implementation Guidelines
1. Ensure consistent behavior across different operating systems
2. Provide clear error messages with specific error codes
3. Include thorough documentation for each system call
4. Implement memory-safe wrappers around raw system calls
5. Support both synchronous and asynchronous system call patterns
6. Optimize for performance in high-frequency system call paths
7. Handle platform-specific differences transparently when possible
8. Provide fallbacks for unsupported system calls on specific platforms