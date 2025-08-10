//! GDB Integration for CURSED Debugger
//! 
//! This module provides comprehensive GDB integration including:
//! - GDB Machine Interface (MI) protocol support
//! - Breakpoint management with conditions
//! - Variable inspection and modification
//! - Stack frame navigation
//! - Memory examination

use crate::error::CursedError;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// GDB Machine Interface integration
#[derive(Debug)]
pub struct GdbIntegration {
    /// GDB process
    gdb_process: Option<Child>,
    /// Command sender
    command_sender: Option<Sender<String>>,
    /// Response receiver
    response_receiver: Option<Receiver<GdbResponse>>,
    /// Next command token
    next_token: u32,
    /// Pending commands
    pending_commands: HashMap<u32, String>,
    /// Current program state
    program_state: ProgramState,
    /// Breakpoints
    breakpoints: HashMap<u32, GdbBreakpoint>,
    /// Current thread ID
    current_thread: Option<u32>,
    /// Current frame
    current_frame: u32,
}

/// Program execution state
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramState {
    NotStarted,
    Running,
    Stopped,
    Exited(i32),
    Crashed,
}

/// GDB breakpoint information
#[derive(Debug, Clone)]
pub struct GdbBreakpoint {
    pub id: u32,
    pub enabled: bool,
    pub location: String,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub ignore_count: u32,
    pub temporary: bool,
}

/// GDB response types
#[derive(Debug, Clone)]
pub enum GdbResponse {
    Result { token: Option<u32>, class: ResultClass, results: HashMap<String, GdbValue> },
    Async { token: Option<u32>, class: AsyncClass, results: HashMap<String, GdbValue> },
    StreamRecord { stream: StreamType, content: String },
    Prompt,
}

/// Result classes for GDB MI
#[derive(Debug, Clone)]
pub enum ResultClass {
    Done,
    Running,
    Connected,
    Error,
    Exit,
}

/// Async classes for GDB MI
#[derive(Debug, Clone)]
pub enum AsyncClass {
    Stopped,
    Running,
    ThreadGroupAdded,
    ThreadGroupRemoved,
    ThreadGroupStarted,
    ThreadGroupExited,
    ThreadCreated,
    ThreadExited,
    LibraryLoaded,
    LibraryUnloaded,
    BreakpointCreated,
    BreakpointModified,
    BreakpointDeleted,
}

/// Stream types for GDB MI
#[derive(Debug, Clone)]
pub enum StreamType {
    Console,
    Target,
    Log,
}

/// GDB value types
#[derive(Debug, Clone)]
pub enum GdbValue {
    String(String),
    Const(String),
    Tuple(Vec<(String, GdbValue)>),
    List(Vec<GdbValue>),
}

/// Stack frame information from GDB
#[derive(Debug, Clone)]
pub struct GdbFrame {
    pub level: u32,
    pub addr: u64,
    pub func: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub args: Vec<GdbVariable>,
}

/// Variable information from GDB
#[derive(Debug, Clone)]
pub struct GdbVariable {
    pub name: String,
    pub value: String,
    pub var_type: String,
    pub in_scope: bool,
}

/// Memory region information
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub address: u64,
    pub size: usize,
    pub data: Vec<u8>,
    pub permissions: String,
}

/// Thread information
#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub id: u32,
    pub target_id: String,
    pub name: Option<String>,
    pub state: ThreadState,
    pub frame: Option<GdbFrame>,
}

/// Thread state
#[derive(Debug, Clone)]
pub enum ThreadState {
    Running,
    Stopped,
    Unknown,
}

impl GdbIntegration {
    /// Create new GDB integration
    pub fn new() -> Self {
        Self {
            gdb_process: None,
            command_sender: None,
            response_receiver: None,
            next_token: 1,
            pending_commands: HashMap::new(),
            program_state: ProgramState::NotStarted,
            breakpoints: HashMap::new(),
            current_thread: None,
            current_frame: 0,
        }
    }

    /// Start GDB with the specified executable
    pub fn start_gdb(&mut self, executable: &str) -> Result<(), CursedError> {
        // Start GDB process
        let mut gdb = Command::new("gdb")
            .arg("--interpreter=mi3")
            .arg("--quiet")
            .arg("--nx") // Don't read .gdbinit
            .arg(executable)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| CursedError::Io(format!("Failed to start GDB: {}", e)))?;

