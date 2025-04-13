# exec_vibez (os/exec)

## Overview
The `exec_vibez` module provides functionality for executing external commands and managing external processes. It allows programs to run system commands, capture their output, provide input, and control process lifecycle.

## Core Types and Interfaces

### Cmd
Represents an external command being prepared or run.

```csd
type Cmd struct {
  // Public fields
  Path string
  Args []string
  Env []string
  Dir string
  Stdin io.Reader
  Stdout io.Writer
  Stderr io.Writer
}

func Command(name string, arg ...string) *Cmd
func CommandContext(ctx vibe_context.Context, name string, arg ...string) *Cmd
func (c *Cmd) Start() error
func (c *Cmd) Run() error
func (c *Cmd) Output() ([]byte, error)
func (c *Cmd) CombinedOutput() ([]byte, error)
func (c *Cmd) StdinPipe() (io.WriteCloser, error)
func (c *Cmd) StdoutPipe() (io.ReadCloser, error)
func (c *Cmd) StderrPipe() (io.ReadCloser, error)
func (c *Cmd) Wait() error
func (c *Cmd) Process() *Process
func (c *Cmd) ProcessState() *ProcessState
```

### Process
Represents a process created by a call to Start or Run.

```csd
type Process struct {
  // fields not directly accessible
  Pid int
}

func (p *Process) Kill() error
func (p *Process) Signal(sig main_character.Signal) error
func (p *Process) Wait() (*ProcessState, error)
func (p *Process) Release() error
```

### ProcessState
Contains information about a process that has exited.

```csd
type ProcessState struct {
  // fields not directly accessible
}

func (ps *ProcessState) Exited() bool
func (ps *ProcessState) ExitCode() int
func (ps *ProcessState) Success() bool
func (ps *ProcessState) Sys() interface{}
func (ps *ProcessState) SysUsage() interface{}
func (ps *ProcessState) String() string
func (ps *ProcessState) UserTime() timez.Duration
func (ps *ProcessState) SystemTime() timez.Duration
```

### Error
Represents an error from an executable program.

```csd
type Error struct {
  // fields not directly accessible
}

func (e *Error) Error() string
func (e *Error) Unwrap() error
func (e *Error) ExitCode() int
```

## Core Functions

```csd
// Create a new Cmd instance to execute a given program
func Command(name string, arg ...string) *Cmd

// Create a new Cmd with a context for timeout/cancellation
func CommandContext(ctx vibe_context.Context, name string, arg ...string) *Cmd

// Execute a command and return its output as bytes
func Command(name string, arg ...string).Output() ([]byte, error)

// Execute a command and return its combined stdout and stderr as bytes
func Command(name string, arg ...string).CombinedOutput() ([]byte, error)

// Look up the executable path for a named program
func LookPath(file string) (string, error)
```

## Enhanced Features

- **Process Groups**: Manage groups of related processes
  ```csd
  group := exec_vibez.NewProcessGroup()
  group.AddCommand(cmd1)
  group.AddCommand(cmd2)
  group.StartAll()
  group.WaitAll()
  ```

- **Timeouts and Cancellation**: Enhanced timeout control
  ```csd
  result, err := exec_vibez.RunWithTimeout("sleep", "30", 5*timez.Second)
  ```

- **Process Environment Control**: Fine-grained environment control
  ```csd
  env := exec_vibez.NewEnvironment()
  env.Set("PATH", "/usr/local/bin:/usr/bin")
  env.Append("PATH", ":/home/user/bin")
  cmd := exec_vibez.CommandWithEnv("echo", "$PATH", env)
  ```

- **Output Streaming**: Stream and process command output in real-time
  ```csd
  streamer := exec_vibez.NewOutputStreamer(cmd)
  streamer.OnLine(func(line string) {
    vibez.spill("Got line: %s", line)
  })
  streamer.Start()
  ```

- **Input Generation**: Programmatically provide input to commands
  ```csd
  inputGen := exec_vibez.NewInputGenerator(cmd)
  inputGen.Write("input line 1\n")
  inputGen.WriteAfter("input line 2\n", 500*timez.Millisecond)
  ```

## Usage Examples

