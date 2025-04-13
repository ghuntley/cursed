use crate::lexer::token::Token;
use crate::memory::{Traceable, HeapObject, GcHeapObject, Trace};
use crate::object::{Environment, Object, ObjectType};
use crate::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;

// Flag constants for Logger
pub const LDATE: i64 = 1 << 0;         // the date: 2025/04/13
pub const LTIME: i64 = 1 << 1;         // the time: 15:04:05
pub const LMICROSECONDS: i64 = 1 << 2; // microsecond resolution: 15:04:05.123123
pub const LLONGFILE: i64 = 1 << 3;     // full file name and line number: /a/b/c/d.go:23
pub const LSHORTFILE: i64 = 1 << 4;    // final file name element and line number: d.go:23
pub const LUTC: i64 = 1 << 5;          // use UTC rather than local time zone
pub const LMSGPREFIX: i64 = 1 << 6;    // move prefix to before message
pub const LSTDFLAGS: i64 = LDATE | LTIME; // initial values for the standard logger

// WriterObject represents any type that implements Write
#[derive(Clone)]
pub struct WriterObject {
    pub writer: Rc<RefCell<dyn Write>>,
}

impl WriterObject {
    pub fn new<W: Write + 'static>(writer: W) -> Self {
        WriterObject {
            writer: Rc::new(RefCell::new(writer)),
        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.writer.borrow_mut().write(buf)
    }
}

impl Traceable for WriterObject {
    fn trace(&self, trace: &mut Trace) {
        // No heap objects to trace
    }
}

// Logger represents an active logging object
#[derive(Clone)]
pub struct Logger {
    writer: WriterObject,
    prefix: String,
    flag: i64,
}

impl Logger {
    pub fn new(writer: WriterObject, prefix: String, flag: i64) -> Self {
        Logger {
            writer,
            prefix,
            flag,
        }
    }

    // Returns the prefix
    pub fn prefix(&self) -> String {
        self.prefix.clone()
    }

