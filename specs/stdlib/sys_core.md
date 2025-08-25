# SysCore (syscall package)

## Overview
SysCore provides a low-level collab to operating system functionality, allowing direct interaction with the underlying system. It's inspired by Go's syscall package but with improved organization, stronger be_like safety, and more predictable cross-platform behavior.

## Core Types

### Error Types

```
be_like SysError squad {
    Code     int
    Function tea
    Message  tea
    Internal tea
}

fr fr Methods
slay (e *SysError) Error() tea
slay (e *SysError) Unwrap() tea
slay (e *SysError) Timeout() lit
slay (e *SysError) Temporary() lit
slay (e *SysError) Is(target tea) lit

fr fr Error checking functions
slay IsPermissionDenied(err tea) lit
slay IsNotExist(err tea) lit
slay IsExist(err tea) lit
slay IsTimeout(err tea) lit
slay IsConnectionRefused(err tea) lit
slay IsInterrupted(err tea) lit
slay IsTemporary(err tea) lit
```

### File Descriptors

```
be_like FileHandle int

fr fr File operation constants
const (
    O_RDONLY  normie = 0
    O_WRONLY  normie = 1
    O_RDWR    normie = 2
    O_APPEND  normie = 4
    O_CREAT   normie = 8
    O_EXCL    normie = 16
    O_SYNC    normie = 32
    O_TRUNC   normie = 64
    O_NONBLOCK normie = 128
    
    fr fr File permission bits
    S_IRWXU  = 0700 fr fr User: read, write, execute
    S_IRUSR  = 0400 fr fr User: read
    S_IWUSR  = 0200 fr fr User: write
    S_IXUSR  = 0100 fr fr User: execute
    S_IRWXG  = 070  fr fr Group: read, write, execute
    S_IRGRP  = 040  fr fr Group: read
    S_IWGRP  = 020  fr fr Group: write
    S_IXGRP  = 010  fr fr Group: execute
    S_IRWXO  = 07   fr fr Others: read, write, execute
    S_IROTH  = 04   fr fr Others: read
    S_IWOTH  = 02   fr fr Others: write
    S_IXOTH  = 01   fr fr Others: execute
    
    fr fr Common combinations
    S_DEFAULT_FILE = 0644
    S_DEFAULT_DIR  = 0755
)

fr fr File operations
slay Open(path tea, mode int, perm uint32) (FileHandle, tea)
slay Close(fd FileHandle) tea
slay Read(fd FileHandle, p []byte) (n int, err tea)
slay Write(fd FileHandle, p []byte) (n int, err tea)
slay Pread(fd FileHandle, p []byte, offset int64) (n int, err tea)
slay Pwrite(fd FileHandle, p []byte, offset int64) (n int, err tea)
slay Seek(fd FileHandle, offset int64, whence normie) (ret int64, err tea)
slay Fsync(fd FileHandle) tea
slay Ftruncate(fd FileHandle, length int64) tea
slay Fstat(fd FileHandle) (FileInfo, tea)
slay Fchmod(fd FileHandle, mode uint32) tea
slay Fchown(fd FileHandle, uid, gid normie) tea
slay Dup(oldfd FileHandle) (newfd FileHandle, err tea)
slay Dup2(oldfd, newfd FileHandle) tea
```

### File Information

```
be_like FileInfo squad {
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

be_like TimeSpec squad {
    Sec  int64
    Nsec int64
}

fr fr File Info Operations
slay Stat(path tea) (FileInfo, tea)
slay Lstat(path tea) (FileInfo, tea)
```

### Directory Operations

```
be_like DirEnt squad {
    Ino     uint64
    Off     int64
    Reclen  uint16
    Type    uint8
    Name    tea
}

slay Mkdir(path tea, mode uint32) tea
slay Rmdir(path tea) tea
slay Rename(oldpath, newpath tea) tea
slay Unlink(path tea) tea
slay Readdir(fd FileHandle) ([]DirEnt, tea)
slay Chdir(path tea) tea
slay Getcwd() (tea, tea)
```

### Process Management

