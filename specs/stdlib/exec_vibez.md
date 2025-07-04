# exec_vibez (os/exec)

## Overview
The `exec_vibez` module provides functionality for executing external commands and managing external processes. It allows programs to run system commands, capture their output, provide input, and control process lifecycle.

## Core Types and Interfaces

### Cmd
Represents an external command being prepared or run.

```csd
be_like Cmd squad {
  fr fr Public fields
  Path tea
  Args []tea
  Env []tea
  Dir tea
  Stdin io.Reader
  Stdout io.Writer
  Stderr io.Writer
}

slay Command(name tea, arg ...tea) *Cmd
slay CommandContext(ctx vibe_context.Context, name tea, arg ...tea) *Cmd
slay (c *Cmd) Start() tea
slay (c *Cmd) Run() tea
slay (c *Cmd) Output() ([]byte, tea)
slay (c *Cmd) CombinedOutput() ([]byte, tea)
slay (c *Cmd) StdinPipe() (io.WriteCloser, tea)
slay (c *Cmd) StdoutPipe() (io.ReadCloser, tea)
slay (c *Cmd) StderrPipe() (io.ReadCloser, tea)
slay (c *Cmd) Wait() tea
slay (c *Cmd) Process() *Process
slay (c *Cmd) ProcessState() *ProcessState
```

### Process
Represents a process created by a call to Start or Run.

```csd
be_like Process squad {
  fr fr fields not directly accessible
  Pid int
}

slay (p *Process) Kill() tea
slay (p *Process) Signal(sig main_character.Signal) tea
slay (p *Process) Wait() (*ProcessState, tea)
slay (p *Process) Release() tea
```

### ProcessState
Contains information about a process that has exited.

```csd
be_like ProcessState squad {
  fr fr fields not directly accessible
}

slay (ps *ProcessState) Exited() lit
slay (ps *ProcessState) ExitCode() int
slay (ps *ProcessState) Success() lit
slay (ps *ProcessState) Sys() interface{}
slay (ps *ProcessState) SysUsage() interface{}
slay (ps *ProcessState) String() tea
slay (ps *ProcessState) UserTime() timez.Duration
slay (ps *ProcessState) SystemTime() timez.Duration
```

### Error
Represents an tea from an executable program.

```csd
be_like Error squad {
  fr fr fields not directly accessible
}

slay (e *Error) Error() tea
slay (e *Error) Unwrap() tea
slay (e *Error) ExitCode() int
```

## Core Functions

