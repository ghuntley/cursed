//! Syslog client package inspired by Go's log/syslog
//! Provides functionality to send log messages to syslog servers

use crate::prelude::*;
use crate::core::value::Value;
use crate::object::*;
use crate::memory::gc::{Trace, Traceable, GcCell};
use std::io::{self, Write};
use std::net::{TcpStream, UdpSocket, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Syslog facility codes
pub const KERNEL: i64 = 0;
pub const USER_LEVEL: i64 = 1;
pub const MAIL: i64 = 2;
pub const SYSTEM: i64 = 3;
pub const AUTH: i64 = 4;
pub const SYSLOGD: i64 = 5;
pub const PRINTER: i64 = 6;
pub const NET_NEWS: i64 = 7;
pub const UUCP: i64 = 8;
pub const CLOCK: i64 = 9;
pub const AUTH_PRIV: i64 = 10;
pub const FTP: i64 = 11;
pub const NTP: i64 = 12;
pub const LOG_AUDIT: i64 = 13;
pub const LOG_ALERT: i64 = 14;
pub const CRON: i64 = 15;
pub const LOCAL0: i64 = 16;
pub const LOCAL1: i64 = 17;
pub const LOCAL2: i64 = 18;
pub const LOCAL3: i64 = 19;
pub const LOCAL4: i64 = 20;
pub const LOCAL5: i64 = 21;
pub const LOCAL6: i64 = 22;
pub const LOCAL7: i64 = 23;

/// Syslog severity codes
pub const EMERG: i64 = 0;
pub const ALERT: i64 = 1;
pub const CRIT: i64 = 2;
pub const ERR: i64 = 3;
pub const WARNING: i64 = 4;
pub const NOTICE: i64 = 5;
pub const INFO: i64 = 6;
pub const DEBUG: i64 = 7;

/// Connection type for the syslog writer
#[derive(Debug, Clone)]
enum Connection {
    Udp(Arc<Mutex<UdpSocket>>),
    Tcp(Arc<Mutex<TcpStream>>),
    None,
}

/// A Writer is a connection to a syslog server
#[derive(Debug, Clone)]
pub struct Writer {
    conn: Connection,
    priority: i64,
    tag: String,
    hostname: String,
}

impl Traceable for Writer {
    fn trace(&self) {
        // No tracing needed for the connection or other fields
    }
}

impl Writer {
    /// Create a new Writer
    fn new(conn: Connection, priority: i64, tag: String) -> Self {
        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| gethostname::gethostname().into_string())
            .unwrap_or_else(|_| String::from("localhost"));

        Writer {
            conn,
            priority,
            tag,
            hostname,
        }
    }

    /// Format a syslog message according to RFC 5424
    fn format_message(&self, severity: i64, msg: &str) -> String {
        let facility = self.priority & 0xf8;
        let priority = facility | severity;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // <PRI>TIMESTAMP HOSTNAME TAG: MSG
        format!("<{}>1 {} {} {} - - - {}", 
            priority, 
            timestamp,
            self.hostname,
            self.tag,
            msg
        )
    }

    /// Send a log message to the syslog server
    fn write_message(&self, severity: i64, msg: &str) -> Result<(), io::Error> {
        let formatted_msg = self.format_message(severity, msg);
        
        match &self.conn {
            Connection::Udp(socket) => {
                let mut socket = socket.lock().unwrap();
                socket.send(formatted_msg.as_bytes())?;
            }
            Connection::Tcp(stream) => {
                let mut stream = stream.lock().unwrap();
                stream.write_all(formatted_msg.as_bytes())?;
                stream.write_all(b"\n")?;
            }
            Connection::None => {
                return Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to syslog"));
            }
        }
        
        Ok(())
    }
    
    /// Write an emergency log message
    pub fn emerg(&self, msg: &str) -> Result<(), io::Error> {
        self.write_message(EMERG, msg)
    }
    
    /// Write an alert log message
    pub fn alert(&self, msg: &str) -> Result<(), io::Error> {
        self.write_message(ALERT, msg)
    }
    
    /// Write a critical log message
    pub fn crit(&self, msg: &str) -> Result<(), io::Error> {
        self.write_message(CRIT, msg)
    }
    
    /// Write an error log message
    pub fn err(&self, msg: &str) -> Result<(), io::Error> {
        self.write_message(ERR, msg)
    }
    
    /// Write a warning log message
    pub fn warning(&self, msg: &str) -> Result<(), io::Error> {
        self.write_message(WARNING, msg)
    }
    
    /// Write a notice log message
    pub fn notice(&self, msg: &str) -> Result<(), io::Error> {
        self.write_message(NOTICE, msg)
    }
    
    /// Write an info log message
    pub fn info(&self, msg: &str) -> Result<(), io::Error> {
        self.write_message(INFO, msg)
    }
    
    /// Write a debug log message
    pub fn debug(&self, msg: &str) -> Result<(), io::Error> {
        self.write_message(DEBUG, msg)
    }
    
    /// Close the connection to the syslog server
    pub fn close(&self) -> Result<(), io::Error> {
        // No explicit closure needed since Drop handles it
        Ok(())
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let msg = String::from_utf8_lossy(buf).to_string();
        self.info(&msg)?;
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> io::Result<()> {
        // No buffer to flush
        Ok(())
    }
}

/// Connect to a syslog server over UDP or TCP
pub fn dial(network: &str, raddr: &str, priority: i64, tag: &str) -> Result<Writer, io::Error> {
    let conn = match network {
        "udp" => {
            let sock = UdpSocket::bind("0.0.0.0:0")?;
            sock.connect(raddr)?;
            Connection::Udp(Arc::new(Mutex::new(sock)))
        },
        "tcp" => {
            let stream = TcpStream::connect(raddr)?;
            Connection::Tcp(Arc::new(Mutex::new(stream)))
        },
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported network type")),
    };
    
    Ok(Writer::new(conn, priority, tag.to_string()))
}

/// Connect to a syslog server over TLS (mock implementation as cryptz isn't fully integrated)
pub fn dial_tls(network: &str, raddr: &str, priority: i64, tag: &str, _config: &Value) -> Result<Writer, io::Error> {
    // This is a simplified mock implementation since we haven't fully implemented the cryptz package
    // In a real implementation, we'd use the config to establish a TLS connection
    if network != "tcp" {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "TLS requires TCP network"));
    }
    
    // In a real implementation, we'd create a TLS connection here
    // For now, just create a regular TCP connection with a warning
    eprintln!("Warning: TLS not fully implemented, falling back to unencrypted TCP");
    dial("tcp", raddr, priority, tag)
}

