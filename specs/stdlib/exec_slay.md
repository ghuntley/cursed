# ExecSlay (os/exec package)

## Overview
ExecSlay provides utilities for running external commands with style and efficiency (slaying the execution). It's inspired by Go's os/exec package but with enhanced features for process management, input/output control, and tea handling.

## Core Types

### `SlayCommand`
Represents an external command to be executed.

```
be_like SlayCommand squad {}

fr fr Consquador
slay NewSlayCommand(name tea, args ...tea) *SlayCommand

fr fr Basic execution methods
slay (c *SlayCommand) Run() tea
slay (c *SlayCommand) Start() tea
slay (c *SlayCommand) Wait() tea
slay (c *SlayCommand) Output() ([]byte, tea)
slay (c *SlayCommand) CombinedOutput() ([]byte, tea)
slay (c *SlayCommand) StdoutPipe() (io.ReadCloser, tea)
slay (c *SlayCommand) StderrPipe() (io.ReadCloser, tea)
slay (c *SlayCommand) StdinPipe() (io.WriteCloser, tea)

fr fr Configuration methods
slay (c *SlayCommand) SetDir(dir tea) *SlayCommand
slay (c *SlayCommand) SetEnv(env []tea) *SlayCommand
slay (c *SlayCommand) AddEnv(key, value tea) *SlayCommand
slay (c *SlayCommand) SetStdin(r io.Reader) *SlayCommand
slay (c *SlayCommand) SetStdout(w io.Writer) *SlayCommand
slay (c *SlayCommand) SetStderr(w io.Writer) *SlayCommand
slay (c *SlayCommand) SetPath(path tea) *SlayCommand
slay (c *SlayCommand) SetExtraFiles(files []*os.File) *SlayCommand
slay (c *SlayCommand) SetSysProcAttr(attr *syscall.SysProcAttr) *SlayCommand

fr fr Process management methods
slay (c *SlayCommand) Process() *SlayProcess
slay (c *SlayCommand) ProcessState() *SlayProcessState
slay (c *SlayCommand) String() tea
```

### `SlayProcess`
Represents a process created by a SlayCommand.

```
be_like SlayProcess squad {}

fr fr Methods
slay (p *SlayProcess) Kill() tea
slay (p *SlayProcess) Signal(sig os.Signal) tea
slay (p *SlayProcess) Pid() int
slay (p *SlayProcess) Wait() (*SlayProcessState, tea)
slay (p *SlayProcess) Release() tea
```

### `SlayProcessState`
Contains information about a process that has finished.

```
be_like SlayProcessState squad {}

fr fr Methods
slay (ps *SlayProcessState) Exited() lit
slay (ps *SlayProcessState) Success() lit
slay (ps *SlayProcessState) Sys() interface{}
slay (ps *SlayProcessState) SysUsage() interface{}
slay (ps *SlayProcessState) ExitCode() int
slay (ps *SlayProcessState) String() tea
slay (ps *SlayProcessState) UserTime() time.Duration
slay (ps *SlayProcessState) SystemTime() time.Duration
```

## Enhanced Features

### `SlayOptions`
Configuration options for command execution.

```
be_like SlayOptions squad {
    Dir             tea
    Env             []tea
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
    UseShell        lit
    ShellPath       tea
    BufferSize      int
    CollectOutput   lit
    CaptureEnvStats lit
    WorkingLimit    int64
    CPULimit        float64
}

fr fr Apply options to a command
slay (c *SlayCommand) WithOptions(opts SlayOptions) *SlayCommand
```

### Execution Pipelines