    // Sets the prefix
    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }

    // Returns the flags
    pub fn flags(&self) -> i64 {
        self.flag
    }

    // Sets the flags
    pub fn set_flags(&mut self, flag: i64) {
        self.flag = flag;
    }

    // Returns the writer
    pub fn writer(&self) -> WriterObject {
        self.writer.clone()
    }

    // Sets the writer
    pub fn set_output(&mut self, writer: WriterObject) {
        self.writer = writer;
    }

    // Formats the header for log messages
    fn format_header(&self, buf: &mut String, calldepth: i32, file: Option<&str>, line: Option<i32>) {
        // If Lmsgprefix is set, we put the prefix at the start of the message instead of the header
        if (self.flag & LMSGPREFIX) == 0 {
            buf.push_str(&self.prefix);
        }

        // Add date if flag is set
        if (self.flag & (LDATE | LTIME)) != 0 {
            let now = SystemTime::now();
            let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
            let secs = duration.as_secs() as i64;
            let nsecs = duration.subsec_nanos() as i64;

            if (self.flag & LDATE) != 0 {
                let (year, month, day) = timestamp_to_date(secs, (self.flag & LUTC) != 0);
                buf.push_str(&format!("{:04}/{:02}/{:02} ", year, month, day));
            }

            if (self.flag & LTIME) != 0 {
                let (hour, min, sec) = timestamp_to_time(secs, (self.flag & LUTC) != 0);
                if (self.flag & LMICROSECONDS) != 0 {
                    let micros = nsecs / 1000;
                    buf.push_str(&format!("{:02}:{:02}:{:02}.{:06} ", hour, min, sec, micros));
                } else {
                    buf.push_str(&format!("{:02}:{:02}:{:02} ", hour, min, sec));
                }
            }
        }

        // Add file and line if flag is set
        if (self.flag & (LSHORTFILE | LLONGFILE)) != 0 {
            if let (Some(file_path), Some(line_num)) = (file, line) {
                let filename = if (self.flag & LSHORTFILE) != 0 {
                    Path::new(file_path)
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or(file_path)
                } else {
                    file_path
                };
                buf.push_str(&format!("{:}:{:} ", filename, line_num));
            }
        }

        // If Lmsgprefix is set, put the prefix here
        if (self.flag & LMSGPREFIX) != 0 {
            buf.push_str(&self.prefix);
        }
    }

    // Low-level output method
    pub fn output(&self, calldepth: i32, s: String, file: Option<&str>, line: Option<i32>) -> Result<(), io::Error> {
        let mut buf = String::new();
        self.format_header(&mut buf, calldepth, file, line);
        
        buf.push_str(&s);
        if !s.ends_with('\n') {
            buf.push('\n');
        }

        self.writer.write(buf.as_bytes()).map(|_| ())
    }

    // Print outputs a message
    pub fn print(&self, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> Result<(), io::Error> {
        let message = format_objects(args);
        self.output(2, message, file, line)
    }

    // Printf outputs a formatted message
    pub fn printf(&self, format: String, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> Result<(), io::Error> {
        let message = format_with_args(format, args);
        self.output(2, message, file, line)
    }

    // Println outputs a message with a newline
    pub fn println(&self, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> Result<(), io::Error> {
        let mut message = format_objects(args);
        message.push('\n');
        self.output(2, message, file, line)
    }

    // Fatal outputs a message and calls exit(1)
    pub fn fatal(&self, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
        let _ = self.print(args, file, line);
        std::process::exit(1);
    }

    // Fatalf outputs a formatted message and calls exit(1)
    pub fn fatalf(&self, format: String, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
        let _ = self.printf(format, args, file, line);
        std::process::exit(1);
    }

    // Fatalln outputs a message with a newline and calls exit(1)
    pub fn fatalln(&self, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
        let _ = self.println(args, file, line);
        std::process::exit(1);
    }

    // Panic outputs a message and calls panic
    pub fn panic(&self, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
        let message = format_objects(args);
        let _ = self.output(2, message.clone(), file, line);
        panic!("{}", message);
    }

    // Panicf outputs a formatted message and calls panic
    pub fn panicf(&self, format: String, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
        let message = format_with_args(format, args);
        let _ = self.output(2, message.clone(), file, line);
        panic!("{}", message);
    }

    // Panicln outputs a message with a newline and calls panic
    pub fn panicln(&self, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
        let mut message = format_objects(args);
        if !message.ends_with('\n') {
            message.push('\n');
        }
        let _ = self.output(2, message.clone(), file, line);
        panic!("{}", message);
    }
}

impl Traceable for Logger {
    fn trace(&self, trace: &mut Trace) {
        self.writer.trace(trace);
    }
}

// Helper functions for time formatting
fn timestamp_to_date(timestamp: i64, utc: bool) -> (i32, i32, i32) {
    // For simplicity, this is a basic implementation
    // A more accurate one would use chrono or similar
    use time::OffsetDateTime;
    let dt = if utc {
        OffsetDateTime::from_unix_timestamp(timestamp).unwrap().to_utc()
    } else {
        OffsetDateTime::from_unix_timestamp(timestamp).unwrap().to_offset(
            time::UtcOffset::local_offset_at(timestamp).unwrap_or(time::UtcOffset::UTC)
        )
    };
    (dt.year(), dt.month() as i32, dt.day())
}

fn timestamp_to_time(timestamp: i64, utc: bool) -> (i32, i32, i32) {
    use time::OffsetDateTime;
    let dt = if utc {
        OffsetDateTime::from_unix_timestamp(timestamp).unwrap().to_utc()
    } else {
        OffsetDateTime::from_unix_timestamp(timestamp).unwrap().to_offset(
            time::UtcOffset::local_offset_at(timestamp).unwrap_or(time::UtcOffset::UTC)
        )
    };
    (dt.hour(), dt.minute(), dt.second())
}

// Format a sequence of objects as a string (similar to fmt.Print)
fn format_objects(args: Vec<Object>) -> String {
    args.iter()
        .map(|arg| format!("{}", arg))
        .collect::<Vec<String>>()
        .join(" ")
}

// Format with sprintf-like functionality
fn format_with_args(format_str: String, args: Vec<Object>) -> String {
    // A simple implementation - in a real one you'd parse format specifiers correctly
    let mut result = format_str.clone();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("%{}", i + 1);
        result = result.replace(&placeholder, &format!("{}", arg));
        
        // Also replace %v, %s, %d for the first few arguments
        if i == 0 {
            result = result.replace("%v", &format!("{}", arg));
            result = result.replace("%s", &format!("{}", arg));
            result = result.replace("%d", &format!("{}", arg));
        }
    }
    result
}

// Standard logger instance
thread_local! {
    static STD_LOGGER: RefCell<Logger> = RefCell::new({
        let stderr = WriterObject::new(io::stderr());
        Logger::new(stderr, String::new(), LSTDFLAGS)
    });
}

// Module functions
pub fn prefix() -> String {
    STD_LOGGER.with(|logger| logger.borrow().prefix())
}

pub fn set_prefix(prefix: String) {
    STD_LOGGER.with(|logger| logger.borrow_mut().set_prefix(prefix))
}

pub fn flags() -> i64 {
    STD_LOGGER.with(|logger| logger.borrow().flags())
}

pub fn set_flags(flag: i64) {
    STD_LOGGER.with(|logger| logger.borrow_mut().set_flags(flag))
}

pub fn set_output(writer: WriterObject) {
    STD_LOGGER.with(|logger| logger.borrow_mut().set_output(writer))
}

pub fn writer() -> WriterObject {
    STD_LOGGER.with(|logger| logger.borrow().writer())
}

pub fn print(args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> Result<(), io::Error> {
    STD_LOGGER.with(|logger| logger.borrow().print(args, file, line))
}

pub fn printf(format: String, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> Result<(), io::Error> {
    STD_LOGGER.with(|logger| logger.borrow().printf(format, args, file, line))
}

pub fn println(args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> Result<(), io::Error> {
    STD_LOGGER.with(|logger| logger.borrow().println(args, file, line))
}

pub fn fatal(args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
    STD_LOGGER.with(|logger| logger.borrow().fatal(args, file, line))
}

pub fn fatalf(format: String, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
    STD_LOGGER.with(|logger| logger.borrow().fatalf(format, args, file, line))
}

pub fn fatalln(args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
    STD_LOGGER.with(|logger| logger.borrow().fatalln(args, file, line))
}

pub fn panic(args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
    STD_LOGGER.with(|logger| logger.borrow().panic(args, file, line))
}

pub fn panicf(format: String, args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
    STD_LOGGER.with(|logger| logger.borrow().panicf(format, args, file, line))
}

pub fn panicln(args: Vec<Object>, file: Option<&str>, line: Option<i32>) -> ! {
    STD_LOGGER.with(|logger| logger.borrow().panicln(args, file, line))
}

// Add the oglogging module to the standard library
pub fn init_oglogging(env: &mut Environment) {
    // Create the oglogging module
    let module = Object::new_module("oglogging".to_string());
    let mut module_env = module.get_module_env().unwrap();
    
    // Add constants
    module_env.define_builtin("Ldate", Object::new_int(LDATE));
    module_env.define_builtin("Ltime", Object::new_int(LTIME));
    module_env.define_builtin("Lmicroseconds", Object::new_int(LMICROSECONDS));
    module_env.define_builtin("Llongfile", Object::new_int(LLONGFILE));
    module_env.define_builtin("Lshortfile", Object::new_int(LSHORTFILE));
    module_env.define_builtin("LUTC", Object::new_int(LUTC));
    module_env.define_builtin("Lmsgprefix", Object::new_int(LMSGPREFIX));
    module_env.define_builtin("LstdFlags", Object::new_int(LSTDFLAGS));

    // Add the new function
    module_env.define_builtin("new", Object::new_builtin_function("new".to_string(), |args| {
        if args.len() < 3 {
            return Err("new requires writer, prefix, and flags".to_string());
        }
        
        let writer = match &args[0] {
            Object::OStream(stream) => WriterObject::new(stream.clone()),
            _ => return Err("first argument must be a Writer".to_string())
        };
        
        let prefix = match &args[1] {
            Object::OString(s) => s.clone(),
            _ => return Err("second argument must be a string".to_string())
        };
        
        let flags = match &args[2] {
            Object::OInteger(i) => *i,
            _ => return Err("third argument must be an integer".to_string())
        };
        
        let logger = Logger::new(writer, prefix, flags);
        Ok(Object::new_external(logger))
    }));
    
    // Add the standard logger functions
    module_env.define_builtin("spill", Object::new_builtin_function("spill".to_string(), |args| {
        print(args, None, None)?;
        Ok(Object::NULL)
    }));
    
    module_env.define_builtin("spillf", Object::new_builtin_function("spillf".to_string(), |args| {
        if args.is_empty() {
            return Err("spillf requires a format string".to_string());
        }
        
        let format = match &args[0] {
            Object::OString(s) => s.clone(),
            _ => return Err("first argument must be a string".to_string())
        };
        
        let args = args[1..].to_vec();
        printf(format, args, None, None)?;
        Ok(Object::NULL)
    }));
    
    module_env.define_builtin("fatal", Object::new_builtin_function("fatal".to_string(), |args| {
        fatal(args, None, None);
        // This never returns
        Ok(Object::NULL)
    }));
    
    module_env.define_builtin("fatalf", Object::new_builtin_function("fatalf".to_string(), |args| {
        if args.is_empty() {
            return Err("fatalf requires a format string".to_string());
        }
        
        let format = match &args[0] {
            Object::OString(s) => s.clone(),
            _ => return Err("first argument must be a string".to_string())
        };
        
        let args = args[1..].to_vec();
        fatalf(format, args, None, None);
        // This never returns
        Ok(Object::NULL)
    }));
    
    module_env.define_builtin("panic", Object::new_builtin_function("panic".to_string(), |args| {
        panic(args, None, None);
        // This never returns
        Ok(Object::NULL)
    }));
    
    module_env.define_builtin("panicf", Object::new_builtin_function("panicf".to_string(), |args| {
        if args.is_empty() {
            return Err("panicf requires a format string".to_string());
        }
        
        let format = match &args[0] {
            Object::OString(s) => s.clone(),
            _ => return Err("first argument must be a string".to_string())
        };
        
        let args = args[1..].to_vec();
        panicf(format, args, None, None);
        // This never returns
        Ok(Object::NULL)
    }));
    
    module_env.define_builtin("setFlags", Object::new_builtin_function("setFlags".to_string(), |args| {
        if args.len() != 1 {
            return Err("setFlags requires one argument".to_string());
        }
        
        let flags = match &args[0] {
            Object::OInteger(i) => *i,
            _ => return Err("argument must be an integer".to_string())
        };
        
        set_flags(flags);
        Ok(Object::NULL)
    }));
    
    module_env.define_builtin("setPrefix", Object::new_builtin_function("setPrefix".to_string(), |args| {
        if args.len() != 1 {
            return Err("setPrefix requires one argument".to_string());
        }
        
        let prefix = match &args[0] {
            Object::OString(s) => s.clone(),
            _ => return Err("argument must be a string".to_string())
        };
        
        set_prefix(prefix);
        Ok(Object::NULL)
    }));
    
    module_env.define_builtin("setOutput", Object::new_builtin_function("setOutput".to_string(), |args| {
        if args.len() != 1 {
            return Err("setOutput requires one argument".to_string());
        }
        
        let writer = match &args[0] {
            Object::OStream(stream) => WriterObject::new(stream.clone()),
            _ => return Err("argument must be a Writer".to_string())
        };
        
        set_output(writer);
        Ok(Object::NULL)
    }));
    
    env.define_module("oglogging".to_string(), module);
}