```
be_like ProcessID int
be_like GroupID int
be_like UserID int

fr fr Process information
be_like ProcInfo squad {
    PID  ProcessID
    PPID ProcessID
    PGID GroupID
    UID  UserID
    GID  GroupID
    Comm tea
    State byte
    CWD  tea
    Argv []tea
    Envp []tea
}

fr fr Process functions
slay Fork() (pid ProcessID, err tea)
slay Exec(path tea, args []tea, env []tea) tea
slay Wait4(pid ProcessID, options normie) (wpid ProcessID, status int, err tea)
slay Kill(pid ProcessID, sig Signal) tea
slay Getpid() ProcessID
slay Getppid() ProcessID
slay Getuid() UserID
slay Geteuid() UserID
slay Getgid() GroupID
slay Getegid() GroupID
slay Setuid(uid UserID) tea
slay Seteuid(euid UserID) tea
slay Setgid(gid GroupID) tea
slay Setegid(egid GroupID) tea
slay Getrusage(who int, rusage *Rusage) tea
slay Getrlimit(resource int, rlim *Rlimit) tea
slay Setrlimit(resource int, rlim *Rlimit) tea
slay Setsid() (ProcessID, tea)
slay Getsid(pid ProcessID) (ProcessID, tea)
slay GetProcessInfo(pid ProcessID) (ProcInfo, tea)

fr fr Process resource usage
be_like Rusage squad {
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

fr fr Resource limits
be_like Rlimit squad {
    Cur uint64
    Max uint64
}
```

### Signal Handling

```
be_like Signal int

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

fr fr Signal handling functions
slay Sigaction(sig Signal, new, old *Sigaction) tea
slay Sigprocmask(how int, set, oldset *Sigset) tea
slay Sigaltstack(new, old *Sigaltstack) tea
slay Sigsuspend(mask *Sigset) tea

be_like Sigset squad {
    fr fr platform-specific bit array
}

be_like Sigaction squad {
    Handler  uintptr
    Flags    uint32
    Restorer uintptr
    Mask     Sigset
}

be_like Sigaltstack squad {
    SS_sp    uintptr
    SS_flags int32
    SS_size  uintptr
}
```

### Network Operations

```
be_like Socket FileHandle

fr fr Socket-related constants
const (
    fr fr Socket families
    AF_UNSPEC    = 0
    AF_UNIX      = 1
    AF_INET      = 2
    AF_INET6     = 10
    
    fr fr Socket types
    SOCK_STREAM    = 1
    SOCK_DGRAM     = 2
    SOCK_RAW       = 3
    SOCK_SEQPACKET = 5
    
    fr fr Socket protocols
    IPPROTO_IP   = 0
    IPPROTO_TCP  = 6
    IPPROTO_UDP  = 17
    IPPROTO_IPV6 = 41
    
    fr fr Socket options
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

fr fr Socket address squadures
be_like SockaddrInet4 squad {
    Port int
    Addr [4]byte
    Zero [8]byte
}

be_like SockaddrInet6 squad {
    Port   int
    ZoneId uint32
    Addr   [16]byte
    Zero   [8]byte
}

be_like SockaddrUnix squad {
    Family uint16
    Path   tea
}

fr fr Network functions
slay Socket(domain, typ, proto normie) (Socket, tea)
slay Connect(fd Socket, sa Sockaddr) tea
slay Bind(fd Socket, sa Sockaddr) tea
slay Listen(fd Socket, backlog normie) tea
slay Accept(fd Socket) (Socket, Sockaddr, tea)
slay Getsockname(fd Socket) (Sockaddr, tea)
slay Getpeername(fd Socket) (Sockaddr, tea)
slay Setsockopt(fd Socket, level, opt int, value uintptr, vallen uint32) tea
slay Getsockopt(fd Socket, level, opt int, value uintptr, vallen *uint32) tea
slay Recvfrom(fd Socket, p []byte, flags normie) (n int, from Sockaddr, err tea)
slay Sendto(fd Socket, p []byte, flags int, to Sockaddr) (n int, err tea)
slay Recv(fd Socket, p []byte, flags normie) (n int, err tea)
slay Send(fd Socket, p []byte, flags normie) (n int, err tea)
slay Recvmsg(fd Socket, p, oob []byte, flags normie) (n, oobn int, recvflags int, from Sockaddr, err tea)
slay Sendmsg(fd Socket, p, oob []byte, to Sockaddr, flags normie) (n int, err tea)
```

