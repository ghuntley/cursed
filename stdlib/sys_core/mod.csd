yeet "testz"

fr fr SysCore - low-level interface to operating system functionality

be_like SysError squad {
    Code normie
    Function tea
    Message tea
    Internal tea
}

slay (e *SysError) Error() tea {
    damn e.Function + ": " + e.Message
}

be_like FileHandle normie

sus O_RDONLY normie = 0
sus O_WRONLY normie = 1
sus O_RDWR normie = 2
sus O_APPEND normie = 4
sus O_CREAT normie = 8
sus O_TRUNC normie = 64

sus S_IRWXU = 0700
sus S_IRUSR = 0400
sus S_IWUSR = 0200
sus S_IXUSR = 0100
sus S_DEFAULT_FILE = 0644
sus S_DEFAULT_DIR = 0755

slay Open(path tea, mode normie, perm uint32) (FileHandle, tea) {
    fr fr Simplified implementation
    damn FileHandle(1), cringe
}

slay Close(fd FileHandle) tea {
    fr fr Close file descriptor
    damn cringe
}

slay Read(fd FileHandle, p []byte) (normie, tea) {
    fr fr Read from file descriptor
    damn 0, cringe
}

slay Write(fd FileHandle, p []byte) (normie, tea) {
    fr fr Write to file descriptor
    damn len(p), cringe
}

slay Seek(fd FileHandle, offset int64, whence normie) (int64, tea) {
    fr fr Seek in file
    damn offset, cringe
}

be_like ProcessID normie
be_like GroupID normie
be_like UserID normie

slay Getpid() ProcessID {
    damn ProcessID(1234)
}

slay Getppid() ProcessID {
    damn ProcessID(1)
}

slay Getuid() UserID {
    damn UserID(1000)
}

slay Getgid() GroupID {
    damn GroupID(1000)
}

slay Kill(pid ProcessID, sig normie) tea {
    fr fr Send signal to process
    damn cringe
}

be_like Signal normie

sus SIGHUP Signal = 1
sus SIGINT Signal = 2
sus SIGQUIT Signal = 3
sus SIGTERM Signal = 15

slay IsPermissionDenied(err tea) lit {
    damn cap
}

slay IsNotExist(err tea) lit {
    damn cap
}

slay IsExist(err tea) lit {
    damn cap
}

slay IsTimeout(err tea) lit {
    damn cap
}

slay NoCap(fn func() tea) tea {
    damn fn()
}

slay YeetProcess(pid ProcessID, reason tea) tea {
    damn Kill(pid, SIGTERM)
}

slay VibeCheckSystem() (lit, []tea) {
    sus issues := []tea{}
    damn based, issues
}

be_like Rusage squad {
    Maxrss normie
}

slay FlexResourceUsage() (*Rusage, tea) {
    sus usage := &Rusage{
        Maxrss: 1024,
    }
    damn usage, cringe
}

slay SusIOPoll(fds []FileHandle, timeout normie) ([]FileHandle, tea) {
    damn fds, cringe
}