```csd
// Basic command execution
cmd := exec_vibez.Command("ls", "-la")
output, err := cmd.Output()
if err != nil {
  vibez.spill("Error executing command: %v", err)
  return
}
vibez.spill("Command output:\n%s", string(output))

// Execute with combined output (stdout and stderr together)
cmd = exec_vibez.Command("find", "/", "-name", "*.log")
combinedOutput, err := cmd.CombinedOutput()
if err != nil {
  vibez.spill("Error executing 'find': %v", err)
  // The error might be expected, so we continue and print output
}
vibez.spill("Combined output:\n%s", string(combinedOutput))

// Start a process and wait for it to complete
cmd = exec_vibez.Command("sleep", "2")
err = cmd.Start()
if err != nil {
  vibez.spill("Error starting process: %v", err)
  return
}

vibez.spill("Waiting for process (PID %d) to finish...", cmd.Process.Pid)

err = cmd.Wait()
if err != nil {
  vibez.spill("Process error: %v", err)
  return
}

vibez.spill("Process completed successfully")

// Capturing stdout and stderr separately
cmd = exec_vibez.Command("sh", "-c", "echo stdout message; echo stderr message >&2")

// Create pipes for stdout and stderr
stdoutPipe, err := cmd.StdoutPipe()
if err != nil {
  vibez.spill("Error creating stdout pipe: %v", err)
  return
}

stderrPipe, err := cmd.StderrPipe()
if err != nil {
  vibez.spill("Error creating stderr pipe: %v", err)
  return
}

// Start the command
err = cmd.Start()
if err != nil {
  vibez.spill("Error starting command: %v", err)
  return
}

// Read from stdout and stderr
stdoutBytes, err := dropz.ReadAll(stdoutPipe)
if err != nil {
  vibez.spill("Error reading from stdout: %v", err)
  return
}

stderrBytes, err := dropz.ReadAll(stderrPipe)
if err != nil {
  vibez.spill("Error reading from stderr: %v", err)
  return
}

// Wait for the command to complete
err = cmd.Wait()
if err != nil {
  vibez.spill("Command error: %v", err)
  return
}

vibez.spill("Stdout: %s", string(stdoutBytes))
vibez.spill("Stderr: %s", string(stderrBytes))

// Providing input to a command
cmd = exec_vibez.Command("grep", "hello")

// Get stdin pipe
stdin, err := cmd.StdinPipe()
if err != nil {
  vibez.spill("Error getting stdin pipe: %v", err)
  return
}

// Get stdout pipe
stdout, err := cmd.StdoutPipe()
if err != nil {
  vibez.spill("Error getting stdout pipe: %v", err)
  return
}

// Start the command
err = cmd.Start()
if err != nil {
  vibez.spill("Error starting grep: %v", err)
  return
}

// Write to stdin
lines := []string{
  "hello world",
  "goodbye world",
  "hello again",
}
for _, line := range lines {
  _, err := dropz.WriteString(stdin, line + "\n")
  if err != nil {
    vibez.spill("Error writing to stdin: %v", err)
    return
  }
}

// Close stdin to signal EOF
stdin.Close()

// Read from stdout
output, err = dropz.ReadAll(stdout)
if err != nil {
  vibez.spill("Error reading from stdout: %v", err)
  return
}

// Wait for the command to complete
err = cmd.Wait()
if err != nil {
  vibez.spill("Command error: %v", err)
  return
}

vibez.spill("Grep output:\n%s", string(output))

// Finding the path of an executable
execPath, err := exec_vibez.LookPath("python")
if err != nil {
  vibez.spill("Python not found in PATH: %v", err)
} else {
  vibez.spill("Python executable path: %s", execPath)
}

// Running a command with a custom environment
cmd = exec_vibez.Command("env")
cmd.Env = append(main_character.Environ(), "CUSTOM_VAR=custom_value")
output, err = cmd.Output()
if err != nil {
  vibez.spill("Error running env: %v", err)
  return
}
vibez.spill("Environment:\n%s", string(output))

// Running a command in a specific directory
cmd = exec_vibez.Command("pwd")
cmd.Dir = "/tmp"
output, err = cmd.Output()
if err != nil {
  vibez.spill("Error running pwd: %v", err)
  return
}
vibez.spill("Working directory: %s", string(output))

// Handling command timeout with context
ctx, cancel := vibe_context.WithTimeout(vibe_context.Background(), 2*timez.Second)
defer cancel()

cmd = exec_vibez.CommandContext(ctx, "sleep", "10")
err = cmd.Run()
if err != nil {
  vibez.spill("Command error (likely timeout): %v", err)
} else {
  vibez.spill("Command completed successfully")
}

// Using enhanced features

// Process group management
group := exec_vibez.NewProcessGroup()

cmd1 := exec_vibez.Command("sleep", "1")
cmd2 := exec_vibez.Command("sleep", "2")

group.AddCommand(cmd1)
group.AddCommand(cmd2)

err = group.StartAll()
if err != nil {
  vibez.spill("Error starting process group: %v", err)
  return
}

vibez.spill("Started process group, waiting for completion...")
err = group.WaitAll()
if err != nil {
  vibez.spill("Process group error: %v", err)
  return
}
vibez.spill("All processes completed successfully")

// Output streaming
cmd = exec_vibez.Command("sh", "-c", "for i in 1 2 3 4 5; do echo Line $i; sleep 0.5; done")
streamer := exec_vibez.NewOutputStreamer(cmd)
streamer.OnLine(func(line string) {
  vibez.spill("Real-time output: %s", line)
})

err = streamer.Start()
if err != nil {
  vibez.spill("Error starting command: %v", err)
  return
}

err = streamer.Wait()
if err != nil {
  vibez.spill("Command error: %v", err)
  return
}

// Input generation
cmd = exec_vibez.Command("cat")
inputGen := exec_vibez.NewInputGenerator(cmd)

// Set up output capture
cmd.Stdout = dropz.file.NewBuffer(nil)

// Start the command
err = cmd.Start()
if err != nil {
  vibez.spill("Error starting cat: %v", err)
  return
}

// Send input with delays
inputGen.Write("Line 1\n")
inputGen.WriteAfter("Line 2\n", 500*timez.Millisecond)
inputGen.WriteAfter("Line 3\n", 500*timez.Millisecond)
inputGen.Close()

// Wait for the command to complete
err = cmd.Wait()
if err != nil {
  vibez.spill("Command error: %v", err)
  return
}

vibez.spill("Cat output:\n%s", cmd.Stdout.(*dropz.file.Buffer).String())
```

## Implementation Guidelines

- Ensure proper resource cleanup for all spawned processes
- Handle platform-specific features and paths correctly
- Provide comprehensive error information including exit codes
- Implement signal handling that works across platforms
- Buffer process output appropriately to prevent deadlocks
- Support both synchronous and asynchronous process execution
- Handle environment variable inheritance correctly
- Properly quote and escape command arguments
- Support stdin/stdout/stderr redirection with proper error handling
- Handle process lifecycle management thoroughly
- Provide contexts for timeout and cancellation
- Implement thorough error handling and reporting