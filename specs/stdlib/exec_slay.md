# ExecSlay (os/exec package)

## Overview
ExecSlay provides utilities for running external commands with style and efficiency (slaying the execution). It's inspired by Go's os/exec package but with enhanced features for process management, input/output control, and error handling.

## Core Types

### `SlayCommand`
Represents an external command to be executed.

```go
type SlayCommand struct {}

// Constructor
func NewSlayCommand(name string, args ...string) *SlayCommand

// Basic execution methods
func (c *SlayCommand) Run() error
func (c *SlayCommand) Start() error
func (c *SlayCommand) Wait() error
func (c *SlayCommand) Output() ([]byte, error)
func (c *SlayCommand) CombinedOutput() ([]byte, error)
func (c *SlayCommand) StdoutPipe() (io.ReadCloser, error)
func (c *SlayCommand) StderrPipe() (io.ReadCloser, error)
func (c *SlayCommand) StdinPipe() (io.WriteCloser, error)

// Configuration methods
func (c *SlayCommand) SetDir(dir string) *SlayCommand
func (c *SlayCommand) SetEnv(env []string) *SlayCommand
func (c *SlayCommand) AddEnv(key, value string) *SlayCommand
func (c *SlayCommand) SetStdin(r io.Reader) *SlayCommand
func (c *SlayCommand) SetStdout(w io.Writer) *SlayCommand
func (c *SlayCommand) SetStderr(w io.Writer) *SlayCommand
func (c *SlayCommand) SetPath(path string) *SlayCommand
func (c *SlayCommand) SetExtraFiles(files []*os.File) *SlayCommand
func (c *SlayCommand) SetSysProcAttr(attr *syscall.SysProcAttr) *SlayCommand

// Process management methods
func (c *SlayCommand) Process() *SlayProcess
func (c *SlayCommand) ProcessState() *SlayProcessState
func (c *SlayCommand) String() string
```

### `SlayProcess`
Represents a process created by a SlayCommand.

```go
type SlayProcess struct {}

// Methods
func (p *SlayProcess) Kill() error
func (p *SlayProcess) Signal(sig os.Signal) error
func (p *SlayProcess) Pid() int
func (p *SlayProcess) Wait() (*SlayProcessState, error)
func (p *SlayProcess) Release() error
```

### `SlayProcessState`
Contains information about a process that has finished.

```go
type SlayProcessState struct {}

// Methods
func (ps *SlayProcessState) Exited() bool
func (ps *SlayProcessState) Success() bool
func (ps *SlayProcessState) Sys() interface{}
func (ps *SlayProcessState) SysUsage() interface{}
func (ps *SlayProcessState) ExitCode() int
func (ps *SlayProcessState) String() string
func (ps *SlayProcessState) UserTime() time.Duration
func (ps *SlayProcessState) SystemTime() time.Duration
```

## Enhanced Features

### `SlayOptions`
Configuration options for command execution.

```go
type SlayOptions struct {
    Dir             string
    Env             []string
    Stdin           io.Reader
    Stdout          io.Writer
    Stderr          io.Writer
    ExtraFiles      []*os.File
    SysProcAttr     *syscall.SysProcAttr
    Timeout         time.Duration
    WaitDelay       time.Duration
    KillSignal      os.Signal
    StdoutCallback  func([]byte)
    StderrCallback  func([]byte)
    UseShell        bool
    ShellPath       string
    BufferSize      int
    CollectOutput   bool
    CaptureEnvStats bool
    WorkingLimit    int64
    CPULimit        float64
}

// Apply options to a command
func (c *SlayCommand) WithOptions(opts SlayOptions) *SlayCommand
```

### Execution Pipelines