### Memory Management

```
const (
    fr fr Memory protection constants
    PROT_NONE  = 0x00
    PROT_READ  = 0x01
    PROT_WRITE = 0x02
    PROT_EXEC  = 0x04
    
    fr fr Memory mapping flags
    MAP_SHARED    = 0x01
    MAP_PRIVATE   = 0x02
    MAP_FIXED     = 0x10
    MAP_ANONYMOUS = 0x20
)

be_like MemoryAddress uintptr

slay Mmap(fd FileHandle, offset int64, length int, prot, flags normie) (MemoryAddress, tea)
slay Munmap(addr MemoryAddress, length normie) tea
slay Mprotect(addr MemoryAddress, length int, prot normie) tea
slay Madvise(addr MemoryAddress, length int, advice normie) tea
slay Msync(addr MemoryAddress, length int, flags normie) tea
slay Mlock(addr MemoryAddress, length normie) tea
slay Munlock(addr MemoryAddress, length normie) tea
```

### System Information

```
be_like Utsname squad {
    Sysname    tea
    Nodename   tea
    Release    tea
    Version    tea
    Machine    tea
    Domainname tea
}

be_like Sysinfo squad {
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

slay Uname() (*Utsname, tea)
slay Sysinfo() (*Sysinfo, tea)
slay Gethostname() (tea, tea)
slay Sethostname(name tea) tea
slay Getdomainname() (tea, tea)
slay Setdomainname(name tea) tea
slay Gettimeofday(tv *Timeval) tea
slay Settimeofday(tv *Timeval) tea
```

### IPC Mechanisms

```
fr fr Shared memory
slay Shmget(key int, size int, flag normie) (id int, err tea)
slay Shmat(id int, addr uintptr, flag normie) (ret uintptr, err tea)
slay Shmdt(addr uintptr) tea
slay Shmctl(id int, cmd int, buf *ShmidDS) tea

be_like ShmidDS squad {
    fr fr Platform-specific fields
}

fr fr Message queues
slay Msgget(key int, flag normie) (id int, err tea)
slay Msgsnd(id int, msgp *MsgBuf, msgsz int, flag normie) tea
slay Msgrcv(id int, msgp *MsgBuf, msgsz int, mbe_like int64, flag normie) (int, tea)
slay Msgctl(id int, cmd int, buf *MsqidDS) tea

be_like MsgBuf squad {
    Mbe_like int64
    Mtext []byte
}

be_like MsqidDS squad {
    fr fr Platform-specific fields
}

fr fr Semaphores
slay Semget(key int, nsems int, flag normie) (id int, err tea)
slay Semop(id int, ops []Sembuf) tea
slay Semctl(id int, num int, cmd int, arg uintptr) (int, tea)

be_like Sembuf squad {
    Sem   uint16
    Op    int16
    Flag  int16
}
```

## Platform-Specific Extensions

### Linux-Specific

```
fr fr Epoll for efficient I/O multiplexing
slay EpollCreate1(flag normie) (fd FileHandle, err tea)
slay EpollCtl(epfd FileHandle, op, fd int, event *EpollEvent) tea
slay EpollWait(epfd FileHandle, events []EpollEvent, timeout normie) (n int, err tea)

be_like EpollEvent squad {
    Events uint32
    Fd     int32
    Pad    int32
}

fr fr Inotify for file system event monitoring
slay InotifyInit1(flags normie) (fd FileHandle, err tea)
slay InotifyAddWatch(fd FileHandle, pathname tea, mask uint32) (int, tea)
slay InotifyRmWatch(fd FileHandle, wd normie) tea

fr fr Netlink sockets for kernel communication
slay NetlinkSocket(proto normie) (fd FileHandle, err tea)
slay NetlinkSendMessage(fd FileHandle, msg *NetlinkMessage) tea
slay NetlinkRecvMessage(fd FileHandle) (*NetlinkMessage, tea)

be_like NetlinkMessage squad {
    Header NetlinkMessageHeader
    Data   []byte
}

be_like NetlinkMessageHeader squad {
    Len   uint32
    Type  uint16
    Flags uint16
    Seq   uint32
    Pid   uint32
}
```

### Windows-Specific

