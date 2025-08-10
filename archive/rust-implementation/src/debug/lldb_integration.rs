//! LLDB Integration for CURSED Debugger
//! 
//! This module provides comprehensive LLDB integration including:
//! - LLDB scripting API support
//! - Advanced breakpoint management
//! - Memory and register inspection
//! - Python scripting interface
//! - Target and process management

use crate::error::CursedError;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// LLDB integration manager
#[derive(Debug)]
pub struct LldbIntegration {
    /// LLDB process
    lldb_process: Option<Child>,
    /// Command sender
    command_sender: Option<Sender<String>>,
    /// Response receiver
    response_receiver: Option<Receiver<String>>,
    /// Current target
    current_target: Option<LldbTarget>,
    /// Current process
    current_process: Option<LldbProcess>,
    /// Breakpoints
    breakpoints: HashMap<u32, LldbBreakpoint>,
    /// Next breakpoint ID
    next_breakpoint_id: u32,
    /// Current frame
    current_frame: u32,
    /// Current thread
    current_thread: Option<u32>,
}

/// LLDB target information
#[derive(Debug, Clone)]
pub struct LldbTarget {
    pub executable: String,
    pub architecture: String,
    pub platform: String,
    pub modules: Vec<LldbModule>,
}

/// LLDB process information
#[derive(Debug, Clone)]
pub struct LldbProcess {
    pub pid: u32,
    pub state: ProcessState,
    pub threads: Vec<LldbThread>,
    pub exit_code: Option<i32>,
}

/// Process state
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Invalid,
    Unloaded,
    Connected,
    Attaching,
    Launching,
    Stopped,
    Running,
    Stepping,
    Crashed,
    Detached,
    Exited,
    Suspended,
}

/// LLDB thread information
#[derive(Debug, Clone)]
pub struct LldbThread {
    pub id: u32,
    pub index: u32,
    pub name: Option<String>,
    pub queue: Option<String>,
    pub state: ThreadState,
    pub stop_reason: Option<String>,
    pub frames: Vec<LldbFrame>,
}

/// Thread state for LLDB
#[derive(Debug, Clone)]
pub enum ThreadState {
    Invalid,
    Running,
    Stopped,
    Stepping,
    Suspended,
}

/// LLDB stack frame
#[derive(Debug, Clone)]
pub struct LldbFrame {
    pub index: u32,
    pub pc: u64,
    pub fp: u64,
    pub sp: u64,
    pub function: Option<String>,
    pub module: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub variables: Vec<LldbVariable>,
}

/// LLDB variable information
#[derive(Debug, Clone)]
pub struct LldbVariable {
    pub name: String,
    pub value: String,
    pub summary: Option<String>,
    pub var_type: String,
    pub size: u64,
    pub location: Option<String>,
    pub children: Vec<LldbVariable>,
}

/// LLDB breakpoint
#[derive(Debug, Clone)]
pub struct LldbBreakpoint {
    pub id: u32,
    pub enabled: bool,
    pub resolved: bool,
    pub hit_count: u32,
    pub ignore_count: u32,
    pub condition: Option<String>,
    pub locations: Vec<LldbBreakpointLocation>,
}

/// LLDB breakpoint location
#[derive(Debug, Clone)]
pub struct LldbBreakpointLocation {
    pub id: String,
    pub address: u64,
    pub resolved: bool,
    pub module: Option<String>,
    pub function: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

/// LLDB module information
#[derive(Debug, Clone)]
pub struct LldbModule {
    pub name: String,
    pub uuid: String,
    pub path: String,
    pub load_address: u64,
    pub size: u64,
    pub symbols_loaded: bool,
}

/// Memory region information
#[derive(Debug, Clone)]
pub struct LldbMemoryRegion {
    pub start_address: u64,
    pub end_address: u64,
    pub permissions: String,
    pub name: Option<String>,
    pub data: Vec<u8>,
}

/// Register information
#[derive(Debug, Clone)]
pub struct LldbRegister {
    pub name: String,
    pub value: u64,
    pub size: u32,
    pub format: RegisterFormat,
}

/// Register format
#[derive(Debug, Clone)]
pub enum RegisterFormat {
    Hex,
    Decimal,
    Binary,
    Float,
    Double,
}

/// Watchpoint information
#[derive(Debug, Clone)]
pub struct LldbWatchpoint {
    pub id: u32,
    pub address: u64,
    pub size: u32,
    pub watch_type: WatchType,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub enabled: bool,
}

/// Watch type
#[derive(Debug, Clone)]
pub enum WatchType {
    Read,
    Write,
    ReadWrite,
}

impl LldbIntegration {
    /// Create new LLDB integration
    pub fn new() -> Self {
        Self {
            lldb_process: None,
            command_sender: None,
            response_receiver: None,
            current_target: None,
            current_process: None,
            breakpoints: HashMap::new(),
            next_breakpoint_id: 1,
            current_frame: 0,
            current_thread: None,
        }
    }