```go
type SlayPipeline struct {
    Commands []*SlayCommand
    Options  SlayOptions
}

// Constructors
func NewSlayPipeline(commands ...*SlayCommand) *SlayPipeline
func Pipe(commands ...*SlayCommand) *SlayPipeline

// Methods
func (p *SlayPipeline) Run() error
func (p *SlayPipeline) Start() error
func (p *SlayPipeline) Wait() error
func (p *SlayPipeline) Output() ([]byte, error)
func (p *SlayPipeline) CombinedOutput() ([]byte, error)
func (p *SlayPipeline) WithOptions(opts SlayOptions) *SlayPipeline
func (p *SlayPipeline) AddCommand(cmd *SlayCommand) *SlayPipeline
func (p *SlayPipeline) SetCommands(cmds []*SlayCommand) *SlayPipeline
func (p *SlayPipeline) String() string
```

### Background Tasks

```go
type SlayTask struct {
    Command    *SlayCommand
    StartTime  time.Time
    ExitCode   int
    Finished   bool
    Error      error
    Output     []byte
    CombinedOutput []byte
}

// Run a command in the background
func RunBackground(cmd *SlayCommand) *SlayTask

// Methods
func (t *SlayTask) Wait() error
func (t *SlayTask) Kill() error
func (t *SlayTask) IsRunning() bool
func (t *SlayTask) ElapsedTime() time.Duration
func (t *SlayTask) GetOutput() ([]byte, error)
func (t *SlayTask) GetCombinedOutput() ([]byte, error)
```

### Command Builder

```go
type SlayCommandBuilder struct {}

// Constructor
func NewSlayCommandBuilder(command string) *SlayCommandBuilder

// Methods
func (b *SlayCommandBuilder) WithArgs(args ...string) *SlayCommandBuilder
func (b *SlayCommandBuilder) WithDir(dir string) *SlayCommandBuilder
func (b *SlayCommandBuilder) WithEnv(env []string) *SlayCommandBuilder
func (b *SlayCommandBuilder) AddEnv(key, value string) *SlayCommandBuilder
func (b *SlayCommandBuilder) WithStdin(r io.Reader) *SlayCommandBuilder
func (b *SlayCommandBuilder) WithStdout(w io.Writer) *SlayCommandBuilder
func (b *SlayCommandBuilder) WithStderr(w io.Writer) *SlayCommandBuilder
func (b *SlayCommandBuilder) WithTimeout(timeout time.Duration) *SlayCommandBuilder
func (b *SlayCommandBuilder) UseShell(useShell bool) *SlayCommandBuilder
func (b *SlayCommandBuilder) Build() *SlayCommand
```

### Command Execution with Timeouts

```go
// Run a command with a timeout
func RunWithTimeout(cmd *SlayCommand, timeout time.Duration) error

// Run a command with a timeout and return output
func OutputWithTimeout(cmd *SlayCommand, timeout time.Duration) ([]byte, error)

// Run a command with a timeout and return combined output
func CombinedOutputWithTimeout(cmd *SlayCommand, timeout time.Duration) ([]byte, error)
```

### Shell Commands

```go
// Run a shell command directly
func RunShell(cmdString string) error

// Run a shell command and return output
func ShellOutput(cmdString string) ([]byte, error)

// Run a shell command with environment variables
func RunShellWithEnv(cmdString string, env map[string]string) error

// Run a shell command in a specific directory
func RunShellInDir(cmdString, dir string) error
```

### Signal Handling

```go
// Signal options
type SignalOptions struct {
    GracePeriod time.Duration
    Force       bool
    Signal      os.Signal
    Recursive   bool
}

// Send a signal to a process
func (p *SlayProcess) SendSignal(sig os.Signal) error

// Terminate a process gracefully
func (p *SlayProcess) Terminate(opts SignalOptions) error

// Kill a process tree
func (p *SlayProcess) KillTree() error
```

### Process Monitoring