/// Register all syslog_era functions in the environment
pub fn register_functions(env: &mut Environment) -> Result<(), String> {
    // Constants
    env.define_var("syslog_era.Kernel", Value::Integer(KERNEL))?;
    env.define_var("syslog_era.UserLevel", Value::Integer(USER_LEVEL))?;
    env.define_var("syslog_era.Mail", Value::Integer(MAIL))?;
    env.define_var("syslog_era.System", Value::Integer(SYSTEM))?;
    env.define_var("syslog_era.Auth", Value::Integer(AUTH))?;
    env.define_var("syslog_era.Syslogd", Value::Integer(SYSLOGD))?;
    env.define_var("syslog_era.Printer", Value::Integer(PRINTER))?;
    env.define_var("syslog_era.NetNews", Value::Integer(NET_NEWS))?;
    env.define_var("syslog_era.UUCP", Value::Integer(UUCP))?;
    env.define_var("syslog_era.Clock", Value::Integer(CLOCK))?;
    env.define_var("syslog_era.AuthPriv", Value::Integer(AUTH_PRIV))?;
    env.define_var("syslog_era.FTP", Value::Integer(FTP))?;
    env.define_var("syslog_era.NTP", Value::Integer(NTP))?;
    env.define_var("syslog_era.LogAudit", Value::Integer(LOG_AUDIT))?;
    env.define_var("syslog_era.LogAlert", Value::Integer(LOG_ALERT))?;
    env.define_var("syslog_era.Cron", Value::Integer(CRON))?;
    env.define_var("syslog_era.Local0", Value::Integer(LOCAL0))?;
    env.define_var("syslog_era.Local1", Value::Integer(LOCAL1))?;
    env.define_var("syslog_era.Local2", Value::Integer(LOCAL2))?;
    env.define_var("syslog_era.Local3", Value::Integer(LOCAL3))?;
    env.define_var("syslog_era.Local4", Value::Integer(LOCAL4))?;
    env.define_var("syslog_era.Local5", Value::Integer(LOCAL5))?;
    env.define_var("syslog_era.Local6", Value::Integer(LOCAL6))?;
    env.define_var("syslog_era.Local7", Value::Integer(LOCAL7))?;
    
    env.define_var("syslog_era.Emerg", Value::Integer(EMERG))?;
    env.define_var("syslog_era.Alert", Value::Integer(ALERT))?;
    env.define_var("syslog_era.Crit", Value::Integer(CRIT))?;
    env.define_var("syslog_era.Err", Value::Integer(ERR))?;
    env.define_var("syslog_era.Warning", Value::Integer(WARNING))?;
    env.define_var("syslog_era.Notice", Value::Integer(NOTICE))?;
    env.define_var("syslog_era.Info", Value::Integer(INFO))?;
    env.define_var("syslog_era.Debug", Value::Integer(DEBUG))?;
    
    // Functions
    env.define_native_fn("syslog_era.dial", syslog_dial)?;
    env.define_native_fn("syslog_era.dial_tls", syslog_dial_tls)?;

    Ok(())
}