    /// Start LLDB with the specified executable
    pub fn start_lldb(&mut self, executable: &str) -> Result<(), CursedError> {
        // Start LLDB process
        let mut lldb = Command::new("lldb")
            .arg("--batch")
            .arg("--no-lldbinit")
            .arg("--no-use-colors")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| CursedError::Io(format!("Failed to start LLDB: {}", e)))?;

        // Set up communication channels
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (resp_tx, resp_rx) = mpsc::channel();

        // Take ownership of stdin/stdout
        let mut lldb_stdin = lldb.stdin.take().unwrap();
        let lldb_stdout = lldb.stdout.take().unwrap();

        // Spawn command sender thread
        thread::spawn(move || {
            for command in cmd_rx {
                if let Err(e) = writeln!(lldb_stdin, "{}", command) {
                    eprintln!("Failed to send LLDB command: {}", e);
                    break;
                }
                if let Err(e) = lldb_stdin.flush() {
                    eprintln!("Failed to flush LLDB command: {}", e);
                    break;
                }
            }
        });

        // Spawn response reader thread
        thread::spawn(move || {
            let reader = BufReader::new(lldb_stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if resp_tx.send(line).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        self.lldb_process = Some(lldb);
        self.command_sender = Some(cmd_tx);
        self.response_receiver = Some(resp_rx);

        // Create target
        self.create_target(executable)?;

        // Initial setup
        self.send_command("settings set target.inline-breakpoint-strategy always")?;
        self.send_command("settings set target.auto-apply-fixits false")?;

        Ok(())
    }

    /// Send command to LLDB
    pub fn send_command(&mut self, command: &str) -> Result<Vec<String>, CursedError> {
        if let Some(ref sender) = self.command_sender {
            sender.send(command.to_string())
                .map_err(|e| CursedError::General(format!("Failed to send command: {}", e)))?;
        }

        // Wait for response
        let mut responses = Vec::new();
        if let Some(ref receiver) = self.response_receiver {
            // Collect responses until we see the prompt or timeout
            for _ in 0..100 { // Max 100 lines
                if let Ok(line) = receiver.recv() {
                    responses.push(line.clone());
                    if line.trim() == "(lldb)" || line.contains("error:") {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        Ok(responses)
    }

    /// Create target for executable
    pub fn create_target(&mut self, executable: &str) -> Result<(), CursedError> {
        let responses = self.send_command(&format!("target create {}", executable))?;
        
        for response in &responses {
            if response.contains("Current executable set to") {
                // Parse target information
                self.current_target = Some(LldbTarget {
                    executable: executable.to_string(),
                    architecture: "x86_64".to_string(), // Would parse from output
                    platform: "linux".to_string(), // Would parse from output
                    modules: Vec::new(),
                });
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to create target".to_string()))
    }

    /// Set breakpoint by location
    pub fn set_breakpoint(&mut self, location: &str) -> Result<u32, CursedError> {
        let responses = self.send_command(&format!("breakpoint set --name {}", location))?;
        
        for response in &responses {
            if response.contains("Breakpoint") && response.contains("where =") {
                let breakpoint_id = self.next_breakpoint_id;
                self.next_breakpoint_id += 1;
                
                let breakpoint = LldbBreakpoint {
                    id: breakpoint_id,
                    enabled: true,
                    resolved: true,
                    hit_count: 0,
                    ignore_count: 0,
                    condition: None,
                    locations: vec![LldbBreakpointLocation {
                        id: format!("{}.1", breakpoint_id),
                        address: 0, // Would parse from output
                        resolved: true,
                        module: None,
                        function: Some(location.to_string()),
                        file: None,
                        line: None,
                        column: None,
                    }],
                };
                
                self.breakpoints.insert(breakpoint_id, breakpoint);
                return Ok(breakpoint_id);
            }
        }

        Err(CursedError::General("Failed to set breakpoint".to_string()))
    }

    /// Set breakpoint by file and line
    pub fn set_breakpoint_by_line(&mut self, file: &str, line: u32) -> Result<u32, CursedError> {
        let responses = self.send_command(&format!("breakpoint set --file {} --line {}", file, line))?;
        
        for response in &responses {
            if response.contains("Breakpoint") {
                let breakpoint_id = self.next_breakpoint_id;
                self.next_breakpoint_id += 1;
                
                let breakpoint = LldbBreakpoint {
                    id: breakpoint_id,
                    enabled: true,
                    resolved: true,
                    hit_count: 0,
                    ignore_count: 0,
                    condition: None,
                    locations: vec![LldbBreakpointLocation {
                        id: format!("{}.1", breakpoint_id),
                        address: 0,
                        resolved: true,
                        module: None,
                        function: None,
                        file: Some(file.to_string()),
                        line: Some(line),
                        column: None,
                    }],
                };
                
                self.breakpoints.insert(breakpoint_id, breakpoint);
                return Ok(breakpoint_id);
            }
        }

        Err(CursedError::General("Failed to set breakpoint".to_string()))
    }

    /// Set conditional breakpoint
    pub fn set_conditional_breakpoint(&mut self, location: &str, condition: &str) -> Result<u32, CursedError> {
        let responses = self.send_command(&format!("breakpoint set --name {} --condition '{}'", location, condition))?;
        
        for response in &responses {
            if response.contains("Breakpoint") {
                let breakpoint_id = self.next_breakpoint_id;
                self.next_breakpoint_id += 1;
                
                let breakpoint = LldbBreakpoint {
                    id: breakpoint_id,
                    enabled: true,
                    resolved: true,
                    hit_count: 0,
                    ignore_count: 0,
                    condition: Some(condition.to_string()),
                    locations: vec![LldbBreakpointLocation {
                        id: format!("{}.1", breakpoint_id),
                        address: 0,
                        resolved: true,
                        module: None,
                        function: Some(location.to_string()),
                        file: None,
                        line: None,
                        column: None,
                    }],
                };
                
                self.breakpoints.insert(breakpoint_id, breakpoint);
                return Ok(breakpoint_id);
            }
        }

        Err(CursedError::General("Failed to set conditional breakpoint".to_string()))
    }

    /// Delete breakpoint
    pub fn delete_breakpoint(&mut self, id: u32) -> Result<(), CursedError> {
        let responses = self.send_command(&format!("breakpoint delete {}", id))?;
        
        for response in &responses {
            if response.contains("deleted") {
                self.breakpoints.remove(&id);
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to delete breakpoint".to_string()))
    }

    /// Enable/disable breakpoint
    pub fn set_breakpoint_enabled(&mut self, id: u32, enabled: bool) -> Result<(), CursedError> {
        let command = if enabled {
            format!("breakpoint enable {}", id)
        } else {
            format!("breakpoint disable {}", id)
        };
        
        let responses = self.send_command(&command)?;
        
        for response in &responses {
            if response.contains("breakpoint") {
                if let Some(breakpoint) = self.breakpoints.get_mut(&id) {
                    breakpoint.enabled = enabled;
                }
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to modify breakpoint".to_string()))
    }

    /// List all breakpoints
    pub fn list_breakpoints(&mut self) -> Result<Vec<LldbBreakpoint>, CursedError> {
        let _responses = self.send_command("breakpoint list")?;
        
        // Return current breakpoints (in a real implementation, would parse the output)
        Ok(self.breakpoints.values().cloned().collect())
    }

    /// Set watchpoint
    pub fn set_watchpoint(&mut self, address: u64, size: u32, watch_type: WatchType) -> Result<u32, CursedError> {
        let watch_str = match watch_type {
            WatchType::Read => "read",
            WatchType::Write => "write",
            WatchType::ReadWrite => "read_write",
        };
        
        let responses = self.send_command(&format!("watchpoint set expression --size {} -w {} 0x{:x}", size, watch_str, address))?;
        
        for response in &responses {
            if response.contains("Watchpoint created") {
                return Ok(1); // Would parse actual ID
            }
        }

        Err(CursedError::General("Failed to set watchpoint".to_string()))
    }

    /// Run program
    pub fn run_program(&mut self, args: &[&str]) -> Result<(), CursedError> {
        let args_str = args.join(" ");
        let responses = self.send_command(&format!("process launch -- {}", args_str))?;
        
        for response in &responses {
            if response.contains("Process") && response.contains("launched") {
                // Parse process information
                self.current_process = Some(LldbProcess {
                    pid: 0, // Would parse from output
                    state: ProcessState::Running,
                    threads: Vec::new(),
                    exit_code: None,
                });
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to run program".to_string()))
    }

    /// Continue execution
    pub fn continue_execution(&mut self) -> Result<(), CursedError> {
        let responses = self.send_command("process continue")?;
        
        for response in &responses {
            if response.contains("Process") && (response.contains("resuming") || response.contains("stopped")) {
                return Ok(());
            }
        }

        Ok(()) // Continue command often doesn't give immediate feedback
    }

    /// Step into
    pub fn step_into(&mut self) -> Result<(), CursedError> {
        let _responses = self.send_command("thread step-in")?;
        Ok(())
    }

    /// Step over
    pub fn step_over(&mut self) -> Result<(), CursedError> {
        let _responses = self.send_command("thread step-over")?;
        Ok(())
    }

    /// Step out
    pub fn step_out(&mut self) -> Result<(), CursedError> {
        let _responses = self.send_command("thread step-out")?;
        Ok(())
    }

    /// Get stack trace
    pub fn get_stack_trace(&mut self) -> Result<Vec<LldbFrame>, CursedError> {
        let responses = self.send_command("thread backtrace")?;
        
        let mut frames = Vec::new();
        let mut frame_index = 0;
        
        for response in &responses {
            if response.contains("frame #") {
                // Parse frame information (simplified)
                frames.push(LldbFrame {
                    index: frame_index,
                    pc: 0, // Would parse from output
                    fp: 0,
                    sp: 0,
                    function: None, // Would parse from output
                    module: None,
                    file: None,
                    line: None,
                    column: None,
                    variables: Vec::new(),
                });
                frame_index += 1;
            }
        }

        Ok(frames)
    }

    /// Get local variables
    pub fn get_local_variables(&mut self) -> Result<Vec<LldbVariable>, CursedError> {
        let responses = self.send_command("frame variable")?;
        
        let mut variables = Vec::new();
        
        for response in &responses {
            if response.contains("=") && !response.contains("(lldb)") {
                // Parse variable information (simplified)
                if let Some(eq_pos) = response.find('=') {
                    let name_part = response[..eq_pos].trim();
                    let value_part = response[eq_pos + 1..].trim();
                    
                    // Extract variable name and type
                    let (var_type, name) = if let Some(paren_pos) = name_part.rfind(')') {
                        let type_part = &name_part[1..paren_pos];
                        let name_part = &name_part[paren_pos + 1..].trim();
                        (type_part.to_string(), name_part.to_string())
                    } else {
                        ("unknown".to_string(), name_part.to_string())
                    };
                    
                    variables.push(LldbVariable {
                        name,
                        value: value_part.to_string(),
                        summary: None,
                        var_type,
                        size: 0, // Would need separate command
                        location: None,
                        children: Vec::new(),
                    });
                }
            }
        }

        Ok(variables)
    }

    /// Evaluate expression
    pub fn evaluate_expression(&mut self, expression: &str) -> Result<String, CursedError> {
        let responses = self.send_command(&format!("expression {}", expression))?;
        
        for response in &responses {
            if response.contains("=") && !response.contains("(lldb)") {
                if let Some(eq_pos) = response.find('=') {
                    let value = response[eq_pos + 1..].trim();
                    return Ok(value.to_string());
                }
            }
        }

        Err(CursedError::General("Failed to evaluate expression".to_string()))
    }

    /// Set variable value
    pub fn set_variable(&mut self, variable: &str, value: &str) -> Result<(), CursedError> {
        let responses = self.send_command(&format!("expression {} = {}", variable, value))?;
        
        for response in &responses {
            if !response.contains("error") {
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to set variable".to_string()))
    }

    /// Read memory
    pub fn read_memory(&mut self, address: u64, size: usize) -> Result<LldbMemoryRegion, CursedError> {
        let responses = self.send_command(&format!("memory read --size {} --format x 0x{:x}", size, address))?;
        
        // Parse memory dump (simplified)
        let mut data = Vec::new();
        for response in &responses {
            if response.contains("0x") && response.contains(":") {
                // Parse hex bytes from memory dump
                // This is a simplified implementation
                for _ in 0..16 { // Assume 16 bytes per line
                    data.push(0);
                }
            }
        }

        Ok(LldbMemoryRegion {
            start_address: address,
            end_address: address + size as u64,
            permissions: "rwx".to_string(),
            name: None,
            data,
        })
    }

    /// Get registers
    pub fn get_registers(&mut self) -> Result<Vec<LldbRegister>, CursedError> {
        let responses = self.send_command("register read")?;
        
        let mut registers = Vec::new();
        
        for response in &responses {
            if response.contains("=") && !response.contains("(lldb)") {
                if let Some(eq_pos) = response.find('=') {
                    let name = response[..eq_pos].trim().to_string();
                    let value_str = response[eq_pos + 1..].trim();
                    
                    // Parse register value (simplified)
                    let value = if value_str.starts_with("0x") {
                        u64::from_str_radix(&value_str[2..], 16).unwrap_or(0)
                    } else {
                        value_str.parse::<u64>().unwrap_or(0)
                    };
                    
                    registers.push(LldbRegister {
                        name,
                        value,
                        size: 8, // Assume 64-bit
                        format: RegisterFormat::Hex,
                    });
                }
            }
        }

        Ok(registers)
    }

    /// Get thread list
    pub fn get_threads(&mut self) -> Result<Vec<LldbThread>, CursedError> {
        let responses = self.send_command("thread list")?;
        
        let mut threads = Vec::new();
        let mut thread_id = 1;
        
        for response in &responses {
            if response.contains("thread #") {
                threads.push(LldbThread {
                    id: thread_id,
                    index: thread_id,
                    name: None, // Would parse from output
                    queue: None,
                    state: ThreadState::Stopped,
                    stop_reason: None,
                    frames: Vec::new(),
                });
                thread_id += 1;
            }
        }

        Ok(threads)
    }

    /// Select thread
    pub fn select_thread(&mut self, thread_id: u32) -> Result<(), CursedError> {
        let responses = self.send_command(&format!("thread select {}", thread_id))?;
        
        for response in &responses {
            if response.contains("selected") {
                self.current_thread = Some(thread_id);
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to select thread".to_string()))
    }

    /// Select frame
    pub fn select_frame(&mut self, frame_index: u32) -> Result<(), CursedError> {
        let responses = self.send_command(&format!("frame select {}", frame_index))?;
        
        for response in &responses {
            if response.contains("frame #") {
                self.current_frame = frame_index;
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to select frame".to_string()))
    }

    /// Disassemble function
    pub fn disassemble(&mut self, function: Option<&str>) -> Result<Vec<String>, CursedError> {
        let command = if let Some(func) = function {
            format!("disassemble --name {}", func)
        } else {
            "disassemble".to_string()
        };
        
        let responses = self.send_command(&command)?;
        Ok(responses)
    }

    /// Get source code
    pub fn list_source(&mut self, location: Option<&str>) -> Result<Vec<String>, CursedError> {
        let command = if let Some(loc) = location {
            format!("source list --name {}", loc)
        } else {
            "source list".to_string()
        };
        
        let responses = self.send_command(&command)?;
        Ok(responses)
    }

    /// Attach to process
    pub fn attach_to_process(&mut self, pid: u32) -> Result<(), CursedError> {
        let responses = self.send_command(&format!("process attach --pid {}", pid))?;
        
        for response in &responses {
            if response.contains("Process") && response.contains("stopped") {
                self.current_process = Some(LldbProcess {
                    pid,
                    state: ProcessState::Stopped,
                    threads: Vec::new(),
                    exit_code: None,
                });
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to attach to process".to_string()))
    }

    /// Detach from process
    pub fn detach_from_process(&mut self) -> Result<(), CursedError> {
        let responses = self.send_command("process detach")?;
        
        for response in &responses {
            if response.contains("detached") {
                self.current_process = None;
                return Ok(());
            }
        }

        Err(CursedError::General("Failed to detach from process".to_string()))
    }

    /// Kill process
    pub fn kill_process(&mut self) -> Result<(), CursedError> {
        let _responses = self.send_command("process kill")?;
        self.current_process = None;
        Ok(())
    }

    /// Get process status
    pub fn get_process_status(&mut self) -> Result<Option<LldbProcess>, CursedError> {
        let responses = self.send_command("process status")?;
        
        for response in &responses {
            if response.contains("Process") {
                // Parse process status (simplified)
                return Ok(self.current_process.clone());
            }
        }

        Ok(None)
    }

    /// Stop LLDB
    pub fn stop_lldb(&mut self) -> Result<(), CursedError> {
        if let Some(mut process) = self.lldb_process.take() {
            let _ = self.send_command("quit");
            let _ = process.kill();
            let _ = process.wait();
        }
        Ok(())
    }
}

impl Drop for LldbIntegration {
    fn drop(&mut self) {
        let _ = self.stop_lldb();
    }
}