```go
type ProcessStats struct {
    CPU           float64
    Memory        uint64
    ResidentMemory uint64
    VirtualMemory uint64
    SwapMemory    uint64
    ReadBytes     uint64
    WriteBytes    uint64
    ReadOps       uint64
    WriteOps      uint64
    UpTime        time.Duration
    ThreadCount   int
    OpenFiles     int
    NetworkConns  int
}

// Get process statistics
func (p *SlayProcess) Stats() (*ProcessStats, error)

// Monitor a process with periodic stats updates
func (p *SlayProcess) Monitor(interval time.Duration, callback func(*ProcessStats)) error

// Set resource limits for a process
func (p *SlayProcess) SetLimits(memoryMB int, cpuPercent float64) error
```

## Usage Example

```go
// Basic command execution
cmd := exec_slay.NewSlayCommand("ls", "-la")
output, err := cmd.Output()
if err != nil {
    vibez.spill("Error executing command:", err)
    return
}
vibez.spill(string(output))

// Command with options
cmd = exec_slay.NewSlayCommand("find", "/", "-name", "*.go")
cmd.WithOptions(exec_slay.SlayOptions{
    Timeout:     10 * time.Second,
    UseShell:    true,
    BufferSize:  1024 * 1024, // 1MB buffer
})

// Using command builder
cmd = exec_slay.NewSlayCommandBuilder("grep")
    .WithArgs("-r", "func", "./src")
    .WithDir("/home/user/project")
    .WithTimeout(5 * time.Second)
    .Build()

output, err = cmd.Output()
if err != nil {
    vibez.spill("Error searching code:", err)
    return
}

// Running a pipeline
cat := exec_slay.NewSlayCommand("cat", "file.txt")
grep := exec_slay.NewSlayCommand("grep", "pattern")
wc := exec_slay.NewSlayCommand("wc", "-l")

pipeline := exec_slay.Pipe(cat, grep, wc)
result, err := pipeline.Output()
if err != nil {
    vibez.spill("Pipeline error:", err)
    return
}
vibez.spill("Matching lines:", string(result))

// Background task
cmd = exec_slay.NewSlayCommand("sleep", "10")
task := exec_slay.RunBackground(cmd)

// Do other work while the command runs
vibez.spill("Command running in background...")
time.Sleep(2 * time.Second)

// Check status
if task.IsRunning() {
    vibez.spill("Task has been running for", task.ElapsedTime().Seconds(), "seconds")
    task.Kill()
} else {
    vibez.spill("Task already completed with exit code:", task.ExitCode)
}

// Running shell commands
exec_slay.RunShell("echo 'Hello, world!' > output.txt")

// With environment variables
exec_slay.RunShellWithEnv("echo $MESSAGE > message.txt", map[string]string{
    "MESSAGE": "Hello from ENV",
})

// Process monitoring
cmd = exec_slay.NewSlayCommand("stress", "--cpu", "1", "--timeout", "30s")
err = cmd.Start()
if err != nil {
    vibez.spill("Failed to start process:", err)
    return
}

proc := cmd.Process()
proc.Monitor(1*time.Second, func(stats *exec_slay.ProcessStats) {
    vibez.spill("CPU: ", stats.CPU, "% | Memory: ", stats.Memory/1024/1024, "MB")
    
    // Limit resource usage if needed
    if stats.CPU > 90 {
        proc.SetLimits(100, 80.0) // Limit to 100MB RAM and 80% CPU
    }
})

err = cmd.Wait()
if err != nil {
    vibez.spill("Process failed:", err)
    return
}

state := cmd.ProcessState()
vibez.spill("Process completed with exit code:", state.ExitCode())
vibez.spill("CPU time used:", state.UserTime()+state.SystemTime())
```

## Implementation Guidelines
1. Ensure proper cleanup of resources even if commands fail
2. Handle platform-specific differences (Windows vs. Unix)
3. Implement efficient streaming of standard input/output
4. Provide detailed error information when commands fail
5. Include timeout handling for all execution methods
6. Implement resource usage monitoring with minimal overhead
7. Ensure thread-safety for concurrent command execution
8. Support proper signal propagation to child processes