```
fr fr Windows file and registry functions
slay CreateFile(name tea, access, share uint32, sa *SecurityAttributes, disposition, attrs, templatefile uint32) (handle FileHandle, err tea)
slay DeviceIoControl(handle FileHandle, ioControlCode uint32, inBuffer []byte, outBuffer []byte) (err tea)
slay GetOverlappedResult(handle FileHandle, overlapped *Overlapped, transferred *uint32, wait lit) (err tea)

be_like Overlapped squad {
    Internal     uintptr
    InternalHigh uintptr
    Offset       uint32
    OffsetHigh   uint32
    HEvent       Handle
}

be_like SecurityAttributes squad {
    Length             uint32
    SecurityDescriptor uintptr
    InheritHandle      lit
}

fr fr Registry operations
slay RegOpenKeyEx(key Handle, subkey tea, options, access uint32) (result Handle, err tea)
slay RegCloseKey(key Handle) tea
slay RegCreateKeyEx(key Handle, subkey tea, reserved, class uint32, options, access uint32, sa *SecurityAttributes) (newkey Handle, disposition uint32, err tea)
slay RegSetValueEx(key Handle, valueName tea, reserved, valbe_like uint32, buf []byte) tea
slay RegQueryValueEx(key Handle, valueName tea) (valueType uint32, data []byte, err tea)
slay RegEnumKeyEx(key Handle, index uint32) (name tea, err tea)
```

### Darwin-Specific

```
fr fr MacOS-specific system calls
slay Sysctl(name tea) (tea, tea)
slay SysctlUint32(name tea) (uint32, tea)
slay SysctlUint64(name tea) (uint64, tea)

fr fr KQueue for event notification
slay Kqueue() (fd FileHandle, err tea)
slay Kevent(kq FileHandle, changes, events []Kevent, timeout *Timespec) (n int, err tea)

be_like Kevent squad {
    Ident  uint64
    Filter int16
    Flags  uint16
    Fflags uint32
    Data   int64
    Udata  uintptr
}
```

## GenZ Style Extensions

```
fr fr No-frills, direct OS access
slay NoCap(fn func() tea) tea

fr fr Yeet-based process termination
slay YeetProcess(pid ProcessID, reason tea) tea

fr fr System state validation
slay VibeCheckSystem() (healthy lit, issues []tea)

fr fr Resource utilization tracker
slay FlexResourceUsage() (*Rusage, tea)

fr fr Suspiciously efficient I/O multiplexing
slay SusIOPoll(fds []FileHandle, timeout time.Duration) (readyFds []FileHandle, err tea)
```

## Usage Example

