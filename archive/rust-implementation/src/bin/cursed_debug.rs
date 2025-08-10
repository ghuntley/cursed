//! CURSED Debugger CLI Interface
//! 
//! Interactive debugger for CURSED programs with GDB/LLDB integration

use cursed::debug::{DwarfDebugGenerator, EnhancedDebugManager};
use cursed::error::CursedError;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write, BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio, Child};

/// CURSED Debugger main structure
pub struct CursedDebugger {
    /// Source file being debugged
    source_file: String,
    /// Compiled executable path
    executable_path: Option<String>,
    /// Debug information manager
    debug_manager: EnhancedDebugManager,
    /// DWARF debug generator
    dwarf_generator: DwarfDebugGenerator,
    /// Breakpoints
    breakpoints: HashMap<String, Vec<u32>>, // file -> line numbers
    /// Current debugger process
    debugger_process: Option<Child>,
    /// Debugger type (gdb or lldb)
    debugger_type: DebuggerType,
    /// Debug mode
    debug_mode: DebugMode,
    /// Variables being watched
    watch_variables: Vec<String>,
    /// Current stack frame
    current_frame: usize,
}

/// Supported debugger types
#[derive(Debug, Clone)]
pub enum DebuggerType {
    Gdb,
    Lldb,
}

/// Debug modes
#[derive(Debug, Clone)]
pub enum DebugMode {
    Interactive,
    Script(Vec<String>),
    Attach(u32), // PID
}

/// Breakpoint information
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub file: String,
    pub line: u32,
    pub condition: Option<String>,
    pub enabled: bool,
    pub hit_count: u32,
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub frame_id: usize,
    pub function_name: String,
    pub file: String,
    pub line: u32,
    pub address: u64,
}

/// Variable information
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub value: String,
    pub var_type: String,
    pub in_scope: bool,
}

impl CursedDebugger {
    /// Create new CURSED debugger
    pub fn new(source_file: String) -> Result<Self, CursedError> {
        let mut debug_manager = EnhancedDebugManager::new();
        debug_manager.enable_debug();
        debug_manager.add_source_file(&source_file)?;

        // Detect available debugger
        let debugger_type = Self::detect_debugger()?;

        Ok(Self {
            source_file,
            executable_path: None,
            debug_manager,
            dwarf_generator: DwarfDebugGenerator::new(),
            breakpoints: HashMap::new(),
            debugger_process: None,
            debugger_type,
            debug_mode: DebugMode::Interactive,
            watch_variables: Vec::new(),
            current_frame: 0,
        })
    }

    /// Detect available debugger
    fn detect_debugger() -> Result<DebuggerType, CursedError> {
        // Try GDB first
        if Command::new("gdb").arg("--version").output().is_ok() {
            return Ok(DebuggerType::Gdb);
        }

        // Try LLDB
        if Command::new("lldb").arg("--version").output().is_ok() {
            return Ok(DebuggerType::Lldb);
        }

        Err(CursedError::General("No compatible debugger found. Please install GDB or LLDB.".to_string()))
    }

    /// Compile source file with debug information
    pub fn compile_with_debug(&mut self) -> Result<(), CursedError> {
        println!("🔧 Compiling {} with debug information...", self.source_file);

        let executable_name = Path::new(&self.source_file)
            .file_stem()
            .ok_or_else(|| CursedError::General("Invalid source file name".to_string()))?
            .to_string_lossy();
            
        let executable_path = format!("{}_debug", executable_name);

        // Generate debug sections
        let debug_sections = self.dwarf_generator.encode_debug_sections()?;
        
        // Use async runtime to call async function
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| CursedError::General(format!("Failed to create runtime: {}", e)))?;
        
        rt.block_on(cursed::compile_with_debug(&self.source_file, &executable_path, debug_sections))?;
        
        self.executable_path = Some(executable_path.clone());
        