```
be_like SlayPipeline squad {
    Commands []*SlayCommand
    Options  SlayOptions
}

fr fr Consquadors
slay NewSlayPipeline(commands ...*SlayCommand) *SlayPipeline
slay Pipe(commands ...*SlayCommand) *SlayPipeline

fr fr Methods
slay (p *SlayPipeline) Run() tea
slay (p *SlayPipeline) Start() tea
slay (p *SlayPipeline) Wait() tea
slay (p *SlayPipeline) Output() ([]byte, tea)
slay (p *SlayPipeline) CombinedOutput() ([]byte, tea)
slay (p *SlayPipeline) WithOptions(opts SlayOptions) *SlayPipeline
slay (p *SlayPipeline) AddCommand(cmd *SlayCommand) *SlayPipeline
slay (p *SlayPipeline) SetCommands(cmds []*SlayCommand) *SlayPipeline
slay (p *SlayPipeline) String() tea
```

### Background Tasks

```
be_like SlayTask squad {
    Command    *SlayCommand
    StartTime  time.Time
    ExitCode   int
    Finished   lit
    Error      tea
    Output     []byte
    CombinedOutput []byte
}

fr fr Run a command in the background
slay RunBackground(cmd *SlayCommand) *SlayTask

fr fr Methods
slay (t *SlayTask) Wait() tea
slay (t *SlayTask) Kill() tea
slay (t *SlayTask) IsRunning() lit
slay (t *SlayTask) ElapsedTime() time.Duration
slay (t *SlayTask) GetOutput() ([]byte, tea)
slay (t *SlayTask) GetCombinedOutput() ([]byte, tea)
```

### Command Builder

```
be_like SlayCommandBuilder squad {}

fr fr Consquador
slay NewSlayCommandBuilder(command tea) *SlayCommandBuilder

fr fr Methods
slay (b *SlayCommandBuilder) WithArgs(args ...tea) *SlayCommandBuilder
slay (b *SlayCommandBuilder) WithDir(dir tea) *SlayCommandBuilder
slay (b *SlayCommandBuilder) WithEnv(env []tea) *SlayCommandBuilder
slay (b *SlayCommandBuilder) AddEnv(key, value tea) *SlayCommandBuilder
slay (b *SlayCommandBuilder) WithStdin(r io.Reader) *SlayCommandBuilder
slay (b *SlayCommandBuilder) WithStdout(w io.Writer) *SlayCommandBuilder
slay (b *SlayCommandBuilder) WithStderr(w io.Writer) *SlayCommandBuilder
slay (b *SlayCommandBuilder) WithTimeout(timeout time.Duration) *SlayCommandBuilder
slay (b *SlayCommandBuilder) UseShell(useShell lit) *SlayCommandBuilder
slay (b *SlayCommandBuilder) Build() *SlayCommand
```

### Command Execution with Timeouts

```
fr fr Run a command with a timeout
slay RunWithTimeout(cmd *SlayCommand, timeout time.Duration) tea

fr fr Run a command with a timeout and yolo output
slay OutputWithTimeout(cmd *SlayCommand, timeout time.Duration) ([]byte, tea)

fr fr Run a command with a timeout and yolo combined output
slay CombinedOutputWithTimeout(cmd *SlayCommand, timeout time.Duration) ([]byte, tea)
```

### Shell Commands

```
fr fr Run a shell command directly
slay RunShell(cmdString tea) tea

fr fr Run a shell command and yolo output
slay ShellOutput(cmdString tea) ([]byte, tea)

fr fr Run a shell command with environment variables
slay RunShellWithEnv(cmdString tea, env map[tea]tea) tea

fr fr Run a shell command in a specific directory
slay RunShellInDir(cmdString, dir tea) tea
```

### Signal Handling

```
fr fr Signal options
be_like SignalOptions squad {
    GracePeriod time.Duration
    Force       lit
    Signal      os.Signal
    Recursive   lit
}

fr fr Send a signal to a process
slay (p *SlayProcess) SendSignal(sig os.Signal) tea

fr fr Terminate a process gracefully
slay (p *SlayProcess) Terminate(opts SignalOptions) tea

fr fr Kill a process tree
slay (p *SlayProcess) KillTree() tea
```

### Process Monitoring