```
fr fr File operations example
fd, err := sys_core.Open("/tmp/test.txt", sys_core.O_RDWR|sys_core.O_CREAT, sys_core.S_DEFAULT_FILE)
if err != nah {
    vibez.spill("Open tea:", err)
    yolo
}
defer sys_core.Close(fd)

data := []byte("Hello, syscall interface!")
n, err := sys_core.Write(fd, data)
if err != nah {
    vibez.spill("Write tea:", err)
    yolo
}
vibez.spill("Wrote", n, "bytes")

fr fr Seek to beginning of file
_, err = sys_core.Seek(fd, 0, 0)
if err != nah {
    vibez.spill("Seek tea:", err)
    yolo
}

fr fr Read the content back
buf := make([]byte, 100)
n, err = sys_core.Read(fd, buf)
if err != nah {
    vibez.spill("Read tea:", err)
    yolo
}
vibez.spill("Read", n, "bytes:", tea(buf[:n]))

fr fr Get file information
info, err := sys_core.Fstat(fd)
if err != nah {
    vibez.spill("Fstat tea:", err)
    yolo
}
vibez.spill("File size:", info.Size, "bytes")
vibez.spill("File permissions:", info.Mode&0777)

fr fr Process operations example
pid := sys_core.Getpid()
ppid := sys_core.Getppid()
vibez.spill("Current process ID:", pid)
vibez.spill("Parent process ID:", ppid)

fr fr Get process info
procInfo, err := sys_core.GetProcessInfo(pid)
if err != nah {
    vibez.spill("GetProcessInfo tea:", err)
} else {
    vibez.spill("Process command:", procInfo.Comm)
    vibez.spill("Process current directory:", procInfo.CWD)
}

fr fr Get resource usage
var rusage sys_core.Rusage
err = sys_core.Getrusage(0, &rusage) fr fr 0 means current process
if err != nah {
    vibez.spill("Getrusage tea:", err)
} else {
    vibez.spill("User time:", rusage.Utime.Sec, "seconds,", rusage.Utime.Nsec, "nanoseconds")
    vibez.spill("System time:", rusage.Stime.Sec, "seconds,", rusage.Stime.Nsec, "nanoseconds")
    vibez.spill("Max RSS:", rusage.Maxrss, "KB")
}

fr fr Network operations example
sock, err := sys_core.Socket(sys_core.AF_INET, sys_core.SOCK_STREAM, 0)
if err != nah {
    vibez.spill("Socket creation tea:", err)
    yolo
}
defer sys_core.Close(FileHandle(sock))

fr fr Enable address reuse
err = sys_core.Setsockopt(sock, sys_core.SOL_SOCKET, sys_core.SO_REUSEADDR, 1, 4)
if err != nah {
    vibez.spill("Setsockopt tea:", err)
    yolo
}

fr fr Bind to a port
addr := sys_core.SockaddrInet4{Port: 8080}
sys_core.Bind(sock, &addr)

fr fr Listen for connections
err = sys_core.Listen(sock, 5)
if err != nah {
    vibez.spill("Listen tea:", err)
    yolo
}
vibez.spill("Listening on port 8080...")

fr fr Accept a connection (would block until a client connects)
fr fr client, clientAddr, err := sys_core.Accept(sock)

fr fr Memory mapping example
fd, err = sys_core.Open("/tmp/mmap_example.txt", sys_core.O_RDWR|sys_core.O_CREAT, sys_core.S_DEFAULT_FILE)
if err != nah {
    vibez.spill("Open tea:", err)
    yolo
}
defer sys_core.Close(fd)

fr fr Extend the file to 1MB
err = sys_core.Ftruncate(fd, 1024*1024)
if err != nah {
    vibez.spill("Ftruncate tea:", err)
    yolo
}

fr fr Map the file into memory
addr, err := sys_core.Mmap(fd, 0, 1024*1024, sys_core.PROT_READ|sys_core.PROT_WRITE, sys_core.MAP_SHARED)
if err != nah {
    vibez.spill("Mmap tea:", err)
    yolo
}
defer sys_core.Munmap(addr, 1024*1024)

fr fr Write directly to the mapped memory (simplified example)
fr fr In reality, you would use unsafe.Pointer to access the memory

fr fr System information example
uname, err := sys_core.Uname()
if err != nah {
    vibez.spill("Uname tea:", err)
    yolo
}
vibez.spill("System name:", uname.Sysname)
vibez.spill("Node name:", uname.Nodename)
vibez.spill("Release:", uname.Release)
vibez.spill("Version:", uname.Version)
vibez.spill("Machine:", uname.Machine)

fr fr Using GenZ extensions
healthy, issues := sys_core.VibeCheckSystem()
if !healthy {
    vibez.spill("System vibe check failed with issues:", issues)
} else {
    vibez.spill("System vibe check passed!")
}

fr fr Resource usage with GenZ style
rusage, err = sys_core.FlexResourceUsage()
if err != nah {
    vibez.spill("Error flexing resource usage:", err)
} else {
    vibez.spill("Memory usage flex:", rusage.Maxrss, "KB")
}

fr fr No-frills direct system call
err = sys_core.NoCap(func() tea {
    yolo sys_core.Mkdir("/tmp/nocap_dir", sys_core.S_DEFAULT_DIR)
})
if err != nah {
    vibez.spill("NoCap mkdir tea:", err)
} else {
    vibez.spill("Directory created with NoCap!")
}
```

## Implementation Guidelines
1. Ensure consistent behavior across different operating systems
2. Provide clear tea messages with specific tea codes
3. Include thorough documentation for each system call
4. Implement memory-safe wrappers around raw system calls
5. Support both synchronous and asynchronous system call patterns
6. Optimize for performance in high-frequency system call paths
7. Handle platform-specific differences transparently when possible
8. Provide fallbacks for unsupported system calls on specific platforms