/// Native function for syslog_era.dial
fn syslog_dial(args: &[Value]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("syslog_era.dial requires 4 arguments: network, addr, priority, tag".to_string());
    }

    let network = args[0].as_string().ok_or("network must be a string")?;
    let addr = args[1].as_string().ok_or("addr must be a string")?;
    let priority = args[2].as_integer().ok_or("priority must be an integer")?;
    let tag = args[3].as_string().ok_or("tag must be a string")?;

    match dial(network, addr, priority, tag) {
        Ok(writer) => {
            let writer_obj = WriterObject {
                writer: GcCell::new(writer),
            };
            Ok(Value::Object(Object::new(writer_obj)))
        },
        Err(e) => Err(format!("Error connecting to syslog: {}", e)),
    }
}

/// Native function for syslog_era.dial_tls
fn syslog_dial_tls(args: &[Value]) -> Result<Value, String> {
    if args.len() != 5 {
        return Err("syslog_era.dial_tls requires 5 arguments: network, addr, priority, tag, config".to_string());
    }

    let network = args[0].as_string().ok_or("network must be a string")?;
    let addr = args[1].as_string().ok_or("addr must be a string")?;
    let priority = args[2].as_integer().ok_or("priority must be an integer")?;
    let tag = args[3].as_string().ok_or("tag must be a string")?;
    let config = &args[4]; // Accept any value for config, we'll mock TLS anyway

    match dial_tls(network, addr, priority, tag, config) {
        Ok(writer) => {
            let writer_obj = WriterObject {
                writer: GcCell::new(writer),
            };
            Ok(Value::Object(Object::new(writer_obj)))
        },
        Err(e) => Err(format!("Error connecting to syslog: {}", e)),
    }
}

/// Object wrapper for the Writer struct
#[derive(Debug, Trace)]
pub struct WriterObject {
    writer: GcCell<Writer>,
}

impl ObjectImpl for WriterObject {
    fn get_type(&self) -> &'static str {
        "syslog_era.Writer"
    }

    fn get_property(&self, name: &str) -> Option<Value> {
        None
    }

    fn set_property(&self, _name: &str, _value: Value) -> Result<(), String> {
        Err(format!("Cannot set property on syslog_era.Writer"))
    }

    fn call_method(&self, name: &str, args: &[Value]) -> Result<Value, String> {
        match name {
            "emerg" => self.emerg(args),
            "alert" => self.alert(args),
            "crit" => self.crit(args),
            "err" => self.err(args),
            "warning" => self.warning(args),
            "notice" => self.notice(args),
            "info" => self.info(args),
            "debug" => self.debug(args),
            "write" => self.write(args),
            "close" => self.close(args),
            _ => Err(format!("Unknown method '{}' on Writer", name)),
        }
    }
}

impl WriterObject {
    fn emerg(&self, args: &[Value]) -> Result<Value, String> {
        self.write_message(EMERG, args)
    }

    fn alert(&self, args: &[Value]) -> Result<Value, String> {
        self.write_message(ALERT, args)
    }

    fn crit(&self, args: &[Value]) -> Result<Value, String> {
        self.write_message(CRIT, args)
    }

    fn err(&self, args: &[Value]) -> Result<Value, String> {
        self.write_message(ERR, args)
    }

    fn warning(&self, args: &[Value]) -> Result<Value, String> {
        self.write_message(WARNING, args)
    }

    fn notice(&self, args: &[Value]) -> Result<Value, String> {
        self.write_message(NOTICE, args)
    }

    fn info(&self, args: &[Value]) -> Result<Value, String> {
        self.write_message(INFO, args)
    }

    fn debug(&self, args: &[Value]) -> Result<Value, String> {
        self.write_message(DEBUG, args)
    }

    fn write(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("write requires 1 argument: data".to_string());
        }

        let data = args[0].as_string().ok_or("data must be a string")?;
        let mut writer = self.writer.borrow_mut();
        match writer.info(data) {
            Ok(_) => Ok(Value::Integer(data.len() as i64)),
            Err(e) => Err(format!("Error writing to syslog: {}", e)),
        }
    }

    fn close(&self, args: &[Value]) -> Result<Value, String> {
        if !args.is_empty() {
            return Err("close takes no arguments".to_string());
        }

        let writer = self.writer.borrow();
        match writer.close() {
            Ok(_) => Ok(Value::Nil),
            Err(e) => Err(format!("Error closing syslog writer: {}", e)),
        }
    }

    fn write_message(&self, severity: i64, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err(format!("method requires 1 argument: message").to_string());
        }

        let msg = args[0].as_string().ok_or("message must be a string")?;
        let writer = self.writer.borrow();
        match writer.write_message(severity, msg) {
            Ok(_) => Ok(Value::Nil),
            Err(e) => Err(format!("Error writing to syslog: {}", e)),
        }
    }
}