        println!("✅ Compiled successfully: {}", executable_path);
        Ok(())
    }

    /// Start interactive debugging session
    pub fn start_debug_session(&mut self) -> Result<(), CursedError> {
        if self.executable_path.is_none() {
            self.compile_with_debug()?;
        }

        let executable = self.executable_path.clone().unwrap();
        
        println!("🚀 Starting debug session for {}", executable);
        println!("Type 'help' for available commands");

        // Start debugger process
        self.start_debugger(&executable)?;

        // Interactive command loop
        self.command_loop()
    }

    /// Start debugger process
    fn start_debugger(&mut self, executable: &str) -> Result<(), CursedError> {
        let mut cmd = match self.debugger_type {
            DebuggerType::Gdb => {
                let mut cmd = Command::new("gdb");
                cmd.arg("--interpreter=mi3")
                    .arg("--quiet")
                    .arg(executable);
                cmd
            }
            DebuggerType::Lldb => {
                let mut cmd = Command::new("lldb");
                cmd.arg("--batch")
                    .arg("-o").arg(&format!("file {}", executable));
                cmd
            }
        };

        let process = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| CursedError::Io(format!("Failed to start debugger: {}", e)))?;

        self.debugger_process = Some(process);
        println!("✅ Debugger started successfully");
        Ok(())
    }

    /// Interactive command loop
    fn command_loop(&mut self) -> Result<(), CursedError> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("(cursed-debug) ");
            stdout.flush().unwrap();

            let mut input = String::new();
            stdin.read_line(&mut input)
                .map_err(|e| CursedError::Io(format!("Failed to read input: {}", e)))?;

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            match self.process_command(input) {
                Ok(should_continue) => {
                    if !should_continue {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        self.cleanup();
        Ok(())
    }

    /// Process debug command
    fn process_command(&mut self, input: &str) -> Result<bool, CursedError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(true);
        }

        match parts[0] {
            "help" | "h" => {
                self.print_help();
            }
            "run" | "r" => {
                self.run_program(&parts[1..])?;
            }
            "break" | "b" => {
                if parts.len() >= 2 {
                    self.set_breakpoint(&parts[1..])?;
                } else {
                    self.list_breakpoints();
                }
            }
            "continue" | "c" => {
                self.continue_execution()?;
            }
            "step" | "s" => {
                self.step_execution()?;
            }
            "next" | "n" => {
                self.next_execution()?;
            }
            "finish" | "f" => {
                self.finish_function()?;
            }
            "backtrace" | "bt" => {
                self.print_backtrace()?;
            }
            "frame" | "fr" => {
                if parts.len() >= 2 {
                    self.select_frame(parts[1])?;
                } else {
                    self.print_current_frame()?;
                }
            }
            "print" | "p" => {
                if parts.len() >= 2 {
                    self.print_variable(&parts[1..])?;
                } else {
                    println!("Usage: print <variable>");
                }
            }
            "watch" | "w" => {
                if parts.len() >= 2 {
                    self.watch_variable(parts[1])?;
                } else {
                    self.list_watch_variables();
                }
            }
            "info" | "i" => {
                if parts.len() >= 2 {
                    self.info_command(&parts[1..])?;
                } else {
                    println!("Usage: info <topic>");
                }
            }
            "list" | "l" => {
                if parts.len() >= 2 {
                    self.list_source(Some(parts[1]))?;
                } else {
                    self.list_source(None)?;
                }
            }
            "disassemble" | "disas" => {
                if parts.len() >= 2 {
                    self.disassemble(Some(parts[1]))?;
                } else {
                    self.disassemble(None)?;
                }
            }
            "delete" | "d" => {
                if parts.len() >= 2 {
                    self.delete_breakpoint(parts[1])?;
                } else {
                    println!("Usage: delete <breakpoint_id>");
                }
            }
            "enable" | "en" => {
                if parts.len() >= 2 {
                    self.enable_breakpoint(parts[1])?;
                } else {
                    println!("Usage: enable <breakpoint_id>");
                }
            }
            "disable" | "dis" => {
                if parts.len() >= 2 {
                    self.disable_breakpoint(parts[1])?;
                } else {
                    println!("Usage: disable <breakpoint_id>");
                }
            }
            "set" => {
                if parts.len() >= 3 {
                    self.set_variable(&parts[1], &parts[2..])?;
                } else {
                    println!("Usage: set <variable> <value>");
                }
            }
            "quit" | "q" | "exit" => {
                return Ok(false);
            }
            _ => {
                println!("Unknown command: {}. Type 'help' for available commands.", parts[0]);
            }
        }

        Ok(true)
    }

    /// Print help information
    fn print_help(&self) {
        println!("CURSED Debugger Commands:");
        println!("  help, h                 - Show this help");
        println!("  run, r [args]           - Run the program");
        println!("  break, b <location>     - Set breakpoint");
        println!("  continue, c             - Continue execution");
        println!("  step, s                 - Step into");
        println!("  next, n                 - Step over");
        println!("  finish, f               - Step out");
        println!("  backtrace, bt           - Show stack trace");
        println!("  frame, fr [n]           - Select frame");
        println!("  print, p <var>          - Print variable");
        println!("  watch, w <var>          - Watch variable");
        println!("  info, i <topic>         - Show information");
        println!("  list, l [location]      - List source code");
        println!("  disassemble, disas      - Show disassembly");
        println!("  delete, d <id>          - Delete breakpoint");
        println!("  enable, en <id>         - Enable breakpoint");
        println!("  disable, dis <id>       - Disable breakpoint");
        println!("  set <var> <value>       - Set variable value");
        println!("  quit, q, exit           - Exit debugger");
    }

    /// Run program with arguments
    fn run_program(&mut self, args: &[&str]) -> Result<(), CursedError> {
        println!("🏃 Running program with args: {:?}", args);
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => format!("run {}", args.join(" ")),
            DebuggerType::Lldb => format!("process launch -- {}", args.join(" ")),
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// Set breakpoint
    fn set_breakpoint(&mut self, location: &[&str]) -> Result<(), CursedError> {
        let location_str = location.join(" ");
        println!("🔴 Setting breakpoint at: {}", location_str);

        // Parse location (file:line or function_name)
        if let Some(colon_pos) = location_str.find(':') {
            let file = &location_str[..colon_pos];
            let line_str = &location_str[colon_pos + 1..];
            if let Ok(line) = line_str.parse::<u32>() {
                self.breakpoints.entry(file.to_string()).or_insert_with(Vec::new).push(line);
            }
        }

        let cmd = match self.debugger_type {
            DebuggerType::Gdb => format!("break {}", location_str),
            DebuggerType::Lldb => format!("breakpoint set --name {}", location_str),
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// List breakpoints
    fn list_breakpoints(&self) {
        println!("📍 Breakpoints:");
        for (file, lines) in &self.breakpoints {
            for line in lines {
                println!("  {}:{}", file, line);
            }
        }
    }

    /// Continue execution
    fn continue_execution(&mut self) -> Result<(), CursedError> {
        println!("▶️ Continuing execution...");
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => "continue",
            DebuggerType::Lldb => "continue",
        };
        
        self.send_debugger_command(cmd)?;
        Ok(())
    }

    /// Step execution (into)
    fn step_execution(&mut self) -> Result<(), CursedError> {
        println!("👣 Stepping into...");
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => "step",
            DebuggerType::Lldb => "step",
        };
        
        self.send_debugger_command(cmd)?;
        Ok(())
    }

    /// Next execution (over)
    fn next_execution(&mut self) -> Result<(), CursedError> {
        println!("⏭️ Stepping over...");
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => "next",
            DebuggerType::Lldb => "next",
        };
        
        self.send_debugger_command(cmd)?;
        Ok(())
    }

    /// Finish function (step out)
    fn finish_function(&mut self) -> Result<(), CursedError> {
        println!("🏁 Finishing function...");
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => "finish",
            DebuggerType::Lldb => "finish",
        };
        
        self.send_debugger_command(cmd)?;
        Ok(())
    }

    /// Print backtrace
    fn print_backtrace(&mut self) -> Result<(), CursedError> {
        println!("📚 Stack trace:");
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => "backtrace",
            DebuggerType::Lldb => "bt",
        };
        
        self.send_debugger_command(cmd)?;
        Ok(())
    }

    /// Select frame
    fn select_frame(&mut self, frame_str: &str) -> Result<(), CursedError> {
        if let Ok(frame_num) = frame_str.parse::<usize>() {
            self.current_frame = frame_num;
            println!("🎯 Selected frame {}", frame_num);
            
            let cmd = match self.debugger_type {
                DebuggerType::Gdb => format!("frame {}", frame_num),
                DebuggerType::Lldb => format!("frame select {}", frame_num),
            };
            
            self.send_debugger_command(&cmd)?;
        } else {
            return Err(CursedError::General("Invalid frame number".to_string()));
        }
        Ok(())
    }

    /// Print current frame
    fn print_current_frame(&self) -> Result<(), CursedError> {
        println!("📍 Current frame: {}", self.current_frame);
        Ok(())
    }

    /// Print variable
    fn print_variable(&mut self, var_parts: &[&str]) -> Result<(), CursedError> {
        let var_name = var_parts.join(" ");
        println!("🔍 Printing variable: {}", var_name);
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => format!("print {}", var_name),
            DebuggerType::Lldb => format!("print {}", var_name),
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// Watch variable
    fn watch_variable(&mut self, var_name: &str) -> Result<(), CursedError> {
        self.watch_variables.push(var_name.to_string());
        println!("👁️ Watching variable: {}", var_name);
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => format!("watch {}", var_name),
            DebuggerType::Lldb => format!("watchpoint set variable {}", var_name),
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// List watch variables
    fn list_watch_variables(&self) {
        println!("👁️ Watched variables:");
        for var in &self.watch_variables {
            println!("  {}", var);
        }
    }

    /// Info command
    fn info_command(&mut self, args: &[&str]) -> Result<(), CursedError> {
        let topic = args[0];
        match topic {
            "breakpoints" => {
                self.list_breakpoints();
            }
            "variables" | "locals" => {
                let cmd = match self.debugger_type {
                    DebuggerType::Gdb => "info locals",
                    DebuggerType::Lldb => "frame variable",
                };
                self.send_debugger_command(cmd)?;
            }
            "registers" => {
                let cmd = match self.debugger_type {
                    DebuggerType::Gdb => "info registers",
                    DebuggerType::Lldb => "register read",
                };
                self.send_debugger_command(cmd)?;
            }
            "threads" => {
                let cmd = match self.debugger_type {
                    DebuggerType::Gdb => "info threads",
                    DebuggerType::Lldb => "thread list",
                };
                self.send_debugger_command(cmd)?;
            }
            _ => {
                println!("Available info topics: breakpoints, variables, registers, threads");
            }
        }
        Ok(())
    }

    /// List source code
    fn list_source(&mut self, location: Option<&str>) -> Result<(), CursedError> {
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => {
                if let Some(loc) = location {
                    format!("list {}", loc)
                } else {
                    "list".to_string()
                }
            }
            DebuggerType::Lldb => {
                if let Some(loc) = location {
                    format!("source list --name {}", loc)
                } else {
                    "source list".to_string()
                }
            }
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// Disassemble code
    fn disassemble(&mut self, location: Option<&str>) -> Result<(), CursedError> {
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => {
                if let Some(loc) = location {
                    format!("disassemble {}", loc)
                } else {
                    "disassemble".to_string()
                }
            }
            DebuggerType::Lldb => {
                if let Some(loc) = location {
                    format!("disassemble --name {}", loc)
                } else {
                    "disassemble".to_string()
                }
            }
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// Delete breakpoint
    fn delete_breakpoint(&mut self, id: &str) -> Result<(), CursedError> {
        println!("🗑️ Deleting breakpoint: {}", id);
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => format!("delete {}", id),
            DebuggerType::Lldb => format!("breakpoint delete {}", id),
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// Enable breakpoint
    fn enable_breakpoint(&mut self, id: &str) -> Result<(), CursedError> {
        println!("✅ Enabling breakpoint: {}", id);
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => format!("enable {}", id),
            DebuggerType::Lldb => format!("breakpoint enable {}", id),
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// Disable breakpoint
    fn disable_breakpoint(&mut self, id: &str) -> Result<(), CursedError> {
        println!("❌ Disabling breakpoint: {}", id);
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => format!("disable {}", id),
            DebuggerType::Lldb => format!("breakpoint disable {}", id),
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// Set variable value
    fn set_variable(&mut self, var_name: &str, value_parts: &[&str]) -> Result<(), CursedError> {
        let value = value_parts.join(" ");
        println!("📝 Setting {} = {}", var_name, value);
        
        let cmd = match self.debugger_type {
            DebuggerType::Gdb => format!("set variable {} = {}", var_name, value),
            DebuggerType::Lldb => format!("expression {} = {}", var_name, value),
        };
        
        self.send_debugger_command(&cmd)?;
        Ok(())
    }

    /// Send command to debugger
    fn send_debugger_command(&mut self, command: &str) -> Result<(), CursedError> {
        if let Some(ref mut process) = self.debugger_process {
            if let Some(mut stdin) = process.stdin.take() {
                writeln!(stdin, "{}", command)
                    .map_err(|e| CursedError::Io(format!("Failed to send command: {}", e)))?;
                stdin.flush()
                    .map_err(|e| CursedError::Io(format!("Failed to flush command: {}", e)))?;
                process.stdin = Some(stdin);
            }
            
            // Read response (simplified)
            if let Some(mut stdout) = process.stdout.take() {
                let mut reader = BufReader::new(&mut stdout);
                for line in reader.by_ref().lines().take(10) { // Read up to 10 lines
                    if let Ok(line) = line {
                        println!("{}", line);
                        if line.contains("(gdb)") || line.contains("(lldb)") {
                            break;
                        }
                    }
                }
                process.stdout = Some(stdout);
            }
        }
        Ok(())
    }

    /// Cleanup debugger process
    fn cleanup(&mut self) {
        if let Some(mut process) = self.debugger_process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
        println!("👋 Debug session ended");
    }
}

/// Main entry point for CURSED debugger
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file.csd> [options]", args[0]);
        eprintln!("");
        eprintln!("Options:");
        eprintln!("  --gdb          Force use of GDB");
        eprintln!("  --lldb         Force use of LLDB");
        eprintln!("  --attach <pid> Attach to running process");
        eprintln!("  --script <file> Run debug script");
        return Ok(());
    }

    let source_file = &args[1];
    
    // Verify source file exists
    if !Path::new(source_file).exists() {
        eprintln!("Error: Source file '{}' not found", source_file);
        return Ok(());
    }

    println!("🐛 CURSED Debugger v1.0");
    println!("📁 Source file: {}", source_file);

    // Create and start debugger
    let mut debugger = CursedDebugger::new(source_file.clone())
        .map_err(|e| format!("Failed to create debugger: {}", e))?;

    // Process command line options
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--gdb" => {
                debugger.debugger_type = DebuggerType::Gdb;
            }
            "--lldb" => {
                debugger.debugger_type = DebuggerType::Lldb;
            }
            "--attach" => {
                if i + 1 < args.len() {
                    if let Ok(pid) = args[i + 1].parse::<u32>() {
                        debugger.debug_mode = DebugMode::Attach(pid);
                        i += 1;
                    }
                }
            }
            "--script" => {
                if i + 1 < args.len() {
                    let script_content = fs::read_to_string(&args[i + 1])
                        .map_err(|e| format!("Failed to read script file: {}", e))?;
                    let commands: Vec<String> = script_content.lines().map(|s| s.to_string()).collect();
                    debugger.debug_mode = DebugMode::Script(commands);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Start debug session
    debugger.start_debug_session()
        .map_err(|e| format!("Debug session failed: {}", e))?;

    Ok(())
}

impl Drop for CursedDebugger {
    fn drop(&mut self) {
        self.cleanup();
    }
}