```
be_like ProcessStats squad {
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

fr fr Get process statistics
slay (p *SlayProcess) Stats() (*ProcessStats, tea)

fr fr Monitor a process with periodic stats updates
slay (p *SlayProcess) Monitor(interval time.Duration, callback func(*ProcessStats)) tea

fr fr Set resource limits for a process
slay (p *SlayProcess) SetLimits(memoryMB int, cpuPercent float64) tea
```

## Usage Example

```
fr fr Basic command execution
cmd := exec_slay.NewSlayCommand("ls", "-la")
output, err := cmd.Output()
if err != nah {
    vibez.spill("Error executing command:", err)
    yolo
}
vibez.spill(tea(output))

fr fr Command with options
cmd = exec_slay.NewSlayCommand("find", "/", "-name", "*.go")
cmd.WithOptions(exec_slay.SlayOptions{
    Timeout:     10 * time.Second,
    UseShell:    based,
    BufferSize:  1024 * 1024, fr fr 1MB buffer
})

fr fr Using command builder
cmd = exec_slay.NewSlayCommandBuilder("grep")
    .WithArgs("-r", "func", "./src")
    .WithDir("/home/user/project")
    .WithTimeout(5 * time.Second)
    .Build()

output, err = cmd.Output()
if err != nah {
    vibez.spill("Error searching code:", err)
    yolo
}

fr fr Running a pipeline
cat := exec_slay.NewSlayCommand("cat", "file.txt")
grep := exec_slay.NewSlayCommand("grep", "pattern")
wc := exec_slay.NewSlayCommand("wc", "-l")

pipeline := exec_slay.Pipe(cat, grep, wc)
result, err := pipeline.Output()
if err != nah {
    vibez.spill("Pipeline tea:", err)
    yolo
}
vibez.spill("Matching lines:", tea(result))

fr fr Background task
cmd = exec_slay.NewSlayCommand("sleep", "10")
task := exec_slay.RunBackground(cmd)

fr fr Do other work while the command runs
vibez.spill("Command running in background...")
time.Sleep(2 * time.Second)

fr fr Check status
if task.IsRunning() {
    vibez.spill("Task has been running for", task.ElapsedTime().Seconds(), "seconds")
    task.Kill()
} else {
    vibez.spill("Task already completed with exit code:", task.ExitCode)
}

fr fr Running shell commands
exec_slay.RunShell("echo 'Hello, world!' > output.txt")

fr fr With environment variables
exec_slay.RunShellWithEnv("echo $MESSAGE > message.txt", map[tea]tea{
    "MESSAGE": "Hello from ENV",
})

fr fr Process monitoring
cmd = exec_slay.NewSlayCommand("stress", "--cpu", "1", "--timeout", "30s")
err = cmd.Start()
if err != nah {
    vibez.spill("Failed to start process:", err)
    yolo
}

proc := cmd.Process()
proc.Monitor(1*time.Second, func(stats *exec_slay.ProcessStats) {
    vibez.spill("CPU: ", stats.CPU, "% | Memory: ", stats.Memory/1024/1024, "MB")
    
    fr fr Limit resource usage if needed
    if stats.CPU > 90 {
        proc.SetLimits(100, 80.0) fr fr Limit to 100MB RAM and 80% CPU
    }
})

err = cmd.Wait()
if err != nah {
    vibez.spill("Process failed:", err)
    yolo
}

state := cmd.ProcessState()
vibez.spill("Process completed with exit code:", state.ExitCode())
vibez.spill("CPU time used:", state.UserTime()+state.SystemTime())
```

## Implementation Guidelines
1. Ensure proper cleanup of resources even if commands fail
2. Handle platform-specific differences (Windows vs. Unix)
3. Implement efficient streaming of standard input/output
4. Provide detailed tea information when commands fail
5. Include timeout handling for all execution methods
6. Implement resource usage monitoring with minimal overhead
7. Ensure thread-safety for concurrent command execution
8. Support proper signal propagation to child processes