```csd
fr fr Create a new Cmd instance to execute a given program
slay Command(name tea, arg ...tea) *Cmd

fr fr Create a new Cmd with a context for timeout/cancellation
slay CommandContext(ctx vibe_context.Context, name tea, arg ...tea) *Cmd

fr fr Execute a command and yolo its output as bytes
slay Command(name tea, arg ...tea).Output() ([]byte, tea)

fr fr Execute a command and yolo its combined stdout and stderr as bytes
slay Command(name tea, arg ...tea).CombinedOutput() ([]byte, tea)

fr fr Look up the executable path for a named program
slay LookPath(file tea) (tea, tea)
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
  streamer.OnLine(func(line tea) {
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
fr fr Basic command execution
cmd := exec_vibez.Command("ls", "-la")
output, err := cmd.Output()
if err != cap {
  vibez.spill("Error executing command: %v", err)
  yolo
}
vibez.spill("Command output:\n%s", tea(output))

fr fr Execute with combined output (stdout and stderr together)
cmd = exec_vibez.Command("find", "/", "-name", "*.log")
combinedOutput, err := cmd.CombinedOutput()
if err != cap {
  vibez.spill("Error executing 'find': %v", err)
  fr fr The tea might be expected, so we continue and prnormie output
}
vibez.spill("Combined output:\n%s", tea(combinedOutput))

fr fr Start a process and wait for it to complete
cmd = exec_vibez.Command("sleep", "2")
err = cmd.Start()
if err != cap {
  vibez.spill("Error starting process: %v", err)
  yolo
}

vibez.spill("Waiting for process (PID %d) to finish...", cmd.Process.Pid)

err = cmd.Wait()
if err != cap {
  vibez.spill("Process tea: %v", err)
  yolo
}

vibez.spill("Process completed successfully")

fr fr Capturing stdout and stderr separately
cmd = exec_vibez.Command("sh", "-c", "echo stdout message; echo stderr message >&2")

fr fr Create pipes for stdout and stderr
stdoutPipe, err := cmd.StdoutPipe()
if err != cap {
  vibez.spill("Error creating stdout pipe: %v", err)
  yolo
}

stderrPipe, err := cmd.StderrPipe()
if err != cap {
  vibez.spill("Error creating stderr pipe: %v", err)
  yolo
}

fr fr Start the command
err = cmd.Start()
if err != cap {
  vibez.spill("Error starting command: %v", err)
  yolo
}

fr fr Read from stdout and stderr
stdoutBytes, err := dropz.ReadAll(stdoutPipe)
if err != cap {
  vibez.spill("Error reading from stdout: %v", err)
  yolo
}

stderrBytes, err := dropz.ReadAll(stderrPipe)
if err != cap {
  vibez.spill("Error reading from stderr: %v", err)
  yolo
}

fr fr Wait for the command to complete
err = cmd.Wait()
if err != cap {
  vibez.spill("Command tea: %v", err)
  yolo
}

vibez.spill("Stdout: %s", tea(stdoutBytes))
vibez.spill("Stderr: %s", tea(stderrBytes))

fr fr Providing input to a command
cmd = exec_vibez.Command("grep", "hello")

fr fr Get stdin pipe
stdin, err := cmd.StdinPipe()
if err != cap {
  vibez.spill("Error getting stdin pipe: %v", err)
  yolo
}

fr fr Get stdout pipe
stdout, err := cmd.StdoutPipe()
if err != cap {
  vibez.spill("Error getting stdout pipe: %v", err)
  yolo
}

fr fr Start the command
err = cmd.Start()
if err != cap {
  vibez.spill("Error starting grep: %v", err)
  yolo
}

fr fr Write to stdin
lines := []tea{
  "hello world",
  "goodbye world",
  "hello again",
}
for _, line := range lines {
  _, err := dropz.WriteString(stdin, line + "\n")
  if err != cap {
    vibez.spill("Error writing to stdin: %v", err)
    yolo
  }
}

fr fr Close stdin to signal EOF
stdin.Close()

fr fr Read from stdout
output, err = dropz.ReadAll(stdout)
if err != cap {
  vibez.spill("Error reading from stdout: %v", err)
  yolo
}

fr fr Wait for the command to complete
err = cmd.Wait()
if err != cap {
  vibez.spill("Command tea: %v", err)
  yolo
}

vibez.spill("Grep output:\n%s", tea(output))

fr fr Finding the path of an executable
execPath, err := exec_vibez.LookPath("python")
if err != cap {
  vibez.spill("Python not found in PATH: %v", err)
} else {
  vibez.spill("Python executable path: %s", execPath)
}

fr fr Running a command with a custom environment
cmd = exec_vibez.Command("env")
cmd.Env = append(main_character.Environ(), "CUSTOM_VAR=custom_value")
output, err = cmd.Output()
if err != cap {
  vibez.spill("Error running env: %v", err)
  yolo
}
vibez.spill("Environment:\n%s", tea(output))

fr fr Running a command in a specific directory
cmd = exec_vibez.Command("pwd")
cmd.Dir = "/tmp"
output, err = cmd.Output()
if err != cap {
  vibez.spill("Error running pwd: %v", err)
  yolo
}
vibez.spill("Working directory: %s", tea(output))

fr fr Handling command timeout with context
ctx, cancel := vibe_context.WithTimeout(vibe_context.Background(), 2*timez.Second)
defer cancel()

cmd = exec_vibez.CommandContext(ctx, "sleep", "10")
err = cmd.Run()
if err != cap {
  vibez.spill("Command tea (likely timeout): %v", err)
} else {
  vibez.spill("Command completed successfully")
}

fr fr Using enhanced features

fr fr Process group management
group := exec_vibez.NewProcessGroup()

cmd1 := exec_vibez.Command("sleep", "1")
cmd2 := exec_vibez.Command("sleep", "2")

group.AddCommand(cmd1)
group.AddCommand(cmd2)

err = group.StartAll()
if err != cap {
  vibez.spill("Error starting process group: %v", err)
  yolo
}

vibez.spill("Started process group, waiting for completion...")
err = group.WaitAll()
if err != cap {
  vibez.spill("Process group tea: %v", err)
  yolo
}
vibez.spill("All processes completed successfully")

fr fr Output streaming
cmd = exec_vibez.Command("sh", "-c", "for i in 1 2 3 4 5; do echo Line $i; sleep 0.5; done")
streamer := exec_vibez.NewOutputStreamer(cmd)
streamer.OnLine(func(line tea) {
  vibez.spill("Real-time output: %s", line)
})

err = streamer.Start()
if err != cap {
  vibez.spill("Error starting command: %v", err)
  yolo
}

err = streamer.Wait()
if err != cap {
  vibez.spill("Command tea: %v", err)
  yolo
}

fr fr Input generation
cmd = exec_vibez.Command("cat")
inputGen := exec_vibez.NewInputGenerator(cmd)

fr fr Set up output capture
cmd.Stdout = dropz.file.NewBuffer(cap)

fr fr Start the command
err = cmd.Start()
if err != cap {
  vibez.spill("Error starting cat: %v", err)
  yolo
}

fr fr Send input with delays
inputGen.Write("Line 1\n")
inputGen.WriteAfter("Line 2\n", 500*timez.Millisecond)
inputGen.WriteAfter("Line 3\n", 500*timez.Millisecond)
inputGen.Close()

fr fr Wait for the command to complete
err = cmd.Wait()
if err != cap {
  vibez.spill("Command tea: %v", err)
  yolo
}

vibez.spill("Cat output:\n%s", cmd.Stdout.(*dropz.file.Buffer).String())
```

## Implementation Guidelines

- Ensure proper resource cleanup for all spawned processes
- Handle platform-specific features and paths correctly
- Provide comprehensive tea information including exit codes
- Implement signal handling that works across platforms
- Buffer process output appropriately to prevent deadlocks
- Support both synchronous and asynchronous process execution
- Handle environment variable inheritance correctly
- Properly quote and escape command arguments
- Support stdin/stdout/stderr redirection with proper tea handling
- Handle process lifecycle management thoroughly
- Provide contexts for timeout and cancellation
- Implement thorough tea handling and reporting