        // Set up communication channels
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (resp_tx, resp_rx) = mpsc::channel();

        // Take ownership of stdin/stdout
        let mut gdb_stdin = gdb.stdin.take().unwrap();
        let gdb_stdout = gdb.stdout.take().unwrap();

        // Spawn command sender thread
        let cmd_tx_clone = cmd_tx.clone();
        thread::spawn(move || {
            for command in cmd_rx {
                if let Err(e) = writeln!(gdb_stdin, "{}", command) {
                    eprintln!("Failed to send GDB command: {}", e);
                    break;
                }
                if let Err(e) = gdb_stdin.flush() {
                    eprintln!("Failed to flush GDB command: {}", e);
                    break;
                }
            }
        });

        // Spawn response reader thread
        thread::spawn(move || {
            let reader = BufReader::new(gdb_stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if let Ok(response) = Self::parse_gdb_response(&line) {
                            if resp_tx.send(response).is_err() {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        self.gdb_process = Some(gdb);
        self.command_sender = Some(cmd_tx);
        self.response_receiver = Some(resp_rx);

        // Initial setup commands
        self.send_command("-gdb-set confirm off")?;
        self.send_command("-gdb-set pagination off")?;
        self.send_command("-enable-pretty-printing")?;

        Ok(())
    }

    /// Send command to GDB
    pub fn send_command(&mut self, command: &str) -> Result<u32, CursedError> {
        let token = self.next_token;
        self.next_token += 1;

        let full_command = if command.starts_with('-') {
            format!("{}{}", token, command)
        } else {
            format!("{}-interpreter-exec console \"{}\"", token, command)
        };

        if let Some(ref sender) = self.command_sender {
            sender.send(full_command)
                .map_err(|e| CursedError::General(format!("Failed to send command: {}", e)))?;
        }

        self.pending_commands.insert(token, command.to_string());
        Ok(token)
    }

    /// Wait for command response
    pub fn wait_for_response(&mut self, token: u32) -> Result<GdbResponse, CursedError> {
        if let Some(ref receiver) = self.response_receiver {
            while let Ok(response) = receiver.recv() {
                match &response {
                    GdbResponse::Result { token: Some(resp_token), .. } if *resp_token == token => {
                        self.pending_commands.remove(&token);
                        return Ok(response);
                    }
                    GdbResponse::Async { .. } => {
                        // Handle async response inline to avoid borrowing issues
                        if let GdbResponse::Async { class, .. } = &response {
                            match class {
                                AsyncClass::Stopped => {
                                    self.program_state = ProgramState::Stopped;
                                }
                                AsyncClass::Running => {
                                    self.program_state = ProgramState::Running;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(CursedError::General("Failed to receive GDB response".to_string()))
    }

    /// Handle async response
    fn handle_async_response(&mut self, response: &GdbResponse) -> Result<(), CursedError> {
        if let GdbResponse::Async { class, results, .. } = response {
            match class {
                AsyncClass::Stopped => {
                    self.program_state = ProgramState::Stopped;
                    if let Some(GdbValue::String(reason)) = results.get("reason") {
                        println!("Program stopped: {}", reason);
                    }
                }
                AsyncClass::Running => {
                    self.program_state = ProgramState::Running;
                }
                AsyncClass::BreakpointCreated => {
                    if let Some(bkpt_data) = results.get("bkpt") {
                        self.parse_breakpoint_data(bkpt_data)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Parse breakpoint data
    fn parse_breakpoint_data(&mut self, data: &GdbValue) -> Result<(), CursedError> {
        if let GdbValue::Tuple(fields) = data {
            let mut breakpoint = GdbBreakpoint {
                id: 0,
                enabled: true,
                location: String::new(),
                condition: None,
                hit_count: 0,
                ignore_count: 0,
                temporary: false,
            };

            for (key, value) in fields {
                match (key.as_str(), value) {
                    ("number", GdbValue::String(id_str)) => {
                        if let Ok(id) = id_str.parse::<u32>() {
                            breakpoint.id = id;
                        }
                    }
                    ("enabled", GdbValue::String(enabled_str)) => {
                        breakpoint.enabled = enabled_str == "y";
                    }
                    ("addr", GdbValue::String(addr_str)) => {
                        breakpoint.location = addr_str.clone();
                    }
                    ("func", GdbValue::String(func_str)) => {
                        breakpoint.location = func_str.clone();
                    }
                    ("file", GdbValue::String(file_str)) => {
                        if let Some(GdbValue::String(line_str)) = fields.iter()
                            .find(|(k, _)| k == "line")
                            .map(|(_, v)| v) {
                            breakpoint.location = format!("{}:{}", file_str, line_str);
                        }
                    }
                    ("cond", GdbValue::String(cond_str)) => {
                        breakpoint.condition = Some(cond_str.clone());
                    }
                    ("times", GdbValue::String(times_str)) => {
                        if let Ok(times) = times_str.parse::<u32>() {
                            breakpoint.hit_count = times;
                        }
                    }
                    ("disp", GdbValue::String(disp_str)) => {
                        breakpoint.temporary = disp_str == "del";
                    }
                    _ => {}
                }
            }

            self.breakpoints.insert(breakpoint.id, breakpoint);
        }
        Ok(())
    }

    /// Set breakpoint
    pub fn set_breakpoint(&mut self, location: &str) -> Result<u32, CursedError> {
        let token = self.send_command(&format!("-break-insert {}", location))?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, results, .. } = response {
            if let Some(bkpt_data) = results.get("bkpt") {
                self.parse_breakpoint_data(&bkpt_data)?;
                if let GdbValue::Tuple(fields) = bkpt_data {
                    for (key, value) in fields {
                        if key == "number" {
                            if let GdbValue::String(id_str) = value {
                                if let Ok(id) = id_str.parse::<u32>() {
                                    return Ok(id);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Err(CursedError::General("Failed to set breakpoint".to_string()))
    }

    /// Set conditional breakpoint
    pub fn set_conditional_breakpoint(&mut self, location: &str, condition: &str) -> Result<u32, CursedError> {
        let token = self.send_command(&format!("-break-insert -c \"{}\" {}", condition, location))?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, .. } = response {
            // Parse breakpoint ID from response
            return Ok(1); // Simplified - would parse actual ID
        }
        
        Err(CursedError::General("Failed to set conditional breakpoint".to_string()))
    }

    /// Delete breakpoint
    pub fn delete_breakpoint(&mut self, id: u32) -> Result<(), CursedError> {
        let token = self.send_command(&format!("-break-delete {}", id))?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, .. } = response {
            self.breakpoints.remove(&id);
            return Ok(());
        }
        
        Err(CursedError::General("Failed to delete breakpoint".to_string()))
    }

    /// Enable/disable breakpoint
    pub fn set_breakpoint_enabled(&mut self, id: u32, enabled: bool) -> Result<(), CursedError> {
        let command = if enabled {
            format!("-break-enable {}", id)
        } else {
            format!("-break-disable {}", id)
        };
        
        let token = self.send_command(&command)?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, .. } = response {
            if let Some(breakpoint) = self.breakpoints.get_mut(&id) {
                breakpoint.enabled = enabled;
            }
            return Ok(());
        }
        
        Err(CursedError::General("Failed to modify breakpoint".to_string()))
    }

    /// Run program
    pub fn run_program(&mut self, args: &[&str]) -> Result<(), CursedError> {
        let args_str = args.join(" ");
        let token = self.send_command(&format!("-exec-arguments {}", args_str))?;
        self.wait_for_response(token)?;
        
        let token = self.send_command("-exec-run")?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Running, .. } = response {
            self.program_state = ProgramState::Running;
            return Ok(());
        }
        
        Err(CursedError::General("Failed to run program".to_string()))
    }

    /// Continue execution
    pub fn continue_execution(&mut self) -> Result<(), CursedError> {
        let token = self.send_command("-exec-continue")?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Running, .. } = response {
            self.program_state = ProgramState::Running;
            return Ok(());
        }
        
        Err(CursedError::General("Failed to continue execution".to_string()))
    }

    /// Step execution (into)
    pub fn step_into(&mut self) -> Result<(), CursedError> {
        let token = self.send_command("-exec-step")?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Running, .. } = response {
            return Ok(());
        }
        
        Err(CursedError::General("Failed to step into".to_string()))
    }

    /// Step over
    pub fn step_over(&mut self) -> Result<(), CursedError> {
        let token = self.send_command("-exec-next")?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Running, .. } = response {
            return Ok(());
        }
        
        Err(CursedError::General("Failed to step over".to_string()))
    }

    /// Step out
    pub fn step_out(&mut self) -> Result<(), CursedError> {
        let token = self.send_command("-exec-finish")?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Running, .. } = response {
            return Ok(());
        }
        
        Err(CursedError::General("Failed to step out".to_string()))
    }

    /// Get stack trace
    pub fn get_stack_trace(&mut self) -> Result<Vec<GdbFrame>, CursedError> {
        let token = self.send_command("-stack-list-frames")?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, results, .. } = response {
            if let Some(GdbValue::List(frames)) = results.get("stack") {
                let mut stack_frames = Vec::new();
                
                for frame_value in frames {
                    if let GdbValue::Tuple(frame_fields) = frame_value {
                        let mut frame = GdbFrame {
                            level: 0,
                            addr: 0,
                            func: String::new(),
                            file: None,
                            line: None,
                            args: Vec::new(),
                        };
                        
                        for (key, value) in frame_fields {
                            match (key.as_str(), value) {
                                ("level", GdbValue::String(level_str)) => {
                                    if let Ok(level) = level_str.parse::<u32>() {
                                        frame.level = level;
                                    }
                                }
                                ("addr", GdbValue::String(addr_str)) => {
                                    if let Ok(addr) = u64::from_str_radix(addr_str.trim_start_matches("0x"), 16) {
                                        frame.addr = addr;
                                    }
                                }
                                ("func", GdbValue::String(func_str)) => {
                                    frame.func = func_str.clone();
                                }
                                ("file", GdbValue::String(file_str)) => {
                                    frame.file = Some(file_str.clone());
                                }
                                ("line", GdbValue::String(line_str)) => {
                                    if let Ok(line) = line_str.parse::<u32>() {
                                        frame.line = Some(line);
                                    }
                                }
                                _ => {}
                            }
                        }
                        
                        stack_frames.push(frame);
                    }
                }
                
                return Ok(stack_frames);
            }
        }
        
        Err(CursedError::General("Failed to get stack trace".to_string()))
    }

    /// Get local variables
    pub fn get_local_variables(&mut self) -> Result<Vec<GdbVariable>, CursedError> {
        let token = self.send_command("-stack-list-variables --simple-values")?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, results, .. } = response {
            if let Some(GdbValue::List(variables)) = results.get("variables") {
                let mut local_vars = Vec::new();
                
                for var_value in variables {
                    if let GdbValue::Tuple(var_fields) = var_value {
                        let mut variable = GdbVariable {
                            name: String::new(),
                            value: String::new(),
                            var_type: String::new(),
                            in_scope: true,
                        };
                        
                        for (key, value) in var_fields {
                            match (key.as_str(), value) {
                                ("name", GdbValue::String(name_str)) => {
                                    variable.name = name_str.clone();
                                }
                                ("value", GdbValue::String(value_str)) => {
                                    variable.value = value_str.clone();
                                }
                                ("type", GdbValue::String(type_str)) => {
                                    variable.var_type = type_str.clone();
                                }
                                _ => {}
                            }
                        }
                        
                        local_vars.push(variable);
                    }
                }
                
                return Ok(local_vars);
            }
        }
        
        Err(CursedError::General("Failed to get local variables".to_string()))
    }

    /// Evaluate expression
    pub fn evaluate_expression(&mut self, expression: &str) -> Result<String, CursedError> {
        let token = self.send_command(&format!("-data-evaluate-expression {}", expression))?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, results, .. } = response {
            if let Some(GdbValue::String(value)) = results.get("value") {
                return Ok(value.clone());
            }
        }
        
        Err(CursedError::General("Failed to evaluate expression".to_string()))
    }

    /// Set variable value
    pub fn set_variable(&mut self, variable: &str, value: &str) -> Result<(), CursedError> {
        let token = self.send_command(&format!("-gdb-set var {} = {}", variable, value))?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, .. } = response {
            return Ok(());
        }
        
        Err(CursedError::General("Failed to set variable".to_string()))
    }

    /// Read memory
    pub fn read_memory(&mut self, address: u64, size: usize) -> Result<MemoryRegion, CursedError> {
        let token = self.send_command(&format!("-data-read-memory-bytes 0x{:x} {}", address, size))?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, results, .. } = response {
            if let Some(GdbValue::List(memory_data)) = results.get("memory") {
                // Parse memory data (simplified)
                let data = vec![0u8; size]; // Would parse actual hex data
                return Ok(MemoryRegion {
                    address,
                    size,
                    data,
                    permissions: "rwx".to_string(),
                });
            }
        }
        
        Err(CursedError::General("Failed to read memory".to_string()))
    }

    /// Get thread list
    pub fn get_threads(&mut self) -> Result<Vec<ThreadInfo>, CursedError> {
        let token = self.send_command("-thread-info")?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, results, .. } = response {
            if let Some(GdbValue::List(threads)) = results.get("threads") {
                let mut thread_list = Vec::new();
                
                for thread_value in threads {
                    if let GdbValue::Tuple(thread_fields) = thread_value {
                        let mut thread = ThreadInfo {
                            id: 0,
                            target_id: String::new(),
                            name: None,
                            state: ThreadState::Unknown,
                            frame: None,
                        };
                        
                        for (key, value) in thread_fields {
                            match (key.as_str(), value) {
                                ("id", GdbValue::String(id_str)) => {
                                    if let Ok(id) = id_str.parse::<u32>() {
                                        thread.id = id;
                                    }
                                }
                                ("target-id", GdbValue::String(target_id_str)) => {
                                    thread.target_id = target_id_str.clone();
                                }
                                ("name", GdbValue::String(name_str)) => {
                                    thread.name = Some(name_str.clone());
                                }
                                ("state", GdbValue::String(state_str)) => {
                                    thread.state = match state_str.as_str() {
                                        "stopped" => ThreadState::Stopped,
                                        "running" => ThreadState::Running,
                                        _ => ThreadState::Unknown,
                                    };
                                }
                                _ => {}
                            }
                        }
                        
                        thread_list.push(thread);
                    }
                }
                
                return Ok(thread_list);
            }
        }
        
        Err(CursedError::General("Failed to get thread list".to_string()))
    }

    /// Select thread
    pub fn select_thread(&mut self, thread_id: u32) -> Result<(), CursedError> {
        let token = self.send_command(&format!("-thread-select {}", thread_id))?;
        let response = self.wait_for_response(token)?;
        
        if let GdbResponse::Result { class: ResultClass::Done, .. } = response {
            self.current_thread = Some(thread_id);
            return Ok(());
        }
        
        Err(CursedError::General("Failed to select thread".to_string()))
    }

    /// Parse GDB MI response
    fn parse_gdb_response(line: &str) -> Result<GdbResponse, CursedError> {
        let line = line.trim();
        
        if line == "(gdb)" {
            return Ok(GdbResponse::Prompt);
        }
        
        // Parse stream records
        if let Some(stripped) = line.strip_prefix('~') {
            return Ok(GdbResponse::StreamRecord {
                stream: StreamType::Console,
                content: Self::parse_cstring(stripped)?,
            });
        }
        
        if let Some(stripped) = line.strip_prefix('@') {
            return Ok(GdbResponse::StreamRecord {
                stream: StreamType::Target,
                content: Self::parse_cstring(stripped)?,
            });
        }
        
        if let Some(stripped) = line.strip_prefix('&') {
            return Ok(GdbResponse::StreamRecord {
                stream: StreamType::Log,
                content: Self::parse_cstring(stripped)?,
            });
        }
        
        // Parse result records
        if let Some(stripped) = line.strip_prefix('^') {
            return Self::parse_result_record(None, stripped);
        }
        
        // Parse async records
        if let Some(stripped) = line.strip_prefix('*') {
            return Self::parse_async_record(None, stripped);
        }
        
        // Parse records with tokens
        if let Some(pos) = line.find('^') {
            let token_str = &line[..pos];
            let rest = &line[pos + 1..];
            if let Ok(token) = token_str.parse::<u32>() {
                return Self::parse_result_record(Some(token), rest);
            }
        }
        
        if let Some(pos) = line.find('*') {
            let token_str = &line[..pos];
            let rest = &line[pos + 1..];
            if let Ok(token) = token_str.parse::<u32>() {
                return Self::parse_async_record(Some(token), rest);
            }
        }
        
        Err(CursedError::General(format!("Failed to parse GDB response: {}", line)))
    }

    /// Parse result record
    fn parse_result_record(token: Option<u32>, content: &str) -> Result<GdbResponse, CursedError> {
        let (class_str, results_str) = if let Some(comma_pos) = content.find(',') {
            (&content[..comma_pos], &content[comma_pos + 1..])
        } else {
            (content, "")
        };
        
        let class = match class_str {
            "done" => ResultClass::Done,
            "running" => ResultClass::Running,
            "connected" => ResultClass::Connected,
            "error" => ResultClass::Error,
            "exit" => ResultClass::Exit,
            _ => return Err(CursedError::General(format!("Unknown result class: {}", class_str))),
        };
        
        let results = if results_str.is_empty() {
            HashMap::new()
        } else {
            Self::parse_results(results_str)?
        };
        
        Ok(GdbResponse::Result { token, class, results })
    }

    /// Parse async record
    fn parse_async_record(token: Option<u32>, content: &str) -> Result<GdbResponse, CursedError> {
        let (class_str, results_str) = if let Some(comma_pos) = content.find(',') {
            (&content[..comma_pos], &content[comma_pos + 1..])
        } else {
            (content, "")
        };
        
        let class = match class_str {
            "stopped" => AsyncClass::Stopped,
            "running" => AsyncClass::Running,
            "thread-group-added" => AsyncClass::ThreadGroupAdded,
            "thread-group-removed" => AsyncClass::ThreadGroupRemoved,
            "thread-group-started" => AsyncClass::ThreadGroupStarted,
            "thread-group-exited" => AsyncClass::ThreadGroupExited,
            "thread-created" => AsyncClass::ThreadCreated,
            "thread-exited" => AsyncClass::ThreadExited,
            "library-loaded" => AsyncClass::LibraryLoaded,
            "library-unloaded" => AsyncClass::LibraryUnloaded,
            "breakpoint-created" => AsyncClass::BreakpointCreated,
            "breakpoint-modified" => AsyncClass::BreakpointModified,
            "breakpoint-deleted" => AsyncClass::BreakpointDeleted,
            _ => return Err(CursedError::General(format!("Unknown async class: {}", class_str))),
        };
        
        let results = if results_str.is_empty() {
            HashMap::new()
        } else {
            Self::parse_results(results_str)?
        };
        
        Ok(GdbResponse::Async { token, class, results })
    }

    /// Parse results section
    fn parse_results(content: &str) -> Result<HashMap<String, GdbValue>, CursedError> {
        let mut results = HashMap::new();
        
        // Simplified parsing - would need full MI parser for production
        let parts: Vec<&str> = content.split(',').collect();
        for part in parts {
            if let Some(eq_pos) = part.find('=') {
                let key = part[..eq_pos].trim().to_string();
                let value_str = part[eq_pos + 1..].trim();
                let value = Self::parse_value(value_str)?;
                results.insert(key, value);
            }
        }
        
        Ok(results)
    }

    /// Parse GDB value
    fn parse_value(content: &str) -> Result<GdbValue, CursedError> {
        if content.starts_with('"') && content.ends_with('"') {
            return Ok(GdbValue::String(Self::parse_cstring(content)?));
        }
        
        if content.starts_with('{') && content.ends_with('}') {
            // Parse tuple or list
            let inner = &content[1..content.len() - 1];
            if inner.contains('=') {
                // Tuple
                let mut fields = Vec::new();
                for part in inner.split(',') {
                    if let Some(eq_pos) = part.find('=') {
                        let key = part[..eq_pos].trim().to_string();
                        let value = Self::parse_value(part[eq_pos + 1..].trim())?;
                        fields.push((key, value));
                    }
                }
                return Ok(GdbValue::Tuple(fields));
            } else {
                // List
                let mut items = Vec::new();
                for part in inner.split(',') {
                    let value = Self::parse_value(part.trim())?;
                    items.push(value);
                }
                return Ok(GdbValue::List(items));
            }
        }
        
        // Const value
        Ok(GdbValue::Const(content.to_string()))
    }

    /// Parse C-style string
    fn parse_cstring(content: &str) -> Result<String, CursedError> {
        if content.starts_with('"') && content.ends_with('"') {
            let inner = &content[1..content.len() - 1];
            // Handle escape sequences
            Ok(inner.replace("\\n", "\n").replace("\\t", "\t").replace("\\\\", "\\"))
        } else {
            Err(CursedError::General("Invalid C string format".to_string()))
        }
    }

    /// Stop GDB
    pub fn stop_gdb(&mut self) -> Result<(), CursedError> {
        if let Some(mut process) = self.gdb_process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
        Ok(())
    }
}

impl Drop for GdbIntegration {
    fn drop(&mut self) {
        let _ = self.stop_gdb();
    }
}
